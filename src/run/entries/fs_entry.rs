use std::{io::Error, path::PathBuf, time::SystemTime};

pub struct FsEntry {
    path: PathBuf,
    kind: FileKind,
    metadata: Option<EntryMetaData>,
    errors: Vec<EntryError>,
}

pub enum FileKind {
    File,
    Directory,
    Symlink,
    Other,
}

pub struct EntryMetaData {
    size_bytes: Option<u64>,
    modified: Option<SystemTime>,
    created: Option<SystemTime>,
    accessed: Option<SystemTime>,
    readonly: Option<bool>,
    mime: Option<String>,
    canonical_path: Option<PathBuf>,
}

pub struct EntryError {
    path: PathBuf,
    operation: EntryOp,
    source: Error,
    severity: Severity,
    outcome: Outcome,
}

pub enum EntryOp {
    // EntryError
    ReadDir,

    // fetching metadata
    Metadata,

    // (resolving symlinks/absolute identity)
    Canonicalize,

    // (if later you inspect content)
    Open,
    Move,
    Rename,
    Copy,
    Delete,

    // (if you do ~ expansion / env substitution)
    ParsePath,
    MatchRule, // (optional: if you want rule-engine errors to be structured similarly)
}

pub enum Severity {
    Warning,
    Error,
    Fatal,
}

pub enum Outcome {
    Skipped,
    Aborted,
    Retried(u32),
}
