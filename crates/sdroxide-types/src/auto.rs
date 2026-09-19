//! Auto mode — the policy for the unattended 11 m sequencer.
//!
//! This is pure: given the decodes just heard and the operator's log, it
//! decides whether auto mode may run and, when it may, which station to answer
//! or that nobody is worth answering and a CQ should go out instead. The UI
//! owns the log and the input clock, so the decision lives here beside them
//! while the actual sequencing stays in the engine's QSO machine.
//!
//! Auto mode is deliberately narrow. It is for the 11 m citizens' band only —
//! the fork's WSJT-CB operation, where an unattended run is the point — and it
//! only ever answers a station calling CQ whose callsign is *new*: not one
//! already in the log. Calling a station mid-exchange, or working a dupe, is
//! left to the operator.

use std::collections::HashSet;

use crate::awards::LogIndex;
use crate::digi::{Decode, cq_is_for_us};
use crate::{Band, Mode};

/// How long the app may go without a single input event before auto mode
/// disarms. The backstop for an unattended transmitter: a window nobody has
/// touched for this long is a window nobody is watching.
pub const AUTO_IDLE_STOP_S: f64 = 20.0 * 60.0;

/// How long after issuing a command the tick waits before issuing another.
///
/// Only to cover the round trip to the engine: the command is queued and the
/// status that reflects it arrives a frame or two later. Without it, the idle
/// status still showing would each frame queue another CQ.
pub const AUTO_COOLDOWN_S: f64 = 3.0;

/// The station auto mode has chosen to answer.
#[derive(Debug, Clone, PartialEq)]
pub struct AutoTarget {
    pub call: String,
    /// Their grid, when the CQ carried one. 11 m CQs usually do not.
    pub grid: Option<String>,
    /// Their signal at us — the report we will send.
    pub snr_db: i16,
    /// The tone offset their CQ was heard at, to answer on.
    pub audio_hz: f32,
}

/// Whether auto mode may run: a slotted FT mode on 11 m, transmit permitted
/// there, and a non-zero transmit watchdog to bound an unattended run.
///
/// The watchdog cannot be zero: it is the only thing that stops a station
/// calling CQ into an empty band forever, and auto mode is exactly the case
/// where nobody is watching to notice.
pub fn auto_ready(mode: Mode, dial_hz: f64, watchdog_min: u32, cb_tx_allowed: bool) -> bool {
    matches!(mode, Mode::Ft8 | Mode::Ft4 | Mode::Ft2)
        && Band::containing(dial_hz) == Band::M11
        && cb_tx_allowed
        && watchdog_min > 0
}

/// Why auto mode may not run, as a sentence for the toggle's tooltip, or
/// `None` when it may. The same tests as [`auto_ready`], in the order the
/// operator can do something about them.
pub fn auto_block_reason(
    mode: Mode,
    dial_hz: f64,
    watchdog_min: u32,
    cb_tx_allowed: bool,
) -> Option<&'static str> {
    if !matches!(mode, Mode::Ft8 | Mode::Ft4 | Mode::Ft2) {
        return Some("Auto mode runs FT8, FT4 and FT2 only.");
    }
    if Band::containing(dial_hz) != Band::M11 {
        return Some("Auto mode is for the 11 m band only — tune there to arm it.");
    }
    if !cb_tx_allowed {
        return Some("Enable transmit on 11 m first (Settings → General).");
    }
    if watchdog_min == 0 {
        return Some(
            "Set a transmit watchdog first (the digi setup): auto mode needs it to pace itself.",
        );
    }
    None
}

/// Pick the CQ auto mode should answer, or `None` when calling CQ is the move.
///
/// A candidate is a CQ that passes [`cq_is_for_us`], names a callsign, is not
/// already in the log, and has not already been tried this session. The
/// strongest is chosen — the one most likely to complete a contact — with the
/// newest slot breaking ties.
pub fn pick_cq(
    decodes: &[Decode],
    log: &LogIndex,
    my_call: &str,
    my_grid: &str,
    tried: &HashSet<String>,
) -> Option<AutoTarget> {
    let mut best: Option<&Decode> = None;
    for d in decodes {
        let Some(from) = d.from.as_deref().filter(|c| !c.is_empty()) else { continue };
        if !d.is_cq || !cq_is_for_us(d, my_call, my_grid) {
            continue;
        }
        let key = from.to_ascii_uppercase();
        if tried.contains(&key) {
            continue;
        }
        // The empty band: 11 m has no ADIF enumeration and no per-band log
        // judgement, so this is the all-time "never worked this call" test.
        if !log.novelty(&key, d.grid.as_deref(), "").new_call {
            continue;
        }
        match best {
            Some(b) if (d.snr_db, d.slot_utc) <= (b.snr_db, b.slot_utc) => {}
            _ => best = Some(d),
        }
    }
    best.map(|d| AutoTarget {
        call: d.from.clone().unwrap_or_default(),
        grid: d.grid.clone(),
        snr_db: d.snr_db,
        audio_hz: d.audio_hz,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QsoRecord;

    fn cq(from: &str, snr: i16, slot: i64) -> Decode {
        Decode {
            slot_utc: slot,
            snr_db: snr,
            dt: 0.0,
            audio_hz: 1000.0,
            message: format!("CQ {from}"),
            to: None,
            from: Some(from.to_string()),
            grid: None,
            is_cq: true,
            cq_to: None,
            free_text: false,
            rr73_to: None,
        }
    }

    fn log_with(calls: &[&str]) -> LogIndex {
        let log: Vec<QsoRecord> = calls
            .iter()
            .map(|c| QsoRecord { call: (*c).to_string(), mode: "FT8".into(), ..Default::default() })
            .collect();
        LogIndex::build(&log)
    }

    #[test]
    fn picks_the_strongest_new_cq() {
        let d = [cq("19LR121", -5, 1), cq("14AT276", 12, 2), cq("30AO010", 3, 3)];
        let got = pick_cq(&d, &log_with(&[]), "26AT715", "", &HashSet::new()).expect("a CQ");
        assert_eq!(got.call, "14AT276");
        assert_eq!(got.snr_db, 12);
    }

    #[test]
    fn never_answers_a_call_already_in_the_log() {
        let d = [cq("19LR121", 20, 1)];
        assert!(pick_cq(&d, &log_with(&["19LR121"]), "26AT715", "", &HashSet::new()).is_none());
    }

    #[test]
    fn skips_a_station_already_tried_this_session() {
        let d = [cq("19LR121", 20, 1)];
        let tried: HashSet<String> = ["19LR121".to_string()].into_iter().collect();
        assert!(pick_cq(&d, &log_with(&[]), "26AT715", "", &tried).is_none());
    }

    #[test]
    fn ignores_a_cq_aimed_elsewhere() {
        let mut d = cq("JA1ABC", 20, 1);
        d.cq_to = Some("JA".to_string());
        // Not a Japanese station: "CQ JA" is not ours to answer.
        assert!(pick_cq(&[d], &log_with(&[]), "26AT715", "", &HashSet::new()).is_none());
    }

    #[test]
    fn ignores_non_cq_traffic() {
        let mut d = cq("19LR121", 20, 1);
        d.is_cq = false;
        assert!(pick_cq(&[d], &log_with(&[]), "26AT715", "", &HashSet::new()).is_none());
    }

    #[test]
    fn readiness_needs_an_ft_mode_on_11m_with_tx_and_a_watchdog() {
        assert!(auto_ready(Mode::Ft8, 27_245_000.0, 6, true));
        assert!(auto_ready(Mode::Ft4, 27_185_000.0, 6, true));
        assert!(auto_ready(Mode::Ft2, 27_245_000.0, 1, true));
        // Wrong mode, wrong band, no 11 m transmit, no watchdog.
        assert!(!auto_ready(Mode::Js8, 27_245_000.0, 6, true));
        assert!(!auto_ready(Mode::Ft8, 14_074_000.0, 6, true));
        assert!(!auto_ready(Mode::Ft8, 27_245_000.0, 6, false));
        assert!(!auto_ready(Mode::Ft8, 27_245_000.0, 0, true));
    }
}
