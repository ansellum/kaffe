use clap::{Args, Parser, Subcommand};
use rusqlite::{Connection, Result};

//pub mod bean;
pub mod equipment;

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

/////////////////////
// WIZARD COMMANDS //
/////////////////////
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

fn main() -> Result<()> {
    // Initialization
    let args = Cli::parse();
    let conn = Connection::open_in_memory()?;

    match args.command {
<<<<<<< HEAD
=======

>>>>>>> d961c7c018749f13745d470d3d833a05842af36d
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
        Modes::Import { file } => {

        }
    }
<<<<<<< HEAD

    Ok(())
=======
>>>>>>> d961c7c018749f13745d470d3d833a05842af36d
}

