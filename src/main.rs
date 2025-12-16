mod args;
mod config_file_data;
mod init;

use args::{Cli, Commands};
use clap::Parser;
use config_file_data::config_file_data;
use init::create_or_override_config_file;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path, force }) => {
            println!("Noiceee you have initialized.. now you can edit the config file!!!");
            _ = create_or_override_config_file("config.toml", config_file_data(), force, path);
        }
        Some(Commands::DryRun) => {
            println!("Noiceee you just want to see what files would be affected!!!");
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
        None => {
            println!("Usage: neatd <command> [options]\nRun `neatd --help` for more information.")
        }
    }
}
