use jiff::Timestamp;
use jiff::civil::Time;
use std::fmt;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub enum EquipmentKind {
    Brewer,
    Grinder,
}

impl fmt::Display for EquipmentKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for EquipmentKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "grinder" => Ok(EquipmentKind::Grinder),
            "brewer" => Ok(EquipmentKind::Brewer),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Equipment {
    id: u32, // TODO: Assigned by SQLite
    name: String,
    kind: EquipmentKind,
    price_ct: u32,
    purchase_date: Timestamp,
    decommission_date: Option<Timestamp>,
    timestamp: Timestamp,
}

impl Equipment {
    pub fn to_sql(&self) -> String {
        format!(
            "INSERT INTO equipment (name, kind, purchase_date, decommission_date, price_ct, timestamp) 
                VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", 
            self.name,
            self.kind.to_string(),
            self.purchase_date.to_string(),
            self.decommission_date.map(|v| v.to_string()).unwrap_or("".to_string()),
            self.price_ct,
            self.timestamp.to_string()
        )
    }
}

pub fn new(record: csv::StringRecord) -> Result<Equipment, Box<dyn Error>> {
    let e = Equipment {
        id: 0,
        name: record[0].trim().to_string(),
        kind: EquipmentKind::from_str(&record[1]).expect("EquipmentType parsing error"),
        purchase_date: format!("{}T00:00:00Z", &record[2]).parse()?,
        decommission_date: (!record[3].is_empty()).then(|| format!("{}T00:00:00Z", &record[3]).parse()).transpose()?,
        price_ct: record[4].parse()?,
        timestamp: Timestamp::now()
    };

    Ok(e)
}