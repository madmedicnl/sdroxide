//! The ATS Mini (ESP32-S3 + Si4732) "ad hoc" remote protocol — host side.
//!
//! The firmware exposes a single-character command protocol over TCP, USB
//! serial or BLE. `t` turns on a 500 ms telemetry monitor on the same link;
//! multi-character commands (`F`, `#`) take a trailing CR. This module is the
//! pure codec for both directions — command bytes out, telemetry in — so it can
//! be unit-tested against captured lines without any I/O. The socket, the
//! sound-card audio and the `IqSource` wrapper live in the binary crate.
//!
//! Upstream reference: `esp32-si4732/ats-mini`, `Remote.cpp` (`remoteDoCommand`,
//! `remotePrintStatus`) and `docs/source/remote.md`. The protocol is from the
//! firmware's own source, reimplemented here; see
//! `docs/ats-mini-handover.md` for the bench notes.

/// Default TCP port of the firmware's ad hoc control server.
pub const DEFAULT_PORT: u16 = 60000;

/// Default mDNS host. The bench box has no mDNS resolver, so an IP works too.
pub const DEFAULT_HOST: &str = "atsmini.local";

/// The monitor cadence the firmware emits (`remoteTickTime`, 500 ms).
pub const MONITOR_PERIOD_MS: u32 = 500;

/// The modulation the *radio* is running, as reported in the telemetry `mode`
/// field. This is the hardware demod; sdroxide's own receive mode only decides
/// what it does with the resulting audio.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirmwareMode {
    Am,
    Lsb,
    Usb,
    Fm,
}

impl FirmwareMode {
    /// Parse the telemetry spelling.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "AM" => Some(Self::Am),
            "LSB" => Some(Self::Lsb),
            "USB" => Some(Self::Usb),
            "FM" => Some(Self::Fm),
            _ => None,
        }
    }

    /// The telemetry spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Am => "AM",
            Self::Lsb => "LSB",
            Self::Usb => "USB",
            Self::Fm => "FM",
        }
    }
}

/// One line of the 500 ms monitor, parsed.
///
/// Field order from `remotePrintStatus`:
/// `version, freq, bfo, bandCal, band, mode, step, bw, agc, volume, rssi, snr,
/// tuningCap, voltage, seq`.
#[derive(Clone, Debug, PartialEq)]
pub struct Telemetry {
    /// Firmware version ×100 (`240` = v2.40).
    pub version: u16,
    /// Frequency in the firmware's units: 10 kHz for FM, kHz for AM/SSB.
    pub freq_raw: u32,
    /// BFO in Hz (SSB only; 0 in the other modes).
    pub bfo_hz: i32,
    /// Per-band SSB calibration in Hz (SSB only).
    pub band_cal_hz: i32,
    /// Band name, e.g. `VHF`, `ALL`, `CB` (see the handover doc for the table).
    pub band: String,
    pub mode: FirmwareMode,
    /// Step description, e.g. `100k`, `1k`.
    pub step: String,
    /// Bandwidth description, e.g. `Auto`, `3.0k`.
    pub bandwidth: String,
    pub agc: u16,
    /// 0–63, 0 = mute.
    pub volume: u16,
    /// 0–127 dBµV.
    pub rssi_dbuv: u16,
    /// 0–127 dB.
    pub snr_db: u16,
    /// Antenna tuning capacitor, 0–6143.
    pub tuning_cap: u16,
    /// Battery/supply voltage in volts (already scaled by the firmware).
    pub voltage: f32,
    /// 0–255, wrapping.
    pub seq: u16,
}

impl Telemetry {
    /// Parse one telemetry line. `None` for anything else — a command echo or
    /// an `Error:` reply shares the socket and must not be mistaken for state.
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim_end_matches(['\r', '\n']);
        let f: Vec<&str> = line.split(',').collect();
        if f.len() != 15 {
            return None;
        }
        let mode = FirmwareMode::parse(f[5])?;
        Some(Telemetry {
            version: f[0].parse().ok()?,
            freq_raw: f[1].parse().ok()?,
            bfo_hz: f[2].parse().ok()?,
            band_cal_hz: f[3].parse().ok()?,
            band: f[4].to_string(),
            mode,
            step: f[6].to_string(),
            bandwidth: f[7].to_string(),
            agc: f[8].parse().ok()?,
            volume: f[9].parse().ok()?,
            rssi_dbuv: f[10].parse().ok()?,
            snr_db: f[11].parse().ok()?,
            tuning_cap: f[12].parse().ok()?,
            voltage: f[13].parse().ok()?,
            seq: f[14].parse().ok()?,
        })
    }

    /// The displayed receive frequency in Hz.
    ///
    /// FM reports `freq_raw` in 10 kHz units; AM/SSB in kHz, and in SSB the BFO
    /// is added (`remotePrintStatus`'s "display frequency" rule).
    pub fn dial_hz(&self) -> f64 {
        match self.mode {
            FirmwareMode::Fm => f64::from(self.freq_raw) * 10_000.0,
            _ => f64::from(self.freq_raw) * 1000.0 + f64::from(self.bfo_hz),
        }
    }

    /// Firmware version as a float (v2.40 → 2.40).
    pub fn fw_version(&self) -> f32 {
        f32::from(self.version) / 100.0
    }
}

/// Toggle the 500 ms telemetry monitor (`t`).
pub fn monitor_toggle() -> char {
    't'
}

/// Set the frequency (`F<Hz>\r`). The firmware rejects it unless it falls
/// inside the *current* band, so the caller cycles bands first.
pub fn set_frequency(hz: u64) -> String {
    format!("F{hz}\r")
}

/// Band up/down (`B`/`b`). There is no direct band-select.
pub fn band_step(up: bool) -> char {
    if up { 'B' } else { 'b' }
}

/// Mode up/down (`M`/`m`): cycles `LSB → USB → AM → LSB`, FM only on VHF.
pub fn mode_step(up: bool) -> char {
    if up { 'M' } else { 'm' }
}

/// Volume up/down (`V`/`v`).
pub fn volume_step(up: bool) -> char {
    if up { 'V' } else { 'v' }
}

/// Bandwidth up/down (`W`/`w`).
pub fn bandwidth_step(up: bool) -> char {
    if up { 'W' } else { 'w' }
}

/// AGC/attenuator up/down (`A`/`a`).
pub fn agc_step(up: bool) -> char {
    if up { 'A' } else { 'a' }
}

/// Tuning-step up/down (`S`/`s`).
pub fn tuning_step(up: bool) -> char {
    if up { 'S' } else { 's' }
}

/// Dump the 99 memory slots (`$`).
pub fn dump_memories() -> char {
    '$'
}

/// Set a memory slot (`#nn,band,hz,mode\r`). `hz` in Hz; 0 clears the slot.
pub fn set_memory(slot: u8, band: &str, hz: u64, mode: FirmwareMode) -> String {
    format!("#{slot:02},{band},{hz},{}\r", mode.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Lines captured from the bench radio, firmware v2.40.

    #[test]
    fn parses_an_fm_telemetry_line() {
        let t =
            Telemetry::parse("240,10390,0,0,VHF,FM,100k,Auto,0,35,18,0,1,4.46,1").expect("parses");
        assert_eq!(t.version, 240);
        assert_eq!(t.fw_version(), 2.40);
        assert_eq!(t.band, "VHF");
        assert_eq!(t.mode, FirmwareMode::Fm);
        assert_eq!(t.volume, 35);
        assert_eq!(t.rssi_dbuv, 18);
        assert_eq!(t.snr_db, 0);
        assert!((t.voltage - 4.46).abs() < 1e-6);
        // FM: freq_raw is 10 kHz units.
        assert_eq!(t.dial_hz(), 103_900_000.0);
    }

    #[test]
    fn parses_an_am_telemetry_line() {
        let t =
            Telemetry::parse("240,27185,0,0,ALL,AM,1k,3.0k,0,35,19,0,1,4.47,2").expect("parses");
        assert_eq!(t.mode, FirmwareMode::Am);
        assert_eq!(t.band, "ALL");
        // AM: freq_raw is kHz, no BFO.
        assert_eq!(t.dial_hz(), 27_185_000.0);
    }

    #[test]
    fn parses_an_ssb_telemetry_line_and_adds_the_bfo() {
        let t = Telemetry::parse("240,14100,-1500,0,20M,USB,10,3.0k,0,35,33,0,1,4.47,5")
            .expect("parses");
        assert_eq!(t.mode, FirmwareMode::Usb);
        // Displayed = freq_kHz*1000 + bfo.
        assert_eq!(t.dial_hz(), 14_098_500.0);
    }

    #[test]
    fn rejects_command_echoes_and_errors() {
        // The socket carries the echo and any error alongside the monitor.
        for line in [
            "F27185000",
            "Error: Frequency is out of range for the current band",
            "",
            "240,10390,0,0,VHF,FM,100k,Auto,0,35,18,0,1,4.46",
            "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o",
        ] {
            assert!(Telemetry::parse(line).is_none(), "should reject {line:?}");
        }
    }

    #[test]
    fn command_builders_match_the_firmware_wire_format() {
        assert_eq!(monitor_toggle(), 't');
        assert_eq!(set_frequency(27_185_000), "F27185000\r");
        assert_eq!(band_step(true), 'B');
        assert_eq!(band_step(false), 'b');
        assert_eq!(mode_step(false), 'm');
        assert_eq!(volume_step(true), 'V');
        assert_eq!(bandwidth_step(false), 'w');
        assert_eq!(agc_step(true), 'A');
        assert_eq!(tuning_step(false), 's');
        assert_eq!(dump_memories(), '$');
        assert_eq!(set_memory(1, "VHF", 107_900_000, FirmwareMode::Fm), "#01,VHF,107900000,FM\r");
    }
}
