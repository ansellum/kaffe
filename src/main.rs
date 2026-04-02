use clap::{Args, Parser, Subcommand};
use rusqlite::Connection;
use std::path::Path;
use std::ffi::OsStr;
use std::fs;
use std::io;
use serde::{Deserialize};
use std::error::Error;

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
    Equipment(EquipmentArgs),
    Bag(BagArgs),
    Coffee(CoffeeArgs),
    Brew(BrewArgs),
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

////////////////////
// IMPORT STRUCTS //
////////////////////

#[derive(Deserialize)]
struct JSONItems {
    equipment: Vec<equipment::Equipment>,
    coffee: Vec<coffee::Coffee>,
    bag: Vec<bag::Bag>,
    brew: Vec<brew::Brew>,
}

/////////////////////
// IMPORT COMMANDS //
/////////////////////
fn import_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Parse file type
    let extension = Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .expect("Invalid file type. Please use a JSON or CSV.");                /* TODO: pattern matching */ 
    
    match extension {
        "json" => {
            let file = fs::File::open(path)?;                                   /* TODO: pattern matching */ 
            let reader = io::BufReader::new(file);
            let j: JSONItems = serde_json::from_reader(reader)?;

            // Open SQLite database
            let conn = Connection::open("./kaffe.db")?;                         /* TODO: pattern matching */ 
            //let conn = Connection::open_in_memory()?;
            let schema_str = fs::read_to_string("./kaffe.sql")?;                /* TODO: pattern matching */ 
            conn.execute_batch(&schema_str)
                .expect("Schema reading error!");                               /* TODO: pattern matching */ 

            for e in j.equipment {
                //Handle required items

                let decomission_date_str = match e.decommission_date {
                    Some(date) => date.to_string(),
                    None => String::new()
                };

                let str = format!(
                    "INSERT INTO equipment (name, kind, purchase_date, decommission_date, price_ct, timestamp) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')", 
                    {e.name}, {e.kind.to_string()}, {e.purchase_date.to_string()}, {decomission_date_str}, {e.price_ct}, {e.timestamp}.to_string()
                );

                conn.execute(&str, [])?;
            }
            for coffee in j.coffee {
                dbg!(coffee);
            }
            for bag in j.bag {
                dbg!(bag);
            }
            for brew in j.brew {
                dbg!(brew);
            }

            // Read Database
            let mut sql_select = conn.prepare("SELECT id, name, price_ct, decommission_date FROM equipment")?;

            let rows = sql_select.query_map([], |row| {
                Ok((
                    row.get::<_, u32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, u16>(2)?,
                    row.get::<_, String>(3)?
                ))
            })?;

            for row in rows {
                let (id, name, price_ct, timestamp) = row?;
                println!("ID: {}, Name: {}, Price: {}, Timestamp: {}", id, name, price_ct, timestamp);
            }
        }
        "csv" => {
            // Read CSV
            let mut rdr = csv::Reader::from_path(path)?;

            // Wizard
            // TODO: Replace with auto-check
            let mut input = String::new();
            println!("What type of item are you importing?");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            println!("Importing {input}...");

            match input.to_lowercase().as_str().trim() {
                "equipment" => {
                    for line in rdr.deserialize() {
                        let equipment: equipment::Equipment = line?;
                        dbg!(equipment);
                    }
                },
                "coffee" => {    
                    for line in rdr.deserialize() {
                        let coffee: coffee::Coffee = line?;
                        dbg!(coffee);
                    }
                },
                "bag" => {    
                    for line in rdr.deserialize() {
                        let bag: bag::Bag = line?;
                        dbg!(bag);
                    }
                },
                "brew" => {    
                    for line in rdr.deserialize() {
                        let brew: brew::Brew = line?;
                        dbg!(brew);
                    }
                },

                _ => panic!("hey man that's not cool")
            }
        },
        _ => panic!("invalid type") /* panic! macro */ 
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialization
    let args = Cli::parse();

    match args.command {
        // WIZARD
        Modes::Equipment(equipment) => {
            match equipment.command {
                EquipmentCommands::Add => println!("kaffe equipment add"),
                EquipmentCommands::Remove => println!("kaffe equipment remove"),
                EquipmentCommands::List => println!("kaffe equipment list"),
            }
        }
        Modes::Bag(bag) => {
            match bag.command {
                BagCommands::Add => println!("kaffe bag add"),
                BagCommands::Remove => println!("kaffe bag remove"),
                BagCommands::List => println!("kaffe bag list"),
            }
        }
        Modes::Coffee(coffee) => {
            match coffee.command {
                CoffeeCommands::Add => println!("kaffe coffee add"),
                CoffeeCommands::Remove => println!("kaffe coffee remove"),
                CoffeeCommands::List => println!("kaffe coffee list"),
            }
        }
        Modes::Brew(brew) => {
            match brew.command {
                BrewCommands::Add => println!("kaffe brew add"),
                BrewCommands::Remove => println!("kaffe brew remove"),
                BrewCommands::List => println!("kaffe brew list"),
            }
        }

        // MAIN
        Modes::Import { file } => import_from_file(&file)?
    }

    Ok(())
}

