use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::{
    run::config::config::{Config, ConfigPaths, Rule},
    validate::error::{ValidationError, ValidationResult},
};

pub fn validate_config(config: &Config, check_paths: bool) -> ValidationResult {
    let mut errors: Vec<ValidationError> = Vec::new();

    validate_paths(&config.paths, &mut errors, check_paths);
    validate_rules(&config.rules, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_paths(paths: &ConfigPaths, errors: &mut Vec<ValidationError>, check_paths: bool) {
    for root in &paths.roots {
        if check_paths {
            handle_invalid_path_error(root, errors);
        }
    }

    let state_dir = &paths.state_dir;
    handle_invalid_path_error(state_dir, errors);

    let quarantine = &paths.quarantine;
    handle_invalid_path_error(quarantine, errors);
}

fn handle_invalid_path_error(path: &PathBuf, errors: &mut Vec<ValidationError>) {
    if !Path::new(path).exists() {
        errors.push(ValidationError::InvalidPath {
            path: path.to_string_lossy().to_string(),
            reason: "Path does not exist".to_string(),
        });
    } else if !Path::new(path).is_dir() {
        errors.push(ValidationError::InvalidPath {
            path: path.to_string_lossy().to_string(),
            reason: "Path exists but is not a directory".to_string(),
        });
    }
}

fn validate_rules(rules: &[Rule], errors: &mut Vec<ValidationError>) {
    let mut seen_priorities = HashSet::new();

    for rule in rules {
        if seen_priorities.contains(&rule.priority) {
            errors.push(ValidationError::RuleError {
                rule_name: rule.name.clone(),
                reason: format!("Duplicate priority: {}", rule.priority),
            });
        }
        seen_priorities.insert(rule.priority);

        if let Some(match_criteria) = &rule.r#match {
            let has_extensions = match_criteria.extensions.is_some();
            let has_any = match_criteria.any.unwrap_or(false);

            if !has_extensions && !has_any {
                errors.push(ValidationError::RuleError {
                    rule_name: rule.name.clone(),
                    reason: "Rule must have at least one match criteria (extensions or any)"
                        .to_string(),
                });
            }
        }
    }
}
