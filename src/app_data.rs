use serde::{Serialize, Deserialize};

pub mod weapons;
use weapons::Weapon;
use crate::app_data::weapons::{WeaponTag};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppData {
    pub weapon_tags: Vec<WeaponTag>,
    pub weapons: Vec<Weapon>
}