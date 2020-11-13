use strum_macros::Display;
use egui::containers::{combo_box, Frame};
use serde::{Serialize, Deserialize};
use egui::{Stroke, Srgba, Slider, TextEdit};

mod app_data;

use crate::app_data::AppData;
use crate::app_data::weapons::*;

const APP_NAME: &str = "data";

impl egui::app::App for AppData {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn ui(
        &mut self,
        ctx: &std::sync::Arc<egui::Context>,
        integration_context: &mut egui::app::IntegrationContext,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::auto_sized().always_show_scroll(true).show(ui, |ui| {
                ui.heading("Data editor");
                ui.separator();

                ui.collapsing("Weapons", |ui| {
                    let mut dropped_weapons = Vec::new();

                    if ui.button("Add weapon").clicked {
                        self.weapons.push(Default::default())
                    }

                    ui.advance_cursor(8.0);

                    // NOTE on combo boxes:
                    // We can't use combo_box_with_label because it generates its ID == label, which results in conflict.
                    // The ID must be valid only when combo box is opened (it should not have a state otherwise),
                    // and as clicking on <Drop> button will close it, it is safe to just use a counter.
                    // (verified by experiment, not by reading the code)

                    for (i, weapon) in self.weapons.iter_mut().enumerate() {
                        ui.separator();
                        ui.advance_cursor(8.0);

                        ui.horizontal(|ui| {
                            ui.text_edit(&mut weapon.name);
                            ui.label("Name");
                        });

                        ui.horizontal(|ui| {
                            ui.add(TextEdit::new(&mut weapon.fluff).multiline(true));
                            ui.label("Fluff");
                        });

                        ui.add(Slider::i32(&mut weapon.cost, -5..=10).text("Cost"));

                        ui.horizontal(|ui| {
                            combo_box(ui, ui.make_persistent_id(format!("Weapon class {}", i)), &weapon.class.to_string(), |ui| {
                                ui.radio_value(&mut weapon.class, WeaponClass::Primary, WeaponClass::Primary.to_string());
                                ui.radio_value(&mut weapon.class, WeaponClass::Superheavy, WeaponClass::Superheavy.to_string());
                                ui.radio_value(&mut weapon.class, WeaponClass::Auxiliary, WeaponClass::Auxiliary.to_string());
                            });
                            ui.label("Weapon class")
                        });

                        ui.horizontal(|ui| {
                            combo_box(ui, ui.make_persistent_id(format!("Targeting {}", i)), &weapon.targeting.to_string(), |ui| {
                                ui.radio_value(&mut weapon.targeting, WeaponTargeting::SingleTarget, WeaponTargeting::SingleTarget.to_string());
                                ui.radio_value(&mut weapon.targeting, WeaponTargeting::Area, WeaponTargeting::Area.to_string());
                            })
                        });

                        ui.add(Slider::i32(&mut weapon.range_max, 0..=5).text("Maximum range"));
                        ui.add(Slider::i32(&mut weapon.range_min, 0..=5).text("Minimum range"));

                        // TODO: weapon tags. Requires input of the associated data (and probably a giant match block)

                        ui.horizontal(|ui| {
                            ui.text_edit(&mut weapon.damage);
                            ui.label("Damage");
                        });

                        ui.horizontal(|ui| {
                            ui.add(TextEdit::new(&mut weapon.rules).multiline(true));
                            ui.label("Rules");
                        });

                        if ui.button("Drop").clicked {
                            dropped_weapons.push(i);
                        }
                    }

                    // NOTE: this has O(n^2) complexity, but there should be < 1000 ships, which means < 1M ops
                    // over sequential data, done only on user interaction. This should be OK.
                    for i in dropped_weapons { self.weapons.remove(i); }
                });


                // WORKAROUND
                ui.advance_cursor(800.0);
                ui.separator();
            });
        });

        integration_context.output.window_size = Some(ctx.used_size()); // resize the window to be just the size we need it to be
    }

    fn on_exit(&mut self, storage: &mut dyn egui::app::Storage) {
        egui::app::set_value(storage, APP_NAME, self);
    }
}

fn main() {
    let title = "Oxidized Battlegroup";

    // Persist app state to file:
    let storage = egui_glium::storage::FileStorage::from_path("data.json".into());

    // Alternative: store nowhere
    // let storage = egui::app::DummyStorage::default();

    let app: AppData = egui::app::get_value(&storage, APP_NAME).unwrap_or_default(); // Restore `MyApp` from file, or create new `MyApp`.
    egui_glium::run(title, Box::new(storage), app);
}