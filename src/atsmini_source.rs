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
use sdroxide_radio::{Complex32, ControlUpdate, IqSource, Result};
use sdroxide_types::Mode;
use sdroxide_types::atsmini::{self, FirmwareMode, Telemetry};

use crate::audio_cat_source::DropWatch;

/// The audio band the engine draws the panadapter over, in Hz either side of
/// the dial — the same figure the sound-card-only source uses. The radio's own
/// filter decides what is really there.
const AUDIO_BW_HZ: f64 = 4000.0;

/// The smallest dial move treated as an out-of-band change rather than
/// telemetry jitter. The coarsest step is 100 kHz and the finest a few Hz.
const OUT_OF_BAND_MIN_HZ: i64 = 5;

/// Commands the app sends the control thread.
enum Cmd {
    Freq(f64),
    Mode(Mode),
    /// One step command straight from a panel button (`V`/`v`, `A`/`a`,
    /// `W`/`w`, `S`/`s`).
    Raw(char),
    /// Select one of the firmware's bands by index (its own cycle, so this
    /// costs up to half a lap of `B`/`b`).
    BandIndex(usize),
    /// Ask the radio for its memory slots (`$`); the answer comes back as a
    /// `ControlUpdate::AtsMiniMemories`.
    DumpMemories,
    /// Write one memory slot (`#NN,…`).
    SetMemory(sdroxide_types::atsmini::AtsMiniMemory),
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
    /// Out-of-band changes the radio made itself (its own knob/mode), drained
    /// by [`IqSource::poll_control`].
    control_rx: std::sync::mpsc::Receiver<ControlUpdate>,
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
        let (control_tx, control_rx) = std::sync::mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let thread = std::thread::Builder::new()
            .name("atsmini-control".into())
            .spawn({
                let shared = Arc::clone(&shared);
                let stop = Arc::clone(&stop);
                let host = host.to_string();
                move || control_thread(host, port, shared, cmd_rx, control_tx, stop)
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
            control_rx,
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
        // The engine re-asserts the dial on unrelated state changes; do not
        // hand the radio the same tune again. Every `F` it receives writes NVS
        // and can glitch its audio, so repeats are not free.
        if (hz - self.center).abs() < 1.0 {
            return Ok(());
        }
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

    /// The audio warning *plus* the control link's own state, so a link that
    /// never opens is visible (a settings tab that cannot reach the radio used
    /// to look exactly like one that could).
    fn open_status(&self) -> Option<String> {
        let link = self.shared.lock().ok().and_then(|s| s.status.clone());
        match (self.status.clone(), link) {
            (Some(a), Some(b)) => Some(format!("{a}\n{b}")),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
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

    /// Panel step buttons, by key. The radio's own volume/AGC/bandwidth/step are
    /// relative on the wire (there is no "set volume to 12"), so the panel sends
    /// a direction and the firmware steps.
    fn set_device_setting(&mut self, key: &str, value: &str) -> Result<()> {
        // A band pick names an index into the firmware's own table; the control
        // thread steps its cycle to reach it.
        if key == "band-index" {
            if let Ok(i) = value.parse::<usize>() {
                let _ = self.cmd_tx.send(Cmd::BandIndex(i));
            }
            return Ok(());
        }
        // The memory table: `$` asks, `#NN,…` writes one slot. The dump is
        // collected in the control thread and comes back as an event.
        if key == "memories-dump" {
            let _ = self.cmd_tx.send(Cmd::DumpMemories);
            return Ok(());
        }
        if key == "memory-set" {
            if let Some(m) = sdroxide_types::atsmini::AtsMiniMemory::parse(value) {
                let _ = self.cmd_tx.send(Cmd::SetMemory(m));
            }
            return Ok(());
        }
        let up = value != "down";
        let step = match key {
            "volume" => atsmini::volume_step(up),
            "agc" => atsmini::agc_step(up),
            "bandwidth" => atsmini::bandwidth_step(up),
            "step" => atsmini::tuning_step(up),
            // Band and mode are the firmware's own cycle commands; stepping
            // them here is the direct route to what the main band/mode menu
            // reaches indirectly through the dial.
            "band" => atsmini::band_step(up),
            "mode" => atsmini::mode_step(up),
            _ => return Ok(()),
        };
        let _ = self.cmd_tx.send(Cmd::Raw(step));
        Ok(())
    }

    /// The radio's own knob and mode buttons reaching sdroxide. The control
    /// thread reports a dial or mode it changed without being asked; a change
    /// it did cause is suppressed there, so this never echoes our own tune.
    fn poll_control(&mut self) -> Vec<ControlUpdate> {
        let mut out = Vec::new();
        while let Ok(u) = self.control_rx.try_recv() {
            if let ControlUpdate::Freq(hz) = u {
                // Adopt it here too, so the engine's echo of this change is not
                // handed straight back as a tune command.
                self.center = hz;
            }
            out.push(u);
        }
        out
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

/// The `B`/`b` presses that move the firmware's band cycle from `from` to `to`
/// the shorter way round, as one string (a burst of steps, not a chat).
fn band_burst(from: usize, to: usize) -> String {
    let n = atsmini::BANDS.len();
    let up = (to + n - from) % n;
    let down = (from + n - to) % n;
    let (steps, key) =
        if up <= down { (up, atsmini::band_step(true)) } else { (down, atsmini::band_step(false)) };
    std::iter::repeat_n(key, steps).collect()
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
    control_tx: std::sync::mpsc::Sender<ControlUpdate>,
    stop: Arc<AtomicBool>,
) {
    let mut last_send = Instant::now() - Duration::from_secs(10);
    let mut pending: Vec<u8> = Vec::new();
    let mut buf = [0u8; 2048];
    let mut last_error: Option<String> = None;

    while !stop.load(Ordering::Relaxed) {
        let mut stream = match TcpStream::connect((host.as_str(), port)) {
            Ok(s) => s,
            Err(e) => {
                // Said out loud, once per distinct error: a host that does not
                // resolve (`atsmini.local` with no mDNS resolver) otherwise
                // leaves the link silently dead and the tab looking no
                // different from one that works.
                let msg = format!("ATS Mini {host}:{port} unreachable ({e})");
                if last_error.as_deref() != Some(msg.as_str()) {
                    tracing::warn!("{msg}");
                    last_error = Some(msg.clone());
                }
                set_status(&shared, Some(msg));
                std::thread::sleep(Duration::from_millis(1500));
                continue;
            }
        };
        last_error = None;
        let _ = stream.set_read_timeout(Some(Duration::from_millis(200)));
        let _ = stream.set_nodelay(true);
        let Ok(mut write) = stream.try_clone() else {
            std::thread::sleep(Duration::from_millis(1000));
            continue;
        };
        set_status(&shared, None);
        tracing::info!("ATS Mini: connected to {host}:{port}");
        let _ = write.write_all(&[atsmini::monitor_toggle() as u8]);
        // Pending work is per connection: a fresh session starts with none, and
        // no dial/mode baseline either — the first telemetry sets one.
        pending.clear();
        let mut target_hz: Option<u64> = None;
        let mut sent_for: Option<u64> = None;
        let mut target_mode: Option<FirmwareMode> = None;
        let mut saw_error = false;
        let mut last_dial: Option<i64> = None;
        let mut last_mode: Option<FirmwareMode> = None;
        let mut commanded_hz: Option<i64> = None;
        let mut commanded_mode: Option<FirmwareMode> = None;
        let mut pending_band: Option<usize> = None;
        // The frequency whose band we have already put the radio in, so the
        // band is ensured once per tune rather than per F.
        let mut band_ensured_for: Option<u64> = None;
        // A dump in progress: when it started and the slots gathered so far.
        let mut collecting: Option<(Instant, Vec<sdroxide_types::atsmini::AtsMiniMemory>)> = None;

        'session: while !stop.load(Ordering::Relaxed) {
            // Commands from the app.
            loop {
                match cmd_rx.try_recv() {
                    Ok(Cmd::Freq(hz)) => {
                        target_hz = Some(hz.max(0.0) as u64);
                        sent_for = None;
                        saw_error = false;
                        band_ensured_for = None;
                    }
                    Ok(Cmd::Mode(m)) => target_mode = firmware_mode(m),
                    Ok(Cmd::BandIndex(i)) => pending_band = Some(i),
                    Ok(Cmd::DumpMemories) => {
                        let _ = write.write_all(&[atsmini::dump_memories() as u8]);
                        collecting = Some((Instant::now(), Vec::new()));
                    }
                    Ok(Cmd::SetMemory(m)) => {
                        let _ = write.write_all(m.command().as_bytes());
                    }
                    // A panel step button: send it now, no confirmation loop.
                    Ok(Cmd::Raw(c)) => {
                        let _ = write.write_all(&[c as u8]);
                    }
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
                // A band pick from the popup: the firmware has no "go to band
                // N", so step its own cycle the shorter way round.
                if let Some(idx) = pending_band
                    && let Some(t) = shared.lock().ok().and_then(|s| s.telemetry.clone())
                    && let Some(cur) = atsmini::band_index(&t.band, t.dial_hz())
                {
                    if cur != idx {
                        out = Some(band_burst(cur, idx));
                    }
                    pending_band = None;
                }
                if out.is_some() {
                    // Band step is out; nothing else this tick.
                } else if let Some(hz) = target_hz {
                    let cur = shared.lock().ok().and_then(|s| {
                        s.telemetry.as_ref().and_then(|t| atsmini::band_index(&t.band, t.dial_hz()))
                    });
                    if band_ensured_for != Some(hz) {
                        // Put the radio in a band that holds the frequency before
                        // offering the `F`: `F` is band-locked, and a scroll that
                        // cycles bands one error at a time never catches the
                        // dial. `ALL` (150 kHz–30 MHz) or `VHF` covers the
                        // overlap-free case; a band already holding it is kept.
                        if let Some(cur) = cur {
                            if let Some(want) = atsmini::band_for_tuning(hz as f64, Some(cur))
                                && want != cur
                            {
                                out = Some(band_burst(cur, want));
                            }
                            band_ensured_for = Some(hz);
                        }
                        // No telemetry yet: wait for a band to appear.
                    } else if saw_error && sent_for == Some(hz) {
                        // The band it landed in still refuses it (a user-edited
                        // band table, say): step on and offer it again.
                        saw_error = false;
                        sent_for = None;
                        out = Some(band_burst(
                            cur.unwrap_or(0),
                            (cur.unwrap_or(0) + 1) % atsmini::BANDS.len(),
                        ));
                    } else if sent_for != Some(hz) {
                        // Send the tune once; its acceptance is judged by the
                        // absence of an error by the next telemetry, not by the
                        // dial matching — the radio clamps to its own step.
                        sent_for = Some(hz);
                        out = Some(atsmini::set_frequency(hz));
                    }
                    // else: the `F` is out and its answer has not arrived.
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
                    tracing::debug!(cmd = %bytes.trim_end(), "ATS Mini > command");
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
                        if let Some(t) = Telemetry::parse(&line) {
                            let dial = t.dial_hz().round() as i64;
                            // A tune of ours that the radio did not reject has
                            // landed: it answers an out-of-band `F` with an error
                            // line straight away, so the first telemetry after an
                            // accepted one is the confirmation. Judged on the
                            // error, not on the dial matching, because the radio
                            // clamps to its own step (1.001 MHz on a 9 kHz step
                            // becomes 1.000, which no dial comparison settles).
                            if target_hz.is_some() && sent_for.is_some() && !saw_error {
                                target_hz = None;
                                commanded_hz = Some(dial);
                            }
                            // A dial/mode the radio moved on its own reaches the
                            // engine; one we commanded is suppressed, or the two
                            // ends would echo each other.
                            if let Some(prev) = last_dial
                                && (dial - prev).abs() >= OUT_OF_BAND_MIN_HZ
                            {
                                // While a tune is in flight every dial change is
                                // ours (the band cycle steps through frequencies);
                                // afterwards compare against where we left it with a
                                // *tight* tolerance, not the convergence one, or a
                                // small knob step is mistaken for our own command
                                // and swallowed.
                                let ours = target_hz.is_some()
                                    || commanded_hz
                                        .is_some_and(|c| (dial - c).abs() <= OUT_OF_BAND_MIN_HZ);
                                if !ours {
                                    tracing::debug!(
                                        hz = dial,
                                        "ATS Mini: radio dial moved out-of-band"
                                    );
                                    let _ = control_tx.send(ControlUpdate::Freq(dial as f64));
                                }
                            }
                            if let Some(prev) = last_mode
                                && t.mode != prev
                            {
                                let ours = target_mode.is_some() || commanded_mode == Some(t.mode);
                                if !ours {
                                    tracing::debug!(
                                        mode = t.mode.as_str(),
                                        "ATS Mini: radio mode moved out-of-band"
                                    );
                                    let _ =
                                        control_tx.send(ControlUpdate::Mode(t.mode.as_rx_mode()));
                                }
                            }
                            if target_mode == Some(t.mode) {
                                commanded_mode = Some(t.mode);
                            }
                            last_dial = Some(dial);
                            last_mode = Some(t.mode);
                            if let Ok(mut s) = shared.lock() {
                                s.telemetry = Some(t);
                            }
                        } else if let Some((_, mems)) = collecting.as_mut()
                            && let Some(m) = sdroxide_types::atsmini::AtsMiniMemory::parse(&line)
                        {
                            mems.push(m);
                        } else if line.to_ascii_lowercase().contains("error") {
                            saw_error = true;
                        }
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {}
                Err(_) => break 'session,
            }

            // The dump arrives in one burst; give the lines a moment to clear
            // the socket, then hand the table over. There is no terminator to
            // wait for — the firmware just stops printing.
            if collecting
                .as_ref()
                .is_some_and(|(start, _)| start.elapsed() > Duration::from_millis(800))
            {
                let (_, mems) = collecting.take().expect("checked just above");
                tracing::debug!(slots = mems.len(), "ATS Mini: memory dump");
                let _ = control_tx.send(ControlUpdate::AtsMiniMemories(mems));
            }
        }
        set_status(&shared, Some("ATS Mini disconnected".into()));
        tracing::warn!("ATS Mini: control link {host}:{port} closed; reconnecting");
        std::thread::sleep(Duration::from_millis(1000));
    }
}
