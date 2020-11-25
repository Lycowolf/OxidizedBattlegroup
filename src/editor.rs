use egui::containers::{combo_box, Frame};
use egui::{Slider, TextEdit, Style, Ui};
use serde::{Serialize, Deserialize};

mod app_data;

use crate::app_data::AppData;
use crate::app_data::weapons::*;
use std::fs::File;
use std::io::Write;
use indexmap::map::IndexMap;
use crate::app_data::tags::Tag;

const APP_NAME: &str = "data";

impl egui::app::App for AppData {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn ui(
        &mut self,
        ctx: &std::sync::Arc<egui::Context>,
        integration_context: &mut egui::app::IntegrationContext,
    ) {
        // decompose AppData to shorten the code and avoid issues with repeated borrowing of self
        let AppData { tag_db, weapons, systems } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::auto_sized().always_show_scroll(true).show(ui, |ui| {
                ui.heading("Data editor");
                ui.separator();

                ui.collapsing("Tags", |ui| {
                    let mut dropped_tag: Option<usize> = None;

                    ui.horizontal(|ui| {
                        if ui.button("Add tag").clicked {
                            tag_db.push(Tag::default());
                        }
                        ui.separator();
                        if ui.button("Sort").clicked {
                            tag_db.sort_by_key(|tag| tag.name.clone());
                        }
                    });

                    for (i, tag) in tag_db.iter_mut().enumerate() {
                        ui.vertical(|ui| {
                            ui.advance_cursor(8.0);

                            ui.horizontal(|ui| {
                                ui.add(TextEdit::new(&mut tag.name).multiline(false));
                                ui.label("Name");
                            });

                            ui.horizontal(|ui| {
                                ui.add(TextEdit::new(&mut tag.fluff).multiline(true));
                                ui.label("Fluff");
                            });

                            if ui.button("Drop").clicked {
                                dropped_tag = Some(i);
                            }
                        });
                    }

                    if let Some(i) = dropped_tag { tag_db.remove(i); }
                });

                let mut tag_map: IndexMap<String, Tag> = tag_db.iter().map(|tag| (tag.name.clone(), tag.clone())).collect();
                tag_map.sort_keys();


                ui.collapsing("Weapons", |ui| {
                    let mut dropped_weapon = Option::None;

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
                        if weapon.editable_ui(ui, i, &tag_map) { dropped_weapon = Some(i) }
                    }

                    if let Some(i) = dropped_weapon { weapons.remove(i); }
                });

                ui.collapsing("Systems", |ui| {
                    let mut dropped_system = Option::None;

                    if ui.button("Add system").clicked {
                        systems.push(Default::default())
                    }

                    ui.advance_cursor(8.0);

                    for (i, system) in systems.iter_mut().enumerate() {
                        if system.editable_ui(ui, i, &tag_map) { dropped_system = Some(i) }
                    }

                    if let Some(i) = dropped_system { systems.remove(i); }
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
        file.write(&serde_json::to_vec(self).unwrap()).unwrap();
        file.sync_all().unwrap();
    }
}

fn main() {
    let title = "Data editor for Oxidized Battlegroup";

    // Persist app state to file:
    let storage = egui_glium::storage::FileStorage::from_path("egui_data.json".into());

    // Alternative: store nowhere
    // let storage = egui::app::DummyStorage::default();

    let app: AppData = egui::app::get_value(&storage, APP_NAME).unwrap_or_default(); // Restore `MyApp` from file, or create new `MyApp`.
    egui_glium::run(title, Box::new(storage), app);
}