use jiff::Timestamp;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Brew {
    id: Option<i32>,
    bag_id: Option<i32>,
    grinder_id: Option<i32>,
    brewer_id: Option<i32>,
    timestamp: Timestamp,
    grind_level: u16,
    coffee_g: u16,
    water_g: Option<u16>,
    brew_g: u16,
    rating: u8, // Likert Scale, 1-5
    notes: Option<String>,
}