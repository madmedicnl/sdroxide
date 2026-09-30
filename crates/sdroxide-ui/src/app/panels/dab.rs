//! The DAB / DAB+ panel: the ensemble, and the services it carries.
//!
//! A service list is the whole of what an operator does with DAB — tune a
//! channel, see what is on it, pick a station — so that is the panel. Received
//! audio plays through the ordinary speaker path, so there is no waveform to
//! draw here.
//!
//! The header says what the receiver is doing, for the reason the ADS-B panel's
//! does: an empty list has several quite different causes — a quiet channel, a
//! receiver not tuned to Band III, a stream too narrow to hold an ensemble — and
//! only one of them is anything to do with the decoder.

use eframe::egui::{self, RichText};
use sdroxide_types::{Command, DabStatus};

use crate::app::SdroxideApp;
use crate::theme;

impl SdroxideApp {
    pub(in crate::app) fn dab_panel(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        panel_h: f32,
    ) {
        let st: DabStatus = match self.dab_status.as_ref() {
            Some(s) => (**s).clone(),
            None => {
                ui.label(RichText::new("starting the DAB receiver…").weak());
                return;
            }
        };
        self.dab_header(ui, cmds, &st);
        ui.add_space(4.0);

        let avail_h = (panel_h - ui.cursor().top().min(panel_h) + ui.cursor().top()).max(80.0);
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), avail_h.max(80.0)),
            egui::Layout::top_down(egui::Align::Min),
            |ui| self.dab_list(ui, cmds, &st),
        );
    }

    /// The channel picker, what the receiver is doing, and the one sentence
    /// that fixes the commonest problem.
    fn dab_header(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>, st: &DabStatus) {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("CHANNEL").size(10.0).color(theme::CYAN_DIM()));
            // What a scan has found, which is the operator's own country — not
            // a hardcoded list that only fits one. Falls back to a couple of
            // common blocks before the first scan.
            let channels: Vec<String> = if self.state.dab.found.is_empty() {
                ["8B", "11C", "12B", "12C"].iter().map(|s| s.to_string()).collect()
            } else {
                self.state.dab.found.clone()
            };
            for name in &channels {
                let here = self.state.dab.channel == *name;
                if crate::chrome::chip(ui, here, name).clicked() {
                    self.state.dab.channel = name.clone();
                    cmds.push(Command::SetDabConfig(self.state.dab.clone()));
                    // Put the dial on the channel, as the ADS-B lane does with
                    // 1090 MHz: the decoder is fed from a window there, and an
                    // operator who picked a channel should see the receiver
                    // move to it rather than have to tune twice.
                    if let Some((_, hz)) =
                        sdroxide_types::DAB_BAND_III.iter().find(|(n, _)| n == name)
                    {
                        cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz: *hz });
                    }
                }
            }
            // The scan itself. Sweeping every Band III block is how an operator
            // learns which carry a transmission where they are; the ones that
            // do become the picker above.
            let scanning = self.dab_scan.is_some();
            if crate::chrome::chip(ui, scanning, if scanning { "SCANNING…" } else { "SCAN" })
                .on_hover_text(
                    "Walk every Band III block and keep the ones that carry an ensemble. \
                     What is found becomes the channel list above, and is remembered.",
                )
                .clicked()
                && !scanning
            {
                self.dab_scan = Some(crate::app::frame::DabScan {
                    at: 0,
                    since: None,
                    found: Vec::new(),
                });
                // Start the sweep on the first block at once.
                self.state.dab.channel = sdroxide_types::DAB_BAND_III[0].0.to_string();
                cmds.push(Command::SetVfo {
                    vfo: self.state.active_vfo,
                    hz: sdroxide_types::DAB_BAND_III[0].1,
                });
                cmds.push(Command::SetDabConfig(self.state.dab.clone()));
            }
            ui.separator();
            if let Some(e) = &st.ensemble {
                ui.label(RichText::new(e).strong());
            } else if st.unavailable.is_none() {
                ui.label(
                    RichText::new("no ensemble named yet — is a DAB transmission on this channel?")
                        .weak(),
                );
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{:.0} MHz", st.window_center_hz / 1e6))
                        .size(10.0)
                        .color(theme::gray(140)),
                );
            });
        });
        if let Some(why) = &st.unavailable {
            ui.label(RichText::new(why).color(theme::ALERT()).size(11.0));
        } else if let Some(why) = &st.degraded {
            ui.label(RichText::new(why).color(theme::YELLOW()).size(11.0));
        }
    }

    /// The ensemble's services, the one playing marked, each a press to play.
    fn dab_list(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>, st: &DabStatus) {
        if st.services.is_empty() {
            ui.add_space(8.0);
            ui.label(
                RichText::new(if st.unavailable.is_some() {
                    "the receiver cannot hold a DAB ensemble here"
                } else {
                    "nothing decoded yet — the FIC takes a moment after the channel is in tune"
                })
                .weak(),
            );
            return;
        }
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for s in &st.services {
                    let playing = st.playing.as_deref() == Some(s.service_id.as_str());
                    let label = if s.label.trim().is_empty() {
                        format!("({})", s.service_id)
                    } else {
                        s.label.clone()
                    };
                    ui.horizontal(|ui| {
                        if crate::chrome::chip(ui, playing, label).clicked() {
                            self.state.dab.service_id = s.service_id.clone();
                            cmds.push(Command::SetDabConfig(self.state.dab.clone()));
                        }
                        let mut extra = s.service_id.clone();
                        if let Some(b) = s.bitrate {
                            extra.push_str(&format!(" · {b} kbps"));
                        }
                        if let Some(p) = &s.protection {
                            extra.push_str(&format!(" · {p}"));
                        }
                        if let Some(c) = s.subchannel {
                            extra.push_str(&format!(" · sub {c}"));
                        }
                        ui.label(RichText::new(extra).size(10.0).color(theme::gray(140)));
                    });
                }
            });
    }
}
