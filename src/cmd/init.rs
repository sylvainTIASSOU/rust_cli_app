use crate::utils::{self, config::Builder};
use anyhow::{Ok, Result};
use clap::Args;

#[derive(Debug, Args)]
#[command(version, about, long_about=None)]

pub struct InitArgs {
    #[arg(short = 'n', long = "name", default_value = "rust-cli")]
    app_name: String,
    #[arg(default_value="makedeveasy/rust-cli", short = 'p', long = "path")]
    default_path: String,
    #[arg(short = 's', help = "Database connection string", long = "db-conn-str")]
    pub db_conn_str: String,
}

pub fn main(args: &InitArgs, cfg: &mut utils::config::AppConfig) -> Result<()> {
    if args.db_conn_str == "" {
        println!("Please provide a database connection string using the --db-conn-str option.");
       return Ok(())
    }
    let mut config = utils::config::AppConfig::new();
    config.set_db_conn_str(&args.db_conn_str)
     .build();
     Ok(()) 
}
