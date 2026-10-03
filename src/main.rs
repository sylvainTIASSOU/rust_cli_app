use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

mod cmd;
mod utils;
#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]

// definir nos commande en utilisant clap
struct Cli {
    #[command(subcommand)] //annotation
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(cmd::init::InitArgs),
    Update(cmd::update::UpdateArgs),
    Configure(cmd::conf::ConfigArgs),
}
// La fonction main retourne un resultat
fn main() -> Result<()> {
    let mut cfg: utils::config::AppConfig = confy::load("rust-cli", None)?;
    println!("config vars: {:?}", cfg);
    let full_path = confy::get_configuration_file_path("rust-cli", None);
    println!("path vars: {:?}", full_path);
    // initialisation au cli
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => {
            cmd::init::main(args, &mut cfg)?;
        }
        Commands::Update(args) => {
            cmd::update::main(args, &mut cfg)?;
        }
        Commands::Configure(args) => {
            cmd::conf::main(args, &mut cfg)?;
        }
    }
    Ok(())
}
