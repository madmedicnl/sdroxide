//! `JttyController` — the JTTY asynchronous text receiver, receive only.
//!
//! JTTY is not a slotted mode: a transmission can start at any instant, so
//! there is no T/R period and no frame boundary to lock to. Like
//! [`crate::UvPacketController`] and the ACARS/DSC controllers, this holds a
//! rolling window of audio and re-scans it as it advances, then keeps a
//! short-lived set of decoded-message hashes so an overlapping window does not
//! file the same message twice.
//!
//! # Messages, not frames
//!
//! A JTTY **message** is one or more ~1.888 s **frames**; the last carries the
//! end-of-message flag. The receiver searches the window for a frame, decodes
//! it, and appends it to a message until EOM or until the run breaks. What the
//! panel shows is that accumulated text.
//!
//! Receive only in this build. Transmit needs the source-grammar packer and the
//! asynchronous keying path, which is the next stage — as FSK441 shipped
//! receive before transmit.

use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::sync::mpsc::{Receiver, Sender};
use std::time::SystemTime;

use sdroxide_dsp::jtty::{self, JTTY_RATE};
use sdroxide_dsp::MonoResampler;
use sdroxide_types::{DigiConfig, DigiStatus, JTTY_MESSAGE_MAX, JttyMessage, JttyStatus, Mode, QsoStep};

use crate::DigiEngine;
use crate::controller::DigiAction;

/// Audio window scanned, in seconds. Long enough for a several-frame message.
const WINDOW_S: f64 = 12.0;
/// How much new audio accumulates before the window is scanned again.
const SCAN_STEP_S: f64 = 0.5;
/// How long a decoded-message hash suppresses a repeat, in seconds.
const DEDUP_TTL_S: u64 = 15;

/// One window handed to the decode worker.
struct DecodeJob {
    audio: Vec<f32>,
    at: i64,
}

pub struct JttyController {
    cfg: DigiConfig,
    resampler: Option<MonoResampler>,
    buf: Vec<f32>,
    tap_scratch: Vec<f32>,
    level: f32,
    messages: Vec<JttyMessage>,
    total: u64,
    seen: VecDeque<(u64, SystemTime)>,
    since_scan: usize,

    job_tx: Sender<DecodeJob>,
    res_rx: Receiver<Vec<JttyMessage>>,
    _worker: std::thread::JoinHandle<()>,
    pending: bool,
    status_dirty: bool,
}

impl JttyController {
    pub fn new(cfg: DigiConfig, tap_rate: f64) -> Self {
        let (job_tx, job_rx) = std::sync::mpsc::channel::<DecodeJob>();
        let (res_tx, res_rx) = std::sync::mpsc::channel::<Vec<JttyMessage>>();
        let worker = std::thread::Builder::new()
            .name("sdroxide-jtty-decode".into())
            .spawn(move || {
                while let Ok(job) = job_rx.recv() {
                    let msgs = decode_window(&job.audio, job.at);
                    if res_tx.send(msgs).is_err() {
                        break;
                    }
                }
            })
            .expect("spawn jtty decode worker");

        JttyController {
            cfg,
            resampler: MonoResampler::new(tap_rate, JTTY_RATE),
            buf: Vec::new(),
            tap_scratch: Vec::new(),
            level: 0.0,
            messages: Vec::new(),
            total: 0,
            seen: VecDeque::new(),
            since_scan: 0,
            job_tx,
            res_rx,
            _worker: worker,
            pending: false,
            status_dirty: true,
        }
    }

    fn window_samples() -> usize {
        (WINDOW_S * JTTY_RATE) as usize
    }

    fn step_samples() -> usize {
        (SCAN_STEP_S * JTTY_RATE) as usize
    }

    fn seen_recently(&self, hash: u64, now: SystemTime) -> bool {
        self.seen.iter().any(|&(h, t)| {
            h == hash
                && now.duration_since(t).map(|d| d.as_secs() < DEDUP_TTL_S).unwrap_or(false)
        })
    }

    fn prune_seen(&mut self, now: SystemTime) {
        while let Some(&(_, t)) = self.seen.front() {
            let expired =
                now.duration_since(t).map(|d| d.as_secs() >= DEDUP_TTL_S).unwrap_or(true);
            if expired {
                self.seen.pop_front();
            } else {
                break;
            }
        }
    }

    fn digi_status(&self) -> DigiStatus {
        let mut s = DigiStatus::idle(self.cfg.clone());
        s.mode = Mode::Jtty;
        s.step = QsoStep::Idle;
        s.jtty = Some(JttyStatus {
            level: self.level,
            messages: self.messages.clone(),
            total: self.total,
        });
        s
    }
}

/// Hash over the identifying content of a message, for the recent-seen set.
fn message_hash(m: &JttyMessage) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    m.text.hash(&mut h);
    h.finish()
}

fn now_unix(now: SystemTime) -> i64 {
    now.duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

/// Scan a window of 6 kHz audio for JTTY frames and assemble messages.
///
/// The window is searched start to end for sync; each frame found is decoded
/// and accumulated into a message until the EOM flag is seen, then a new
/// message begins. A frame that fails to decode breaks the run.
pub fn decode_window(x: &[f32], at: i64) -> Vec<JttyMessage> {
    let mut out = Vec::new();
    let frame_samples = jtty::JTTY_FRAME_SYMBOLS * jtty::JTTY_NSPS;
    let mut pos = 0usize;
    let mut acc: Vec<jtty::JttyFrame> = Vec::new();
    let mut first_hz = 0.0f32;
    let mut first_snr = 0.0f32;

    while pos + frame_samples <= x.len() {
        let slice = &x[pos..];
        let Some(sync) = jtty::find_sync(slice, 200.0) else {
            break;
        };
        if sync.sync_hits <= 6 {
            // Not a real frame at this position; step past and retry.
            pos += jtty::JTTY_NSPS;
            continue;
        }
        let energies = jtty::payload_energies(slice, &sync);
        match jtty::decode_tones(&energies, 32, 2) {
            Some(frame) => {
                if acc.is_empty() {
                    first_hz = sync.f0;
                    first_snr = sync.snr_db;
                }
                let eom = jtty::is_eom(&frame.payload);
                acc.push(frame);
                // Advance past this frame.
                pos += jtty::JTTY_SYNC_SYMBOLS * jtty::JTTY_NSPS
                    + jtty::INFORMATION_BITS * jtty::JTTY_NSPS;
                if eom {
                    if let Some(msg) = jtty::decode_source(&acc) {
                        out.push(JttyMessage {
                            at_unix: at,
                            text: msg.text,
                            audio_hz: first_hz,
                            snr_db: first_snr.round() as i16,
                            complete: msg.complete,
                        });
                    }
                    acc.clear();
                }
            }
            None => {
                // A failed frame ends any message in progress.
                if !acc.is_empty() {
                    if let Some(msg) = jtty::decode_source(&acc) {
                        out.push(JttyMessage {
                            at_unix: at,
                            text: msg.text,
                            audio_hz: first_hz,
                            snr_db: first_snr.round() as i16,
                            complete: msg.complete,
                        });
                    }
                    acc.clear();
                }
                pos += jtty::JTTY_NSPS;
            }
        }
    }
    // A run that never saw EOM is still worth showing.
    if !acc.is_empty() {
        if let Some(msg) = jtty::decode_source(&acc) {
            out.push(JttyMessage {
                at_unix: at,
                text: msg.text,
                audio_hz: first_hz,
                snr_db: first_snr.round() as i16,
                complete: msg.complete,
            });
        }
    }
    out
}

impl DigiEngine for JttyController {
    fn mode(&self) -> Mode {
        Mode::Jtty
    }

    fn on_rx_audio(&mut self, tap: &[f32]) {
        self.tap_scratch.clear();
        match &mut self.resampler {
            Some(r) => r.push(tap, &mut self.tap_scratch),
            None => self.tap_scratch.extend_from_slice(tap),
        }
        self.buf.extend_from_slice(&self.tap_scratch);
        let cap = Self::window_samples();
        if self.buf.len() > cap {
            let excess = self.buf.len() - cap;
            self.buf.drain(..excess);
        }
        self.since_scan += self.tap_scratch.len();

        if !tap.is_empty() {
            let energy = tap.iter().map(|s| s * s).sum::<f32>() / tap.len() as f32;
            self.level = 0.9 * self.level + 0.1 * energy.sqrt();
        }
    }

    fn poll(&mut self, now: SystemTime, _dial_hz: f64) -> Vec<DigiAction> {
        let mut actions = Vec::new();

        while let Ok(msgs) = self.res_rx.try_recv() {
            self.pending = false;
            for m in msgs {
                let hash = message_hash(&m);
                if !self.seen_recently(hash, now) {
                    self.seen.push_back((hash, now));
                    self.messages.push(m);
                    self.total += 1;
                    self.status_dirty = true;
                }
            }
            if self.messages.len() > JTTY_MESSAGE_MAX {
                let excess = self.messages.len() - JTTY_MESSAGE_MAX;
                self.messages.drain(..excess);
            }
            self.prune_seen(now);
        }

        if !self.pending && self.since_scan >= Self::step_samples() {
            self.since_scan = 0;
            let audio = self.buf.clone();
            self.pending = true;
            let _ = self.job_tx.send(DecodeJob { audio, at: now_unix(now) });
        }

        if self.status_dirty {
            self.status_dirty = false;
            actions.push(DigiAction::Status(self.digi_status()));
        }
        actions
    }

    fn tx_burst_active(&self) -> bool {
        false
    }

    fn fill_tx_block(&mut self, _out: &mut [f32]) -> bool {
        false
    }

    fn on_burst_done(&mut self) {}

    fn abort(&mut self) {
        self.buf.clear();
        self.since_scan = 0;
        self.status_dirty = true;
    }

    fn abort_tx(&mut self) {}

    fn set_config(&mut self, cfg: DigiConfig) {
        self.cfg = cfg;
        self.status_dirty = true;
    }

    fn clear_rx(&mut self) {
        self.messages.clear();
        self.total = 0;
        self.seen.clear();
        self.status_dirty = true;
    }

    fn set_audio_hz(&mut self, _hz: f32) {}

    /// Fixed by the mode: the reference demodulates a candidate-centred
    /// baseband, so there is no operator tone to set.
    fn audio_hz(&self) -> f32 {
        1500.0
    }

    fn status(&self) -> DigiStatus {
        self.digi_status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A synthesized two-frame message decodes through the controller's window
    /// scanner: frame one non-EOM, frame two EOM, accumulated into one message.
    #[test]
    fn a_synthesized_message_is_assembled() {
        use sdroxide_dsp::jtty::{encode_tones, synthesize_frame, JTTY_SYNC};
        // `CQ K1ABC CQ` is one atom; send it twice with EOM only on the second
        // to exercise accumulation. Use the spec's CQ vector.
        let cq = {
            let bits = "0000100110111101111000110101000001";
            let mut p = [0u8; jtty::PAYLOAD_BITS];
            for (i, c) in bits.chars().enumerate() {
                p[i] = c.to_digit(2).unwrap() as u8;
            }
            p
        };
        // Payload bit 34 is the EOM flag: clear it on the first frame, set it
        // on the second so the two accumulate into one message.
        let mut first = cq;
        first[33] = 0;
        let mut last = cq;
        last[33] = 1;

        let mut frame_tones = Vec::new();
        frame_tones.extend_from_slice(&JTTY_SYNC);
        frame_tones.extend_from_slice(&encode_tones(&first));
        frame_tones.extend_from_slice(&JTTY_SYNC);
        frame_tones.extend_from_slice(&encode_tones(&last));

        let audio = synthesize_frame(&frame_tones, jtty::JTTY_TONE_SPACING_HZ);
        let mut buf = vec![0.0f32; jtty::JTTY_NSPS * 4];
        buf.extend_from_slice(&audio);
        buf.extend(std::iter::repeat_n(0.0, jtty::JTTY_NSPS * 8));

        let msgs = decode_window(&buf, 0);
        assert!(!msgs.is_empty(), "no message decoded");
        assert!(msgs.iter().any(|m| m.text.contains("K1ABC")), "{msgs:?}");
    }
}
