# neatd - High-Level Design Document

## 1. Overview

**neatd** (neat daemon) is a cross-platform CLI tool that automatically organizes files in directories using configurable, rule-based policies. It runs as a background daemon or can execute as a one-time operation to clean up messy folders according to user-defined rules.

### 1.1 Purpose
- Automatically organize files based on configurable rules (file extensions, patterns)
- Support both daemon mode (continuous monitoring) and one-shot execution
- Provide safety features (dry-run, quarantine, validation)
- Cross-platform support (Windows, macOS, Linux)

### 1.2 Key Features
- Rule-based file organization with priority-based matching
- TOML configuration with validation
- Dry-run mode for safe testing
- Configurable file layouts (date-based organization)
- Ignore patterns (globs, hidden files, extensions)
- Safety controls (quarantine directory, delete protection)
- Colored configuration display

## 2. Architecture

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Interface                          │
│  (Commands: init, run, dry-run, status, validate, config)  │
└────────────────────┬────────────────────────────────────────┘
                     │
         ┌───────────┴───────────┐
         │                       │
┌────────▼────────┐    ┌─────────▼────────┐
│  Configuration  │    │   Execution      │
│   Management    │    │     Engine       │
└────────┬────────┘    └─────────┬────────┘
         │                       │
         │              ┌────────┴─────────┐
         │              │                  │
    ┌────▼────┐   ┌────▼────┐      ┌─────▼─────┐
    │  Parse  │   │ Policy  │      │  File     │
    │  Config │   │ System  │      │  System   │
    └─────────┘   └─────────┘      └───────────┘
```

### 2.2 Component Layers

1. **CLI Layer** (`src/main.rs`, `src/args.rs`)
   - Command parsing and routing
   - User interaction and output formatting

2. **Configuration Layer** (`src/parse/`, `src/config_file_data.rs`, `src/init/`)
   - Configuration file parsing (TOML)
   - Configuration validation
   - Default configuration generation

3. **Execution Layer** (`src/run/`)
   - Rule engine and matching logic
   - File system operations
   - Policy enforcement

4. **Infrastructure Layer** (`src/directory/`)
   - Directory management
   - Path resolution and normalization

## 3. Core Components

### 3.1 Configuration System

**Location**: `src/run/config/`, `src/parse/`

The configuration system manages the TOML-based configuration file that defines:
- **General settings**: execution mode, default actions, recursive scanning
- **Paths**: root directories, state directory, quarantine location
- **Ignore rules**: globs, hidden files, extensions
- **Naming**: normalization settings
- **Layout**: date-based organization format
- **Logging**: log levels
- **Reporting**: report formats
- **Safety**: protection mechanisms
- **Rules**: file organization rules with priorities

**Key Structures**:
- `Config`: Root configuration structure
- `General`: Execution mode and default behaviors
- `ConfigPaths`: Directory paths for roots, state, and quarantine
- `Rule`: Individual organization rule with match criteria and actions
- `Match`: File matching criteria (extensions, patterns)
- `Action`: Actions to perform (move, copy, delete) with destination

### 3.2 Rule Engine

**Location**: `src/run/config/config.rs`

The rule engine processes files according to prioritized rules:

1. **Rule Matching**:
   - Rules are evaluated by priority (lower number = higher priority)
   - Matching based on file extensions or catch-all patterns
   - First matching rule wins (priority-based)

2. **Rule Structure**:
   - `name`: Human-readable rule identifier
   - `enabled`: Toggle rule on/off
   - `priority`: Evaluation order (lower = first)
   - `match`: Matching criteria (extensions, any flag)
   - `action`: Action to perform (move/copy/delete, destination, layout)

3. **Actions**:
   - `type`: Action type (move, copy, delete)
   - `to`: Destination directory (relative or absolute)
   - `use_layout`: Enable date-based subdirectory layout

### 3.3 Policy System

**Location**: `src/run/policies/`

The policy system manages which paths should be processed:

1. **ConfigPolicy** (`src/run/policies/config_policy.rs`):
   - Extracts include/exclude paths from configuration
   - Normalizes paths (handles relative/absolute, resolves components)
   - Builds include roots (user-specified root directories)
   - Builds exclude roots (quarantine, state_dir, rule destinations)

2. **EffectivePolicy** (`src/run/policies/effective_policy.rs`):
   - Computes effective include/exclude sets by:
     - Removing redundant ancestor paths
     - Sorting by depth
     - Resolving conflicts (excludes take precedence)
   - Provides `should_process()` method for path filtering

**Path Resolution Logic**:
- Include roots: Directories to scan
- Exclude roots: Quarantine, state directory, and all rule destination directories
- Effective policy: Resolves overlaps and removes redundancies

### 3.4 File System Entry System

**Location**: `src/run/entries/`

**FsEntry** represents a file system entry with:
- `path`: File/directory path
- `kind`: File type (File, Directory, Symlink, Other)
- `metadata`: File metadata (size, timestamps, permissions, MIME type)
- `errors`: Collection of errors encountered during processing

**EntryError** provides structured error handling:
- `path`: Path where error occurred
- `operation`: Operation type (ReadDir, Metadata, Move, Copy, etc.)
- `source`: Underlying I/O error
- `severity`: Error severity (Warning, Error, Fatal)
- `outcome`: How error was handled (Skipped, Aborted, Retried)

### 3.5 CLI Commands

**Location**: `src/main.rs`, `src/args.rs`

1. **`init`**: 
   - Creates default configuration file at `~/.neatd/config.toml`
   - Supports custom path and force overwrite

2. **`validate`**:
   - Validates configuration file syntax and structure
   - Checks TOML parsing and required fields

3. **`print-config`**:
   - Displays configuration in colored, formatted output
   - Shows all settings, rules, and policies

4. **`dry-run`**:
   - Simulates file organization without making changes
   - Shows what would be organized

5. **`run`**:
   - Executes file organization
   - Supports `--once` (single execution) and `--daemon` (background)

6. **`status`**:
   - Shows daemon status and statistics

## 4. Data Flow

### 4.1 Configuration Flow

```
User runs 'init' command
    ↓
Create ~/.neatd/config.toml with defaults
    ↓
User edits config.toml
    ↓
User runs 'validate' command
    ↓
Parse TOML → Deserialize to Config struct
    ↓
Validate structure and fields
    ↓
Report success/errors
```

### 4.2 Execution Flow (Run Command)

```
User runs 'run' command
    ↓
Load and parse config.toml
    ↓
Build ConfigPolicy (include/exclude roots)
    ↓
Build EffectivePolicy (resolve path conflicts)
    ↓
For each root directory:
    ├─ Scan directory (recursive if enabled)
    ├─ For each file:
    │   ├─ Check if path should_process() (policy)
    │   ├─ Check ignore patterns (globs, hidden, extensions)
    │   ├─ Match against rules (priority order)
    │   ├─ Determine action (move/copy/delete)
    │   ├─ Calculate destination (with layout if enabled)
    │   ├─ Execute action (or simulate in dry-run)
    │   └─ Log result
    └─ Generate report
```

### 4.3 Rule Matching Flow

```
File found
    ↓
Filter by policy (should_process)
    ↓
Filter by ignore patterns
    ↓
Sort rules by priority (ascending)
    ↓
For each rule (in priority order):
    ├─ If rule disabled → Skip
    ├─ Check match criteria:
    │   ├─ Extension match?
    │   ├─ Catch-all (any = true)?
    │   └─ Pattern match?
    ├─ If match found:
    │   ├─ Extract action (move/copy/delete)
    │   ├─ Build destination path
    │   ├─ Apply layout (date format) if enabled
    │   └─ Return action
    └─ Continue to next rule
    ↓
Execute action (or simulate)
```

## 5. Configuration Schema

### 5.1 Configuration Structure

```toml
[general]
mode = "dry_run" | "run"
default_action = "move" | "copy" | "delete"
recursive = true | false

[paths]
roots = ["/path/to/dir1", "/path/to/dir2"]
state_dir = "/path/to/.neatd"
quarantine = "/path/to/quarantine"

[ignore]
globs = ["**/.git/**", "**/node_modules/**"]
ignore_hidden = true | false
extensions = ["swp", "bak"]

[naming]
normalize_names = true | false

[layout]
date_source = "modified" | "created" | "accessed"
date_format = "%Y/%m"  # strftime format

[log]
level = "info" | "error" | "success"

[report]
format = "text" | "spreadsheet" | "analytics"

[safety]
require_within_roots = true | false
allow_delete = true | false

[[rules]]
name = "Rule Name"
enabled = true | false
priority = 10  # Lower = higher priority

[rules.match]
extensions = ["png", "jpg", "jpeg"]
any = true | false  # Catch-all if true

[rules.action]
type = "move" | "copy" | "delete"
to = "destination/directory"
use_layout = true | false
```

### 5.2 Configuration Semantics

- **Mode**: `dry_run` simulates actions, `run` executes them
- **Roots**: Base directories to scan for files
- **State Directory**: Stores daemon state and metadata
- **Quarantine**: Safe location for problematic files
- **Priority**: Lower numbers are evaluated first
- **Layout**: Date-based subdirectory structure (e.g., `2025/12`)

## 6. Module Structure

```
src/
├── main.rs                 # Entry point, command routing
├── args.rs                 # CLI argument definitions
├── config_file_data.rs     # Default configuration template
│
├── directory/              # Directory utilities
│   └── mod.rs             # Home directory, path management
│
├── init/                   # Configuration initialization
│   └── mod.rs             # Create config file
│
├── parse/                  # Configuration parsing
│   └── mod.rs             # TOML parsing and validation
│
└── run/                    # Execution engine
    ├── mod.rs             # Run module exports
    │
    ├── config/            # Configuration structures
    │   ├── mod.rs
    │   ├── config.rs      # Config structs (Config, Rule, Action, etc.)
    │   └── display.rs     # Colored config display
    │
    ├── entries/           # File system entries
    │   ├── mod.rs
    │   └── fs_entry.rs    # FsEntry, EntryError structures
    │
    └── policies/          # Path policy system
        ├── mod.rs
        ├── config_policy.rs    # Extract include/exclude from config
        └── effective_policy.rs # Compute effective policy
```

## 7. Key Design Decisions

### 7.1 Configuration Format (TOML)
- **Rationale**: Human-readable, supports nested structures, good for configuration
- **Trade-offs**: Less flexible than JSON/YAML for complex structures, but better for user editing

### 7.2 Priority-Based Rule Matching
- **Rationale**: Allows fallback rules and predictable ordering
- **Trade-offs**: First-match wins; no rule composition, but simpler mental model

### 7.3 Policy System (Include/Exclude)
- **Rationale**: Prevents processing of destination directories and system paths
- **Trade-offs**: Adds complexity but essential for safety and correctness

### 7.4 Path Normalization
- **Rationale**: Handles relative paths, `.` and `..` components consistently
- **Trade-offs**: Lexical normalization (no symlink resolution) for performance

### 7.5 Structured Error Handling
- **Rationale**: FsEntry and EntryError provide detailed error context
- **Trade-offs**: More verbose but enables better error reporting and debugging

### 7.6 Separate Configuration and Execution
- **Rationale**: Clear separation of concerns, testable components
- **Trade-offs**: Some code duplication, but better maintainability

## 8. Safety Features

1. **Dry-Run Mode**: Test configuration without making changes
2. **Quarantine Directory**: Safe location for files that can't be processed
3. **Delete Protection**: `allow_delete` flag prevents accidental deletions
4. **Path Validation**: `require_within_roots` ensures operations stay within bounds
5. **Exclude Policy**: Automatically excludes destination directories from processing
6. **Configuration Validation**: Syntax and semantic validation before execution

## 9. Future Considerations

### 9.1 Potential Enhancements

1. **File Watching**: Real-time file system monitoring (inotify/fsevents)
2. **State Persistence**: Track processed files to avoid reprocessing
3. **Conflict Resolution**: Handle duplicate filenames in destinations
4. **Advanced Matching**: Regex patterns, file content inspection, MIME types
5. **Action Hooks**: Pre/post-action scripts
6. **Rule Composition**: Multiple match criteria (AND/OR logic)
7. **Scheduling**: Cron-like scheduling for periodic runs
8. **Multi-threading**: Parallel file processing
9. **Progress Reporting**: Real-time progress for large operations
10. **Configuration Migration**: Version management for config schema

### 9.2 Technical Debt Areas

1. **Daemon Implementation**: Currently stubbed, needs full implementation
2. **Dry-Run Implementation**: Needs simulation engine
3. **File System Operations**: Move/copy/delete operations not yet implemented
4. **Date Layout**: Date-based directory creation not implemented
5. **Error Recovery**: Retry logic and error recovery strategies
6. **Logging System**: Structured logging implementation
7. **Report Generation**: Report formats (text, spreadsheet, analytics)

## 10. Dependencies

- **clap**: CLI argument parsing
- **toml**: TOML configuration parsing
- **serde**: Serialization/deserialization
- **chrono**: Date/time handling
- **dirs**: Platform-specific directory resolution
- **os_info**: Operating system detection
- **colored**: Terminal color output

## 11. Platform Considerations

- **Cross-platform**: Uses `dirs` crate for home directory resolution
- **Path Handling**: PathBuf for cross-platform path management
- **File Metadata**: Platform-specific metadata (timestamps, permissions)
- **Daemon Mode**: Will require platform-specific daemon implementation

---

**Document Version**: 1.0  
**Last Updated**: 2025-12-16  
**Application Version**: 0.1.0

