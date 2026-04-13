use clap::{Parser, Subcommand};
use rusqlite::Connection;
use std::fs;
use std::error::Error;
use std::collections::HashMap;

use inquire::{
    required,
    CustomType, DateSelect, Select, Text,
};

pub mod equipment;
pub mod bag;
pub mod coffee;
pub mod brew;

#[derive(Parser)]
#[command(version, about)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Modes,
}

#[derive(Subcommand)]
enum Modes {
    Import {
        file: String,
    },
    Cli
}

enum Items {
    Equipment,
    Coffee,
    // Bag,
    // Brew
}

impl std::str::FromStr for Items {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Equipment" => Ok(Self::Equipment),
            "Coffee" => Ok(Self::Coffee),
            // "Bag" => Ok(Self::Bag),
            // "Brew" => Ok(Self::Brew),
            _ => Err(())
        }
    }
}

fn import_from_csv(path: &str) -> Result<(), Box<dyn Error>> {
    //let conn = Connection::open_in_memory()?;
    let conn = Connection::open("./kaffe.db")?;

    let schema_str = fs::read_to_string("./kaffe.sql")?;                /* TODO: pattern matching */ 
    conn.execute_batch(&schema_str)
        .expect("Schema reading error!");                               /* TODO: pattern matching */ 

    // Read CSV
    let mut rdr = csv::Reader::from_path(path)?;
    let headers = rdr.headers()?.clone();
    let header_map: HashMap<String, usize> = headers.iter()
        .enumerate()
        .map(|(i, h)| (h.to_string(), i))
        .collect();

    for record in rdr.records() {
        let mut record = record?;
        record.trim();

        match headers.len() {
            5 => { // EQUIPMENT
                let e = equipment::build_csv(record, &header_map)?;
                conn.execute(&e.to_sql(), [])?;
            }
            15 => { // COFFEE
                let c = coffee::build_csv(record, &header_map)?;
                conn.execute(&c.to_sql(), [])?;
            }
            6 => { // BAGS
                let b = bag::new(record, &header_map, &conn)?;
                conn.execute(&b.to_sql(), [])?;
            }
            11 => { // BREWS
                let b = brew::new(record, &header_map, &conn)?;
                conn.execute(&b.to_sql(), [])?;
            }

            _ => panic!("hey man that's not cool")
        }
    }

    Ok(())
}

fn equipment_wizard() -> Result<(), Box<dyn Error>> {
    let mut e = equipment::new();

    e.name = Text::new("Name:")
                .with_validator(required!("You wouldn't forget to name your own child, would you?"))
                .with_help_message("Name your vessel.")
                .prompt()?;

    e.kind = Select::new("Kind:", vec!["brewer", "grinder"])
        .prompt()?
        .parse::<equipment::EquipmentKind>()
        .expect("EquipmentKind parse error");

    e.purchase_date = DateSelect::new("Purchase Date:")
        .with_help_message("When was your vessel acquired?")
        .prompt()?
        .to_string() // Convert from chrono::NaiveDate
        .parse::<jiff::Timestamp>()?;

    e.decommission_date = DateSelect::new("Decomission Date:")
        .with_help_message("When was your vessel disowned?")
        .prompt_skippable()?
        .map_or(String::new(), |t| t.to_string())
        .parse::<jiff::Timestamp>()
        .ok();

    let _price: f64 = CustomType::new("Amount:")
        .with_formatter(&|i: f64| format!("${i}"))
        .with_error_message("That isn't right.")
        .with_help_message("How much did it cost you?")
        .prompt()
        .unwrap();

    e.price_ct = (_price  * 100.0)
        .trunc()
        .to_string()
        .parse::<u32>()?;

    //let conn = Connection::open_in_memory()?;
    let conn = Connection::open("./kaffe.db")?;
    conn.execute(&e.to_sql(), [])?;

    println!("Your entry has been successfully recorded.");
    println!("We thank you for your participation.");

    Ok(())
}
fn coffee_wizard() -> Result<(), Box<dyn Error>> {
    let mut c = coffee::new();

    c.roaster = Text::new("Roaster:")
        .with_validator(required!("You can't skip this."))
        .with_help_message("Name the roaster.")
        .with_autocomplete(&coffee::roaster_suggestor)
        .prompt()?;

    c.name = Text::new("Name:")
        .with_validator(required!("You can't skip this."))
        .with_help_message("Name the coffee.")
        .with_placeholder("Ethiopia Yirgacheffe")
        .prompt()?;

    c.roast_level = Select::new("Roast Level:", vec!["dark", "medium", "light"])
        .prompt()?
        .parse()
        .expect("EquipmentKind parse error");

    c.kind = Select::new("Type:", vec!["single-origin", "blend"])
        .prompt()?
        .parse()
        .expect("EquipmentKind parse error");

    if c.kind.to_string() == "single-origin" {
        c.country = Text::new("Country:")
            .with_placeholder("Ethiopia")
            .with_validator(required!("You chose this."))
            .with_autocomplete(&coffee::country_suggestor)
            .prompt()
            .ok();

        c.region = Text::new("Region:")
            .with_placeholder("Bener Meriah, Aceh")
            .with_autocomplete(&coffee::region_suggestor)
            .prompt()
            .ok();

        c.farm = Text::new("Farm:")
            .with_placeholder("Dawencho")
            .with_autocomplete(&coffee::farm_suggestor)
            .prompt()
            .ok();
        
        c.producer = Text::new("Producer:")
            .with_placeholder("Mullugeta Muntasha")
            .with_autocomplete(&coffee::producer_suggestor)
            .prompt()
            .ok();

        let varietals_str = Text::new("Varietals:")
            .with_help_message("Enter each varietal separated by semi-colons")
            .with_placeholder("abyssinia;typica")
            .prompt()?;

        c.varietals = (!varietals_str.is_empty())
            .then(|| varietals_str
                .split(';')
                .map(String::from)
                .collect()
            );

        c.altitude_m = Text::new("Altitude (MASL):")
            .with_help_message("If alitude is given as a range, skip this field")
            .prompt()?
            .parse::<u16>()
            .ok();
        
        if c.altitude_m.is_none() {
            c.altitude_lower_m = Text::new("Altitude Lower (MASL):")
                .prompt()?
                .parse::<u16>()
                .ok();

            c.altitude_upper_m = Text::new("Altitude Upper (MASL):")
                .prompt()?
                .parse::<u16>()
                .ok();
        }
    }

    c.process = Select::new("Process:", vec!["natural", "washed", "wet-hulled", "honey"])
        .prompt_skippable()?
        .map(String::from);

    c.decaf = match Select::new("Decaf:", vec!["Yes", "No"]).prompt()? {
        "Yes" => true,
        _ => false,
    };

    c.tasting_notes = Text::new("Tasting notes:")
        .with_help_message("Enter each tasting note separated by semi-colons")
        .with_placeholder("lemon;red fruit;ginger")
        .with_validator(required!("C'mon, it's on the label"))
        .prompt()?
        .split(';')
        .map(String::from)
        .collect();        
    
    //let conn = Connection::open_in_memory()?;
    let conn = Connection::open("./kaffe.db")?;
    conn.execute(&c.to_sql(), [])?;

    println!("Your entry has been successfully recorded.");
    println!("We thank you for your participation.");

    Ok(())
}

/// Wizard Functions
fn wizard() -> Result<(), Box<dyn Error>> {
    let _category = Select::new("Item:", vec!["Equipment", "Coffee", "Bag", "Brew"]).prompt()?;

    let category = _category.parse::<Items>().expect("How very, very interesting.");

    match category {
        Items::Equipment => equipment_wizard(),
        Items::Coffee => coffee_wizard(),
        // Items::Bag=> bag_wizard(),
        // Items::Brew => brew_wizard(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialization
    let args = Cli::parse();

    match args.command {
        Modes::Import { file } => import_from_csv(&file)?,
        Modes::Cli => wizard()?,
    }

    Ok(())
}

