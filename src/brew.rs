use jiff::Timestamp;
use std::collections::HashMap;
use rusqlite::Connection; // Assume IDs exist!
use std::error::Error;

#[derive(Debug)]
pub struct Brew {
    bag_id: u32,
    grinder_id: u32,
    brewer_id: u32,
    grind_level: u16,
    coffee_g: f64,
    water_g: Option<f64>,
    brew_g: Option<f64>,
    temp_c: Option<u8>,
    time_s: Option<u16>,
    rating: Option<u8>, // Likert Scale, 1-5
    notes: Option<String>,
    timestamp: Timestamp, // Timestamp = time inputted into database
}

impl Brew {
    pub fn to_sql(&self) -> String {
        format!(
            "INSERT INTO brew (bag_id, grinder_id, brewer_id, grind_level, coffee_g, water_g, brew_g, temp_c, time_s, rating, notes, timestamp) 
                VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
            self.bag_id,
            self.grinder_id,
            self.brewer_id,
            self.grind_level,
            self.coffee_g,
            self.water_g.map_or(String::new(), |num| num.to_string()),
            self.brew_g.map_or(String::new(), |num| num.to_string()),
            self.temp_c.map_or(String::new(), |num| num.to_string()),
            self.time_s.map_or(String::new(), |num| num.to_string()),
            self.rating.map_or(String::new(), |num| num.to_string()),
            self.notes.as_deref().unwrap_or_default(),
            self.timestamp.to_string()
        )
    }
}

pub fn new(record: csv::StringRecord, h: &HashMap<String, usize>, conn: &Connection) -> Result<Brew, Box<dyn Error>> {
    let b = Brew {
        bag_id: get_id(conn, "SELECT id FROM bag WHERE id = ?1", &record[h["bag_id"]])?,
        grinder_id: get_id(conn, "SELECT id FROM equipment WHERE LOWER(name) = LOWER(?1)", &record[h["grinder"]])?,
        brewer_id: get_id(conn, "SELECT id FROM equipment WHERE LOWER(name) = LOWER(?1)", &record[h["brewer"]])?,
        grind_level: record[h["grind_level"]].parse::<u16>().expect("grind_level parse error!"),
        coffee_g: record[h["coffee_g"]].parse::<f64>().expect("coffee_g parse error!"),
        water_g: none_if_empty(&record[h["water_g"]]).map(|s| s.parse::<f64>().expect("water_g Parse Error")),
        brew_g: none_if_empty(&record[h["brew_g"]]).map(|s| s.parse::<f64>().expect("brew_g Parse Error")),
        temp_c: none_if_empty(&record[h["temp"]]).map(|s| s.parse::<u8>().expect("temp Parse Error")),
        time_s: none_if_empty(&record[h["time_s"]]).map(|s| s.parse::<u16>().expect("time_s Parse Error")),
        rating: none_if_empty(&record[h["rating"]]).map(|s| s.parse::<u8>().expect("rating Parse Error")),
        notes: none_if_empty(&record[h["notes"]]),
        timestamp: Timestamp::now(),
    };

    Ok(b)
}

fn get_id(conn: &Connection, sql: &str, key: &str) -> Result<u32, rusqlite::Error> {
    conn.query_row(
        sql,
        [key],
        |row| {
            row.get(0)
        }
    )
}

fn none_if_empty(field: &str) -> Option<String> {
    if field.is_empty() { None } else { Some(field.to_string()) }
}