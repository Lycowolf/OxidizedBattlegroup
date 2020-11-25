use strum_macros::Display;
use serde::{Serialize, Deserialize};
use egui::{Ui, TextEdit, Slider, Style};
use core::mem;
use egui::containers::{combo_box, Frame};
use indexmap::map::IndexMap;
use crate::app_data::tags::{TagId, Tag};

#[derive(Serialize, Deserialize, Display, PartialEq, Ord, PartialOrd, Eq, Copy, Clone)]
pub enum WeaponClass {
    Superheavy,
    Primary,
    Auxiliary,
}

impl Default for WeaponClass {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(Serialize, Deserialize, Display, PartialEq, Ord, PartialOrd, Eq, Copy, Clone)]
pub enum WeaponTargeting {
    SingleTarget,
    Area,
}

impl Default for WeaponTargeting {
    fn default() -> Self {
        Self::SingleTarget
    }
}

// It would be nice to have type-safe tags, but some tags should have associated values (Charging, Reliable etc.)
// and it is hard to make a reasonable UI for this

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Weapon {
    pub name: String,
    pub fluff: String,
    pub cost: i32,
    pub class: WeaponClass,
    pub targeting: WeaponTargeting,
    pub tags: Vec<TagId>,
    pub range_min: i32,
    pub range_max: i32,
    pub damage: String,
    pub rules: String,
}

impl Weapon {
    /// Adds a GUI widgets that edit the current Weapon. Returns true if the current item should be deleted.
    /// # Arguments
    /// * ui: egui UI handle
    /// * id: unique number (used to generate egui IDs); just use a counter
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
            combo_box(ui, ui.make_persistent_id(format!("Weapon class {}", id)), &self.class.to_string(), |ui| {
                ui.radio_value(&mut self.class, WeaponClass::Primary, WeaponClass::Primary.to_string());
                ui.radio_value(&mut self.class, WeaponClass::Superheavy, WeaponClass::Superheavy.to_string());
                ui.radio_value(&mut self.class, WeaponClass::Auxiliary, WeaponClass::Auxiliary.to_string());
            });
            ui.label("Weapon class")
        });

        ui.horizontal(|ui| {
            combo_box(ui, ui.make_persistent_id(format!("Targeting {}", id)), &self.targeting.to_string(), |ui| {
                ui.radio_value(&mut self.targeting, WeaponTargeting::SingleTarget, WeaponTargeting::SingleTarget.to_string());
                ui.radio_value(&mut self.targeting, WeaponTargeting::Area, WeaponTargeting::Area.to_string());
            });
            ui.label("Targeting");
        });

        ui.add(Slider::i32(&mut self.range_max, 0..=5).text("Maximum range"));
        ui.add(Slider::i32(&mut self.range_min, 0..=5).text("Minimum range"));

        let mut removed_tags: Option<usize> = Option::None;

        ui.horizontal(|ui| {
            for (j, tag_name) in self.tags.iter_mut().enumerate() {
                if Tag::selector_ui(tag_name, ui, format!("Weapon {} tag {}", id, j), tag_db) {
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
            ui.text_edit(&mut self.damage);
            ui.label("Damage");
        });

        ui.horizontal(|ui| {
            ui.add(TextEdit::new(&mut self.rules).multiline(true));
            ui.label("Rules");
        });

        ui.button("Drop").clicked
    }
}