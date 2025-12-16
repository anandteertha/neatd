pub mod directory;

use directory::create_neatd_directory;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Result, Write};

pub fn create_or_override_config_file(filename: &str, data: String) -> Result<()> {
    // create neatd directory will create a .neatd folder and return its path irrespective of the OS
    let file_path = create_neatd_directory().join(filename);

    // It will throw already exists error on the file if it already exists
    // Otherwise will create a new config.toml file
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file_path);

    match file {
        Ok(mut file) => {
            // if a new file is created we have to write the config file rules/data in it.
            let _ = file.write_all(data.as_bytes());
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
    Ok(())
}
