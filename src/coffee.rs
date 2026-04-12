use jiff::Timestamp;
use rusqlite::Connection;
use std::fmt;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use inquire::error::CustomUserError;

#[derive(Debug)]
enum CoffeeKind {
    SingleOrigin,
    Blend
}

impl fmt::Display for CoffeeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SingleOrigin => write!(f, "single-origin"),
            Self::Blend => write!(f, "blend"),
        }
    }
}

impl std::str::FromStr for CoffeeKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "single-origin" => Ok(Self::SingleOrigin),
            "blend" => Ok(Self::Blend),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum RoastLevel {
    Light,
    Medium,
    Dark,
}

impl fmt::Display for RoastLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dark => write!(f, "dark"),
            Self::Medium => write!(f, "medium"),
            Self::Light => write!(f, "light"),
        }
    }
}

impl std::str::FromStr for RoastLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dark" => Ok(Self::Dark),
            "medium" => Ok(Self::Medium),
            "light" => Ok(Self::Light),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Coffee {
    roaster: String,
    name: String,
    roast_level: RoastLevel,
    kind: CoffeeKind,
    country: Option<String>,
    region: Option<String>,
    farm: Option<String>,
    producer: Option<String>,
    varietals: Option<Vec<String>>,
    altitude_m: Option<u16>,
    altitude_lower_m: Option<u16>,
    altitude_upper_m: Option<u16>,
    process: Option<String>,
    decaf: bool,

    tasting_notes: Vec<String>,

    timestamp: Timestamp,
}

impl Coffee {
    pub fn to_sql(&self) -> String {
        format!(
            "INSERT INTO coffee (roaster, name, roast_level, kind, country, region, farm, producer, varietals, altitude_m, altitude_lower_m, altitude_upper_m, process, decaf, tasting_notes, timestamp) 
                VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
            self.roaster,
            self.name,
            self.roast_level,
            self.kind,
            self.country
                .as_deref()
                .unwrap_or_default(),
            self.region
                .as_deref()
                .unwrap_or_default(),
            self.farm
                .as_deref()
                .unwrap_or_default(),
            self.producer
                .as_deref()
                .unwrap_or_default(),
            self.varietals
                .as_deref()
                .map_or_else(|| String::new(), |s| format!("{:?}", s)),
            self.altitude_m.map_or(String::new(), |num| num.to_string()),
            self.altitude_lower_m.map_or(String::new(), |num| num.to_string()),
            self.altitude_upper_m.map_or(String::new(), |num| num.to_string()),
            self.process
                .as_deref()
                .unwrap_or_default(),
            self.decaf,
            format!("{:?}", self.tasting_notes),
            self.timestamp.to_string()
        )
    }
}

pub fn new_csv(record: csv::StringRecord, h: &HashMap<String, usize>) -> Result<Coffee, Box<dyn Error>> {
    let soul = HashMap::from([
        ("roaster", &record[h["roaster"]]),
        ("name", &record[h["name"]]),
        ("kind", &record[h["kind"]]),
        ("country", &record[h["country"]]),
        ("region", &record[h["region"]]),
        ("farm", &record[h["farm"]]),
        ("producer", &record[h["producer"]]),
        ("varietals", &record[h["varietals"]]),
        ("altitude_m", &record[h["altitude_m"]]),
        ("altitude_lower_m", &record[h["altitude_lower_m"]]),
        ("altitude_upper_m", &record[h["altitude_upper_m"]]),
        ("process", &record[h["process"]]),
        ("roast_level", &record[h["roast_level"]]),
        ("tasting_notes", &record[h["tasting_notes"]]),
        ("decaf", &record[h["decaf"]]),
    ]);

    new(soul)
}

pub fn new(soul: HashMap<&str, &str>) -> Result<Coffee, Box<dyn Error>> {
    let c = Coffee {
        roaster: soul["roaster"].to_string(),
        name: soul["name"].to_string(),
        kind: soul["kind"]
            .parse::<CoffeeKind>()
            .expect("CoffeeKind parse error!"),
        country: none_if_empty(soul["country"]),
        region: none_if_empty(soul["region"]),
        farm: none_if_empty(soul["farm"]),
        producer: none_if_empty(soul["producer"]),
        varietals: none_if_empty(soul["varietals"])
            .map(|s| s.split(';')
                .map(str::to_owned)
                .collect()
            ),
        altitude_m: none_if_empty(soul["altitude_m"])
            .map(|s| s.parse::<u16>()
                .expect("altitude_m Parse Error")
            ),
        altitude_lower_m: none_if_empty(soul["altitude_lower_m"])
            .map(|s| s.parse::<u16>()
                .expect("altitude_lower_m Parse Error")
            ),
        altitude_upper_m: none_if_empty(soul["altitude_upper_m"])
            .map(|s| s.parse::<u16>()
                .expect("altitude_upper_m Parse Error")
            ),
        process: none_if_empty(soul["process"]),
        roast_level: soul["roast_level"]
            .parse::<RoastLevel>()
            .expect("RoastLevel parse error!"),
        tasting_notes: soul["tasting_notes"]
            .split(';')
            .map(str::to_owned)
            .collect(),
        decaf: !soul["decaf"].is_empty(),
        timestamp: Timestamp::now(),
    };

    Ok(c)
}

fn none_if_empty(field: &str) -> Option<String> {
    if field.is_empty() { None } else { Some(field.to_string()) }
}

pub fn roaster_suggestor(input: &str) -> Result<Vec<String>, CustomUserError> {
    suggestor(input, "SELECT roaster FROM coffee")
}

pub fn country_suggestor(input: &str) -> Result<Vec<String>, CustomUserError> {
    suggestor(input, "SELECT country FROM coffee")
}

pub fn region_suggestor(input: &str) -> Result<Vec<String>, CustomUserError> {
    suggestor(input, "SELECT region FROM coffee")
}

pub fn farm_suggestor(input: &str) -> Result<Vec<String>, CustomUserError> {
    suggestor(input, "SELECT farm FROM coffee")
}

pub fn producer_suggestor(input: &str) -> Result<Vec<String>, CustomUserError> {
    suggestor(input, "SELECT producer FROM coffee")
}

fn suggestor(input: &str, sql: &str) -> Result<Vec<String>, CustomUserError> {
    let input = input.to_lowercase();

    Ok(get_suggestions_from_db(sql)?
        .into_iter()
        .collect::<HashSet<_>>() // Remove duplicates
        .into_iter()
        .filter(|p| !p.is_empty())
        .filter(|p| p.to_lowercase().contains(&input))
        .take(5)
        .map(|p| String::from(p))
        .collect())
}

fn get_suggestions_from_db(sql: &str) -> Result<Vec<String>, rusqlite::Error>{
    let conn = Connection::open("./kaffe.db")?;

    // Prepare the query
    let mut stmt = conn.prepare(sql)?;

    // Query and map rows to strings
    let suggestions_iter = stmt.query_map([], |row| {
        row.get::<_, String>(0) // Get the first column as String
    })?;

    // Collect into a Vec
    let mut suggestion_list = Vec::new();
    for suggestion in suggestions_iter {
        suggestion_list.push(suggestion?);
    }

    Ok(suggestion_list)
}