use dirs::home_dir;
use std::{fs::create_dir_all, path::PathBuf};

pub fn create_neatd_directory() -> PathBuf {
    let home_dir: PathBuf = home_dir().expect("Failed to get home directory");
    let neatd_dir = home_dir.join(".neatd");
    _ = create_dir_all(&neatd_dir);
    neatd_dir
}
