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
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for CoffeeKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single-origin" => Ok(EquipmentKind::Grinder),
            "brewer" => Ok(EquipmentKind::Brewer),
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
        write!(f, "{:?}", self)
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

    varietals: Option<String>,
    region: Option<String>,
    tasting_notes: String,

    timestamp: Timestamp,
}

impl Coffee {
    pub fn to_sql(&self) -> String {
        format!(
            "INSERT INTO coffee (roaster, name, roast_level, kind, country, farm, producer, altitude_m, altitude_lower_m, altitude_upper_m, process, decaf, varietals, region, tasting_notes, timestamp) 
                VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
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

pub fn new(record: csv::StringRecord) -> Result<Coffee, Box<dyn Error>> {
    let c = Coffee {
        id: 0,
        roaster: record[0].to_string(),
        name: record[1].to_string(),
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

        varietals: Option<String>,
        region: Option<String>,
        tasting_notes: String,

        timestamp: Timestamp,
    };

    Ok(e)
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

// fn json_array_from_delimited(str: String) -> String {
//     format!("{:?}", str.split(';').collect::<Vec<_>>())
// }