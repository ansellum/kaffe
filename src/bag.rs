use jiff::Timestamp;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Bag {
    #[serde(default)]
    id: u32,
    #[serde(default)]
    coffee_id: u32,
    
    roast_date: Timestamp,
    open_date: Option<Timestamp>,
    empty_date: Option<Timestamp>,
    weight_g: u16,
    price_ct: u16,

    #[serde(default = "Timestamp::now")]
    timestamp: Timestamp,
}

impl Bag {
    pub fn to_sql(&self) -> String {
        format!(
            "INSERT INTO bag (id, coffee_id, roast_date, open_date, empty_date, weight_g, price_ct, timestamp) 
                VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')", 
            self.id,
            self.coffee_id.to_string(),
            self.roast_date.to_string(),
            self.open_date.map(|v| v.to_string()).unwrap_or("default".to_string()),
            self.empty_date.map(|v| v.to_string()).unwrap_or("default".to_string()),
            self.weight_g,
            self.price_ct,
            self.timestamp.to_string(),
        )
    }
}