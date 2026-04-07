use jiff::Timestamp;
use std::fmt;
use std::error::Error;
use std::collections::HashMap;

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
        match s {
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
        match s {
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
    farm: Option<String>,
    producer: Option<String>,
    altitude_m: Option<u16>,
    altitude_lower_m: Option<u16>,
    altitude_upper_m: Option<u16>,
    process: Option<String>,
    decaf: bool,

    varietals: Option<Vec<String>>,
    region: Option<Vec<String>>,
    tasting_notes: Vec<String>,

    timestamp: Timestamp,
}

impl Coffee {
    pub fn to_sql(&self) -> String {
        format!(
            "INSERT INTO coffee (roaster, name, roast_level, kind, country, farm, producer, altitude_m, altitude_lower_m, altitude_upper_m, process, decaf, varietals, region, tasting_notes, timestamp) 
                VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
            self.roaster,
            self.name,
            self.roast_level,
            self.kind,
            self.country.as_deref().unwrap_or_default(),
            self.farm.as_deref().unwrap_or_default(),
            self.producer.as_deref().unwrap_or_default(),
            self.altitude_m.map_or(String::new(), |num| num.to_string()),
            self.altitude_lower_m.map_or(String::new(), |num| num.to_string()),
            self.altitude_upper_m.map_or(String::new(), |num| num.to_string()),
            self.process.as_deref().unwrap_or_default(),
            self.decaf,
            self.varietals.as_deref().map_or_else(|| String::new(), |s| format!("{:?}", s)),
            self.region.as_deref().map_or_else(|| String::new(), |s| format!("{:?}", s)),
            format!("{:?}", self.tasting_notes),
            self.timestamp.to_string()
        )
    }
}

pub fn new(record: csv::StringRecord, h: &HashMap<String, usize>) -> Result<Coffee, Box<dyn Error>> {
    let c = Coffee {
        roaster: record[h["roaster"]].to_string(),
        name: record[h["name"]].to_string(),
        kind: record[h["kind"]].parse::<CoffeeKind>().expect("CoffeeKind parse error!"),
        country: none_if_empty(&record[h["country"]]),
        region: none_if_empty(&record[h["region"]]).map(|s|
            s.split(';').map(str::to_owned).collect()),
        farm: none_if_empty(&record[h["farm"]]),
        producer: none_if_empty(&record[h["producer"]]),
        varietals: none_if_empty(&record[h["varietals"]]).map(|s|
            s.split(';').map(str::to_owned).collect()),
        altitude_m: none_if_empty(&record[h["altitude_m"]]).map(|s| s.parse::<u16>().expect("altitude_m Parse Error")),
        altitude_lower_m: none_if_empty(&record[h["altitude_lower_m"]]).map(|s| s.parse::<u16>().expect("altitude_lower_m Parse Error")),
        altitude_upper_m: none_if_empty(&record[h["altitude_upper_m"]]).map(|s| s.parse::<u16>().expect("altitude_upper_m Parse Error")),
        process: none_if_empty(&record[h["process"]]),
        roast_level: record[h["roast_level"]].parse().expect("RoastLevel parse error!"),
        tasting_notes: record[h["tasting_notes"]].split(';').map(str::to_owned).collect(),
        decaf: !record[h["decaf"]].is_empty(),
        timestamp: Timestamp::now(),
    };

    Ok(c)
}

fn none_if_empty(field: &str) -> Option<String> {
    if field.is_empty() { None } else { Some(field.to_string()) }
}