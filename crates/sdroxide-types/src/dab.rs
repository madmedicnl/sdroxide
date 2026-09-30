//! DAB / DAB+ — Digital Audio Broadcasting.
//!
//! The OFDM digital radio band, in **Band III** (174–240 MHz) and L-band. A
//! **wideband** service like ADS-B: a Mode I ensemble is about 1.536 MHz of
//! occupied spectrum, so it is decoded from its own lane of raw I/Q rather than
//! the 12 kHz tap the narrow modes share, and the panel is a service list
//! rather than a waterfall of audio.
//!
//! This module holds only the wire/persisted shape — the band table, the
//! settings and the status the panel draws. The decoder itself is the
//! `sdroxide-dab` crate, which wraps the MIT `dabradio` library and decodes
//! DAB+ audio through the stock faad2 the binary already carries.

use serde::{Deserialize, Serialize};

/// The Band III channels, by name and centre frequency, in the order the band
/// plan lists them — the plan an operator tunes by, not an arithmetic sequence
/// (the block centres are not evenly spaced).
pub const DAB_BAND_III: &[(&str, f64)] = &[
    ("5A", 174_928_000.0),
    ("5B", 176_640_000.0),
    ("5C", 178_352_000.0),
    ("5D", 180_064_000.0),
    ("6A", 181_936_000.0),
    ("6B", 183_648_000.0),
    ("6C", 185_360_000.0),
    ("6D", 187_072_000.0),
    ("7A", 188_928_000.0),
    ("7B", 190_640_000.0),
    ("7C", 192_352_000.0),
    ("7D", 194_064_000.0),
    ("8A", 195_936_000.0),
    ("8B", 197_648_000.0),
    ("8C", 199_360_000.0),
    ("8D", 201_072_000.0),
    ("9A", 202_928_000.0),
    ("9B", 204_640_000.0),
    ("9C", 206_352_000.0),
    ("9D", 208_064_000.0),
    ("10A", 209_936_000.0),
    ("10B", 211_648_000.0),
    ("10C", 213_360_000.0),
    ("10D", 215_072_000.0),
    ("11A", 216_928_000.0),
    ("11B", 218_640_000.0),
    ("11C", 220_352_000.0),
    ("11D", 222_064_000.0),
    ("12A", 223_936_000.0),
    ("12B", 225_648_000.0),
    ("12C", 227_360_000.0),
    ("12D", 229_072_000.0),
    ("13A", 230_784_000.0),
    ("13B", 232_496_000.0),
    ("13C", 234_208_000.0),
    ("13D", 235_776_000.0),
    ("13E", 237_488_000.0),
    ("13F", 239_200_000.0),
];

/// The channel whose centre is nearest `hz`, as `(name, centre)`.
pub fn dab_channel_at(hz: f64) -> Option<(&'static str, f64)> {
    DAB_BAND_III
        .iter()
        .min_by(|a, b| {
            (a.1 - hz).abs().partial_cmp(&(b.1 - hz).abs()).unwrap_or(std::cmp::Ordering::Equal)
        })
        .copied()
}

/// One service the ensemble carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DabService {
    /// The service identifier as the FIC carries it (`0xF201`).
    pub service_id: String,
    /// The service label (programme name).
    pub label: String,
    /// The sub-channel the service's data rides in, when known.
    #[serde(default)]
    pub subchannel: Option<u8>,
    /// Bitrate in kbps.
    #[serde(default)]
    pub bitrate: Option<u16>,
    /// The protection profile ("EEP 3-A" and so on).
    #[serde(default)]
    pub protection: Option<String>,
}

/// How the DAB receiver behaves. Owned by the engine (it lives in
/// [`crate::RadioState`]) and edited from the panel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DabSettings {
    /// The decoder runs. Follows the mode — selecting DAB switches it on — but
    /// kept as a field so a front end that cannot feed it can switch it off and
    /// say so.
    pub enabled: bool,
    /// The channel to tune, by name from [`DAB_BAND_III`] ("8B"). The
    /// ensemble's own centre; the dial follows it, as ADS-B's 1090 MHz does.
    pub channel: String,
    /// The service currently playing, by service id. Empty until one is picked.
    pub service_id: String,
    /// Volume for DAB audio, on the same 0..=1 scale the receiver's own uses.
    pub volume: f32,
    /// Channels a scan has found an ensemble on, remembered across restarts.
    ///
    /// Which blocks carry a transmission is a fact about where the operator
    /// is, not about the band: the same 12C is a multiplex in one country and
    /// silence in the next. A scan fills this, and the panel offers it, so the
    /// operator picks from what is actually on the air rather than a
    /// hardcoded list that only fits one country.
    #[serde(default)]
    pub found: Vec<String>,
}

impl Default for DabSettings {
    fn default() -> Self {
        DabSettings {
            enabled: true,
            channel: "12B".to_string(),
            service_id: String::new(),
            volume: 1.0,
            found: Vec::new(),
        }
    }
}

impl DabSettings {
    /// Nothing running: the decoder is off and the panel says so.
    pub const OFF: DabSettings = DabSettings {
        enabled: false,
        channel: String::new(),
        service_id: String::new(),
        volume: 1.0,
        found: Vec::new(),
    };
}

/// What the DAB panel draws.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DabStatus {
    /// The ensemble's label once the FIC has named it ("Métropolitain 2").
    pub ensemble: Option<String>,
    /// The services the ensemble carries.
    pub services: Vec<DabService>,
    /// Why nothing is running, when nothing is running — a receiver that cannot
    /// reach Band III, or cannot deliver enough bandwidth, produces an empty
    /// panel either way; only this distinguishes that from a quiet channel.
    pub unavailable: Option<String>,
    /// Where the decoder's own window is, and how wide, in Hz — the same
    /// "your receiver is not looking there" the ADS-B window reports.
    pub window_center_hz: f64,
    pub window_rate_hz: f64,
    /// OFDM frames the decoder has seen, and how many FIBs it read out of them.
    /// A frame count that climbs with no services is a channel with something
    /// else on it, worth showing rather than leaving the panel looking broken.
    pub frames: u64,
    pub fibs: u64,
    /// Whether audio is being decoded, and for which service.
    pub playing: Option<String>,
    /// A receiver too narrow to hold an ensemble decodes nothing and says so.
    #[serde(default)]
    pub degraded: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_channel_table_covers_band_iii() {
        assert_eq!(DAB_BAND_III.first().unwrap().0, "5A");
        assert_eq!(DAB_BAND_III.last().unwrap().0, "13F");
        // Ordered ascending by centre, as the plan reads.
        assert!(DAB_BAND_III.windows(2).all(|w| w[0].1 < w[1].1));
    }

    #[test]
    fn a_frequency_resolves_to_the_nearest_channel() {
        assert_eq!(dab_channel_at(197_648_000.0).map(|c| c.0), Some("8B"));
        assert_eq!(dab_channel_at(227_400_000.0).map(|c| c.0), Some("12C"));
        // A few tens of kHz off still lands on the channel.
        assert_eq!(dab_channel_at(197_700_000.0).map(|c| c.0), Some("8B"));
    }
}
