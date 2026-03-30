use jiff::Timestamp;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Brew {
    id: Option<u32>,
    bag_id: Option<u32>,
    grinder_id: Option<u32>,
    brewer_id: Option<u32>,
    timestamp: Timestamp,
    grind_level: u16,
    coffee_g: u16,
    water_g: Option<u16>,
    brew_g: u16,
    rating: u8, // Likert Scale, 1-5
    notes: Option<String>,
}