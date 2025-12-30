use crate::run::entries::fs_entry::{FileKind, FsEntry};
use std::path::Path;

pub fn set_entry_kind(entry: &mut FsEntry, path: &Path) {
    if path.is_dir() {
        entry.kind = FileKind::Directory;
    } else {
        if path.is_symlink() {
            entry.kind = FileKind::Symlink;
        } else if let Some(_) = path.extension() {
            entry.kind = FileKind::File;
        } else {
            entry.kind = FileKind::Other;
        }
    }
}
