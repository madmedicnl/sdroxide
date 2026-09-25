//! A CW paddle read straight from the kernel's input layer (Linux, native).
//!
//! Some USB keyer boxes have no keyer in them: they report the two paddle
//! contacts and nothing else, often as the buttons of an otherwise useless
//! "mouse". This opens one of the kernel's input devices, takes it exclusively
//! (`EVIOCGRAB`, so the contacts cannot also land as mouse clicks in whatever
//! window has focus), runs [`sdroxide_dsp::CwKeyer`] on the contacts, and plays
//! the sidetone locally. It never keys a radio.
//!
//! Native and Linux only: it is raw evdev, and the browser and the other
//! platforms have no equivalent. The keyer itself is portable and lives in the
//! DSP crate; this is only the part that touches a device.

use std::fs::File;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use sdroxide_audio::start_output;
use sdroxide_dsp::{CwKeyer, IambicMode};

/// `_IOW('E', 0x90, int)` — take the device exclusively.
const EVIOCGRAB: libc::c_ulong = 0x4004_4590;
const EV_KEY: u16 = 1;
const BTN_LEFT: u16 = 0x110;
const BTN_RIGHT: u16 = 0x111;
const BTN_MIDDLE: u16 = 0x112;

/// Every candidate paddle device: the kernel's by-id names for the button half
/// of a composite HID keyer. The by-id name is used rather than an event number
/// because it is stable across replug.
pub fn devices() -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Ok(rd) = std::fs::read_dir("/dev/input/by-id") {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.ends_with("-event-mouse") || name.ends_with("-event-kbd") {
                v.push(e.path());
            }
        }
    }
    v.sort();
    v
}

/// The device to open by default: the one whose name says "key", so a real
/// mouse is never grabbed by accident. `None` leaves it to the operator.
pub fn default_device() -> Option<PathBuf> {
    devices().into_iter().find(|p| {
        p.file_name().map(|n| n.to_string_lossy().to_lowercase().contains("key")).unwrap_or(false)
    })
}

/// How the two contacts map onto dit and dah, and how they are keyed.
#[derive(Clone, Copy)]
pub struct KeySetup {
    pub wpm: f32,
    pub pitch_hz: f32,
    pub reverse: bool,
    pub mode: IambicMode,
}

impl Default for KeySetup {
    fn default() -> Self {
        KeySetup { wpm: 20.0, pitch_hz: 700.0, reverse: false, mode: IambicMode::B }
    }
}

struct Shared {
    text: Mutex<String>,
    key_down: AtomicBool,
    dit: AtomicBool,
    dah: AtomicBool,
    marks: AtomicU64,
    error: Mutex<Option<String>>,
}

/// A running paddle source: a keyer thread, a shared decode, and the tone.
pub struct CwKeySource {
    shared: Arc<Shared>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl CwKeySource {
    pub fn start(path: &Path, setup: KeySetup) -> Result<Self, String> {
        let shared = Arc::new(Shared {
            text: Mutex::new(String::new()),
            key_down: AtomicBool::new(false),
            dit: AtomicBool::new(false),
            dah: AtomicBool::new(false),
            marks: AtomicU64::new(0),
            error: Mutex::new(None),
        });
        let stop = Arc::new(AtomicBool::new(false));
        let (shared2, stop2) = (Arc::clone(&shared), Arc::clone(&stop));
        let path2 = path.to_path_buf();
        let thread = std::thread::Builder::new()
            .name("cw-key".into())
            .spawn(move || run(path2, setup, &shared2, &stop2))
            .map_err(|e| format!("could not start the CW key thread: {e}"))?;
        Ok(CwKeySource { shared, stop, thread: Some(thread) })
    }

    /// Characters decoded since the last call, in order.
    pub fn take_text(&self) -> String {
        std::mem::take(&mut *self.shared.text.lock().unwrap())
    }

    pub fn contacts(&self) -> (bool, bool) {
        (self.shared.dit.load(Ordering::Relaxed), self.shared.dah.load(Ordering::Relaxed))
    }

    pub fn marks(&self) -> u64 {
        self.shared.marks.load(Ordering::Relaxed)
    }

    /// A failure the thread hit after starting (device open, audio), said in the
    /// pane rather than swallowed.
    pub fn error(&self) -> Option<String> {
        self.shared.error.lock().unwrap().clone()
    }
}

impl Drop for CwKeySource {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn set_error(shared: &Shared, msg: String) {
    *shared.error.lock().unwrap() = Some(msg);
}

fn run(path: PathBuf, setup: KeySetup, shared: &Shared, stop: &AtomicBool) {
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            set_error(shared, format!("cannot open {}: {e}", path.display()));
            return;
        }
    };
    let fd = file.as_raw_fd();
    let grabbed = unsafe { libc::ioctl(fd, EVIOCGRAB, 1) } == 0;
    if !grabbed {
        // Not fatal: the contacts are still readable, they just also arrive as
        // clicks. Say so instead of pretending the grab worked.
        set_error(
            shared,
            "could not take the device exclusively — its buttons may also click \
             in other windows (is another program using it?)"
                .into(),
        );
    }
    unsafe { libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK) };

    let (out, mut ring) = match start_output(None, 48_000) {
        Ok(v) => v,
        Err(e) => {
            set_error(shared, format!("no audio output for the sidetone: {e}"));
            unsafe { libc::ioctl(fd, EVIOCGRAB, 0) };
            return;
        }
    };
    let rate = out.sample_rate;
    let capacity = out.sample_rate as usize * 2;
    let inc = std::f64::consts::TAU * setup.pitch_hz as f64 / rate;

    let mut keyer = CwKeyer::new(setup.wpm);
    keyer.set_mode(setup.mode);
    let (mut dit, mut dah) = (false, false);
    let mut phase = 0.0f64;
    let start = Instant::now();
    let mut generated: u64 = 0;
    let mut buf = [0u8; 24 * 256];
    let ev_size = std::mem::size_of::<libc::input_event>();

    while !stop.load(Ordering::Relaxed) {
        loop {
            match (&file).read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    for chunk in buf[..n].chunks_exact(ev_size) {
                        let ev = unsafe {
                            std::ptr::read_unaligned(chunk.as_ptr() as *const libc::input_event)
                        };
                        if ev.type_ != EV_KEY {
                            continue;
                        }
                        let down = ev.value != 0;
                        match ev.code {
                            BTN_LEFT => dit = down,
                            BTN_RIGHT => dah = down,
                            BTN_MIDDLE => {}
                            _ => {}
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        let now = start.elapsed().as_secs_f64();
        let (d, a) = if setup.reverse { (dah, dit) } else { (dit, dah) };
        let down = keyer.poll(now, d, a);
        shared.key_down.store(down, Ordering::Relaxed);
        shared.dit.store(dit, Ordering::Relaxed);
        shared.dah.store(dah, Ordering::Relaxed);
        shared.marks.store(keyer.marks(), Ordering::Relaxed);
        let text = keyer.take_text();
        if !text.is_empty() {
            shared.text.lock().unwrap().push_str(&text);
        }

        // Fill the device one millisecond at a time, from the wall clock so it
        // cannot drift; a full ring is skipped rather than blocked on.
        let want = (now * rate) as u64;
        let mut to_gen = want.saturating_sub(generated);
        let cap = (rate / 50.0) as u64; // never chase more than 20 ms
        if to_gen > cap {
            to_gen = cap;
            generated = want - cap;
        }
        while to_gen > 0 {
            if ring.slots() + 2 > capacity {
                break;
            }
            let s = if down { (phase.sin() * 0.3) as f32 } else { 0.0 };
            phase += inc;
            if phase > std::f64::consts::TAU {
                phase -= std::f64::consts::TAU;
            }
            let _ = ring.push(s);
            let _ = ring.push(s);
            to_gen -= 1;
            generated += 1;
        }

        std::thread::sleep(Duration::from_millis(1));
    }
    unsafe { libc::ioctl(fd, EVIOCGRAB, 0) };
}
