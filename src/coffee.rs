use jiff::Timestamp;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
enum CoffeeType {
    SingleOrigin,
    Blend
}

#[derive(Serialize, Deserialize)]
enum RoastLevel {
    Light,
    Medium,
    Dark,
}

#[derive(Serialize, Deserialize)]
pub struct Coffee {
    id: u32,
    timestamp: Timestamp,

    roaster: String,
    name: String,
    roast_level: RoastLevel,
    coffee_type: CoffeeType,
    country: Option<String>,
    region: Option<Vec<String>>,
    farm: Option<String>,
    producer: Option<String>,
    varietals: Option<Vec<String>>,
    altitude_m: Option<u16>,
    altitutde_lower_m: Option<u16>,
    altitutde_upper_m: Option<u16>,
    process: String,
    tasting_notes: Vec<String>,
    decaf: bool,
}