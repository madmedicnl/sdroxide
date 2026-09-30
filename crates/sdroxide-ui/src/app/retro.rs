//! The Retro Radio faceplate — [`sdroxide_types::UiSettings::retro_radio`].
//!
//! A listener's skin over the same engine: one big tuning scale and a needle,
//! band and mode selectors, a volume control, an S-meter and an optional decode
//! window. It holds no station state of its own — every control reaches the
//! radio through the ordinary commands — so switching it off restores the
//! normal workspace exactly as it was, and nothing here is on the wire.

use eframe::egui;
use sdroxide_types::{Band, Command, Mode, RxId, SmeterStyle};

use super::SdroxideApp;

/// The listener's bands, in dial order — the service bands this fork adds, not
/// the amateur ones the OPERATE side already covers.
const BANDS: &[Band] = &[Band::Lw, Band::Mw, Band::Sw, Band::Fm, Band::Air, Band::Mil];

/// The listener's modes. Offered with `SetModeListen`, so no band rule stands
/// between the operator and the dial.
const MODES: &[Mode] = &[Mode::Am, Mode::Sam, Mode::Usb, Mode::Lsb, Mode::Nfm, Mode::Wfm, Mode::Cw];

// Faceplate inks, fixed rather than themed so the wood reads as wood on any
// palette — the one place in the program that does not take its colours from
// the theme.
const BEZEL: egui::Color32 = egui::Color32::from_rgb(30, 21, 14);
const WOOD: egui::Color32 = egui::Color32::from_rgb(84, 54, 33);
const WOOD_EDGE: egui::Color32 = egui::Color32::from_rgb(56, 35, 20);
const BRASS: egui::Color32 = egui::Color32::from_rgb(198, 163, 98);
const SCALE_BG: egui::Color32 = egui::Color32::from_rgb(20, 16, 12);
const SCALE_TICK: egui::Color32 = egui::Color32::from_rgb(150, 130, 96);

impl SdroxideApp {
    /// Draw the faceplate in place of the normal workspace.
    ///
    /// Called from `frame::ui` where the panadapter would be; the dialogs and
    /// the command flush below it still run, so Settings — and turning this
    /// mode off — stay reachable.
    pub(in crate::app) fn retro_view(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let rect = ui.available_rect_before_wrap();
        self.retro_faceplate(ui, rect);

        let body = rect.shrink(24.0);
        ui.scope_builder(egui::UiBuilder::new().max_rect(body), |ui| {
            ui.vertical(|ui| {
                self.retro_readout(ui, cmds);
                ui.add_space(12.0);
                self.retro_scale(ui, cmds);
                ui.add_space(14.0);
                self.retro_selectors(ui, cmds);
                ui.add_space(14.0);
                self.retro_bottom(ui, cmds);
            });
        });

        if self.retro_decode_open {
            self.retro_decode_window(ui, cmds);
        }
    }

    /// The wooden slab behind everything: bezel, a few grain streaks and a
    /// brass edge.
    fn retro_faceplate(&self, ui: &egui::Ui, rect: egui::Rect) {
        let p = ui.painter();
        p.rect_filled(rect, 12.0, BEZEL);
        let wood = rect.shrink(7.0);
        p.rect_filled(wood, 9.0, WOOD);
        // Grain: deterministic from the rect, so it does not crawl frame to
        // frame the way a random streak would.
        let mut y = wood.top() + 22.0;
        let mut i = 0u32;
        while y < wood.bottom() - 14.0 {
            let wob = ((i * 37) % 11) as f32 - 5.0;
            p.line_segment(
                [egui::pos2(wood.left() + 12.0, y + wob), egui::pos2(wood.right() - 12.0, y - wob)],
                egui::Stroke::new(1.0, WOOD_EDGE),
            );
            y += 28.0;
            i += 1;
        }
        p.rect_stroke(wood, 9.0, egui::Stroke::new(1.0, BRASS), egui::StrokeKind::Inside);
    }

    /// The nameplate and the big lit frequency readout.
    fn retro_readout(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("SDR OXIDE").size(22.0).strong().color(BRASS));
            ui.label(egui::RichText::new("BROWN").size(22.0).strong().color(WOOD_EDGE));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(self.state.band.label()).size(18.0).strong().color(BRASS),
                );
            });
        });
        ui.add_space(6.0);
        let hz = self.state.active_freq_hz();
        let wheel = self.input.cfg.wheel;
        if let Some(new_hz) = crate::widgets::freq_display::show(
            ui,
            ui.id().with("retro-freq"),
            hz,
            wheel,
            30.0,
            None,
            crate::widgets::freq_display::DIGITS,
        ) {
            cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz: new_hz });
        }
    }

    /// The tuning scale: ticks over the current band, a needle at the dial, and
    /// drag/wheel tuning.
    fn retro_scale(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let (rect, resp) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), 96.0),
            egui::Sense::click_and_drag(),
        );
        let hz = self.state.active_freq_hz();
        let band = self.state.band;
        // `None` only for general coverage; a listener's bands all have edges.
        let (lo, hi) = band.edges().unwrap_or((hz - 500_000.0, hz + 500_000.0));
        {
            let p = ui.painter_at(rect);
            p.rect_filled(rect, 6.0, SCALE_BG);
            let w = rect.width();
            for i in 0..=20 {
                let f = i as f32 / 20.0;
                let x = rect.left() + f * w;
                let major = i % 5 == 0;
                let h = if major { rect.height() * 0.46 } else { rect.height() * 0.26 };
                p.line_segment(
                    [egui::pos2(x, rect.bottom()), egui::pos2(x, rect.bottom() - h)],
                    egui::Stroke::new(if major { 1.6 } else { 1.0 }, SCALE_TICK),
                );
            }
            let frac = ((hz - lo) / (hi - lo)).clamp(0.0, 1.0) as f32;
            let x = rect.left() + frac * w;
            p.line_segment(
                [egui::pos2(x, rect.top() + 5.0), egui::pos2(x, rect.bottom())],
                egui::Stroke::new(2.0, BRASS),
            );
            p.circle_filled(egui::pos2(x, rect.bottom()), 4.5, BRASS);
        }
        // Drag the needle.
        if resp.dragged() || resp.clicked() {
            if let Some(pos) = resp.interact_pointer_pos() {
                let f = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0) as f64;
                cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz: lo + f * (hi - lo) });
            }
        }
        // Wheel over the scale: 1 kHz a notch, 100 Hz with Shift.
        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll.abs() > 0.0 {
                let step = if ui.input(|i| i.modifiers.shift) { 100.0 } else { 1_000.0 };
                let dir = if scroll > 0.0 { 1.0 } else { -1.0 };
                cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz: hz + dir * step });
            }
        }
    }

    /// BAND and MODE, as two rows of chips.
    fn retro_selectors(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("BAND").size(11.0).strong().color(BRASS));
            for b in BANDS {
                if crate::chrome::chip(ui, self.state.band == *b, b.label()).clicked() {
                    cmds.push(Command::SetBand(*b));
                }
            }
        });
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("MODE").size(11.0).strong().color(BRASS));
            let cur = self.state.rx[0].mode;
            for m in MODES {
                if crate::chrome::chip(ui, cur == *m, m.label()).clicked() {
                    cmds.push(Command::SetModeListen { rx: RxId::Main, mode: *m });
                }
            }
        });
    }

    /// VOLUME, the S-meter, and the decode-window toggle.
    fn retro_bottom(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("VOLUME").size(10.0).color(BRASS));
                let mut v = self.state.rx[0].volume;
                if ui
                    .add_sized(
                        [200.0, 24.0],
                        egui::Slider::new(&mut v, 0.0..=1.0).show_value(false),
                    )
                    .changed()
                {
                    cmds.push(Command::SetVolume { rx: RxId::Main, v });
                }
            });
            ui.add_space(24.0);
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("S-METER").size(10.0).color(BRASS));
                ui.allocate_ui_with_layout(
                    egui::vec2(240.0, 60.0),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        crate::widgets::smeter::show(ui, self.meters.as_ref(), SmeterStyle::Needle);
                    },
                );
            });
            ui.add_space(24.0);
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("DECODE").size(10.0).color(BRASS));
                if crate::chrome::chip(ui, self.retro_decode_open, " DECODE ").clicked() {
                    self.retro_decode_open = !self.retro_decode_open;
                }
            });
        });
    }

    /// The optional decode window: the current mode's operating panel in its own
    /// floating window, so a listener can watch the decoder over the faceplate.
    fn retro_decode_window(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let ctx = ui.ctx().clone();
        let mode = self.state.rx[0].mode;
        let mut open = self.retro_decode_open;
        egui::Window::new(format!("DECODE · {}", mode.label()))
            .id(egui::Id::new("retro-decode"))
            .open(&mut open)
            .default_size([480.0, 340.0])
            .resizable(true)
            .show(&ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.operating_panel(ui, cmds, mode, 280.0);
                });
            });
        self.retro_decode_open = open;
    }
}
