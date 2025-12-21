use crate::config::Config;
use colored::Colorize;
use std::{fs::read_to_string, path::PathBuf};
use toml::from_str;

pub fn read_config(path: &PathBuf) -> Result<Config, ()> {
    let content = read_to_string(path).map_err(|e| {
        eprintln!("Failed to read config at {:?}: {}", path, e);
        ()
    })?;

    let config: Config = from_str(&content).map_err(|e| {
        eprintln!("Failed to parse TOML in {:?}: {}", path, e);
        ()
    })?;

    println!("{}", "Config file validated!".green().bold());
    Ok(config)
}
