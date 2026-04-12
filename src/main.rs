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
                let e = equipment::new_csv(record, &header_map)?;
                conn.execute(&e.to_sql(), [])?;
            }
            15 => { // COFFEE
                let c = coffee::new_csv(record, &header_map)?;
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
    let _name = Text::new("Name:")
                .with_validator(required!("You wouldn't forget to name your own child, would you?"))
                .with_help_message("Name your vessel.")
                .prompt()?;

    let _kind = Select::new("Kind:", vec!["brewer", "grinder"])
        .prompt()?
        .to_string();

    let _purchase_date = DateSelect::new("Purchase Date:")
        .with_help_message("When was your vessel acquired?")
        .prompt()?
        .to_string();

    let _decomission_date = DateSelect::new("Decomission Date:")
        .with_help_message("When was your vessel disowned?")
        .prompt_skippable()?
        .map_or(String::new(), |t| t.to_string());

    let _price: f64 = CustomType::new("Amount:")
        .with_formatter(&|i: f64| format!("${i}"))
        .with_error_message("That isn't right.")
        .with_help_message("How much did it cost you?")
        .prompt()
        .unwrap();

    let price_ct = (_price  * 100.0).trunc().to_string();

    let e = equipment::new(HashMap::from([
        ("name", _name),
        ("kind", _kind),
        ("purchase_date", _purchase_date),
        ("decomission_date", _decomission_date),
        ("price_ct", price_ct),
    ]))?;

    //let conn = Connection::open_in_memory()?;
    let conn = Connection::open("./kaffe.db")?;
    conn.execute(&e.to_sql(), [])?;

    println!("Your entry has been successfully recorded.");
    println!("We thank you for your participation.");

    Ok(())
}
fn coffee_wizard() -> Result<(), Box<dyn Error>> {
    let roaster = Text::new("Roaster:")
        .with_validator(required!("You can't skip this."))
        .with_help_message("Name the roaster.")
        .with_autocomplete(&coffee::roaster_suggestor)
        .prompt()?;

    let name = Text::new("Name:")
        .with_validator(required!("You can't skip this."))
        .with_help_message("Name the coffee.")
        .with_placeholder("Ethiopia Yirgacheffe")
        .prompt()?;

    let roast_level = Select::new("Roast Level:", vec!["dark", "medium", "light"])
        .prompt()?;

    let kind = Select::new("Type:", vec!["single-origin", "blend"])
        .prompt()?;

    let mut country = String::new();
    let mut region = String::new();
    let mut farm = String::new();
    let mut producer = String::new();
    let mut varietals = String::new();
    let mut altitude_m = String::new();
    let mut altitude_lower_m = String::new();
    let mut altitude_upper_m = String::new();

    if kind == "single-origin" {
        country = Text::new("Country:")
            .with_placeholder("Ethiopia")
            .with_validator(required!("You chose this."))
            .with_autocomplete(&coffee::country_suggestor)
            .prompt()?;

        region = Text::new("Region:")
            .with_placeholder("Bener Meriah, Aceh")
            .with_autocomplete(&coffee::region_suggestor)
            .prompt()?;

        farm = Text::new("Farm:")
            .with_placeholder("Dawencho")
            .with_autocomplete(&coffee::farm_suggestor)
            .prompt()?;
        
        producer = Text::new("Producer:")
            .with_placeholder("Mullugeta Muntasha")
            .with_autocomplete(&coffee::producer_suggestor)
            .prompt()?;

        varietals = Text::new("Varietals:")
            .with_help_message("Enter each varietal separated by semi-colons")
            .with_placeholder("abyssinia;typica")
            .prompt()?;
        altitude_m = Text::new("Altitude (MASL):")
            .with_help_message("If alitude is given as a range, skip this field")
            .prompt()?;
        
        if altitude_m.is_empty() {
            altitude_lower_m = Text::new("Altitude Lower (MASL):")
                .prompt()?;
            altitude_upper_m = Text::new("Altitude Upper (MASL):")
                .prompt()?;
        }
    }

    let process = Select::new("Process:", vec!["natural", "washed", "wet-hulled", "honey"])
        .prompt_skippable()?
        .unwrap_or_default();

    let decaf = match Select::new("Decaf:", vec!["Yes", "No"]).prompt()? {
        "Yes" => "yippee!".to_string(),
        _ => String::new(),
    };

    let tasting_notes = Text::new("Tasting notes:")
        .with_help_message("Enter each tasting note separated by semi-colons")
        .with_placeholder("lemon;red fruit;ginger")
        .with_validator(required!("C'mon, it's on the label"))
        .prompt()?;        

    let c = coffee::new(HashMap::from([
        ("roaster", roaster.as_str()),
        ("name", name.as_str()),
        ("roast_level", roast_level),
        ("kind", kind),
        ("country", country.as_str()),
        ("region", region.as_str()),
        ("farm", farm.as_str()),
        ("producer", producer.as_str()),
        ("varietals", varietals.as_str()),
        ("altitude_m", altitude_m.as_str()),
        ("altitude_lower_m", altitude_lower_m.as_str()),
        ("altitude_upper_m", altitude_upper_m.as_str()),
        ("process", process),
        ("decaf", decaf.as_str()),
        ("tasting_notes", tasting_notes.as_str()),
    ]))?;

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

