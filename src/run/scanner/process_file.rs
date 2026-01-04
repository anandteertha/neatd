use std::{
    collections::HashSet,
    default,
    path::{self, Path, PathBuf},
};

use crate::run::{
    config::config::{Config, Match, Rule},
    entries::fs_entry::FsEntry,
};

pub fn apply_rules_to_file(config: &Config, entry: &mut FsEntry) {
    // for the current entry
    // based on the priority of rules apply each rule.
    // before priority you need to check if the rule matches with the entry
    let mut matched_rules = Vec::new();
    let path = &entry.path.to_string_lossy().to_ascii_lowercase();
    let match_default = Match::new();
    for rule in &config.rules {
        let available_extensions = rule
            .r#match
            .as_ref()
            .unwrap_or(&match_default)
            .extensions
            .as_deref();
        let available_extensions_hash_set = get_hash_set(available_extensions.unwrap_or(&[]));
        if available_extensions_hash_set.contains(path) {
            matched_rules.push(rule);
        }
    }
    // once you get the matched rules..
    // sort based on priority
    // after sorting update entry to contain the process that needs to be done.
}

pub fn get_hash_set(vec_of_strings: &[String]) -> HashSet<String> {
    let mut hash_set: HashSet<String> = HashSet::new();
    for val in vec_of_strings {
        hash_set.insert(val.clone().to_ascii_lowercase());
    }
    hash_set
}
