//! The contest logger window.
//!
//! Mode-agnostic: the operator types the exchange by hand for CW, SSB or
//! anything else, and the FT8 side auto-fills the same entry when it drives one
//! of the contests that has an FT8 layout. A session is the operator's own
//! state (see [`sdroxide_types::ContestSession`]); the QSOs it logs go into the
//! ordinary logbook, tagged with the contest, so scoring and the Cabrillo
//! export read the same rows everything else does.

use eframe::egui;
use sdroxide_types::{Command, ContestId, ContestSession, Exchange, QsoRecord};

use super::SdroxideApp;

/// The in-progress entry: what the operator is typing for the station being
/// worked. Session-only, cleared after every logged QSO.
#[derive(Default)]
pub(in crate::app) struct ContestEntry {
    pub call: String,
    /// The report we send. Defaults from the mode — `599` on CW, `59` on phone
    /// — and stays editable, because the operator may have sent something else.
    pub rst_sent: String,
    /// The report they gave us.
    pub rst_rcvd: String,
    /// **One box per received exchange element** beyond the report — the serial
    /// and the locator for EU VHF, the zone for CQ WW, the text for a CB
    /// activity. A single box kept whichever element was typed last and threw
    /// the others away.
    pub fields: Vec<String>,
}

impl SdroxideApp {
    pub(in crate::app) fn contest_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        if !self.show_contest {
            return;
        }
        let mut open = self.show_contest;
        egui::Window::new("CONTEST")
            .id(crate::layout::salted_id(ctx, "CONTEST"))
            .open(&mut open)
            .frame(crate::chrome::window_frame())
            .resizable(true)
            .default_width(crate::layout::window_w(ctx, 660.0))
            .default_height(crate::layout::window_h(ctx, 540.0))
            .show(ctx, |ui| {
                crate::chrome::window_body_bg(ui);
                self.contest_body(ui, cmds);
            });
        self.show_contest = open;
    }

    fn contest_body(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        if self.contest.is_none() {
            self.contest_setup(ui, cmds);
        } else {
            self.contest_run(ui, cmds);
        }
    }

    /// The pre-session setup: pick the contest and enter our own exchange.
    fn contest_setup(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        ui.label(
            "Pick a contest and enter what you send. The logger works on any mode — \
             type the exchange for CW or SSB, and the FT8 side fills it in by itself \
             where the contest has an FT8 layout.",
        );
        ui.add_space(8.0);
        let mut picked = self.contest_pick;
        ui.horizontal_wrapped(|ui| {
            for c in ContestId::CHOICES {
                if crate::chrome::chip(ui, picked == c, c.label()).clicked() {
                    picked = c;
                }
            }
        });
        self.contest_pick = picked;
        let spec = picked.spec();
        ui.add_space(8.0);
        ui.label(egui::RichText::new(spec.name).strong());
        ui.label(format!("  you send: {}", exchange_hint(spec.sent)));
        ui.label(format!("  they send: {}", exchange_hint(spec.rcvd)));
        if picked == ContestId::CbActivity {
            ui.label(
                egui::RichText::new(
                    "CB / 11 m activity: a report and a free-text exchange — put the \
                     channel, name or area the activity uses in it.",
                )
                .size(11.0),
            );
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Your exchange");
            crate::chrome::field(ui, egui::TextEdit::singleline(&mut self.contest_my_exchange));
        });
        ui.add_space(8.0);
        if crate::chrome::chip(ui, true, " START ").clicked() {
            let now = crate::time::now_unix_f64() as i64;
            // Seeded from the log, so a session stopped and restarted — or the
            // program itself restarted — carries on from the last serial
            // instead of sending 001 again.
            self.contest = Some(ContestSession::seeded(
                picked,
                self.contest_my_exchange.trim().into(),
                now,
                &self.qso_log,
            ));
            self.contest_entry = Default::default();
            self.reset_entry_reports();
            // Tell the digi engine which FT8 contest layout to send, when this
            // contest has one. A hand-typed CW/SSB contest sets `None` — there
            // is no message layout to choose — and the logger works regardless.
            //
            // Narrowly, on purpose: this panel is not the digi panel and its
            // `digi_cfg_edit` copy is not authoritative. Pushing a whole
            // `DigiConfig` from here would roll back whatever the engine holds
            // that this copy is stale on, and would clear an FT8 contest layout
            // the operator had already set — since `digi_contest_for` yields
            // `None` for every contest but EU VHF.
            cmds.push(sdroxide_types::Command::SetDigiContest(digi_contest_for(picked)));
        }
    }

    /// Fill the report boxes from the mode in force, and size the exchange
    /// boxes to the contest's exchange.
    ///
    /// Called when a session starts so the boxes match it, and when a QSO is
    /// logged so the next one starts fresh without losing the report default.
    fn reset_entry_reports(&mut self) {
        let mode = self.state.rx[0].mode.label();
        let rst = default_report(mode).to_string();
        self.contest_entry.rst_sent = rst.clone();
        self.contest_entry.rst_rcvd = rst;
        let n = self.contest.as_ref().map(|s| s.contest.received_fields().len()).unwrap_or(0);
        self.contest_entry.fields = vec![String::new(); n];
    }

    /// The running session: entry, dupes, score and the session's log.
    fn contest_run(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let Some(contest) = self.contest.as_ref().map(|s| s.contest) else { return };
        let spec = contest.spec();
        let mine = self.session_qsos();
        let score = sdroxide_types::score(&mine, contest);
        let now = crate::time::now_unix_f64() as i64;
        let r10 = sdroxide_types::rate(&mine, now, 600);
        let r60 = sdroxide_types::rate(&mine, now, 3600);

        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new(contest.label()).strong());
            ui.label(format!(
                "QSO {} · {} pts · {} mult · SCORE {}",
                score.qsos, score.points, score.mults, score.total
            ));
            ui.label(format!("{r10} in 10 min · {r60}/hr"));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if crate::chrome::chip(ui, false, " STOP ").clicked() {
                    self.contest = None;
                }
            });
        });
        if spec.multiplier != sdroxide_types::Multiplier::None {
            ui.label(
                egui::RichText::new("score is an estimate — the sponsor adjudicates")
                    .size(10.0)
                    .color(crate::theme::gray(140)),
            );
        }
        ui.separator();

        // ── The entry form ────────────────────────────────────────────────
        // The dupe check looks at the **session's** QSOs, not the whole
        // logbook. Running it over everything made any pre-contest QSO with
        // the station — one from last month, on the same band — light up as a
        // dupe before the contest had even started.
        let dupe = {
            let call = self.contest_entry.call.trim();
            !call.is_empty()
                && sdroxide_types::worked_before(&mine, call, self.state.band.label(), "", 0)
        };
        if self.contest_entry.fields.len() != contest.received_fields().len() {
            self.contest_entry.fields = vec![String::new(); contest.received_fields().len()];
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("CALL");
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.contest_entry.call).desired_width(100.0),
            );
            if dupe {
                ui.label(egui::RichText::new("DUPE").strong().color(crate::theme::ALERT()));
            }
            ui.label("SENT");
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.contest_entry.rst_sent).desired_width(46.0),
            );
            ui.label("RCVD");
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.contest_entry.rst_rcvd).desired_width(46.0),
            );
            // One box per received element, labelled from the contest's own
            // exchange — so EU VHF asks for its serial *and* its locator.
            for (i, ex) in contest.received_fields().iter().enumerate() {
                ui.label(ex.label());
                crate::chrome::field(
                    ui,
                    egui::TextEdit::singleline(&mut self.contest_entry.fields[i])
                        .desired_width(80.0),
                );
            }
        });
        ui.add_space(4.0);
        let serial = self.contest.as_ref().map(|s| s.next_serial).unwrap_or(1);
        ui.horizontal(|ui| {
            let stx = contest.sends_serial().then_some(serial);
            let ours = self.contest.as_ref().map(|s| s.sent_exchange(stx)).unwrap_or_default();
            let rst = self.contest_entry.rst_sent.trim();
            if ours.is_empty() {
                ui.label(format!("sending {rst}"));
            } else {
                ui.label(format!("sending {rst} {ours}"));
            }
            ui.label(format!("· {} {}", self.state.band.label(), self.state.rx[0].mode.label()));
            if crate::chrome::chip(ui, true, " LOG ").clicked() {
                self.log_contest_qso(cmds);
            }
            if crate::chrome::chip(ui, false, " CABRILLO ").clicked() {
                let cab = sdroxide_types::to_cabrillo(
                    contest,
                    &self.my_call(),
                    &self.contest_my_exchange,
                    &mine,
                    &format!("sdroxide {}", sdroxide_version::VERSION),
                );
                crate::download::save("sdroxide.cab", cab.as_bytes());
            }
        });
        ui.separator();

        // ── The session's log ─────────────────────────────────────────────
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            egui::Grid::new("contest-log-grid")
                .num_columns(5)
                .spacing([12.0, 2.0])
                .striped(true)
                .show(ui, |ui| {
                    for q in mine.iter().rev().take(200) {
                        let (_, _, _, h, mi, _) = sdroxide_types::utc_ymd_hms(q.start_utc);
                        ui.label(format!("{h:02}{mi:02}"));
                        ui.label(&q.call);
                        ui.label(&q.band);
                        ui.label(&q.mode);
                        ui.label(&q.srx_string);
                        ui.end_row();
                    }
                });
        });
    }

    /// The current session's QSOs: the log's rows tagged with this contest and
    /// logged since the session started.
    fn session_qsos(&self) -> Vec<QsoRecord> {
        let Some(s) = self.contest.as_ref() else { return Vec::new() };
        let id = s.contest.log_id();
        self.qso_log
            .iter()
            .filter(|q| q.contest_id == id && q.start_utc >= s.started_utc)
            .cloned()
            .collect()
    }

    /// Log the typed entry and advance the serial.
    fn log_contest_qso(&mut self, cmds: &mut Vec<Command>) {
        let Some(session) = self.contest.as_ref() else { return };
        let call = self.contest_entry.call.trim().to_ascii_uppercase();
        if call.is_empty() {
            return;
        }
        let contest = session.contest;
        let serial = contest.sends_serial().then_some(session.next_serial);
        // Our side: the serial and our own exchange on one line. The report is
        // what the operator typed, or the mode's default if they cleared it.
        let stx_string = session.sent_exchange(serial);
        let my_exchange = session.my_exchange.clone();
        let now = crate::time::now_unix_f64() as i64;

        // Their side: each received element has its own box, so nothing is
        // overwritten by the element next to it.
        let values: Vec<String> =
            self.contest_entry.fields.iter().map(|s| s.trim().to_string()).collect();
        let mut srx = None;
        let mut cq_zone = None;
        let mut grid = None;
        let mut state = String::new();
        for (ex, v) in contest.received_fields().iter().zip(values.iter()) {
            match ex {
                Exchange::Serial => srx = v.parse().ok(),
                Exchange::CqZone => cq_zone = v.parse().ok(),
                Exchange::Grid => {
                    if !v.is_empty() {
                        grid = Some(v.to_ascii_uppercase());
                    }
                }
                Exchange::State => state = v.to_ascii_uppercase(),
                _ => {}
            }
        }
        let mode = self.state.rx[0].mode.label().to_string();
        let rec = QsoRecord {
            call,
            grid,
            rst_sent: parse_rst(&self.contest_entry.rst_sent)
                .or_else(|| Some(default_report(&mode))),
            rst_rcvd: parse_rst(&self.contest_entry.rst_rcvd)
                .or_else(|| Some(default_report(&mode))),
            cq_zone,
            state,
            freq_hz: self.on_air_freq_hz(),
            mode,
            band: self.state.band.label().to_string(),
            start_utc: now,
            end_utc: now,
            my_call: self.my_call(),
            // The sponsor's id, not the UI label — an ADIF `CONTEST_ID` read by
            // anyone else's logger has to say what the sponsor calls it.
            contest_id: contest.log_id().to_string(),
            stx: serial,
            srx,
            stx_string: if stx_string.is_empty() { my_exchange } else { stx_string },
            srx_string: values.join(" "),
            ..Default::default()
        };
        cmds.push(Command::LogQso(Box::new(rec)));
        if let Some(s) = self.contest.as_mut()
            && s.contest.sends_serial()
        {
            s.next_serial = sdroxide_types::next_contest_serial(s.next_serial);
        }
        self.contest_entry = Default::default();
        // Start the next entry with the report default and the right number of
        // exchange boxes, rather than blank ones.
        self.reset_entry_reports();
    }
}

/// "RST + SERIAL + GRID" — the exchange spelled out for the setup screen.
fn exchange_hint(fields: &[Exchange]) -> String {
    if fields.is_empty() {
        return "—".to_string();
    }
    fields.iter().map(|f| f.label()).collect::<Vec<_>>().join(" + ")
}

/// The FT8 message layout a contest logger session should put the digi engine
/// into: EU VHF for an EU VHF contest, the serial layout for CQ WPX and the
/// generic serial one, and `None` for a contest with no FT8 layout — a CQ WW
/// zone exchange, or the CB activity, are typed by hand.
fn digi_contest_for(c: ContestId) -> sdroxide_types::ContestMode {
    match c {
        ContestId::EuVhf => sdroxide_types::ContestMode::EuVhf,
        ContestId::CqWpx | ContestId::Generic => sdroxide_types::ContestMode::RttyRoundup,
        _ => sdroxide_types::ContestMode::None,
    }
}

fn parse_rst(s: &str) -> Option<i16> {
    s.trim().parse().ok()
}

/// The report we send when the operator has not typed one: `599` on CW, and
/// `59` on everything else.
///
/// Copying the *received* report into the sent one — as the original did — told
/// the other station we heard them exactly as well as they heard us, which is
/// not a claim the operator made.
fn default_report(mode: &str) -> i16 {
    if sdroxide_types::cabrillo_mode(mode) == "CW" { 599 } else { 59 }
}
