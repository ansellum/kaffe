use clap::{Parser, Subcommand};
use rusqlite::Connection;
use std::fs;
use std::fmt;
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
    Bag,
    Brew
}

impl std::str::FromStr for Items {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Equipment" => Ok(Self::Equipment),
            "Coffee" => Ok(Self::Coffee),
            "Bag" => Ok(Self::Bag),
            "Brew" => Ok(Self::Brew),
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
                let c = coffee::new(record, &header_map)?;
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

    let _kind = Select::new("Kind:", vec!["Brewer", "Grinder"]).prompt()?;

    let _purchase_date = DateSelect::new("Purchase Date:")
        .prompt()?;

    let _price: f64 = CustomType::new("Amount:")
        .with_formatter(&|i: f64| format!("${i}"))
        .with_error_message("That isn't right.")
        .with_help_message("How much did this cost you?")
        .prompt()
        .unwrap();

    let price_ct = (_price  * 100.0).trunc().to_string();

    let e = equipment::new(_name, _kind.to_string(), _purchase_date.to_string(), String::new(), price_ct)?;

    //let conn = Connection::open_in_memory()?;
    let conn = Connection::open("./kaffe.db")?;
    conn.execute(&e.to_sql(), [])?;

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
        Items::Bag=> bag_wizard(),
        Items::Brew => brew_wizard(),
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

