pub mod log;
pub mod set_error;
pub mod set_kind;

use crate::run::{
    config::config::Config,
    entries::fs_entry::{EntryError, EntryOp, FileKind, FsEntry, Outcome, Severity},
    policies::{
        effective_policy::EffectivePolicy, setup_policy_for_walk, should_descend, should_process,
    },
    scanner::{log::Log, set_error::parse_errors, set_kind::set_entry_kind},
};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub fn walk_policy_setup(config: &Config, base_dir: &Path) {
    let effective_policy: EffectivePolicy = setup_policy_for_walk(config, base_dir);
    for path in &effective_policy.effective_includes {
        recurse_dirs(&effective_policy, path, config);
    }
}

fn recurse_dirs(effective_policy: &EffectivePolicy, path: &Path, config: &Config) -> Log {
    let mut fs_entry: FsEntry = FsEntry {
        path: path.to_path_buf(),
        kind: FileKind::Other,
        errors: Vec::new(),
        metadata: None,
    };
    for entry in read_dir(&path).expect("Cannot read the directories from the given path") {
        let entry = entry;
        match entry {
            Ok(entry) => {
                let entry_path: PathBuf = entry.path();
                let can_descend: bool = should_descend(effective_policy, &entry_path);
                let can_process: bool = should_process(effective_policy, &entry_path);
                set_entry_kind(&mut fs_entry, &entry_path);
                if entry_path.is_dir() {
                    if can_descend {
                        return recurse_dirs(effective_policy, path, config);
                    }
                    parse_errors(&entry_path, None);
                    return Log::CannotDescend;
                } else {
                    if can_process {
                        return Log::Success;
                    }
                    parse_errors(&entry_path, None);
                    return Log::CannotProcess;
                }
            }
            Err(error) => {
                let entry_error = EntryError {
                    path: path.to_path_buf(),
                    operation: EntryOp::ParsePath,
                    source: Some(error),
                    severity: Severity::Error,
                    outcome: Outcome::Skipped,
                };
                parse_errors(path, Some(entry_error));
                return Log::UnknownError;
            }
        }
    }
    return Log::UnknownError;
}
