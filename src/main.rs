mod args;
mod config_file_data;
mod directory;
mod init;
mod parse;
mod run;

use crate::run::scanner::walk_policy_setup;
use args::{Cli, Commands};
use clap::Parser;
use config_file_data::config_file_data;
use directory::{get_file_path, get_hom_directory};
use init::create_or_override_config_file;
use parse::read_config;
use run::config::display::display_config;
use std::path::PathBuf;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path, force }) => {
            println!("Noiceee you have initialized.. now you can edit the config file!!!");
            _ = create_or_override_config_file("config.toml", config_file_data(), force, path);
        }
        Some(Commands::DryRun) => {
            let config_file_path: PathBuf = get_file_path(get_hom_directory(), "config.toml");
            let config = read_config(&config_file_path);
            match config {
                Ok(config_value) => walk_policy_setup(&config_value, &config_file_path),
                Err(error) => {
                    eprintln!(
                        "Failed to read config at {:?}: {:?}",
                        &config_file_path, error
                    );
                }
            }
        }
        Some(Commands::Status) => {
            println!("Okay so you want to see if daemon is running in background or not!!!");
        }
        Some(Commands::Run { once, .. }) => {
            if once {
                println!("okay soo you decided to run the neatd just once.. cool cool cool!!!");
            } else {
                println!(
                    "okay soo you decided to run the neatd as a background daemon.. cool cool cool!!!"
                );
            }
        }
        Some(Commands::Validate { path }) => {
            let config_file_path: PathBuf =
                path.unwrap_or(get_file_path(get_hom_directory(), "config.toml"));
            _ = read_config(&config_file_path);
        }
        Some(Commands::PrintConfig { path }) => {
            let config_file_path: PathBuf =
                path.unwrap_or(get_file_path(get_hom_directory(), "config.toml"));
            let config = read_config(&config_file_path);
            match config {
                Ok(config_value) => {
                    display_config(&config_value);
                }
                Err(error) => {
                    eprintln!(
                        "Failed to read config at {:?}: {:?}",
                        &config_file_path, error
                    );
                }
            }
        }
        None => {
            println!("Usage: neatd <command> [options]\nRun `neatd --help` for more information.")
        }
    }
}
