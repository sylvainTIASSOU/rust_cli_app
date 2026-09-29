use anyhow::{Ok, Result};
use clap::Args;
use crate::utils;

#[derive(Debug, Args)]
#[command(version, about, long_about=None)]

pub struct InitArgs {}

pub fn main(args: &InitArgs, cfg: &mut utils::config::AppConfig) -> Result<()> {
    println!("from the init command");
    Ok(())
}