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
                VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", 
            self.name,
            self.kind.to_string().to_lowercase(),
            self.purchase_date,
            self.decommission_date.map_or(String::new(), |t| t.to_string()),
            self.price_ct,
            self.timestamp
        )
    }
}

pub fn new_csv(record: csv::StringRecord, h: &HashMap<String, usize>) -> Result<Equipment, Box<dyn Error>> {
    new(record[h["name"]].to_string(), 
        record[h["kind"]].to_string(),
        record[h["purchase_date"]].to_string(),
        record[h["decomission_date"]].to_string(),
        record[h["price_ct"]].to_string()
    )
}

pub fn new(name: String, kind: String, purchase_date: String, decomission_date: String, price_ct: String) -> Result<Equipment, Box<dyn Error>> {
    let e = Equipment {
        name: name.to_string(),
        kind: kind.parse::<EquipmentKind>().expect("EquipmentKind parsing error!"),
        purchase_date: format!("{}T00:00:00Z", purchase_date).parse()?,
        decommission_date: none_if_empty(decomission_date)
            .map(|day| format!("{}T00:00:00Z", day).parse::<Timestamp>())
            .transpose()?,
        price_ct: price_ct.parse()?,
        timestamp: Timestamp::now()
    };

    Ok(e)
}
fn none_if_empty(field: String) -> Option<String> {
    if field.is_empty() { None } else { Some(field.to_string()) }
}