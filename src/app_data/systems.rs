use strum_macros::Display;
use serde::{Serialize, Deserialize};
use egui::{Ui, TextEdit, Slider, Style};
use core::mem;
use egui::containers::{combo_box, Frame};
use indexmap::map::IndexMap;
use crate::app_data::tags::{TagId, Tag};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct System {
    pub name: String,
    pub fluff: String,
    pub cost: i32,
    pub limited: Option<i32>,
    pub tags: Vec<TagId>,
    pub rules: String,
    // TODO: decide how to store maneuvers and tactics
}

impl System {
    pub fn editable_ui(&mut self, ui: &mut Ui, id: usize, tag_db: &IndexMap<TagId, Tag>) -> bool {
        ui.separator();
        ui.advance_cursor(8.0);

        ui.horizontal(|ui| {
            ui.text_edit(&mut self.name);
            ui.label("Name");
        });

        ui.horizontal(|ui| {
            ui.add(TextEdit::new(&mut self.fluff).multiline(true));
            ui.label("Fluff");
        });

        ui.add(Slider::i32(&mut self.cost, -5..=10).text("Cost"));

        ui.horizontal(|ui| {
            match self.limited {
                None => {
                    if ui.button("Unlimited").clicked {
                        self.limited = Some(1)
                    }
                }
                Some(_) => {
                    if ui.button("Limited").clicked {
                        self.limited = None
                    }
                }
            }
            if let Some(limit) = self.limited.as_mut() {
                ui.add(Slider::i32(limit, 1..=5).text("")); // text also adds numeric value input box
            }
        });

        // TODO: copied from weapons, deduplicate
        let mut removed_tags: Option<usize> = Option::None;
        ui.horizontal(|ui| {
            for (j, tag_name) in self.tags.iter_mut().enumerate() {
                if Tag::selector_ui(tag_name, ui, format!("System {} tag {}", id, j), tag_db) {
                    removed_tags = Some(j)
                }
            }

            if let Some(j) = removed_tags { self.tags.remove(j); }

            if ui.button("Add Tag").clicked {
                // don't crash just because we haven't defined any tags yet
                if let Some((default_tag_name, _)) = tag_db.iter().next() {
                    self.tags.push(default_tag_name.clone())
                }
            }
        });

        ui.horizontal(|ui| {
            ui.add(TextEdit::new(&mut self.rules).multiline(true));
            ui.label("Rules");
        });

        ui.button("Drop").clicked
    }
}