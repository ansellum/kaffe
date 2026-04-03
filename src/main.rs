use clap::{Args, Parser, Subcommand};
use rusqlite::Connection;
use std::fs;
use std::io;
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

/////////////////////
// IMPORT COMMANDS //
/////////////////////

fn import_from_csv(path: &str) -> Result<(), Box<dyn Error>> {
    // Parse file type
    // Read CSV
    let mut rdr = csv::Reader::from_path(path)?;

    // Connect to SQLite database
    //let conn = Connection::open_in_memory()?;
    let conn = Connection::open("./kaffe.db")?;
    let schema_str = fs::read_to_string("./kaffe.sql")?;                /* TODO: pattern matching */ 
    conn.execute_batch(&schema_str)
        .expect("Schema reading error!");                               /* TODO: pattern matching */ 

    // Wizard
    // TODO: Replace with auto-check
    let mut input = String::new();
    println!("What type of item are you importing?");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Importing {input}...");

    match input.to_lowercase().as_str().trim() {
        "equipment" => {
            for line in rdr.deserialize() {
                //let e: equipment::Equipment = line?;
                let e = equipment::new(line.unwrap())?;
                conn.execute(&e.to_sql(), [])?;
            }
        },
        "coffee" => {    
            for line in rdr.deserialize() {
                let c = coffee::new(line.unwrap())?;
                conn.execute(&c.to_sql(), [])?;
            }
        },
        "bag" => {    
            for line in rdr.deserialize() {
                let b = bag::new(&conn, line.unwrap())?;
                conn.execute(&b.to_sql(), [])?;
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
        Modes::Import { file } => import_from_csv(&file)?
    }

    Ok(())
}

