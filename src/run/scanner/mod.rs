pub mod log;
pub mod set_kind;

use crate::run::{
    config::config::Config,
    entries::fs_entry::{FileKind, FsEntry},
    policies::{
        effective_policy::EffectivePolicy, setup_policy_for_walk, should_descend, should_process,
    },
    scanner::{log::Log, set_kind::set_entry_kind},
};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub fn walk_policy_setup(config: &Config, base_dir: &Path) {
    let effective_policy: EffectivePolicy = setup_policy_for_walk(config, base_dir);
    for path in &effective_policy.effective_includes {
        recurse_dirs(&effective_policy, path);
    }
}

fn recurse_dirs(effective_policy: &EffectivePolicy, path: &Path) -> Log {
    let mut fs_entry: FsEntry = FsEntry {
        path,
        kind: FileKind::Other,
        errors: Vec::new(),
        metadata: None,
    };
    for entry in read_dir(&path).expect("Cannot read the directories from the given path") {
        let entry = entry.expect("Cannot get the entry (file/dir) from the given path");
        let entry_path: PathBuf = entry.path();
        let can_descend: bool = should_descend(effective_policy, &entry_path);
        let can_process: bool = should_process(effective_policy, &entry_path);
        set_entry_kind(&mut fs_entry, &entry_path);
        if entry_path.is_dir() {
            if can_descend {
                return recurse_dirs(effective_policy, path);
            }
            return Log::CannotDescend;
        } else {
            if can_process {
                return Log::Success;
            }
            return Log::CannotProcess;
        }
    }
    return Log::UnknownError;
}
