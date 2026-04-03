use jiff::Timestamp;
use serde::Deserialize;
use std::error::Error;
use rusqlite::Connection; // Assume coffees exist!

#[derive(Debug, Deserialize)]
pub struct Bag {
    #[serde(default)]
    id: u32,
    #[serde(skip)]
    coffee_id: u32,
    #[serde(rename="coffee")]
    coffee_str: String,
    
    #[serde(skip_deserializing)]
    roast_date: Timestamp,
    #[serde(skip_deserializing)]
    open_date: Option<Timestamp>,
    #[serde(skip_deserializing)]
    empty_date: Option<Timestamp>,
    weight_g: u16,
    #[serde(rename="price")]
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
            self.open_date.map(|v| v.to_string()).unwrap_or("".to_string()),
            self.empty_date.map(|v| v.to_string()).unwrap_or("".to_string()),
            self.weight_g,
            self.price_ct,
            self.timestamp.to_string(),
        )
    }
}

pub fn new(conn: &Connection, mut b: Bag) -> Result<Bag, Box<dyn Error>> {
    // let query = format!("SELECT id FROM coffee WHERE name = {}", b.coffee_str);
    // conn.execute(&query, []);

    b.coffee_id = conn.query_row(
        "SELECT id FROM coffee WHERE name = ?1",
        [&b.coffee_str],
        
        |row| {
            row.get(0)
        }
    )?;

    Ok(b)
}
