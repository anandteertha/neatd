use std::fs::OpenOptions;
use std::io::{ErrorKind, Result, Write};

pub fn create_or_override_config_file(filename: &str, data: String) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename);
    match file {
        Ok(mut file) => {
            let _ = file.write_all(data.as_bytes());
            println!("Successfully created the `config.toml` file");
        }
        Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {
            println!("You have already initialized, the config file exists, please edit it!");
        }
        Err(e) => {
            println!("An error occurred {}", e);
        }
    }
    Ok(())
}
