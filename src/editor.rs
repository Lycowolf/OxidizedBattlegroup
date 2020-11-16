use egui::containers::{combo_box, Frame};
use egui::{Slider, TextEdit, Style, Ui};
use serde::{Serialize, Deserialize};

mod app_data;

use crate::app_data::AppData;
use crate::app_data::weapons::*;
use std::fs::File;
use std::io::Write;
use core::mem;

const APP_NAME: &str = "data";

#[derive(Serialize, Deserialize, Clone, Default)]
struct EditorData {
    app_data: AppData,
    new_weapon_tag: WeaponTag,
    new_weapon: Weapon,
}

impl egui::app::App for EditorData {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn ui(
        &mut self,
        ctx: &std::sync::Arc<egui::Context>,
        integration_context: &mut egui::app::IntegrationContext,
    ) {
        // decompose AppData to shorten the code and avoid issues with repeated borrowing of self
        let EditorData { app_data, new_weapon_tag, new_weapon } = self;
        let AppData { weapon_tags, weapons } = app_data;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::auto_sized().always_show_scroll(true).show(ui, |ui| {
                ui.heading("Data editor");
                ui.separator();

                ui.collapsing("Weapon tags", |ui| {
                    let mut dropped_tags: Option<usize> = None;

                    ui.collapsing("Add weapon tag", |ui| {
                        ui.horizontal(|ui| {
                            ui.text_edit(&mut new_weapon_tag.name);
                            ui.label("Name");
                        });

                        ui.horizontal(|ui| {
                            ui.add(TextEdit::new(&mut new_weapon_tag.fluff).multiline(true));
                            ui.label("Fluff");
                        });

                        if ui.button("Add tag").clicked {
                            weapon_tags.push(new_weapon_tag.clone());
                            mem::swap(new_weapon_tag, &mut WeaponTag::default());
                        }
                    });

                    for (i, tag) in weapon_tags.iter_mut().enumerate() {
                        ui.vertical(|ui| {
                            ui.advance_cursor(8.0);

                            ui.horizontal(|ui| {
                                ui.text_edit(&mut tag.name);
                                ui.label("Name");
                            });

                            ui.horizontal(|ui| {
                                ui.add(TextEdit::new(&mut tag.fluff).multiline(true));
                                ui.label("Fluff");
                            });

                            if ui.button("Drop").clicked {
                                dropped_tags = Some(i);
                            }
                        });
                    }

                    if let Some(i) = dropped_tags { weapon_tags.remove(i); }
                });

                // sort weapon_tags (we need binary_search())
                let mut sorted_tags = weapon_tags.clone();
                sorted_tags.sort_by_key(|tag| tag.name.clone());
                mem::swap(weapon_tags, &mut sorted_tags); // weapon_tags are now sorted

                ui.collapsing("Weapons", |ui| {
                    let mut dropped_weapons = Option::None;

                    if ui.button("Add weapon").clicked {
                        weapons.push(Default::default())
                    }

                    ui.advance_cursor(8.0);

                    // NOTE on combo boxes:
                    // We can't use combo_box_with_label because it generates its ID == label, which results in conflict.
                    // The ID must be valid only when combo box is opened (it should not have a state otherwise),
                    // and as clicking on <Drop> button will close it, it is safe to just use a counter.
                    // (verified by experiment, not by reading the code)

                    for (i, weapon) in weapons.iter_mut().enumerate() {
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

                        let mut removed_tags: Option<usize> = Option::None; // TODO

                        ui.horizontal(|ui| {
                            for (j, weapon_tag) in weapon.tags.iter_mut().enumerate() {
                                Frame::dark_canvas(&Style::default()).show(ui, |ui| {
                                    if ui.button("[X]").clicked {
                                        removed_tags = Some(j);
                                    }

                                    if let Ok(tag_num) = weapon_tags.binary_search_by_key(weapon_tag, |tag| tag.name.clone()) {
                                        let selected_tag = &weapon_tags[tag_num];
                                        combo_box(ui, ui.make_persistent_id(format!("Weapon {} tag {}", i, j)), &selected_tag.name, |ui: &mut Ui| {
                                            for tag in weapon_tags.iter() {
                                                ui.horizontal(|ui| {
                                                    if ui.button(&tag.name).clicked {
                                                        mem::swap(weapon_tag, &mut tag.name.clone());
                                                    }
                                                    ui.separator();
                                                    ui.label(&tag.fluff);
                                                });
                                            };
                                        });
                                    } else { //invalid tag ID
                                        ui.label(format!("!!! Invalid Tag ID: {} !!!", weapon_tag));
                                    }
                                });
                            }

                            if let Some(j) = removed_tags { weapon.tags.remove(j); }

                            if ui.button("Add Tag").clicked {
                                // don't crash just because we haven't defined any tags yet
                                if let Some(default_tag) = weapon_tags.get(0) {
                                    weapon.tags.push(default_tag.name.clone())
                                }
                            }
                        });


                        ui.horizontal(|ui| {
                            ui.text_edit(&mut weapon.damage);
                            ui.label("Damage");
                        });

                        ui.horizontal(|ui| {
                            ui.add(TextEdit::new(&mut weapon.rules).multiline(true));
                            ui.label("Rules");
                        });

                        if ui.button("Drop").clicked {
                            dropped_weapons = Some(i);
                        }
                    }

                    if let Some(i) = dropped_weapons { weapons.remove(i); }
                });


                // WORKAROUND
                ui.advance_cursor(800.0);
                ui.separator();
                ui.small("The gap above is a workaround for a bug in egui. \
                    Floating menus (used in combo boxes) do not count for parent element size, \
                    and would get clipped when longer than the UI."
                )
            });
        });

        integration_context.output.window_size = Some(ctx.used_size()); // resize the window to be just the size we need it to be
    }

    fn on_exit(&mut self, storage: &mut dyn egui::app::Storage) {
        egui::app::set_value(storage, APP_NAME, self);
        let mut file = File::create("data.json").unwrap();
        file.write(&serde_json::to_vec(self).unwrap());
        file.sync_all().unwrap();
    }
}

fn main() {
    let title = "Data editor for Oxidized Battlegroup";

    // Persist app state to file:
    let storage = egui_glium::storage::FileStorage::from_path("egui_data.json".into());

    // Alternative: store nowhere
    // let storage = egui::app::DummyStorage::default();

    let app: EditorData = egui::app::get_value(&storage, APP_NAME).unwrap_or_default(); // Restore `MyApp` from file, or create new `MyApp`.
    egui_glium::run(title, Box::new(storage), app);
}