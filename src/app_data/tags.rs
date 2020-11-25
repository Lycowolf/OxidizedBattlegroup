use serde::{Serialize, Deserialize};
use egui::{Ui, TextEdit, Slider, Style};
use core::mem;
use egui::containers::{combo_box, Frame};
use indexmap::map::IndexMap;

pub type TagId = String; // tag names

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Tag {
    pub name: TagId,
    pub fluff: String,
}

impl Tag {
    pub fn selector_ui(tag_name: &mut TagId, ui: &mut Ui, id: String, tag_db: &IndexMap<TagId, Tag>) -> bool {
        Frame::dark_canvas(&Style::default()).show(ui, |ui| {
            if let Some(selected_tag) = tag_db.get(tag_name) {
                combo_box(ui, ui.make_persistent_id(id), &selected_tag.name, |ui: &mut Ui| {
                    for (_tag_name, tag) in tag_db.iter() {
                        ui.horizontal(|ui| {
                            if ui.button(&tag.name).clicked {
                                mem::swap(tag_name, &mut tag.name.clone());
                            }
                            ui.separator();
                            ui.label(&tag.fluff);
                        });
                    };
                });
            } else { //invalid tag ID
                ui.label(format!("!!! Invalid Tag ID: {} !!!", tag_name));
            }

            ui.button("[X]").clicked
        })
    }
}
