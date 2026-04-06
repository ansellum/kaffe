use jiff::Timestamp;
use std::fmt;
use std::error::Error;

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
            self.decommission_date.map(|v| v.to_string()).unwrap_or_default(),
            self.price_ct,
            self.timestamp.to_string()
        )
    }
}

pub fn new(record: csv::StringRecord) -> Result<Equipment, Box<dyn Error>> {
    let e = Equipment {
        id: 0,
        name: record[0].to_string(),
        kind: record[1].parse().expect("EquipmentKind parsing error!"),
        purchase_date: format!("{}T00:00:00Z", &record[2]).parse()?,
        decommission_date: field_to_optional_timestamp(&record[3])?,
        price_ct: record[4].parse()?,
        timestamp: Timestamp::now()
    };

    Ok(e)
}

fn field_to_optional_timestamp(day: &str) -> Result<Option<Timestamp>, jiff::Error> {
    //format!("{}T00:00:00Z", opt.unwrap()).parse::<Timestamp>().map(Some)
    match day {
        "" => Ok(None),
        _ => format!("{}T00:00:00Z", day).parse::<Timestamp>().map(Some)
    }
}