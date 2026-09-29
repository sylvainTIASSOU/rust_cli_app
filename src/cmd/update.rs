use anyhow::{Ok, Result};
use clap::Args;
use crate::utils;

#[derive(Debug, Args)]
#[command(version, about, long_about=None)]

pub struct UpdateArgs {}

pub fn main(args: &UpdateArgs, cfg: &mut utils::config::AppConfig) -> Result<()> {
    println!("from the update command");
    Ok(())
}
