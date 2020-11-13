use strum_macros::Display;
use egui::containers::{combo_box_with_label, Frame};
use serde::{Serialize, Deserialize};
use egui::{Stroke, Srgba};

mod weapons;

const APP_NAME: &str = "app";

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
struct Battlegroup {
    name: String,
    ships: Vec<Ship>,
}

impl Default for Battlegroup {
    fn default() -> Self {
        Self {
            name: "My Battlegroup".to_owned(),
            ships: vec![
                Ship { name: "Ship1".to_owned(), class: ShipClass::Frigate, hull: ShipHull::SystemShip },
                Ship { name: "Ship2".to_owned(), class: ShipClass::Battleship, hull: ShipHull::WeaponShip }
            ],
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Ship {
    name: String,
    class: ShipClass,
    hull: ShipHull,
}

#[derive(Display, Deserialize, Serialize, PartialEq)]
enum ShipHull {
    WeaponShip,
    SystemShip,
}

#[derive(Display, Deserialize, Serialize, PartialEq)]
enum ShipClass {
    Frigate,
    Carrier,
    Battleship,
}

impl egui::app::App for Battlegroup {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn ui(
        &mut self,
        ctx: &std::sync::Arc<egui::Context>,
        integration_context: &mut egui::app::IntegrationContext,
    ) {
        let Battlegroup { name, ships } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::auto_sized().always_show_scroll(true).show(ui, |ui| {
                ui.heading("Battlegroup");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Battlegroup name: ");
                    ui.text_edit(name);
                });

                ui.advance_cursor(16.0);

                for ship in ships {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Ship: ");
                            ui.text_edit(&mut ship.name);
                        });
                        combo_box_with_label(ui, "Class", &ship.class.to_string(), |ui| {
                            ui.radio_value(&mut ship.class, ShipClass::Frigate, ShipClass::Frigate.to_string());
                            ui.radio_value(&mut ship.class, ShipClass::Carrier, ShipClass::Carrier.to_string());
                            ui.radio_value(&mut ship.class, ShipClass::Battleship, ShipClass::Battleship.to_string());
                        });
                        combo_box_with_label(ui, "Hull", &ship.hull.to_string(), |ui| {
                            ui.radio_value(&mut ship.hull, ShipHull::WeaponShip, ShipHull::WeaponShip.to_string());
                            ui.radio_value(&mut ship.hull, ShipHull::SystemShip, ShipHull::SystemShip.to_string());
                        });
                        ui.separator();
                    });
                }

                ui.advance_cursor(16.0);
                if ui.button("Quit").clicked {
                    integration_context.output.quit = true;
                }

                ui.advance_cursor(160.0);
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
    let storage = egui_glium::storage::FileStorage::from_path("battlegroup.json".into());

    // Alternative: store nowhere
    // let storage = egui::app::DummyStorage::default();

    let app: Battlegroup = egui::app::get_value(&storage, APP_NAME).unwrap_or_default(); // Restore `MyApp` from file, or create new `MyApp`.
    egui_glium::run(title, Box::new(storage), app);
}