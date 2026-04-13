use jiff::Timestamp;
use std::fmt;
use std::error::Error;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub enum EquipmentKind {
    #[default]
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
        match s.to_lowercase().as_str() {
            "grinder" => Ok(Self::Grinder),
            "brewer" => Ok(Self::Brewer),
            _ => Err(()),
        }
    }
}

#[derive(Default, Debug)]
pub struct Equipment {
    pub name: String,
    pub kind: EquipmentKind,
    pub price_ct: u32,
    pub purchase_date: Timestamp,
    pub decommission_date: Option<Timestamp>,
    pub timestamp: Timestamp,
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

pub fn new() -> Equipment {
    Equipment::default()
}

pub fn build_csv(record: csv::StringRecord, h: &HashMap<String, usize>) -> Result<Equipment, Box<dyn Error>> {
    let soul = HashMap::from([
        ("name", &record[h["name"]]),
        ("kind", &record[h["kind"]]),
        ("purchase_date", &record[h["purchase_date"]]),
        ("decomission_date", &record[h["decomission_date"]]),
        ("price_ct", &record[h["price_ct"]]),
    ]);

    build(soul)
}

fn build(soul: HashMap<&str, &str>) -> Result<Equipment, Box<dyn Error>> {
    let e = Equipment {
        name: soul["name"]
            .to_owned(),
        kind: soul["kind"]
            .parse::<EquipmentKind>()
            .expect("EquipmentKind parsing error!"),
        purchase_date: format!("{}T00:00:00Z", soul["purchase_date"]).parse()?,
        decommission_date: none_if_empty(soul["decomission_date"]
            .to_owned())
            .map(|day| format!("{}T00:00:00Z", day).parse::<Timestamp>())
            .transpose()?,
        price_ct: soul["price_ct"].parse()?,
        timestamp: Timestamp::now()
    };

    Ok(e)
}

fn none_if_empty(field: String) -> Option<String> {
    if field.is_empty() { None } else { Some(field.to_owned()) }
}