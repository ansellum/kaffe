use jiff::Timestamp;
use std::fmt;
use std::error::Error;
use std::collections::HashMap;

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
            "grinder" => Ok(Self::Grinder),
            "brewer" => Ok(Self::Brewer),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Equipment {
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
                VALUES ('{:?}', '{:?}', '{:?}', '{:?}', '{:?}', '{:?}')", 
            self.name,
            self.kind,
            self.purchase_date,
            self.decommission_date.map_or(String::new(), |n| n.to_string()),
            self.price_ct,
            self.timestamp
        )
    }
}

pub fn new(record: csv::StringRecord, h: &HashMap<String, usize>) -> Result<Equipment, Box<dyn Error>> {
    let e = Equipment {
        name: record[h["name"]].to_string(),
        kind: record[h["kind"]].parse().expect("EquipmentKind parsing error!"),
        purchase_date: format!("{}T00:00:00Z", &record[h["purchase_date"]]).parse()?,
        decommission_date: field_to_optional_timestamp(&record[h["decomission_date"]])?,
        price_ct: record[h["price_ct"]].parse()?,
        timestamp: Timestamp::now()
    };

    Ok(e)
}

fn field_to_optional_timestamp(day: &str) -> Result<Option<Timestamp>, jiff::Error> {
    match day {
        "" => Ok(None),
        _ => format!("{}T00:00:00Z", day).parse::<Timestamp>().map(Some)
    }
}