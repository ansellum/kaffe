use jiff::Timestamp;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Equipment {
    #[serde(default)]
    id: u32, // TODO: Assigned by SQLite

    name: String,
    kind: EquipmentKind,
    purchase_date: Timestamp,
    #[serde(default)]
    decommission_date: Option<Timestamp>,
    price_ct: u32,
    #[serde(default = "Timestamp::now")]
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
            self.decommission_date.map(|v| v.to_string()).unwrap_or("default".to_string()),
            self.price_ct,
            self.timestamp.to_string()
        )
    }
}