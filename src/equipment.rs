use jiff::Timestamp;
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EquipmentKind {
    Brewer,
    Grinder,
}

impl fmt::Display for EquipmentKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equipment {
    #[serde(default = "Timestamp::now")]
    timestamp: Timestamp,
    #[serde(default)]
    id: u32, // TODO: Assigned by SQLite

    name: String,
    kind: EquipmentKind,
    purchase_date: Timestamp,
    #[serde(default)]
    decommission_date: Option<Timestamp>,
    price_ct: u32,
}

impl Equipment {
    pub fn to_sql(&self) -> String {
        let decomission_date_str = match self.decommission_date {
            Some(date) => date.to_string(),
            None => String::new()
        };

        format!(
            "INSERT INTO equipment (name, kind, purchase_date, decommission_date, price_ct, timestamp) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", 
            {&self.name}, {self.kind.to_string()}, {self.purchase_date.to_string()}, {decomission_date_str}, {self.price_ct}, {self.timestamp}.to_string()
        )
    }
}