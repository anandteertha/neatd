mod args;

use args::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            println!("Noiceee you have initialized.. now you can edit the config file!!!");
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
