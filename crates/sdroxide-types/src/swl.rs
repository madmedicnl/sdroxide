//! The shortwave listener's log: what was **heard**, not what was worked.
//!
//! Deliberately not the QSO log. A reception has no callsign, no RST and no
//! grid to exchange — it has a *station*, a frequency, a time, and a listener's
//! judgement of how well it came through. Folding those into [`crate::QsoRecord`]
//! would give both records fields that are always empty for one of them.
//!
//! The judgement is a **SINPO** report — Strength, Interference, Noise,
//! Propagation, Overall, each 1–5 — or its older three-figure **SIO** form
//! (Strength, Interference, Overall). Both are in use and this fork keeps
//! whichever the listener wrote.

use serde::{Deserialize, Serialize};

/// A five-figure SINPO report. Each figure is 1 (worst) to 5 (best).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sinpo {
    /// Signal strength.
    pub s: u8,
    /// Interference from other stations.
    pub i: u8,
    /// Noise (atmospheric, man-made).
    pub n: u8,
    /// Propagation: fading and distortion.
    pub p: u8,
    /// Overall merit.
    pub o: u8,
}

impl Default for Sinpo {
    fn default() -> Self {
        // A neutral middle for every figure, so a half-filled report reads as
        // "average" rather than "worst".
        Sinpo { s: 3, i: 3, n: 3, p: 3, o: 3 }
    }
}

/// A three-figure SIO report. The older form, still common.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sio {
    pub s: u8,
    pub i: u8,
    pub o: u8,
}

impl Default for Sio {
    fn default() -> Self {
        Sio { s: 3, i: 3, o: 3 }
    }
}

/// How a reception was scored: SINPO or SIO, whichever the listener used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalReport {
    Sinpo(Sinpo),
    Sio(Sio),
}

impl SignalReport {
    /// The tag a listener writes ahead of the figures.
    pub fn label(self) -> &'static str {
        match self {
            SignalReport::Sinpo(_) => "SINPO",
            SignalReport::Sio(_) => "SIO",
        }
    }

    /// The figures as a listener spaces them: `"4 3 3 4 4"`.
    pub fn digits(self) -> String {
        match self {
            SignalReport::Sinpo(r) => format!("{} {} {} {} {}", r.s, r.i, r.n, r.p, r.o),
            SignalReport::Sio(r) => format!("{} {} {}", r.s, r.i, r.o),
        }
    }
}

/// One reception in the listener's log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SwlEntry {
    /// Stable log id (0 = unassigned; the UI assigns on first store).
    pub id: u64,
    /// When it was heard, as Unix seconds UTC. The log sorts and groups by it.
    pub heard_at_unix: u64,
    /// The station, free text or as the schedule names it.
    pub station: String,
    /// The frequency heard, in Hz (dial + any offset).
    pub freq_hz: f64,
    pub mode: crate::Mode,
    /// Programme language, free text ("English", "Dutch").
    pub language: String,
    /// SINPO or SIO, as given. `None` = not judged.
    pub report: Option<SignalReport>,
    /// S-meter reading in dBm when it was heard, if the radio reports one.
    pub smeter_dbm: Option<f32>,
    /// Transmitter site, when the schedule supplied it.
    pub site: String,
    /// The receiving station's Maidenhead locator when it was heard — *where
    /// the reception was made*, which is not the transmitter's [`Self::site`].
    /// A listener who moves the aerial (or the receiver) wants to know which
    /// spot heard it, and a propagation comparison between two evenings needs
    /// it. Pre-filled from the screen's own grid, editable per entry.
    pub recv_grid: String,
    /// The antenna in use when it was heard, in the listener's own words,
    /// captured from the LISTEN window's session field at the moment of
    /// logging. It goes on the reception report's **Antenna:** line, and it is
    /// kept per entry so a report of an older reception names the aerial that
    /// actually heard it.
    pub antenna: String,
    /// Programme notes — what was on, what was said.
    pub notes: String,
    /// When a reception report was sent for this hearing, Unix seconds UTC, or
    /// `None` while it has not been. The `REPORT` button stamps it, and the log
    /// shows a **sent** mark — the second step of the SWL's loop, *hear →
    /// report → await QSL*, which the log otherwise forgot the moment the file
    /// was saved.
    pub report_sent_unix: Option<u64>,
    /// When a QSL (or any verification) came back for it, Unix seconds UTC, or
    /// `None` while it is still awaited — the last step of the loop. The log
    /// shows a **QSL** mark for the ones that completed.
    pub qsl_received_unix: Option<u64>,
    /// The listener marked this as an **unlicensed ("pirate") broadcast** — a
    /// station transmitting outside any allocation, which is a thing shortwave
    /// listeners deliberately hunt. Its own flag rather than a word in the
    /// notes, so the log can show it and a future filter can use it. Off by
    /// default, and nothing infers it: it is the listener's judgement.
    pub pirate: bool,
}

impl Default for SwlEntry {
    fn default() -> Self {
        SwlEntry {
            id: 0,
            heard_at_unix: 0,
            station: String::new(),
            freq_hz: 0.0,
            mode: crate::Mode::Am,
            language: String::new(),
            report: None,
            smeter_dbm: None,
            site: String::new(),
            recv_grid: String::new(),
            antenna: String::new(),
            notes: String::new(),
            report_sent_unix: None,
            qsl_received_unix: None,
            pirate: false,
        }
    }
}

impl SwlEntry {
    /// The frequency as a listener says it: `6185 kHz (6.185 MHz)`.
    pub fn frequency_text(&self) -> String {
        format!("{:.0} kHz ({:.3} MHz)", self.freq_hz / 1e3, self.freq_hz / 1e6)
    }

    /// UTC as the log shows it, e.g. `2026-09-16 19:42 UTC`.
    pub fn utc_text(&self) -> String {
        let (y, mo, d, h, mi, _s) = crate::utc_ymd_hms(self.heard_at_unix as i64);
        format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02} UTC")
    }

    /// A reception report, ready to paste into an email or a station's web
    /// form. `listener`, `grid`, `receiver` and `antenna` describe the
    /// *listening* station: `listener` comes from the screen and `receiver` is
    /// the program's own name, while `grid` and `antenna` are usually this
    /// entry's own [`Self::recv_grid`] and [`Self::antenna`], captured when it
    /// was logged — pass them from the entry so a report names the place and
    /// the aerial that actually made the reception. `listener` is the
    /// listener's own identity — an SWL number, a club number, a name — kept
    /// apart from the transmitting callsign so that reporting a broadcast never
    /// keys a CB transmitter with it.
    ///
    /// Empty lines are left out rather than shown blank, and an unjudged
    /// reception says so instead of printing a row of zeroes.
    pub fn report_text(&self, listener: &str, grid: &str, receiver: &str, antenna: &str) -> String {
        let mut out = String::from("Reception report\n\n");
        out.push_str(&format!("Station:    {}\n", self.station.trim()));
        out.push_str(&format!("Frequency:  {}, {}\n", self.frequency_text(), self.mode.label()));
        out.push_str(&format!("Heard:      {}\n", self.utc_text()));
        let line = |out: &mut String, name: &str, value: &str| {
            if !value.trim().is_empty() {
                out.push_str(&format!("{name:<12}{}\n", value.trim()));
            }
        };
        line(&mut out, "Location:", grid);
        line(&mut out, "Receiver:", receiver);
        line(&mut out, "Antenna:", antenna);
        match self.report {
            Some(r) => out.push_str(&format!("{}:  {}\n", r.label(), r.digits())),
            None => out.push_str("Signal report: not judged\n"),
        }
        line(&mut out, "Notes:", &self.notes);
        line(&mut out, "Reported by:", listener);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry() -> SwlEntry {
        SwlEntry {
            id: 7,
            // 2026-09-16 19:42:00 UTC
            heard_at_unix: 1_789_587_720,
            station: "Radio Taiwan International".into(),
            freq_hz: 6_185_000.0,
            mode: crate::Mode::Am,
            language: "English".into(),
            report: Some(SignalReport::Sinpo(Sinpo { s: 4, i: 3, n: 3, p: 4, o: 4 })),
            smeter_dbm: Some(-73.0),
            site: "Tamsui".into(),
            recv_grid: "JO22".into(),
            antenna: "Longwire 20 m".into(),
            notes: "News, then music".into(),
            // Both stamped, so the serde round-trip and the pre-field-load
            // test cover them.
            report_sent_unix: Some(1_789_588_000),
            qsl_received_unix: Some(1_790_000_000),
            pirate: true,
        }
    }

    #[test]
    fn an_entry_round_trips_through_json() {
        let e = entry();
        let text = serde_json::to_string(&e).unwrap();
        let back: SwlEntry = serde_json::from_str(&text).unwrap();
        assert_eq!(e, back);
    }

    /// A log written before the flag existed has no `pirate` field; it must
    /// load as "not a pirate" rather than fail to parse — the whole reason the
    /// field is `#[serde(default)]`-covered.
    #[test]
    fn an_entry_without_the_flag_loads_as_not_a_pirate() {
        let mut v = serde_json::to_value(entry()).unwrap();
        v.as_object_mut().unwrap().remove("pirate");
        let back: SwlEntry = serde_json::from_value(v).unwrap();
        assert!(!back.pirate);
    }

    /// An entry written by an older build, before a field existed, must load:
    /// every field is `#[serde(default)]`.
    #[test]
    fn an_older_entry_still_loads() {
        let json = r#"{"station":"BBC","freq_hz":9410000.0,"report":{"sinpo":{"s":5,"i":4,"n":4,"p":4,"o":5}}}"#;
        let e: SwlEntry = serde_json::from_str(json).unwrap();
        assert_eq!(e.station, "BBC");
        assert_eq!(e.report, Some(SignalReport::Sinpo(Sinpo { s: 5, i: 4, n: 4, p: 4, o: 5 })));
        assert_eq!(e.mode, crate::Mode::Am, "a missing mode defaults to AM");
        assert!(e.notes.is_empty());
        assert!(e.report_sent_unix.is_none(), "old entry: no report sent yet");
        assert!(e.qsl_received_unix.is_none(), "old entry: no QSL yet");
    }

    #[test]
    fn reports_print_the_way_a_listener_writes_them() {
        assert_eq!(
            SignalReport::Sinpo(Sinpo { s: 4, i: 3, n: 3, p: 4, o: 4 }).digits(),
            "4 3 3 4 4"
        );
        assert_eq!(SignalReport::Sio(Sio { s: 4, i: 3, o: 4 }).digits(), "4 3 4");
        assert_eq!(SignalReport::Sio(Sio::default()).label(), "SIO");
    }

    #[test]
    fn the_frequency_reads_in_khz_and_mhz() {
        assert_eq!(entry().frequency_text(), "6185 kHz (6.185 MHz)");
    }

    #[test]
    fn the_report_is_the_expected_text() {
        let text = entry().report_text("19DCG373", "JO22aa", "RTL-SDR + sdroxide", "long wire");
        let want = "Reception report\n\n\
                    Station:    Radio Taiwan International\n\
                    Frequency:  6185 kHz (6.185 MHz), AM\n\
                    Heard:      2026-09-16 19:42 UTC\n\
                    Location:   JO22aa\n\
                    Receiver:   RTL-SDR + sdroxide\n\
                    Antenna:    long wire\n\
                    SINPO:  4 3 3 4 4\n\
                    Notes:      News, then music\n\
                    Reported by:19DCG373\n";
        assert_eq!(text, want);
    }

    /// The missing pieces say so, rather than printing blanks or zeroes.
    #[test]
    fn an_unjudged_report_says_so() {
        let mut e = entry();
        e.report = None;
        e.notes.clear();
        let text = e.report_text("", "", "", "");
        assert!(text.contains("Signal report: not judged"), "{text}");
        assert!(!text.contains("Location:"), "{text}");
        assert!(!text.contains("Notes:"), "{text}");
        assert!(!text.contains("Reported by:"), "{text}");
    }
}
