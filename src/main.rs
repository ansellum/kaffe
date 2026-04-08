use clap::{Args, Parser, Subcommand};
use rusqlite::Connection;
use std::fs;
use std::error::Error;
use std::collections::HashMap;

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
    }
}

////////////////////
// WIZARD STRUCTS //
////////////////////
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct EquipmentArgs {
    #[command(subcommand)]
    command: EquipmentCommands,
}

#[derive(Subcommand)]
enum EquipmentCommands {
    Add,
    Remove,
    List,
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct BagArgs {
    #[command(subcommand)]
    command: BagCommands,
}

#[derive(Subcommand)]
enum BagCommands {
    Add,
    Remove,
    List,
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct CoffeeArgs {
    #[command(subcommand)]
    command: CoffeeCommands,
}

#[derive(Subcommand)]
enum CoffeeCommands {
    Add,
    Remove,
    List,
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct BrewArgs {
    #[command(subcommand)]
    command: BrewCommands,
}

#[derive(Subcommand)]
enum BrewCommands {
    Add,
    Remove,
    List,
}

/////////////////////
// IMPORT COMMANDS //
/////////////////////

fn import_from_csv(conn: &Connection, path: &str) -> Result<(), Box<dyn Error>> {
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
                let e = equipment::new(record, &header_map)?;
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

fn main() -> Result<(), Box<dyn Error>> {
    // Initialization
    let args = Cli::parse();

    // Connect to SQLite database
    //let conn = Connection::open_in_memory()?;
    let conn = Connection::open("./kaffe.db")?;

    match args.command {
        // MAIN
        Modes::Import { file } => import_from_csv(&conn, &file)?,
    }

    Ok(())
}

