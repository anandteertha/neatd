use crate::run::config::config::Config;
use std::{
    ffi::OsString,
    path::{Component, Path, PathBuf},
};

pub struct ConfigPolicy {
    include_roots: Vec<PathBuf>,
    exclude_roots: Vec<PathBuf>,
}

impl ConfigPolicy {
    pub fn new(&mut self, config: &Config, base_dir: &Path) -> Self {
        let includes: &Vec<PathBuf> = &config.paths.roots;
        let mut excludes: Vec<PathBuf> = Vec::new();

        // base excludes
        excludes.push(config.paths.quarantine.clone());
        excludes.push(config.paths.state_dir.clone());

        // rule destinations
        for rule in &config.rules {
            if let Some(action) = &rule.action {
                excludes.push(action.to.clone());
            }
        }
        Self {
            include_roots: Self::normalize_paths(includes, base_dir),
            exclude_roots: Self::normalize_paths(&excludes, base_dir),
        }
    }

    fn lexical_normalize(path: &Path) -> PathBuf {
        let mut anchor = PathBuf::new();
        let mut normals: Vec<OsString> = vec![];

        for c in path.components() {
            match c {
                Component::Prefix(_) | Component::RootDir => {
                    anchor.push(c.as_os_str());
                }
                Component::CurDir => {}
                Component::ParentDir => {
                    normals.pop();
                }
                Component::Normal(s) => {
                    normals.push(s.to_os_string());
                }
            }
        }

        let mut normalized_path = anchor;
        for normal in normals {
            normalized_path.push(normal);
        }
        normalized_path
    }

    fn normalize(path: &Path, absolute_paths: &mut Vec<PathBuf>) {
        absolute_paths.push(Self::lexical_normalize(path));
    }

    fn normalize_paths(dir: &[PathBuf], base_dir: &Path) -> Vec<PathBuf> {
        let mut absolute_paths = vec![];
        for path in dir {
            if path.is_relative() {
                Self::normalize(&base_dir.join(path), &mut absolute_paths);
            } else {
                Self::normalize(path, &mut absolute_paths);
            }
        }
        absolute_paths
    }
}
