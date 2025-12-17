pub mod directory;

use directory::create_neatd_directory;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Result, Write};
use std::path::PathBuf;

pub fn create_or_override_config_file(
    filename: &str,
    data: String,
    is_force: bool,
    path: Option<PathBuf>,
) -> Result<()> {
    let file_path = path.unwrap_or(create_neatd_directory().join(filename));

    if is_force {
        let file = OpenOptions::new().write(true).open(&file_path);
        match file {
            Ok(mut file) => {
                // Re-write the config file
                _ = file.write_all(data.as_bytes());
                println!(
                    "Successfully re-written the `config.toml` file at {:?}",
                    file_path
                );
            }
            Err(e) => {
                eprintln!("An error occurred {}", e);
            }
        }
    } else {
        // It will throw "already exists" error on the file if it already exists
        // Otherwise will create a new config.toml file
        println!("path is {:?}", file_path);
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file_path);

        match file {
            Ok(mut file) => {
                // if a new file is created we have to write the config file rules/data in it.
                _ = file.write_all(data.as_bytes());
                println!(
                    "Successfully created the `config.toml` file at {:?}",
                    file_path
                );
            }
            Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {
                // if it already exists we should let the user know the location of the file and safely exit.
                println!(
                    "You have already initialized, the config file exists at {:?}, please edit it!",
                    file_path
                );
            }
            Err(e) => {
                eprintln!("An error occurred {}", e);
            }
        }
    }

    Ok(())
}
