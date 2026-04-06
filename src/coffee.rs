use jiff::Timestamp;
use std::fmt;
use std::error::Error;

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
            "brewer" => Ok(Self::Blend),
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
    id: u32,

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
            self.roast_level.to_string(),
            self.kind.to_string(),
            self.country.unwrap_or_default(),
            self.farm.unwrap_or_default(),
            self.producer.as_deref().unwrap_or_default(),
            self.altitude_m.map(|v| v.to_string()).unwrap_or("".to_string()),
            self.altitude_lower_m.unwrap_or_default(),
            self.altitude_upper_m.unwrap_or_default(),
            self.process.as_deref().unwrap_or_default(),
            self.decaf,
            format!("{:?}", self.varietals.as_deref().unwrap_or_default().split(';').collect::<Vec<_>>()),
            format!("{:?}", self.region.as_deref().unwrap_or_default().split(';').collect::<Vec<_>>()),
            format!("{:?}", self.tasting_notes.split(';').collect::<Vec<_>>()),
            self.timestamp.to_string()
        )
    }
}

pub fn new(record: csv::StringRecord) -> Result<Coffee, Box<dyn Error>> {
    let c = Coffee {
        id: 0,
        roaster: record[0].to_string(),
        name: record[1].to_string(),
        kind: record[2].parse().expect("CoffeeKind parse error!"),
        country: none_if_empty(record[3].to_string()),
        region: none_if_empty(record[4].to_string()).map(|s|
            s.split(';').map(str::to_owned).collect()),
        farm: none_if_empty(record[5].to_string()),
        producer: none_if_empty(record[6].to_string()),
        varietals: none_if_empty(record[7].to_string()).map(|s|
            s.split(';').map(str::to_owned).collect()),
        altitude_m: none_if_empty(record[8].to_string()).map(|s| s.parse::<u16>().expect("altitude_m Parse Error")),
        altitude_lower_m: none_if_empty(record[9].to_string()).map(|s| s.parse::<u16>().expect("altitude_lower_m Parse Error")),
        altitude_upper_m: none_if_empty(record[10].to_string()).map(|s| s.parse::<u16>().expect("altitude_upper_m Parse Error")),
        process: none_if_empty(record[3].to_string()),
        roast_level: record[11].parse().expect("RoastLevel parse error!"),
        tasting_notes: record[12].split(';').map(str::to_owned).collect(),
        decaf: !record[13].is_empty(),
        timestamp: Timestamp::now(),
    };

    Ok(c)
}

// pub fn new(mut c: Coffee) -> Result<Coffee, Box<dyn Error>> {
//     if let Some(str) = c.varietals.as_mut() {
//         *str = json_array_from_delimited(str.to_string());
//     }
//     if let Some(str) = c.region.as_mut() {
//         *str = json_array_from_delimited(str.to_string());
//     }
//     c.tasting_notes = json_array_from_delimited(c.tasting_notes);
//     Ok(c)
// }

fn json_array_from_delimited(str: String) -> String {
    format!("{:?}", str.split(';').collect::<Vec<_>>())
}

fn empty_to_opt_str(field: &str) -> Option<String> {
    (!field.is_empty()).then(|| field.to_string())
}

fn empty_to_opt_u16(field: &str) -> Option<u16> {
    (!field.is_empty()).then(|| field.parse().expect("Not a valid number"))
}

fn none_if_empty(field: String) -> Option<String> {
    if field.is_empty() { None } else { Some(field) }
}