# EPIC0-1 Implementation Guide: Enhanced Configuration Validation

**Story**: EPIC0-1 - Comprehensive Configuration Validation with Field-Level Diagnostics  
**Priority**: Critical  
**Target Audience**: Learning Rust while implementing

This guide will teach you the Rust concepts and libraries needed to implement comprehensive configuration validation.

---

## Table of Contents

1. [Overview](#overview)
2. [What You'll Learn](#what-youll-learn)
3. [Prerequisites](#prerequisites)
4. [Rust Concepts You Need](#rust-concepts-you-need)
5. [Libraries/Crates You'll Use](#librariescrates-youll-use)
6. [Step-by-Step Implementation](#step-by-step-implementation)
7. [Code Examples](#code-examples)
8. [Testing Your Implementation](#testing-your-implementation)
9. [Resources for Learning](#resources-for-learning)

---

## Overview

You'll be implementing a validation module that:
- Validates TOML configuration files
- Checks all fields for correctness
- Provides actionable error messages
- Validates paths, globs, and other complex types

**Estimated Learning Time**: 2-3 days if new to Rust  
**Estimated Implementation Time**: 1-2 days after understanding concepts

---

## What You'll Learn

By the end of this guide, you'll understand:
- ✅ Rust error handling (`Result`, `Option`)
- ✅ Rust structs, enums, and pattern matching
- ✅ Working with the `serde` library (serialization/deserialization)
- ✅ Working with the `toml` crate
- ✅ Working with file paths (`std::path::PathBuf`)
- ✅ Working with glob patterns
- ✅ Building validation logic
- ✅ Creating helpful error messages

---

## Prerequisites

### 1. Install Rust

If you haven't already:
```bash
# Install Rust (one-liner from rust-lang.org)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or on Windows, download and run rustup-init.exe from:
# https://rustup.rs/

# Verify installation
rustc --version
cargo --version
```

### 2. Basic Rust Knowledge

You should understand:
- Variables (`let`, `mut`)
- Functions
- Basic types (`String`, `i32`, `bool`, etc.)
- Ownership basics (borrowing `&`, ownership)

If not, complete the first 4 chapters of [The Rust Book](https://doc.rust-lang.org/book/) (30-60 minutes).

---

## Rust Concepts You Need

### 1. Result<T, E> - Error Handling

**What it is**: `Result` is Rust's way of handling operations that can fail.

```rust
// Result has two variants:
enum Result<T, E> {
    Ok(T),   // Success case - contains value
    Err(E),  // Error case - contains error
}
```

**Example**:
```rust
// Function that can fail
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Using it
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),  // Prints: Result: 5
    Err(error) => println!("Error: {}", error),
}

// Or use ? operator (shorthand for error propagation)
fn calculate() -> Result<i32, String> {
    let result = divide(10, 2)?;  // If Err, returns early
    Ok(result * 2)
}
```

**For this story**: You'll use `Result<Config, ValidationError>` to return either a valid config or validation errors.

### 2. Option<T> - Nullable Values

**What it is**: `Option` represents a value that might not exist.

```rust
enum Option<T> {
    Some(T),   // Value exists
    None,      // No value
}
```

**Example**:
```rust
// Function that might not return a value
fn find_item(items: &[String], target: &str) -> Option<usize> {
    for (i, item) in items.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

// Using it
match find_item(&vec!["a".to_string(), "b".to_string()], "b") {
    Some(index) => println!("Found at index: {}", index),
    None => println!("Not found"),
}

// Or use unwrap_or for defaults
let index = find_item(&items, "x").unwrap_or(0);
```

**For this story**: You'll use `Option` for optional config fields.

### 3. Structs - Custom Data Types

**What it is**: Structs group related data together.

```rust
// Define a struct
struct Person {
    name: String,
    age: u32,
}

// Create an instance
let person = Person {
    name: String::from("Alice"),
    age: 30,
};

// Access fields
println!("{} is {} years old", person.name, person.age);

// Methods (functions attached to struct)
impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    
    fn greet(&self) {
        println!("Hello, I'm {}", self.name);
    }
}
```

**For this story**: You'll create `ValidationError` and `ValidationResult` structs.

### 4. Enums - Multiple Variants

**What it is**: Enums allow you to define a type with multiple possible variants.

```rust
// Define an enum
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),  // Variant with data
}

// Use it with pattern matching
fn color_name(color: Color) -> String {
    match color {
        Color::Red => "red".to_string(),
        Color::Green => "green".to_string(),
        Color::Blue => "blue".to_string(),
        Color::RGB(r, g, b) => format!("RGB({}, {}, {})", r, g, b),
    }
}
```

**For this story**: You'll create enums for error types and severity levels.

### 5. Pattern Matching - Match Expressions

**What it is**: `match` is Rust's powerful control flow operator.

```rust
let number = 5;

match number {
    1 => println!("One"),
    2 | 3 => println!("Two or three"),
    4..=10 => println!("Four through ten"),
    _ => println!("Something else"),
}

// Match on Result
match some_result {
    Ok(value) => println!("Got: {}", value),
    Err(error) => println!("Error: {}", error),
}

// Match on Option
match some_option {
    Some(value) => println!("Value: {}", value),
    None => println!("No value"),
}
```

**For this story**: You'll use `match` extensively for validation logic.

### 6. Vectors - Dynamic Arrays

**What it is**: `Vec<T>` is a growable array.

```rust
// Create a vector
let mut numbers: Vec<i32> = Vec::new();

// Add items
numbers.push(1);
numbers.push(2);
numbers.push(3);

// Or create with initial values
let numbers = vec![1, 2, 3];

// Iterate
for num in &numbers {
    println!("{}", num);
}

// Or with indices
for (i, num) in numbers.iter().enumerate() {
    println!("{}: {}", i, num);
}
```

**For this story**: You'll collect multiple validation errors in a `Vec<ValidationError>`.

### 7. String Handling

**What it is**: Rust has two string types.

```rust
// &str - string slice (immutable, borrowed)
let s1: &str = "Hello";

// String - owned string (mutable, owned)
let mut s2 = String::from("Hello");
s2.push_str(" World");

// Convert between them
let s3: String = s1.to_string();
let s4: &str = &s2[..];

// String formatting
let name = "Alice";
let age = 30;
let message = format!("{} is {} years old", name, age);
```

**For this story**: You'll build error messages as `String`s.

### 8. File System Operations

**What it is**: Rust's standard library for file operations.

```rust
use std::fs;
use std::path::Path;

// Check if path exists
if Path::new("/path/to/file").exists() {
    println!("File exists");
}

// Check if path is a file
if Path::new("/path/to/file").is_file() {
    println!("It's a file");
}

// Check if path is a directory
if Path::new("/path/to/dir").is_dir() {
    println!("It's a directory");
}

// Read file
match fs::read_to_string("/path/to/file") {
    Ok(contents) => println!("File contents: {}", contents),
    Err(e) => println!("Error reading file: {}", e),
}
```

**For this story**: You'll check if config paths exist and are readable.

---

## Libraries/Crates You'll Use

### 1. serde - Serialization Framework

**What it is**: The most popular Rust library for converting data structures to/from various formats (JSON, TOML, etc.).

**Why you need it**: Your config is already using `serde` to deserialize TOML. You'll use it to work with the `Config` struct.

**Key concepts**:
```rust
use serde::Deserialize;

// Derive macro automatically generates deserialization code
#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    age: u32,
}
```

**Documentation**: https://serde.rs/

**For this story**: The `Config` struct already uses `serde::Deserialize`. You'll validate the deserialized struct.

### 2. toml - TOML Parser

**What it is**: Parser for TOML (Tom's Obvious Minimal Language) files.

**Why you need it**: Your config files are in TOML format. You'll use it to parse and validate.

**Key concepts**:
```rust
use toml;

// Parse TOML string
let toml_str = r#"
name = "Alice"
age = 30
"#;

let config: Config = toml::from_str(toml_str)?;
```

**Documentation**: https://docs.rs/toml/latest/toml/

**For this story**: You might need to parse TOML manually for advanced validation (though the existing code already does this).

### 3. std::path::PathBuf - Path Handling

**What it is**: Rust's standard library type for file paths.

**Why you need it**: You need to validate that paths in the config exist.

**Key concepts**:
```rust
use std::path::{Path, PathBuf};

// Create a PathBuf
let path = PathBuf::from("/home/user/file.txt");

// Convert to string
let path_str = path.to_string_lossy();  // Returns Cow<str>

// Join paths
let base = PathBuf::from("/home/user");
let full_path = base.join("documents").join("file.txt");

// Check if exists
if path.exists() {
    println!("Path exists");
}

// Check if readable (try to read metadata)
match std::fs::metadata(&path) {
    Ok(metadata) => println!("Can read path"),
    Err(_) => println!("Cannot read path"),
}
```

**Documentation**: https://doc.rust-lang.org/std/path/

**For this story**: You'll validate that `roots`, `state_dir`, and `quarantine` paths exist (if the flag is set).

### 4. glob - Glob Pattern Matching (Optional)

**What it is**: Library for matching file paths against glob patterns (like `**/*.txt`).

**Why you need it**: You need to validate glob patterns in the ignore section.

**Adding it to your project**:
```toml
# In Cargo.toml
[dependencies]
glob = "0.3"
```

**Key concepts**:
```rust
use glob::Pattern;

// Compile a glob pattern
match Pattern::new("**/*.txt") {
    Ok(pattern) => {
        // Use pattern to match
        if pattern.matches("path/to/file.txt") {
            println!("Matches!");
        }
    },
    Err(e) => println!("Invalid glob pattern: {}", e),
}
```

**Documentation**: https://docs.rs/glob/latest/glob/

**For this story**: You'll validate glob patterns in `ignore.globs`.

---

## Step-by-Step Implementation

### Phase 1: Set Up the Validation Module

**Step 1.1: Create the validation module structure**

```rust
// In src/validate/mod.rs
pub mod error;
pub mod validator;

pub use error::{ValidationError, ValidationResult};
pub use validator::validate_config;
```

**Step 1.2: Define error types**

Create `src/validate/error.rs`:
```rust
#[derive(Debug, Clone)]
pub enum ValidationError {
    MissingField {
        field: String,
        section: String,
    },
    InvalidType {
        field: String,
        section: String,
        expected: String,
        got: String,
    },
    InvalidEnumValue {
        field: String,
        section: String,
        expected: Vec<String>,
        got: String,
    },
    InvalidPath {
        path: String,
        reason: String,
    },
    InvalidGlob {
        pattern: String,
        reason: String,
    },
    RuleError {
        rule_name: String,
        reason: String,
    },
}

impl ValidationError {
    pub fn to_string(&self) -> String {
        match self {
            ValidationError::MissingField { field, section } => {
                format!("Missing required field '{field}' in section '[{section}]'")
            }
            // ... other variants
            _ => String::from("Validation error"),
        }
    }
}

pub type ValidationResult = Result<(), Vec<ValidationError>>;
```

**Why this structure**: 
- Enums let you represent different error types
- Using `Vec<ValidationError>` collects all errors (not just the first)
- `to_string()` creates user-friendly error messages

---

### Phase 2: Implement Field Validation

**Step 2.1: Validate required fields**

```rust
// In src/validate/validator.rs
use crate::run::config::config::Config;
use super::error::{ValidationError, ValidationResult};

pub fn validate_config(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();
    
    // Validate general section
    validate_general(&config.general, &mut errors);
    
    // Validate paths section
    validate_paths(&config.paths, &mut errors);
    
    // Validate rules
    validate_rules(&config.rules, &mut errors);
    
    // Return result
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_general(general: &General, errors: &mut Vec<ValidationError>) {
    // Check if mode is valid (already validated by serde, but you can double-check)
    // Most validation happens at deserialization level
}
```

**Step 2.2: Validate paths (if flag is set)**

```rust
use std::path::Path;

fn validate_paths(paths: &ConfigPaths, errors: &mut Vec<ValidationError>, check_existence: bool) {
    // Validate roots
    for root in &paths.roots {
        if check_existence {
            if !Path::new(root).exists() {
                errors.push(ValidationError::InvalidPath {
                    path: root.to_string_lossy().to_string(),
                    reason: "Path does not exist".to_string(),
                });
            } else if !Path::new(root).is_dir() {
                errors.push(ValidationError::InvalidPath {
                    path: root.to_string_lossy().to_string(),
                    reason: "Path exists but is not a directory".to_string(),
                });
            }
        }
    }
    
    // Similar for state_dir and quarantine
}
```

**Learning points**:
- `&mut Vec<ValidationError>` - mutable reference so you can push errors
- `to_string_lossy()` - converts PathBuf to String, handling invalid UTF-8 gracefully
- Pattern matching on `Path::new().exists()` vs `Path::new().is_dir()`

---

### Phase 3: Validate Complex Types

**Step 3.1: Validate glob patterns**

```rust
// Option 1: Use glob crate
use glob::Pattern;

fn validate_globs(globs: &[PathBuf], errors: &mut Vec<ValidationError>) {
    for glob_path in globs {
        let glob_str = glob_path.to_string_lossy();
        match Pattern::new(&glob_str) {
            Ok(_) => {
                // Pattern is valid
            }
            Err(e) => {
                errors.push(ValidationError::InvalidGlob {
                    pattern: glob_str.to_string(),
                    reason: format!("Invalid glob syntax: {}", e),
                });
            }
        }
    }
}
```

**Step 3.2: Validate rule priorities**

```rust
use std::collections::HashSet;

fn validate_rules(rules: &[Rule], errors: &mut Vec<ValidationError>) {
    let mut seen_priorities = HashSet::new();
    
    for rule in rules {
        // Check for duplicate priorities
        if seen_priorities.contains(&rule.priority) {
            errors.push(ValidationError::RuleError {
                rule_name: rule.name.clone(),
                reason: format!("Duplicate priority: {}", rule.priority),
            });
        }
        seen_priorities.insert(rule.priority);
        
        // Validate rule has match criteria
        if let Some(ref match_criteria) = rule.r#match {
            let has_extensions = match_criteria.extensions.is_some();
            let has_globs = match_criteria.globs.is_some();
            let has_any = match_criteria.any.unwrap_or(false);
            
            if !has_extensions && !has_globs && !has_any {
                errors.push(ValidationError::RuleError {
                    rule_name: rule.name.clone(),
                    reason: "Rule must have at least one match criteria (extensions, globs, or any)".to_string(),
                });
            }
        } else {
            errors.push(ValidationError::RuleError {
                rule_name: rule.name.clone(),
                reason: "Rule missing match criteria".to_string(),
            });
        }
        
        // Validate action has destination
        if let Some(ref action) = rule.action {
            // Destination is a PathBuf, so it's always "set"
            // But you could validate it's not empty
        } else {
            errors.push(ValidationError::RuleError {
                rule_name: rule.name.clone(),
                reason: "Rule missing action".to_string(),
            });
        }
    }
}
```

**Learning points**:
- `HashSet` - for tracking unique values (priorities)
- `if let Some(ref x)` - pattern matching for Options
- `clone()` - creates owned copy (needed because we're borrowing)

---

### Phase 4: Generate User-Friendly Error Messages

**Step 4.1: Enhance error messages**

```rust
impl ValidationError {
    pub fn format_for_user(&self) -> String {
        match self {
            ValidationError::MissingField { field, section } => {
                format!(
                    "❌ Missing required field\n\
                     \n\
                     Field: '{}'\n\
                     Section: '[{}]'\n\
                     \n\
                     💡 Fix: Add '{} = <value>' to the [{}] section",
                    field, section, field, section
                )
            }
            ValidationError::InvalidPath { path, reason } => {
                format!(
                    "❌ Invalid path\n\
                     \n\
                     Path: {}\n\
                     Reason: {}\n\
                     \n\
                     💡 Fix: Check that the path exists and is accessible",
                    path, reason
                )
            }
            // ... other variants with helpful messages
            _ => self.to_string(),
        }
    }
}
```

---

### Phase 5: Integrate with Validate Command

**Step 5.1: Update the validate command**

```rust
// In src/main.rs or src/validate/mod.rs
use crate::validate::validate_config;
use crate::parse::read_config;

pub fn run_validate(path: Option<PathBuf>) -> Result<(), ()> {
    let config_file_path = path.unwrap_or_else(|| {
        get_file_path(get_hom_directory(), "config.toml")
    });
    
    // Parse config (this might fail with TOML parse errors)
    let config = match read_config(&config_file_path) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Failed to parse config file");
            return Err(());
        }
    };
    
    // Validate config
    match validate_config(&config) {
        Ok(()) => {
            println!("✅ Configuration is valid!");
            Ok(())
        }
        Err(errors) => {
            eprintln!("❌ Configuration validation failed with {} errors:\n", errors.len());
            
            for (i, error) in errors.iter().enumerate() {
                println!("{}. {}", i + 1, error.format_for_user());
                println!();  // Blank line between errors
            }
            
            Err(())
        }
    }
}
```

---

## Code Examples

### Complete Example: Simple Validator

Here's a minimal working example:

```rust
// src/validate/mod.rs
pub mod error;
pub mod validator;

// src/validate/error.rs
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub section: String,
    pub message: String,
}

impl ValidationError {
    pub fn format(&self) -> String {
        format!(
            "Field '{}' in section '[{}]': {}",
            self.field, self.section, self.message
        )
    }
}

// src/validate/validator.rs
use crate::run::config::config::Config;
use super::error::ValidationError;

pub fn validate_config(config: &Config) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    
    // Example: Validate that at least one root is specified
    if config.paths.roots.is_empty() {
        errors.push(ValidationError {
            field: "roots".to_string(),
            section: "paths".to_string(),
            message: "At least one root directory must be specified".to_string(),
        });
    }
    
    // Return result
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

---

## Testing Your Implementation

### Write Tests

```rust
// In src/validate/validator.rs (or separate test file)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::config::config::Config;
    
    #[test]
    fn test_validate_empty_roots() {
        // Create a config with empty roots
        let config = Config {
            // ... create config with empty roots
        };
        
        let result = validate_config(&config);
        
        assert!(result.is_err());
        if let Err(errors) = result {
            assert!(errors.len() > 0);
            assert!(errors[0].field == "roots");
        }
    }
    
    #[test]
    fn test_validate_valid_config() {
        // Create a valid config
        let config = create_valid_config();
        
        let result = validate_config(&config);
        
        assert!(result.is_ok());
    }
}

// Run tests with:
// cargo test
```

---

## Resources for Learning

### Essential Resources

1. **The Rust Book** (Free, Official)
   - URL: https://doc.rust-lang.org/book/
   - Focus on: Chapters 6 (Enums), 9 (Error Handling), 10 (Generics)
   - Time: 2-3 hours for relevant chapters

2. **Rust by Example** (Free, Interactive)
   - URL: https://doc.rust-lang.org/rust-by-example/
   - Focus on: Error handling, Options, Results
   - Time: 1-2 hours

3. **serde Documentation**
   - URL: https://serde.rs/
   - Focus on: Deserialization, custom deserializers
   - Time: 30 minutes

### Learning Path

**Day 1**: Learn Rust Basics
- Read Rust Book Chapters 1-6
- Understand ownership, borrowing, structs, enums
- Practice with small programs

**Day 2**: Learn Error Handling
- Read Rust Book Chapter 9
- Practice with `Result` and `Option`
- Understand `?` operator

**Day 3**: Learn Relevant Libraries
- Study `serde` documentation
- Study `std::path` documentation
- Study `glob` crate (if using)

**Day 4-5**: Implement Validation
- Start with simple validations
- Build up to complex ones
- Test thoroughly

### Quick Reference

**Common Patterns**:
```rust
// Pattern: Collect errors, don't fail fast
let mut errors = Vec::new();
if condition1 { errors.push(error1); }
if condition2 { errors.push(error2); }
if errors.is_empty() { Ok(()) } else { Err(errors) }

// Pattern: Validate and collect
fn validate_something(value: &T, errors: &mut Vec<Error>) {
    if !is_valid(value) {
        errors.push(Error::new(...));
    }
}

// Pattern: Option handling
match optional_value {
    Some(value) => validate(value, errors),
    None => errors.push(Error::Missing(...)),
}
```

---

## Implementation Checklist

Follow this order:

- [ ] **Step 1**: Understand the existing `Config` struct (read `src/run/config/config.rs`)
- [ ] **Step 2**: Create `src/validate/mod.rs` and module structure
- [ ] **Step 3**: Define `ValidationError` enum with all error types
- [ ] **Step 4**: Implement `validate_config()` function skeleton
- [ ] **Step 5**: Implement simple validations first (required fields)
- [ ] **Step 6**: Implement path validation (optional, behind flag)
- [ ] **Step 7**: Implement glob pattern validation
- [ ] **Step 8**: Implement rule validation (priorities, match criteria)
- [ ] **Step 9**: Add user-friendly error formatting
- [ ] **Step 10**: Integrate with `validate` command
- [ ] **Step 11**: Write tests
- [ ] **Step 12**: Test with real config files

---

## Getting Help

If you get stuck:

1. **Check the Rust Book** - It has excellent explanations
2. **Read Error Messages** - Rust's compiler errors are very helpful
3. **Use rust-analyzer** - IDE plugin that shows type hints
4. **Search Rust Users Forum** - https://users.rust-lang.org/
5. **Check Stack Overflow** - Many common questions answered

---

## Next Steps After Completing This Story

Once you finish EPIC0-1, you'll be ready for:
- **EPIC0-2**: Enhanced Configuration Schema (similar concepts)
- **EPIC1-1**: Fix Scanner Bugs (file system operations, error handling)

---

**Good luck! Remember: Rust has a steep learning curve, but it's worth it. The compiler is your friend - it will catch many bugs before runtime.**

**Last Updated**: 2025-12-16

