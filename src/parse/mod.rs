use std::{fs::read_to_string, path::PathBuf};
use toml::from_str;

use crate::config::Config;

pub fn read_config(path: PathBuf) -> Result<(), ()> {
    let contents = read_to_string(path);
    match contents {
        Ok(content) => {
            let config: Result<Config, toml::de::Error> = from_str(&content);
            match config {
                Ok(config_value) => {
                    println!("the config data is==> {:?}", config_value);
                }
                Err(error) => {
                    eprintln!(
                        "An error occurred while reading the config file, please make sure the path is correct ==> {}",
                        error
                    );
                }
            }
        }
        Err(error) => {
            eprintln!(
                "An error occurred while reading the config file, please make sure the path is correct ==> {}",
                error
            );
        }
    }
    Ok(())
}
