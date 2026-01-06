# EPIC0-1 Quick Start Guide

**For**: Learning Rust while implementing configuration validation  
**Prerequisites**: Basic Rust installed (`cargo --version` should work)

---

## 🚀 Getting Started in 5 Steps

### Step 1: Understand What You're Building

You're creating a module that validates configuration files. It will:
- Check all fields are correct
- Validate paths exist (optional)
- Validate glob patterns are valid
- Check rules are properly configured
- Give helpful error messages

### Step 2: Explore the Existing Code

Before writing new code, understand what exists:

```bash
# Look at the Config structure
cat src/run/config/config.rs

# Look at how config is currently parsed
cat src/parse/mod.rs

# Look at how validate command currently works
cat src/main.rs  # Lines around 51-54
```

**Key insight**: The `Config` struct already exists and uses `serde` to deserialize from TOML. Your job is to validate the deserialized struct.

### Step 3: Create Your First File

Create the validation module:

```bash
# Create the directory
mkdir -p src/validate

# Create the main module file
touch src/validate/mod.rs
touch src/validate/error.rs
touch src/validate/validator.rs
```

### Step 4: Write Your First Validation

Start simple! Here's a minimal working example:

**File: `src/validate/error.rs`**
```rust
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub field: String,
    pub section: String,
}

impl ValidationError {
    pub fn format(&self) -> String {
        format!(
            "❌ Error in field '{}' (section [{}]): {}",
            self.field, self.section, self.message
        )
    }
}
```

**File: `src/validate/validator.rs`**
```rust
use crate::run::config::config::Config;
use super::error::ValidationError;

pub fn validate_config(config: &Config) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    
    // Your first validation: check roots are not empty
    if config.paths.roots.is_empty() {
        errors.push(ValidationError {
            message: "At least one root directory must be specified".to_string(),
            field: "roots".to_string(),
            section: "paths".to_string(),
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

**File: `src/validate/mod.rs`**
```rust
pub mod error;
pub mod validator;

pub use error::ValidationError;
pub use validator::validate_config;
```

**File: `src/main.rs`** (add at top with other mods)
```rust
mod validate;  // Add this line
```

### Step 5: Connect It to the Validate Command

Update the validate command in `src/main.rs`:

```rust
use crate::validate::validate_config;  // Add this import

// In the match statement, update the Validate branch:
Some(Commands::Validate { path }) => {
    let config_file_path: PathBuf =
        path.unwrap_or(get_file_path(get_hom_directory(), "config.toml"));
    
    // Parse config first
    let config = match read_config(&config_file_path) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Failed to parse config file");
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
                println!("{}", error.format());
            }
        }
    }
}
```

### Step 6: Test It!

```bash
# Build the project
cargo build

# Test with a config that has empty roots
neatd validate

# If it works, you should see an error about empty roots!
```

---

## 📚 Learning Path (What to Study First)

### Priority 1: Learn These Now (30 minutes)

1. **Vectors (`Vec<T>`)**
   - How to create: `let mut vec = Vec::new();`
   - How to push: `vec.push(item);`
   - How to check empty: `vec.is_empty()`
   - [Learn more](https://doc.rust-lang.org/book/ch08-01-vectors.html)

2. **Result Type**
   - `Ok(value)` = success
   - `Err(error)` = failure
   - `match` to handle both
   - [Learn more](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

3. **Structs**
   - Define: `struct MyStruct { field: Type }`
   - Create: `MyStruct { field: value }`
   - Access: `instance.field`
   - [Learn more](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

### Priority 2: Learn These Next (1 hour)

4. **Enums** - For different error types
5. **Pattern Matching** - Using `match` expressions
6. **String Handling** - Building error messages
7. **Option Type** - For optional fields

### Priority 3: Learn As You Need (1-2 hours)

8. **File System Operations** - For path validation
9. **serde Concepts** - Understanding the Config struct
10. **Glob Patterns** - If you use the glob crate

---

## 🎯 Implementation Order (Recommended)

Don't try to do everything at once! Implement in this order:

### Phase 1: Foundation (Day 1)
1. ✅ Create module structure
2. ✅ Define `ValidationError` struct (simple version)
3. ✅ Write one simple validation (empty roots check)
4. ✅ Connect to validate command
5. ✅ Test it works

### Phase 2: More Validations (Day 2)
6. ✅ Add more simple validations (empty arrays, etc.)
7. ✅ Validate rule priorities are unique
8. ✅ Validate rules have match criteria

### Phase 3: Complex Validations (Day 3-4)
9. ✅ Add path validation (optional, behind flag)
10. ✅ Add glob pattern validation
11. ✅ Improve error messages

### Phase 4: Polish (Day 5)
12. ✅ Add `--strict` and `--check-paths` flags
13. ✅ Write tests
14. ✅ Test with real config files

---

## 💡 Common Patterns You'll Use

### Pattern 1: Collect Errors (Don't Fail Fast)

```rust
let mut errors = Vec::new();

// Validate multiple things
if condition1 { 
    errors.push(error1); 
}
if condition2 { 
    errors.push(error2); 
}

// Return all errors at once
if errors.is_empty() {
    Ok(())
} else {
    Err(errors)
}
```

### Pattern 2: Validate and Collect

```rust
fn validate_something(value: &ValueType, errors: &mut Vec<ValidationError>) {
    if !is_valid(value) {
        errors.push(ValidationError {
            message: "Something is wrong".to_string(),
            field: "field_name".to_string(),
            section: "section_name".to_string(),
        });
    }
}

// Use it
validate_something(&config.field, &mut errors);
```

### Pattern 3: Option Handling

```rust
// If field is optional
match optional_value {
    Some(value) => {
        // Validate the value
        validate_something(&value, errors);
    }
    None => {
        // Field is missing (if required)
        errors.push(ValidationError {
            message: "Required field is missing".to_string(),
            // ...
        });
    }
}
```

---

## 🔧 Tools That Will Help

### 1. rust-analyzer (VS Code Extension)

**Install**: Search "rust-analyzer" in VS Code extensions

**Why**: 
- Shows types on hover
- Auto-completion
- Error highlighting
- "Go to definition"

### 2. Cargo Commands

```bash
# Build your project
cargo build

# Build and run
cargo run -- validate

# Check for errors without building
cargo check

# Format your code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test
```

### 3. Reading Compiler Errors

Rust's compiler gives GREAT error messages. Read them carefully!

Example:
```rust
error[E0382]: borrow of moved value: `config`
  --> src/validate/validator.rs:10:5
   |
8  |     let config = read_config()?;
   |         ------ move occurs because `config` has type `Config`, which does not implement the `Copy` trait
9  |     validate(&config);
   |              ------- value moved here
10 |     println!("{:?}", config);  // ERROR: config was moved!
   |                      ^^^^^^ value borrowed here after move
```

**What it means**: You moved `config` into `validate()`, so you can't use it again.

**Fix**: Use a reference: `validate(&config)` and change function to take `&Config`.

---

## 📝 Code Template to Copy-Paste

Here's a complete starter template:

```rust
// src/validate/mod.rs
pub mod error;
pub mod validator;

pub use error::ValidationError;
pub use validator::validate_config;

// src/validate/error.rs
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub field: String,
    pub section: String,
}

impl ValidationError {
    pub fn format(&self) -> String {
        format!(
            "❌ Field '{}' in section '[{}]': {}",
            self.field, self.section, self.message
        )
    }
}

// src/validate/validator.rs
use crate::run::config::config::Config;
use super::error::ValidationError;

pub fn validate_config(config: &Config) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    
    // TODO: Add your validations here
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

---

## 🐛 Debugging Tips

### 1. Print Debugging

```rust
// Add this to see what you're working with
println!("{:?}", config);  // Prints the whole config
println!("Roots: {:?}", config.paths.roots);  // Prints just roots
```

### 2. Use `dbg!()` Macro

```rust
// This prints the value AND returns it
let roots = dbg!(&config.paths.roots);
// Prints: [src/validate/validator.rs:10] &config.paths.roots = [...]
```

### 3. Compile Often

```bash
# Run this frequently to catch errors early
cargo check
```

### 4. Read Error Messages

Rust compiler errors are VERY helpful. They often tell you exactly what to do!

---

## ✅ First Validation Checklist

Try implementing this ONE validation first:

- [ ] Check if `config.paths.roots` is empty
- [ ] If empty, add an error to the errors vector
- [ ] Return `Ok(())` if no errors, `Err(errors)` if errors exist
- [ ] Update validate command to call your function
- [ ] Test with a config that has empty roots
- [ ] Verify you see the error message

**Once this works, you understand the pattern!** Then you can add more validations.

---

## 📖 Where to Get Help

1. **Rust Book**: https://doc.rust-lang.org/book/ - Read relevant chapters
2. **Rust by Example**: https://doc.rust-lang.org/rust-by-example/ - Copy-paste examples
3. **Compiler Errors**: Read them! They're super helpful
4. **rust-analyzer**: Hover over code to see types

---

## 🎓 Key Learning Moments

As you implement, you'll encounter these Rust concepts. Here's what they mean:

### "Borrowed value does not live long enough"
**Meaning**: You're trying to use a reference after the value is gone  
**Fix**: Make sure the value lives as long as the reference

### "Cannot move out of borrowed content"
**Meaning**: You're trying to take ownership from a borrowed value  
**Fix**: Clone it or use a reference

### "Expected `&String`, found `String`"
**Meaning**: Function expects a reference, you gave ownership  
**Fix**: Use `&` to create a reference: `&my_string`

### "Value used after move"
**Meaning**: You moved a value into a function, then tried to use it again  
**Fix**: Pass a reference instead: `function(&value)` instead of `function(value)`

---

## 🚦 Next Steps

1. **Read** the full [Implementation Guide](./EPIC0-1-IMPLEMENTATION-GUIDE.md)
2. **Study** the Rust concepts you need (30-60 minutes)
3. **Implement** the first validation (empty roots check)
4. **Test** it works
5. **Expand** to more validations
6. **Ask for help** if stuck!

---

**Remember**: Everyone starts somewhere. The Rust compiler will guide you. You've got this! 💪

**Last Updated**: 2025-12-16

