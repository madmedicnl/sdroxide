//! The JTTY panel: the messages decoded, newest first.
//!
//! JTTY is a keyboard-to-keyboard text mode, so the panel is a log rather than
//! a table: one line per message with its UTC stamp, audio frequency and SNR.
//! A message is free text — a callsign CQ, a `599` exchange, a grid, a control
//! phrase — so there are no per-field columns to break it into; the line is the
//! text.
//!
//! Receive only: there is no transmit row here yet.

use eframe::egui::{self, RichText};
use sdroxide_types::{Command, JttyStatus};

use crate::app::SdroxideApp;
use crate::theme;

impl SdroxideApp {
    pub(in crate::app) fn jtty_panel(
        &mut self,
        ui: &mut egui::Ui,
        _cmds: &mut Vec<Command>,
        _panel_h: f32,
    ) {
        let st: Option<JttyStatus> = self.digi_status.as_ref().and_then(|s| s.jtty.clone());
        let Some(st) = st else {
            ui.label(RichText::new("starting the JTTY receiver…").weak());
            return;
        };

        ui.horizontal(|ui| {
            ui.label(RichText::new("JTTY").strong().color(theme::CYAN()));
            // The audio level: an asynchronous mode is bursts between silences,
            // and the meter tells "nothing on the channel" from "nothing
            // decoded".
            ui.add(
                egui::ProgressBar::new(st.level.clamp(0.0, 1.0))
                    .desired_width(70.0)
                    .fill(theme::CYAN_DIM()),
            )
            .on_hover_text("Receive audio level");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{} rx", st.total)).size(10.0).color(theme::gray(120)),
                );
            });
        });
        ui.add_space(4.0);
        ui.separator();

        if st.messages.is_empty() {
            ui.label(
                RichText::new(
                    "Listening. JTTY transmissions start at any time — a message \
                     appears here a moment after it is heard.",
                )
                .size(10.0)
                .weak(),
            );
            return;
        }

        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            for m in st.messages.iter().rev() {
                let secs = m.at_unix.rem_euclid(86_400);
                let stamp = format!("{:02}:{:02}:{:02}", secs / 3600, (secs / 60) % 60, secs % 60);
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(stamp).monospace().size(11.0).color(theme::gray(120)),
                    );
                    ui.label(
                        RichText::new(format!("{:>5.0}", m.audio_hz))
                            .monospace()
                            .size(11.0)
                            .color(theme::gray(120)),
                    );
                    ui.label(
                        RichText::new(format!("{:>3}", m.snr_db))
                            .monospace()
                            .size(11.0)
                            .color(theme::CYAN_DIM()),
                    );
                    // A partial message (no end-of-message flag) is dimmed, so a
                    // run cut off by the next transmission reads as incomplete.
                    let color = if m.complete { theme::TEXT() } else { theme::gray(140) };
                    ui.label(RichText::new(&m.text).monospace().size(12.5).color(color));
                });
            }
        });
    }
}
