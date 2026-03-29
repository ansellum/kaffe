use jiff::Timestamp;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum CoffeeType {
    #[serde(rename = "single-origin")]
    SingleOrigin,
    #[serde(rename = "blend")]
    Blend
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum RoastLevel {
    Light,
    Medium,
    Dark,
}

#[derive(Debug, Deserialize)]
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