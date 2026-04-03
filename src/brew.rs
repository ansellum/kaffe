use jiff::Timestamp;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Brew {
    #[serde(default)]
    id: u32,
    bag_id: u32,
    grinder_id: u32,
    brewer_id: u32,
    grind_level: u16,
    coffee_g: u16,
    water_g: Option<u16>,
    brew_g: u16,
    rating: u8, // Likert Scale, 1-5
    notes: Option<String>,

    #[serde(skip)]
    timestamp: Timestamp, // Timestamp = time inputted into database
}