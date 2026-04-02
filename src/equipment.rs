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
    pub timestamp: Timestamp,
    // #[serde(default)]
    // pub id: u32, // TODO: Assigned by SQLite

    pub name: String,
    pub kind: EquipmentKind,
    pub purchase_date: Timestamp,
    #[serde(default)]
    pub decommission_date: Option<Timestamp>,
    pub price_ct: u32,
}