//! DAB / DAB+ reception for this program, wrapping the [`dabradio`] decoder.
//!
//! DAB Mode I is a **wideband** service — about 1.536 MHz of occupied spectrum
//! in Band III, demodulated by an OFDM front end that wants its input at
//! 2.048 Msps — so, like ADS-B, it cannot ride the 12 kHz `on_rx_iq` tap the
//! narrow modes share. It is fed raw I/Q from a lane of its own.
//!
//! The decoder is the MIT [`dabradio`] crate, taken as a **library** (upstream
//! PR `xoolive/desperado#52`, branch `lib/split-dabradio`). That split is what
//! makes the audio path licensable: `dabradio`'s DAB+ decoding reaches its AAC
//! stage through **fdk-aac**, which is optional there, and the FDK licence
//! cannot be linked into a GPL build. So this crate runs the same pipeline with
//! fdk-aac **off** and hands the Reed–Solomon-corrected Access Units to the
//! **stock faad2** the binary already carries for DRM ([`AacDecoder`]). DAB (not
//! DAB+) carries MP2, which is pure Rust in `dabradio` and needs no swap.
//!
//! Proven against a real off-air capture (channel 8B, Nancy): the ensemble and
//! service list, and DAB+ audio through faad2. Not yet proven on live RF.

use std::collections::VecDeque;

use num_complex::Complex32;
use thiserror::Error;
use tracing::warn;

pub mod aac;

pub use aac::AacDecoder;

/// The rate the DAB Mode I OFDM front end is defined at, in samples/s.
pub const DAB_SAMPLE_RATE: u32 = 2_048_000;

/// How wide a lane a DAB ensemble occupies, in Hz — Mode I is ~1.536 MHz of
/// occupied spectrum, so a front end opened narrower loses the outer carriers.
pub const DAB_BANDWIDTH_HZ: f64 = 1_536_000.0;

/// A rate the lane is comfortable at.
///
/// At the bare [`DAB_BANDWIDTH_HZ`] the front end is right at its ADC floor,
/// where the SDRplay API in particular drops samples — and a DAB decode needs a
/// *continuous* OFDM stream, so a splice stops sync. Twice the occupied width
/// leaves the margin to deliver it cleanly. Not a refusal below this: a
/// narrower-but-clean stream still decodes, exactly as ADS-B's rate note says.
pub const DAB_GOOD_RATE_HZ: f64 = 3_072_000.0;

#[derive(Debug, Error)]
pub enum DabError {
    #[error(
        "the front end's sample rate is below the {DAB_BANDWIDTH_HZ:.0} Hz a DAB ensemble needs"
    )]
    LaneTooNarrow,
    #[error("could not build a resampler from {0} Hz to {DAB_SAMPLE_RATE} Hz")]
    Resample(f64),
}

/// One service in the ensemble, as the FIC describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DabService {
    /// The service identifier, as the FIC carries it (`0xF201`).
    pub service_id: String,
    /// The service label (programme name).
    pub label: String,
    /// The sub-channel this service's data rides in, when known.
    pub subchannel: Option<u8>,
    /// Bitrate, in kbps.
    pub bitrate: Option<u16>,
    /// The protection profile (`EEP 3-A` and so on).
    pub protection: Option<String>,
}

/// The ensemble a lane is tuned to, and what it carries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DabEnsemble {
    /// The ensemble label ("Métropolitain 2").
    pub label: Option<String>,
    pub services: Vec<DabService>,
}

/// What a service's audio is coded with — the choice that decides which decoder
/// runs, and so whether our faad2 or `dabradio`'s own MP2 does the work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DabCoding {
    /// Legacy DAB, MPEG-1/2 Layer II — pure Rust, no faad2.
    MpegLayer2,
    /// DAB+, HE-AAC — decoded through our [`AacDecoder`] (faad2).
    DabPlus,
    /// A component the FIC named that neither decoder handles.
    Unsupported,
}

/// The audio pipeline for one service: MSC frames in, interleaved PCM out.
struct AudioPipe {
    coding: DabCoding,
    /// Extracts sub-channel frames from the MSC soft bits.
    msc: dabradio::msc::MscHandler,
    /// DAB+: Reed–Solomon super-frames → Access Units.
    superframe: Option<dabradio::audio::SuperframeDecoder>,
    /// DAB+: the AAC decoder (ours, over faad2).
    aac: Option<AacDecoder>,
    /// DAB (MP2): `dabradio`'s own decoder.
    mp2: Option<dabradio::audio::mp2::Mp2Decoder>,
    /// Logical frames waiting to be handed to the super-frame assembler.
    pending: VecDeque<Vec<u8>>,
    /// Whether the AAC decoder has been configured from the super-frame's ASC.
    aac_configured: bool,
}

/// One DAB receiver: raw I/Q in at whatever the front end gives, ensemble,
/// services and — once a service is picked — PCM audio out.
///
/// The front end opens at many rates (a 2.5 Msps capture, a 2.4 Msps dongle, a
/// 6 Msps HF+); the OFDM processor wants one. [`Self::new`] builds the bridge.
pub struct DabReceiver {
    ofdm: dabradio::ofdm::processor::OfdmProcessor,
    ensemble: dabradio::fic::fib::EnsembleInfo,
    /// Resamples the front end's rate down to [`DAB_SAMPLE_RATE`].
    resampler: Option<sdroxide_dsp::ComplexResampler>,
    resample_out: Vec<Complex32>,
    /// The service being decoded, and its pipeline. `None` until one is picked.
    audio: Option<AudioPipe>,
    /// The service to decode, by id or label, held until the FIC names it.
    wanted: Option<String>,
    /// OFDM frames and FIBs read since the receiver started.
    frames: u64,
    fibs: u64,
}

impl DabReceiver {
    /// Build a receiver for a front end running at `input_rate_hz`.
    ///
    /// Errors when the rate is too low to hold an ensemble, or when no
    /// resampler can bridge it — both are the operator's problem to see, not to
    /// discover as silence.
    pub fn new(input_rate_hz: f64) -> Result<Self, DabError> {
        if input_rate_hz < DAB_BANDWIDTH_HZ {
            return Err(DabError::LaneTooNarrow);
        }
        let resampler = sdroxide_dsp::ComplexResampler::new(input_rate_hz, DAB_SAMPLE_RATE as f64)
            .ok_or(DabError::Resample(input_rate_hz))?;
        Ok(DabReceiver {
            ofdm: dabradio::ofdm::processor::OfdmProcessor::new(),
            ensemble: dabradio::fic::fib::EnsembleInfo::new(),
            resampler: (input_rate_hz != DAB_SAMPLE_RATE as f64).then_some(resampler),
            resample_out: Vec::new(),
            audio: None,
            wanted: None,
            frames: 0,
            fibs: 0,
        })
    }

    /// Feed a block of the front end's I/Q. FIBs fold into the ensemble, and —
    /// once a service is selected and named — its audio is decoded.
    pub fn feed(&mut self, iq: &[Complex32]) {
        // Resample into an owned buffer when the front end's rate differs, so
        // the borrow ends before `self.ofdm` is used.
        let baseband: Vec<Complex32> = match self.resampler.as_mut() {
            Some(rs) => {
                self.resample_out.clear();
                rs.push(iq, &mut self.resample_out);
                std::mem::take(&mut self.resample_out)
            }
            None => iq.to_vec(),
        };
        // A selected service can only be started once the FIC has resolved it.
        if self.audio.is_none()
            && let Some(want) = self.wanted.clone()
        {
            self.ensemble.resolve_services();
            self.start_audio(&want);
        }
        for frame in self.ofdm.process(&baseband) {
            self.frames += 1;
            let soft_bits = dabradio::ofdm::decoder::dqpsk_decode(&frame.symbols);
            // The FIC rides the first three symbols of every CIF.
            if soft_bits.len() >= 3 {
                for fib in dabradio::fic::handler::process_fic(&soft_bits[..3]) {
                    self.ensemble.parse_fib(&fib);
                    self.fibs += 1;
                }
            }
            // The MSC rides the rest, once a service is being decoded.
            const FIC_SYMBOLS: usize = 3;
            if let Some(pipe) = self.audio.as_mut() {
                let end = soft_bits.len().min(FIC_SYMBOLS + 72);
                for sym in &soft_bits[FIC_SYMBOLS..end] {
                    if let Some(logical) = pipe.msc.feed_symbol(sym) {
                        pipe.pending.push_back(logical);
                    }
                }
            }
        }
    }

    /// The ensemble as it stands, services resolved by label.
    pub fn ensemble(&self) -> DabEnsemble {
        let mut e = self.ensemble.clone();
        e.resolve_services();
        let out = e.to_output();
        DabEnsemble {
            label: out.ensemble_label,
            services: out
                .services
                .into_iter()
                .map(|s| DabService {
                    service_id: s.service_id,
                    label: s.label.unwrap_or_default(),
                    subchannel: s.subchannel_id,
                    bitrate: s.bitrate,
                    protection: s.protection,
                })
                .collect(),
        }
    }

    /// Whether enough of the FIC has arrived to name the services.
    pub fn has_services(&self) -> bool {
        self.ensemble.has_services()
    }

    /// Frames and FIBs read since the receiver started, for the panel's header.
    pub fn counters(&self) -> (u64, u64) {
        (self.frames, self.fibs)
    }

    /// The service being decoded, by id or label, if any.
    pub fn playing(&self) -> Option<&str> {
        self.audio.as_ref().map(|_| self.wanted.as_deref().unwrap_or(""))
    }

    /// Select a service to decode, by service id (`0xF201`) or label.
    ///
    /// The pipeline is built as soon as the FIC names the service; until then
    /// the request is held, so a selection made the moment the list appears is
    /// not lost to a race with the FIC.
    pub fn select_service(&mut self, which: &str) {
        if self.wanted.as_deref() == Some(which) && self.audio.is_some() {
            return;
        }
        self.wanted = Some(which.to_string());
        self.audio = None;
        self.ensemble.resolve_services();
        self.start_audio(which);
    }

    /// Take the PCM decoded so far. Interleaved, at the codec's own rate.
    pub fn take_pcm(&mut self) -> Vec<f32> {
        let Some(pipe) = self.audio.as_mut() else { return Vec::new() };
        let mut pcm = Vec::new();
        while let Some(frame) = pipe.pending.pop_front() {
            match pipe.coding {
                DabCoding::DabPlus => {
                    let Some(sf) = pipe.superframe.as_mut() else { continue };
                    let decoded = sf.feed_frame(&frame);
                    // faad2 is configured once, from the AudioSpecificConfig the
                    // super-frame carries — where `DabPlusDecoder` configures
                    // fdk-aac.
                    if !pipe.aac_configured
                        && let Some(fmt) = sf.format
                    {
                        if let Some(mut a) = AacDecoder::new() {
                            let asc = fmt.audio_specific_config();
                            if a.configure(&asc) {
                                pipe.aac = Some(a);
                                pipe.aac_configured = true;
                            } else {
                                warn!("DAB+: faad2 refused the AudioSpecificConfig");
                            }
                        } else {
                            warn!("DAB+: faad2 could not open an AAC decoder");
                        }
                    }
                    if let Some(aus) = decoded
                        && let Some(dec) = pipe.aac.as_mut()
                    {
                        for au in aus {
                            pcm.extend(dec.decode_au(&au));
                        }
                    }
                }
                DabCoding::MpegLayer2 => {
                    if let Some(mp2) = pipe.mp2.as_mut() {
                        pcm.extend(mp2.feed_frame(&frame).pcm);
                    }
                }
                DabCoding::Unsupported => {}
            }
        }
        pcm
    }

    /// Build the audio pipeline for `which`, if the FIC names it.
    fn start_audio(&mut self, which: &str) {
        use dabradio::fic::fib::AudioCoding;
        let target = parse_service_id(which);
        let service = match target {
            Some(sid) => self.ensemble.services.get(&sid),
            None => {
                let want = which.trim().to_lowercase();
                self.ensemble.services.values().find(|s| {
                    s.label.as_ref().is_some_and(|l| l.trim().to_lowercase() == want)
                })
            }
        };
        let Some(service) = service else { return };
        let Some(subch_id) = service.subchannel_id else { return };
        let Some(subch) = self.ensemble.subchannels.get(&subch_id) else { return };
        let Some(coding) = service.audio_coding else { return };
        let Some(msc) = dabradio::msc::MscHandler::new(subch) else { return };
        let bitrate = subch.bitrate;
        let (coding, superframe, mp2) = match coding {
            AudioCoding::DabPlus => (
                DabCoding::DabPlus,
                Some(dabradio::audio::SuperframeDecoder::new(bitrate)),
                None,
            ),
            AudioCoding::MpegLayer2 => (
                DabCoding::MpegLayer2,
                None,
                Some(dabradio::audio::mp2::Mp2Decoder::new()),
            ),
            AudioCoding::Other(_) => (DabCoding::Unsupported, None, None),
        };
        self.audio = Some(AudioPipe {
            coding,
            msc,
            superframe,
            aac: None,
            mp2,
            pending: VecDeque::new(),
            aac_configured: false,
        });
    }
}

/// Parse `0xF201` / `F201` into a service id number, or `None` for a label.
fn parse_service_id(s: &str) -> Option<u32> {
    let t = s.trim();
    let hex = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    if hex.len() == 4 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole chain against a real capture, when one is on hand: sync, FIC,
    /// the service list, then DAB+ audio through our faad2 — the path this crate
    /// exists for. Ignored, because it needs a multi-hundred-megabyte off-air
    /// file; point `SDROXIDE_DAB_SAMPLE` at a raw cs16 capture and run with
    /// `--ignored`.
    #[test]
    #[ignore]
    fn an_off_air_capture_decodes_services_and_audio() {
        let (Ok(path), Ok(rate)) = (
            std::env::var("SDROXIDE_DAB_SAMPLE"),
            std::env::var("SDROXIDE_DAB_SAMPLE_RATE"),
        ) else {
            return;
        };
        let rate: f64 = rate.parse().expect("rate");
        let raw = std::fs::read(&path).expect("read capture");
        let iq: Vec<Complex32> = raw
            .chunks_exact(4)
            .map(|b| {
                Complex32::new(
                    i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0,
                    i16::from_le_bytes([b[2], b[3]]) as f32 / 32768.0,
                )
            })
            .collect();
        let mut rx = DabReceiver::new(rate).expect("receiver");
        let mut selected = false;
        let mut pcm = 0usize;
        for block in iq.chunks(32768) {
            rx.feed(block);
            if !selected && rx.has_services() {
                let e = rx.ensemble();
                assert!(!e.services.is_empty(), "the FIC named no services");
                // The first DAB+ service, so the faad2 path is exercised.
                rx.select_service(&e.services[0].service_id);
                selected = true;
            }
            pcm += rx.take_pcm().len();
        }
        assert!(selected, "no ensemble was decoded");
        eprintln!("decoded {pcm} PCM samples ({:.2}s)", pcm as f64 / 48000.0 / 2.0);
        assert!(pcm > 0, "the ensemble decoded but no service produced audio");
    }
}
