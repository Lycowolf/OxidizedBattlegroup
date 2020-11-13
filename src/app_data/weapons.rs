use strum_macros::Display;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Display, PartialEq)]
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

#[derive(Serialize, Deserialize, Display, PartialEq)]
pub enum WeaponTargeting {
    SingleTarget,
    Area,
}

impl Default for WeaponTargeting {
    fn default() -> Self {
        Self::SingleTarget
    }
}

#[derive(Serialize, Deserialize)]
pub enum WeaponTag {
    Accurate,
    Boarding,
    Charge(i32),
    Critical,
    Escort,
    Greywash,
    Inaccurate,
    Interdiction,
    Legionspace,
    Limited(i32),
    Overshield,
    Payload(i32),
    Reliable(i32),
    Reloading(i32),
    System,
    Tenacity,
    Unique,
    Wing,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Weapon {
    pub name: String,
    pub fluff: String,
    pub cost: i32,
    pub class: WeaponClass,
    pub targeting: WeaponTargeting,
    pub tags: Vec<WeaponTag>,
    pub range_min: i32,
    pub range_max: i32,
    pub damage: String,
    pub rules: String,
}

