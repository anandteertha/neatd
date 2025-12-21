use dirs::home_dir;
use std::{fs::create_dir_all, path::PathBuf};

pub fn create_neatd_directory() -> PathBuf {
    let neatd_dir = get_hom_directory();
    _ = create_dir_all(&neatd_dir);
    neatd_dir
}

pub fn get_hom_directory() -> PathBuf {
    let home_dir: PathBuf = home_dir().expect("Failed to get home directory");
    let neatd_dir: PathBuf = home_dir.join(".neatd");
    neatd_dir
}

pub fn get_file_path(dir_path: PathBuf, filename: &str) -> PathBuf {
    dir_path.join(filename)
}
