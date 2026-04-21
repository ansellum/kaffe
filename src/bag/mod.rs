use jiff::Timestamp;
use std::error::Error;
use rusqlite::Connection; // Assume coffees exist!
use std::collections::HashMap;

#[derive(Debug)]
pub struct Bag {
    coffee_id: u32,
    roast_date: Timestamp,
    open_date: Option<Timestamp>,
    empty_date: Option<Timestamp>,
    weight_g: u16,
    price_ct: u16,

    timestamp: Timestamp,
}

impl Bag {
    pub fn to_sql(&self) -> String {format!(
        "INSERT INTO bag (coffee_id, roast_date, open_date, empty_date, weight_g, price_ct, timestamp) 
            VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}')", 
        self.coffee_id.to_string(),
        self.roast_date.to_string(),
        self.open_date.map_or(String::new(), |t| t.to_string()),
        self.empty_date.map_or(String::new(), |t| t.to_string()),
        self.weight_g,
        self.price_ct,
        self.timestamp.to_string())
    }
}

pub fn build_csv(record: csv::StringRecord, h: &HashMap<String, usize>) -> Result<Bag, Box<dyn Error>> {
    let soul = HashMap::from([
        ("coffee_id", &record[h["coffee_id"]]),
        ("roast_date", &record[h["roast_date"]]),
        ("open_date", &record[h["open_date"]]),
        ("empty_date", &record[h["empty_date"]]),
        ("weight_g", &record[h["weight_g"]]),
        ("price_ct", &record[h["price_ct"]]),
    ]);

    build(soul)
}

pub fn build(soul: HashMap<&str, &str>) -> Result<Bag, Box<dyn Error>> {
    let b = Bag {
        coffee_id: get_id("SELECT id FROM coffee WHERE id = ?1", soul["coffee_id"])?,
        roast_date: format!("{}T00:00:00Z", soul["roast_date"])
            .parse::<Timestamp>()?,
        open_date: none_if_empty(soul["open_date"])
            .map(|day| format!("{}T00:00:00Z", day).parse::<Timestamp>())
            .transpose()?,
        empty_date: none_if_empty(soul["empty_date"])
            .map(|day| format!("{}T00:00:00Z", day).parse::<Timestamp>())
            .transpose()?,
        weight_g: soul["weight_g"].parse::<u16>()?,
        price_ct: soul["price_ct"].parse::<u16>()?,
        timestamp: Timestamp::now(),
    };

    Ok(b)
}

fn none_if_empty(field: &str) -> Option<String> {
    if field.is_empty() { None } else { Some(field.to_string()) }
}

fn get_id(sql: &str, key: &str) -> Result<u32, rusqlite::Error> {
    let conn = Connection::open("./kaffe.db")?;
    
    conn.query_row(
        sql,
        [key],
        |row| {
            row.get(0)
        }
    )
}