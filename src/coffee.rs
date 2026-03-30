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
    timestamp: Timestamp, // Timestamp = time inputted into database

    roaster: String,
    name: String,
    roast_level: RoastLevel,
    coffee_type: CoffeeType,
    country: Option<String>,
    region: Option<String>, // Not Vec<> for deserialization
    farm: Option<String>,
    producer: Option<String>,
    varietals: Option<String>, // Not Vec<> for deserialization
    altitude_m: Option<u16>,
    altitude_lower_m: Option<u16>,
    altitude_upper_m: Option<u16>,
    process: Option<String>,
    tasting_notes: String, // Not Vec<> for deserialization
    decaf: bool,
}