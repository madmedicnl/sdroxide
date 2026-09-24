//! An [`IqSource`] for the ATS Mini — an ESP32-S3 + Si4732 pocket receiver.
//!
//! It is receive-only and hands over **already-demodulated audio** from the
//! radio's headphone jack into a host sound card (the Si4732 demodulates in
//! hardware; there is no I/Q). Control — band, frequency, mode, volume — goes
//! over the firmware's "ad hoc" protocol on a TCP socket
//! ([`sdroxide_types::atsmini`]). So this is the audio half of
//! [`crate::audio_cat_source::AudioCatSource`] with a bespoke TCP control link
//! instead of a CAT serial port. See `docs/ats-mini-handover.md`.
//!
//! The firmware rejects a frequency outside the *current* band and offers no
//! direct band-select, so a tune tries `F<hz>`, cycles `B` on the out-of-range
//! error, and confirms from the 500 ms telemetry the radio streams.

use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_radio::rtrb;
use sdroxide_radio::{Complex32, IqSource, Result};
use sdroxide_types::Mode;
use sdroxide_types::atsmini::{self, FirmwareMode, Telemetry};

use crate::audio_cat_source::DropWatch;

/// The audio band the engine draws the panadapter over, in Hz either side of
/// the dial — the same figure the sound-card-only source uses. The radio's own
/// filter decides what is really there.
const AUDIO_BW_HZ: f64 = 4000.0;

/// A frequency closer than this to the requested one counts as tuned (the
/// telemetry is quantised to the step in use).
const TUNED_TOLERANCE_HZ: f64 = 500.0;

/// Commands the app sends the control thread.
enum Cmd {
    Freq(f64),
    Mode(Mode),
}

/// State the control thread publishes for the source and UI to read.
#[derive(Default)]
struct Shared {
    telemetry: Option<Telemetry>,
    /// A human-readable connection state, for the log/open banner.
    status: Option<String>,
}

/// ATS Mini receive source: sound-card audio plus the TCP control link.
pub struct AtsMiniSource {
    in_stream: Option<sdroxide_audio::AudioInput>,
    in_consumer: rtrb::Consumer<f32>,
    in_rate: f64,
    drops: DropWatch,

    shared: Arc<Mutex<Shared>>,
    cmd_tx: std::sync::mpsc::Sender<Cmd>,
    stop: Arc<AtomicBool>,
    _thread: Option<std::thread::JoinHandle<()>>,

    center: f64,
    label: String,
    status: Option<String>,
    released: bool,
}

impl AtsMiniSource {
    /// Open the radio. `audio_in` is the cpal name of the card the radio's
    /// audio reaches the PC on; `center_hz` is the initial dial.
    pub fn open(
        host: &str,
        port: u16,
        audio_in: Option<&str>,
        center_hz: f64,
    ) -> anyhow::Result<Self> {
        if audio_in.is_none() {
            tracing::warn!(
                "no sound card chosen for the ATS Mini — falling back to the system default, \
                 which is not this radio unless it happens to be the default. Pick its input \
                 under Settings → Radio."
            );
        }

        let opened = sdroxide_audio::start_input_buffered(audio_in, 48_000);
        let dev_label = audio_in.unwrap_or("system default");
        let silent = || {
            let (_p, c) = rtrb::RingBuffer::<f32>::new(1);
            c
        };
        let (in_stream, in_consumer, in_rate, status) = match opened {
            Ok((s, c)) => {
                let rate = s.sample_rate;
                (Some(s), c, rate, None)
            }
            Err(e) => {
                let msg = format!(
                    "ATS Mini input “{dev_label}” is unavailable ({e}) — no receive audio. \
                     The device may be in use by another program, unplugged, or held by the \
                     system audio server."
                );
                tracing::warn!("{msg}");
                (None, silent(), 48_000.0, Some(msg))
            }
        };

        let shared = Arc::new(Mutex::new(Shared::default()));
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let thread = std::thread::Builder::new()
            .name("atsmini-control".into())
            .spawn({
                let shared = Arc::clone(&shared);
                let stop = Arc::clone(&stop);
                let host = host.to_string();
                move || control_thread(host, port, shared, cmd_rx, stop)
            })
            .ok();

        if in_stream.is_some() {
            // Reaching the radio is the thread's job; the source came up.
            tracing::info!("ATS Mini control link {host}:{port}");
        }

        Ok(AtsMiniSource {
            in_stream,
            in_consumer,
            in_rate,
            drops: DropWatch::started(Instant::now()),
            shared,
            cmd_tx,
            stop,
            _thread: thread,
            center: center_hz,
            label: format!("ATS Mini at {host}:{port} (audio on {dev_label})"),
            status,
            released: false,
        })
    }

    /// The latest telemetry, for the control panel and a live S-meter readout.
    /// Not called yet — the settings tab is host/port/audio only so far.
    #[allow(dead_code)]
    pub fn telemetry(&self) -> Option<Telemetry> {
        self.shared.lock().ok().and_then(|s| s.telemetry.clone())
    }

    fn check_dropped(&mut self) {
        let Some(total) = self.in_stream.as_ref().map(|s| s.dropped_frames()) else { return };
        let Some((lost, window)) = self.drops.check(Instant::now(), total) else { return };
        tracing::warn!(
            "ATS Mini audio: {lost} capture frames dropped in the last {:.1} s ({:.1} ms) — \
             this machine is not emptying a {:.0} Hz card fast enough.",
            window.as_secs_f64(),
            lost as f64 * 1000.0 / self.in_rate,
            self.in_rate,
        );
    }
}

impl Drop for AtsMiniSource {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl IqSource for AtsMiniSource {
    fn sample_rate(&self) -> f64 {
        self.in_rate
    }
    fn center_hz(&self) -> f64 {
        self.center
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        self.center = hz;
        let _ = self.cmd_tx.send(Cmd::Freq(hz));
        Ok(())
    }

    /// The radio's dial *is* the frequency we command, so it counts as the
    /// dial — unlike the sound-card-only source, which only relabels.
    fn center_is_dial(&self) -> bool {
        true
    }

    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        self.check_dropped();
        let mut n = 0;
        while n < buf.len() {
            match self.in_consumer.pop() {
                Ok(s) => {
                    buf[n] = Complex32::new(s, 0.0);
                    n += 1;
                }
                Err(_) => break,
            }
        }
        if n == 0 {
            std::thread::sleep(Duration::from_millis(5));
        }
        Ok(n)
    }

    fn read_available(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        self.check_dropped();
        let mut n = 0;
        while n < buf.len() {
            let Ok(s) = self.in_consumer.pop() else { break };
            buf[n] = Complex32::new(s, 0.0);
            n += 1;
        }
        Ok(n)
    }

    fn describe(&self) -> String {
        self.label.clone()
    }

    fn open_status(&self) -> Option<String> {
        self.status.clone()
    }

    fn needs_reopen(&self) -> bool {
        self.released
    }

    fn release(&mut self) {
        if self.released {
            return;
        }
        self.in_stream = None;
        self.stop.store(true, Ordering::Relaxed);
        self.released = true;
    }

    fn display_bandwidth(&self) -> Option<f64> {
        Some(AUDIO_BW_HZ)
    }

    /// The operator's mode is commanded to the radio: the Si4732 demodulates
    /// in hardware, so the mode here decides what the audio *is*.
    fn commands_rx_mode(&self) -> bool {
        true
    }

    fn set_control_mode(&mut self, mode: Mode) -> Result<()> {
        let _ = self.cmd_tx.send(Cmd::Mode(mode));
        Ok(())
    }

    /// The only strength measurement there is: the audio arrives after the
    /// radio's own AGC, so the engine cannot measure the band itself. The
    /// firmware's RSSI is dBµV into 50 Ω, i.e. dBm = dBµV − 107.
    fn rx_signal_dbm(&mut self) -> Option<f32> {
        self.shared
            .lock()
            .ok()
            .and_then(|s| s.telemetry.as_ref().map(|t| f32::from(t.rssi_dbuv) - 107.0))
    }

    fn discard_pending_rx(&mut self) {
        while self.in_consumer.pop().is_ok() {}
        if let Some(total) = self.in_stream.as_ref().map(|s| s.dropped_frames()) {
            self.drops.rebase(Instant::now(), total);
        }
    }
}

/// The three-mode cycle the firmware steps through on HF: `LSB → USB → AM`.
const MODE_CYCLE: [FirmwareMode; 3] = [FirmwareMode::Lsb, FirmwareMode::Usb, FirmwareMode::Am];

/// Map an sdroxide receive mode to the firmware's demodulator, or `None` when
/// it has no equivalent (the caller then leaves the radio's mode alone).
fn firmware_mode(mode: Mode) -> Option<FirmwareMode> {
    match mode {
        Mode::Am => Some(FirmwareMode::Am),
        Mode::Usb => Some(FirmwareMode::Usb),
        Mode::Lsb => Some(FirmwareMode::Lsb),
        Mode::Nfm | Mode::Wfm => Some(FirmwareMode::Fm),
        _ => None,
    }
}

/// The single `M`/`m` press that moves `current` toward `target` fastest, if
/// both are on the three-mode cycle.
fn mode_step_toward(current: FirmwareMode, target: FirmwareMode) -> Option<char> {
    let ci = MODE_CYCLE.iter().position(|&m| m == current)?;
    let ti = MODE_CYCLE.iter().position(|&m| m == target)?;
    let up = (ti + 3 - ci) % 3;
    let down = (ci + 3 - ti) % 3;
    Some(atsmini::mode_step(up <= down))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The firmware cycles `LSB → USB → AM` (up); a mode change takes the
    /// shorter way round, and FM is off the cycle so it is left alone.
    #[test]
    fn mode_steps_take_the_shorter_way_round() {
        assert_eq!(mode_step_toward(FirmwareMode::Am, FirmwareMode::Usb), Some('m'));
        assert_eq!(mode_step_toward(FirmwareMode::Am, FirmwareMode::Lsb), Some('M'));
        assert_eq!(mode_step_toward(FirmwareMode::Usb, FirmwareMode::Lsb), Some('m'));
        assert_eq!(mode_step_toward(FirmwareMode::Lsb, FirmwareMode::Usb), Some('M'));
        assert_eq!(mode_step_toward(FirmwareMode::Am, FirmwareMode::Am), Some('M'));
        assert_eq!(mode_step_toward(FirmwareMode::Fm, FirmwareMode::Am), None);
    }

    #[test]
    fn sdroxide_modes_map_to_the_radio_demodulator() {
        assert_eq!(firmware_mode(Mode::Am), Some(FirmwareMode::Am));
        assert_eq!(firmware_mode(Mode::Usb), Some(FirmwareMode::Usb));
        assert_eq!(firmware_mode(Mode::Lsb), Some(FirmwareMode::Lsb));
        assert_eq!(firmware_mode(Mode::Wfm), Some(FirmwareMode::Fm));
        assert_eq!(firmware_mode(Mode::Nfm), Some(FirmwareMode::Fm));
        // A mode the radio has no demodulator for: leave its mode alone.
        assert_eq!(firmware_mode(Mode::Cw), None);
    }
}

/// One line off the control socket: telemetry, or an error to act on.
fn handle_line(shared: &Mutex<Shared>, line: &str, saw_error: &mut bool) {
    if let Some(t) = Telemetry::parse(line) {
        if let Ok(mut s) = shared.lock() {
            s.telemetry = Some(t);
        }
    } else if line.to_ascii_lowercase().contains("error") {
        *saw_error = true;
    }
}

fn set_status(shared: &Mutex<Shared>, status: Option<String>) {
    if let Ok(mut s) = shared.lock() {
        s.status = status;
    }
}

/// The TCP control thread: connect, turn the monitor on, drive tunes and mode
/// changes, parse telemetry, reconnect forever.
fn control_thread(
    host: String,
    port: u16,
    shared: Arc<Mutex<Shared>>,
    cmd_rx: Receiver<Cmd>,
    stop: Arc<AtomicBool>,
) {
    let mut last_send = Instant::now() - Duration::from_secs(10);
    let mut pending: Vec<u8> = Vec::new();
    let mut buf = [0u8; 2048];

    while !stop.load(Ordering::Relaxed) {
        let mut stream = match TcpStream::connect((host.as_str(), port)) {
            Ok(s) => s,
            Err(e) => {
                set_status(&shared, Some(format!("ATS Mini {host}:{port} unreachable ({e})")));
                std::thread::sleep(Duration::from_millis(1500));
                continue;
            }
        };
        let _ = stream.set_read_timeout(Some(Duration::from_millis(200)));
        let _ = stream.set_nodelay(true);
        let Ok(mut write) = stream.try_clone() else {
            std::thread::sleep(Duration::from_millis(1000));
            continue;
        };
        set_status(&shared, None);
        let _ = write.write_all(&[atsmini::monitor_toggle() as u8]);
        // Pending work is per connection: a fresh session starts with none.
        pending.clear();
        let mut target_hz: Option<u64> = None;
        let mut sent_for: Option<u64> = None;
        let mut target_mode: Option<FirmwareMode> = None;
        let mut saw_error = false;

        'session: while !stop.load(Ordering::Relaxed) {
            // Commands from the app.
            loop {
                match cmd_rx.try_recv() {
                    Ok(Cmd::Freq(hz)) => {
                        target_hz = Some(hz.max(0.0) as u64);
                        sent_for = None;
                    }
                    Ok(Cmd::Mode(m)) => target_mode = firmware_mode(m),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        stop.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            }

            // Drive a pending tune / mode, paced so the radio can answer.
            if last_send.elapsed() > Duration::from_millis(150) {
                let mut out: Option<String> = None;
                if let Some(hz) = target_hz {
                    let cur = shared
                        .lock()
                        .ok()
                        .and_then(|s| s.telemetry.as_ref().map(Telemetry::dial_hz));
                    match cur {
                        Some(cur) if (cur - hz as f64).abs() <= TUNED_TOLERANCE_HZ => {
                            target_hz = None;
                        }
                        // Out-of-band error seen for this target: step the band
                        // and try again. `sent_for` tells the two apart, else a
                        // stale error would cycle forever.
                        _ if sent_for == Some(hz) && saw_error => {
                            saw_error = false;
                            sent_for = None;
                            out = Some(atsmini::band_step(true).to_string());
                        }
                        Some(_) | None => {
                            sent_for = Some(hz);
                            out = Some(atsmini::set_frequency(hz));
                        }
                    }
                } else if let Some(tm) = target_mode {
                    let cur = shared.lock().ok().and_then(|s| s.telemetry.as_ref().map(|t| t.mode));
                    match cur {
                        Some(cur) if cur == tm => target_mode = None,
                        Some(cur) => match mode_step_toward(cur, tm) {
                            Some(step) => out = Some(step.to_string()),
                            None => target_mode = None, // FM: no cycle to step
                        },
                        None => {}
                    }
                }
                if let Some(bytes) = out {
                    if write.write_all(bytes.as_bytes()).is_err() {
                        break 'session;
                    }
                    last_send = Instant::now();
                }
            }

            match stream.read(&mut buf) {
                Ok(0) => break 'session,
                Ok(n) => {
                    pending.extend_from_slice(&buf[..n]);
                    while let Some(pos) = pending.iter().position(|&b| b == b'\n') {
                        let line = String::from_utf8_lossy(&pending[..pos]).into_owned();
                        pending.drain(..=pos);
                        handle_line(&shared, &line, &mut saw_error);
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {}
                Err(_) => break 'session,
            }
        }
        set_status(&shared, Some("ATS Mini disconnected".into()));
        std::thread::sleep(Duration::from_millis(1000));
    }
}
