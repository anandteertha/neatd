// ============================================
// EPIC0-1: Configuration Validation Template
// ============================================
// Copy-paste this code as a starting point
// Make sure to understand what each part does!

// ============================================
// FILE 1: src/validate/mod.rs
// ============================================
// This file declares the validate module and exports public items

pub mod error;
pub mod validator;

// Re-export commonly used types
pub use error::ValidationError;
pub use validator::validate_config;

// ============================================
// FILE 2: src/validate/error.rs
// ============================================
// This file defines error types for validation

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub field: String,
    pub section: String,
    pub suggested_fix: Option<String>,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(message: String, field: String, section: String) -> Self {
        Self {
            message,
            field,
            section,
            suggested_fix: None,
        }
    }

    /// Create with suggested fix
    pub fn with_fix(
        message: String,
        field: String,
        section: String,
        suggested_fix: String,
    ) -> Self {
        Self {
            message,
            field,
            section,
            suggested_fix: Some(suggested_fix),
        }
    }

    /// Format error for user display
    pub fn format(&self) -> String {
        let mut output = format!(
            "❌ Field '{}' in section '[{}]': {}",
            self.field, self.section, self.message
        );

        if let Some(ref fix) = self.suggested_fix {
            output.push_str(&format!("\n   💡 Fix: {}", fix));
        }

        output
    }
}

// ============================================
// FILE 3: src/validate/validator.rs
// ============================================
// This file contains the actual validation logic

use crate::run::config::config::Config;
use super::error::ValidationError;

/// Main validation function
/// Returns Ok(()) if config is valid, Err(errors) if there are validation errors
pub fn validate_config(config: &Config) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Validate each section
    validate_general(&config.general, &mut errors);
    validate_paths(&config.paths, &mut errors);
    validate_ignore(&config.ignore, &mut errors);
    validate_rules(&config.rules, &mut errors);
    validate_safety(&config.safety, &mut errors);

    // Return result
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate [general] section
fn validate_general(general: &General, errors: &mut Vec<ValidationError>) {
    // Example: You could check if mode is valid
    // (but this is already validated by serde enum)
    
    // Add your validations here
}

/// Validate [paths] section
fn validate_paths(paths: &ConfigPaths, errors: &mut Vec<ValidationError>) {
    // Check that roots is not empty
    if paths.roots.is_empty() {
        errors.push(ValidationError::with_fix(
            "At least one root directory must be specified".to_string(),
            "roots".to_string(),
            "paths".to_string(),
            "Add at least one path to roots = [\"/path/to/directory\"]".to_string(),
        ));
    }

    // TODO: Add more path validations
    // - Check paths exist (if flag is set)
    // - Check paths are directories
    // - Check paths are readable
}

/// Validate [ignore] section
fn validate_ignore(ignore: &Ignore, errors: &mut Vec<ValidationError>) {
    // TODO: Validate glob patterns
    // - Check glob syntax is correct
    // - Warn about potentially problematic patterns
}

/// Validate rules
fn validate_rules(rules: &[Rule], errors: &mut Vec<ValidationError>) {
    use std::collections::HashSet;

    let mut seen_priorities = HashSet::new();

    for (index, rule) in rules.iter().enumerate() {
        let rule_id = format!("{} (index {})", rule.name, index);

        // Check for duplicate priorities
        if seen_priorities.contains(&rule.priority) {
            errors.push(ValidationError::new(
                format!("Duplicate priority: {}", rule.priority),
                "priority".to_string(),
                format!("rules[{}]", index),
            ));
        } else {
            seen_priorities.insert(rule.priority);
        }

        // Check rule has match criteria
        if let Some(ref match_criteria) = rule.r#match {
            let has_extensions = match_criteria.extensions.as_ref()
                .map(|exts| !exts.is_empty())
                .unwrap_or(false);
            let has_globs = match_criteria.globs.as_ref()
                .map(|globs| !globs.is_empty())
                .unwrap_or(false);
            let has_any = match_criteria.any.unwrap_or(false);

            if !has_extensions && !has_globs && !has_any {
                errors.push(ValidationError::with_fix(
                    "Rule must have at least one match criteria".to_string(),
                    "match".to_string(),
                    format!("rules[{}]", index),
                    "Add extensions, globs, or set any = true".to_string(),
                ));
            }
        } else {
            errors.push(ValidationError::new(
                "Rule is missing match criteria".to_string(),
                "match".to_string(),
                format!("rules[{}]", index),
            ));
        }

        // Check rule has action
        if rule.action.is_none() {
            errors.push(ValidationError::new(
                "Rule is missing action".to_string(),
                "action".to_string(),
                format!("rules[{}]", index),
            ));
        }
    }
}

/// Validate [safety] section
fn validate_safety(safety: &Safety, errors: &mut Vec<ValidationError>) {
    // TODO: Add safety validations
    // - Check mode is valid
    // - Warn if allow_delete is true (risky)
}

// ============================================
// FILE 4: Update src/main.rs
// ============================================
// Add these lines to your main.rs

// At the top with other mod declarations:
// mod validate;

// In the imports:
// use crate::validate::validate_config;

// Update the Validate command handler:
/*
Some(Commands::Validate { path }) => {
    let config_file_path: PathBuf =
        path.unwrap_or(get_file_path(get_hom_directory(), "config.toml"));
    
    // Parse config first
    let config = match read_config(&config_file_path) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("❌ Failed to parse config file");
            return;
        }
    };
    
    // Validate it
    match validate_config(&config) {
        Ok(()) => {
            println!("✅ Configuration is valid!");
        }
        Err(errors) => {
            eprintln!("❌ Found {} validation errors:\n", errors.len());
            for error in errors {
                println!("{}\n", error.format());
            }
        }
    }
}
*/

// ============================================
// USAGE EXAMPLE
// ============================================
/*
fn example_usage() {
    // Load config
    let config = read_config(&config_path).unwrap();
    
    // Validate
    match validate_config(&config) {
        Ok(()) => println!("Valid!"),
        Err(errors) => {
            for error in errors {
                println!("{}", error.format());
            }
        }
    }
}
*/

// ============================================
// TESTING EXAMPLE
// ============================================
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_roots() {
        // Create a config with empty roots
        let mut config = create_test_config();
        config.paths.roots.clear();
        
        let result = validate_config(&config);
        
        assert!(result.is_err());
        if let Err(errors) = result {
            assert!(errors.len() > 0);
            assert!(errors[0].field == "roots");
        }
    }

    #[test]
    fn test_valid_config() {
        let config = create_valid_test_config();
        let result = validate_config(&config);
        assert!(result.is_ok());
    }
}
*/

// ============================================
// NOTES
// ============================================
/*
Key Concepts Used:
1. Vec<ValidationError> - Collects multiple errors
2. Result<(), Vec<ValidationError>> - Returns either success or list of errors
3. &mut Vec<...> - Mutable reference to allow pushing errors
4. Pattern matching (if let, match) - Handle Options and Results
5. String formatting - Build user-friendly messages

Common Patterns:
- Collect errors, don't fail fast
- Use references (&) to avoid moving values
- Use clone() when you need owned values
- Use format!() for string building
*/

