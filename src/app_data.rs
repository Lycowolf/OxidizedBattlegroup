use serde::{Serialize, Deserialize};

pub mod weapons;
use weapons::Weapon;

#[derive(Serialize, Deserialize, Default)]
pub struct AppData {
    pub weapons: Vec<Weapon>
}
