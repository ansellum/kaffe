use jiff::Timestamp;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
enum CoffeeKind {
    #[serde(rename = "single-origin")]
    SingleOrigin,
    #[serde(rename = "blend")]
    Blend
}

impl fmt::Display for CoffeeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum RoastLevel {
    Light,
    Medium,
    Dark,
}

impl fmt::Display for RoastLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Deserialize)]
pub struct Coffee {
    #[serde(default)]
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

    // To be parsed into Vec<String>
    varietals: Option<String>, // Not Vec<> for deserialization
    region: Option<String>, // Not Vec<> for deserialization
    tasting_notes: String, // Not Vec<> for deserialization

    #[serde(default = "Timestamp::now")]
    timestamp: Timestamp, // Timestamp = time inputted into database
}

impl Coffee {
    pub fn to_sql(&self) -> String {format!(
            "INSERT INTO coffee (id, roaster, name, roast_level, kind, country, farm, producer, altitude_m, altitude_lower_m, altitude_upper_m, process, decaf, varietals, region, tasting_notes, timestamp) 
                VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')", 
            self.id,
            self.roaster,
            self.name,
            self.roast_level.to_string().to_lowercase(),
            self.kind.to_string().to_lowercase(), 
            self.country.as_deref().unwrap_or_default(),
            self.farm.as_deref().unwrap_or_default(),
            self.producer.as_deref().unwrap_or_default(),
            self.altitude_m.map(|v| v.to_string()).unwrap_or("default".to_string()),
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