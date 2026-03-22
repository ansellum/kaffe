use jiff::Timestamp;

enum EquipmentType {
    Brewer,
    Grinder,
}

pub struct Equipment {
    id: u32,
    name: String,

    machine: EquipmentType,
    purchase_date: Timestamp,
    decommission_date: Timestamp,
    price: u32,
    timestamp: Timestamp,
}