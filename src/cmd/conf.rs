use anyhow::{Ok, Result};
use clap::Args;
use crate::utils;

#[derive(Debug, Args)]
#[command(version, about, long_about=None)]

pub struct ConfigArgs {}

pub fn main(args: &ConfigArgs, cfg: &mut utils::config::AppConfig) -> Result<()> {
    println!("from the config command");
    Ok(())
}
