#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use crate::{
    KeyboardDevice,
    utils::{
        actions::{ComplexAction, KeyAction, KeyCode},
        layout::{KEYBOARD_LAYOUT, PositionedKey},
    },
};
use eframe::egui;
use strum::IntoEnumIterator;

struct GamakayToolApp<'a> {
    cur_selected_key: Option<&'a PositionedKey>,
    keyboard_device: KeyboardDevice,
    selected_rebind: Box<dyn KeyAction>,
}

const SCALE: f32 = 1.25;

impl eframe::App for GamakayToolApp<'_> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Gamakay Tool");

            let (area, _) = ui.allocate_exact_size(egui::vec2(400.0, 150.0), egui::Sense::hover());
            let base_pos = area.min;

            for pk in KEYBOARD_LAYOUT {
                let key_rect = egui::Rect::from_min_size(
                    base_pos + egui::vec2(pk.x as f32 * SCALE, pk.y as f32 * -SCALE),
                    egui::vec2(pk.w as f32 * SCALE, pk.h as f32 * SCALE),
                );

                if ui
                    .put(
                        key_rect,
                        egui::Button::new(
                            egui::RichText::new(pk.key.legend())
                                .text_style(egui::TextStyle::Monospace),
                        ),
                    )
                    .clicked()
                {
                    self.cur_selected_key = Some(&pk);
                    self.selected_rebind = Box::new(pk.key);
                }
            }

            ui.add_space(20.0);
            ui.separator();

            ui.vertical(|ui| {
                if let Some(&cur_pk) = self.cur_selected_key {
                    ui.label("Clicked button:");
                    ui.code(cur_pk.key.legend());
                    ui.label("Matrix index:");
                    ui.label(cur_pk.matrix_index.to_string());
                    ui.label("Keycode:");
                    ui.label(format!("{:#?}", cur_pk.key));
                    ui.label("KB Internal Representation:");
                    ui.label({
                        let mut keycode = format!("{:#?}", cur_pk.key.to_bytes());
                        keycode.retain(|c| c != '\n' && c != ' ');
                        keycode
                    });
                }
            });

            ui.add_space(20.0);
            ui.separator();

            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", &self.selected_rebind))
                .show_ui(ui, |ui| {
                    for a in KeyCode::iter() {
                        let ka: Box<dyn KeyAction> = Box::new(a);
                        ui.selectable_value(
                            &mut self.selected_rebind,
                            ka,
                            egui::RichText::new(format!("{:#?}", a))
                                .text_style(egui::TextStyle::Monospace),
                        );
                    }
                    for a in ComplexAction::iter() {
                        let ka: Box<dyn KeyAction> = Box::new(a);
                        ui.selectable_value(
                            &mut self.selected_rebind,
                            ka,
                            egui::RichText::new(format!("{:#?}", a))
                                .text_style(egui::TextStyle::Monospace),
                        );
                    }
                });

            if ui
                .add_enabled(
                    self.cur_selected_key.is_some(),
                    egui::Button::new("Apply rebind"),
                )
                .clicked()
                && let Some(&cur_pk) = self.cur_selected_key
            {
                let _result = self
                    .keyboard_device
                    .send_key_remap(cur_pk.matrix_index, &self.selected_rebind.to_bytes());
            }
        });
    }
}

pub fn run_gui(keyboard_device: KeyboardDevice) -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gamakay Tool",
        options,
        Box::new(|_cc| {
            Ok(Box::<GamakayToolApp>::new(GamakayToolApp {
                cur_selected_key: None,
                selected_rebind: Box::new(KeyCode::None),
                keyboard_device,
            }))
        }),
    )
}
