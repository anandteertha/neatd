use crate::run::entries::fs_entry::{EntryError, EntryOp, Outcome, Severity};
use std::path::Path;

pub fn parse_errors(path: &Path, entry_error: Option<EntryError>) -> EntryError {
    entry_error.unwrap_or(EntryError {
        path: path.to_path_buf(),
        operation: EntryOp::ParsePath,
        source: None,
        severity: Severity::Warning,
        outcome: Outcome::Skipped,
    })
}
