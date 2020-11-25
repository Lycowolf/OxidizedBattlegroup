use serde::{Serialize, Deserialize};

pub mod weapons;
pub mod systems;
pub mod tags;
use weapons::Weapon;
use crate::app_data::tags::Tag;
use crate::app_data::systems::System;

#[derive(Serialize, Deserialize, Default, Clone)]

// NOTE on tag DBs:
// We want to edit tags in place (add button should add new empty tag as this is more convenient and consistent wth the rest of the UI).
// We need to store them in a order-preserving structure, else it would jump around when editing.
// On the other hand, we want to look it up every time we display tags, so we need fast lookups.
// We also need stable order of iteration (else tag selector will go crazy).
// The ordering has to be persistent between frames, so we store data in Vec and convert them to IndexMap
// every frame (in data editor) or on load (in app)
// TODO: think about something better
pub struct AppData {
    pub tag_db: Vec<Tag>,
    pub weapons: Vec<Weapon>,
    pub systems: Vec<System>,
}