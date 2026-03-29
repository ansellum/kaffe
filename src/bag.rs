use jiff::Timestamp;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Bag {
    id: u32,
    coffee_id: u32,
    timestamp: Timestamp,
    
    roast_date: Timestamp,
    open_date: Option<Timestamp>,
    empty_date: Option<Timestamp>,
    purchase_date: Timestamp,
    weight_g: u16,
    price_ct: u16,
}