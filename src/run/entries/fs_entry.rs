use std::{io::Error, path::PathBuf, time::SystemTime};

pub struct FsEntry {
    pub path: PathBuf,
    pub kind: FileKind,
    pub metadata: Option<EntryMetaData>,
    pub errors: Vec<EntryError>,
}

pub enum FileKind {
    File,
    Directory,
    Symlink,
    Other,
}

pub struct EntryMetaData {
    pub size_bytes: Option<u64>,
    pub modified: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
    pub readonly: Option<bool>,
    pub mime: Option<String>,
    pub canonical_path: Option<PathBuf>,
}

pub struct EntryError {
    pub path: PathBuf,
    pub operation: EntryOp,
    pub source: Option<Error>,
    pub severity: Severity,
    pub outcome: Outcome,
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
