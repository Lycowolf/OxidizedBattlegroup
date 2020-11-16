use strum_macros::Display;
use serde::{Serialize, Deserialize};

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

pub type WeaponTagId = String; // tag names

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct WeaponTag {
    pub name: WeaponTagId,
    pub fluff: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Weapon {
    pub name: String,
    pub fluff: String,
    pub cost: i32,
    pub class: WeaponClass,
    pub targeting: WeaponTargeting,
    pub tags: Vec<WeaponTagId>,
    pub range_min: i32,
    pub range_max: i32,
    pub damage: String,
    pub rules: String,
}

