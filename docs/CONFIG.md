# neatd - Configuration Reference

This document describes the complete configuration schema for neatd, including all options, examples, and best practices.

## Configuration File Location

By default, neatd looks for `config.toml` in `~/.neatd/`. You can override this with the `--path` option on commands that support it.

Initialize a new configuration with:
```bash
neatd init
```

## Configuration Structure

```toml
# Metadata
version = 1
created_by = "neatd"
created_at = "2025-12-16 07:10:02 PM"

# Global settings
[general]
mode = "dry_run"  # or "run"
default_action = "move"  # or "copy", "none"
recursive = true
dry_run_default = true  # Default to dry-run mode

# Paths
[paths]
roots = ["/Users/you/Downloads", "/Users/you/Desktop"]
state_dir = "/Users/you/.neatd/state"
quarantine = "/Users/you/.neatd/quarantine"

# Ignore patterns
[ignore]
extensions = ["swp", "bak", "tmp"]
globs = [
  "**/.git/**",
  "**/node_modules/**",
  "**/.DS_Store",
  "**/Thumbs.db",
  "**/*.crdownload",
  "**/*.part",
]
ignore_hidden = false
directories = [".git", "node_modules", "target", ".DS_Store"]

# Naming strategies
[naming]
normalize_names = false  # Normalize filenames (remove special chars)
slugify = false  # Convert to URL-safe slugs
preserve_case = true  # Preserve original case

# Layout (date-based organization)
[layout]
date_source = "modified"  # or "created", "accessed"
date_format = "%Y/%m"  # strftime format: 2025/12

# Logging
[log]
level = "info"  # or "error", "warn", "debug"

# Reporting
[report]
format = "text"  # or "json", "spreadsheet"

# Safety settings
[safety]
require_within_roots = true  # All operations must stay within roots
allow_delete = false  # Allow delete operations (requires explicit enable)
mode = "safe"  # or "aggressive" (safe = quarantine on conflict, no deletes)

# Rules (array of rule tables)
[[rules]]
name = "Images"
enabled = true
priority = 10  # Lower = higher priority

[rules.match]
extensions = ["png", "jpg", "jpeg", "gif", "webp", "heic", "tiff"]
globs = ["**/*.photo.*"]  # Optional: additional glob patterns
path_prefixes = []  # Optional: path prefix matches
# Optional: metadata constraints
# size_min = 1024  # Minimum size in bytes
# size_max = 10485760  # Maximum size in bytes
# age_days_min = 0  # Minimum age in days
# age_days_max = 365  # Maximum age in days

[rules.action]
type = "move"  # or "copy", "quarantine", "skip"
to = "images"  # Destination directory (relative to root or absolute)
use_layout = true  # Use date-based layout
conflict_strategy = "rename"  # or "keep_newest", "keep_oldest", "quarantine", "skip", "overwrite"
```

## Configuration Sections

### Metadata

**Fields**:
- `version` (integer): Configuration schema version
- `created_by` (string): Tool that created the config
- `created_at` (string): Creation timestamp

These fields are managed automatically and typically don't need manual editing.

### [general]

Global execution settings.

**Fields**:
- `mode` (string, default: `"dry_run"`): Execution mode
  - `"dry_run"`: Simulate operations without making changes
  - `"run"`: Execute operations
- `default_action` (string, default: `"move"`): Fallback action if no rule matches
  - `"move"`: Move file
  - `"copy"`: Copy file
  - `"none"`: Skip file
- `recursive` (boolean, default: `true`): Enable recursive directory traversal
- `dry_run_default` (boolean, default: `true`): Default to dry-run mode (can be overridden with flags)

**Example**:
```toml
[general]
mode = "dry_run"
default_action = "move"
recursive = true
dry_run_default = true
```

### [paths]

Directory paths for roots, state, and quarantine.

**Fields**:
- `roots` (array of strings): List of root directories to scan
  - Can be absolute or relative paths
  - Relative paths are resolved from current working directory
- `state_dir` (string): Directory for storing run history, audit logs, and state
  - Default: `~/.neatd/state`
- `quarantine` (string): Directory for quarantined files (conflicts, errors)
  - Default: `~/.neatd/quarantine`

**Example**:
```toml
[paths]
roots = ["/Users/you/Downloads", "/Users/you/Desktop"]
state_dir = "/Users/you/.neatd/state"
quarantine = "/Users/you/.neatd/quarantine"
```

**Best Practices**:
- Use absolute paths for clarity
- Ensure quarantine is outside of roots to avoid processing quarantined files
- State directory should be writable and persistent

### [ignore]

Global ignore patterns applied before rule matching.

**Fields**:
- `extensions` (array of strings): File extensions to ignore (case-insensitive)
  - Examples: `["swp", "bak", "tmp"]`
- `globs` (array of strings): Glob patterns to ignore
  - Supports `**` for recursive matching
  - Examples: `["**/.git/**", "**/node_modules/**"]`
- `ignore_hidden` (boolean, default: `false`): Ignore hidden files/directories (start with `.`)
- `directories` (array of strings): Directory names to ignore (exact match, case-sensitive)
  - Examples: `[".git", "node_modules", "target"]`

**Example**:
```toml
[ignore]
extensions = ["swp", "bak", "tmp"]
globs = [
  "**/.git/**",
  "**/node_modules/**",
  "**/.DS_Store",
]
ignore_hidden = false
directories = [".git", "node_modules"]
```

**Best Practices**:
- Ignore temporary files, build artifacts, and system files
- Use globs for complex patterns, extensions for simple cases
- Consider platform-specific files (`.DS_Store`, `Thumbs.db`)

### [naming]

Filename normalization and transformation settings.

**Fields**:
- `normalize_names` (boolean, default: `false`): Normalize filenames (remove/replace special characters)
- `slugify` (boolean, default: `false`): Convert filenames to URL-safe slugs
- `preserve_case` (boolean, default: `true`): Preserve original filename case

**Example**:
```toml
[naming]
normalize_names = false
slugify = false
preserve_case = true
```

**Note**: Naming transformations are applied after matching but before moving/copying.

### [layout]

Date-based directory layout settings.

**Fields**:
- `date_source` (string, default: `"modified"`): Which timestamp to use for date layout
  - `"modified"`: File modification time
  - `"created"`: File creation time
  - `"accessed"`: File access time
- `date_format` (string, default: `"%Y/%m"`): strftime format string for date directories
  - `"%Y/%m"`: Year/Month (e.g., `2025/12`)
  - `"%Y/%m/%d"`: Year/Month/Day (e.g., `2025/12/16`)
  - `"%Y"`: Year only (e.g., `2025`)

**Example**:
```toml
[layout]
date_source = "modified"
date_format = "%Y/%m"
```

**Date Layout Behavior**:
When `use_layout = true` in a rule action, files are organized like:
```
images/
  2025/
    12/
      photo.jpg
```

### [log]

Logging configuration.

**Fields**:
- `level` (string, default: `"info"`): Logging level
  - `"error"`: Only errors
  - `"warn"`: Warnings and errors
  - `"info"`: Informational messages, warnings, and errors
  - `"debug"`: Verbose debug output (includes traversal decisions, match traces)

**Example**:
```toml
[log]
level = "info"
```

### [report]

Report output format.

**Fields**:
- `format` (string, default: `"text"`): Default report format
  - `"text"`: Human-readable text output
  - `"json"`: JSON output (machine-readable)
  - `"spreadsheet"`: Spreadsheet format (future)

**Example**:
```toml
[report]
format = "text"
```

### [safety]

Safety and protection settings.

**Fields**:
- `require_within_roots` (boolean, default: `true`): All operations must stay within configured root directories
- `allow_delete` (boolean, default: `false`): Allow delete operations (requires explicit enable)
- `mode` (string, default: `"safe"`): Safety mode
  - `"safe"`: Quarantine on conflict, no deletes, strict validation
  - `"aggressive"`: Allow overwrites, deletes (if enabled), less strict validation

**Example**:
```toml
[safety]
require_within_roots = true
allow_delete = false
mode = "safe"
```

**Best Practices**:
- Always start with `mode = "safe"` and `allow_delete = false`
- Only enable aggressive mode after thorough testing
- Keep `require_within_roots = true` to prevent accidental moves outside roots

### [[rules]]

Rule definitions for file organization. Rules are evaluated in priority order (lower number = higher priority).

Each rule has:

#### Rule Metadata
- `name` (string): Human-readable rule name
- `enabled` (boolean): Enable/disable this rule
- `priority` (integer): Evaluation order (lower = first)

#### [rules.match]

Matching criteria (at least one must be specified):
- `extensions` (array of strings, optional): File extensions to match (case-insensitive)
- `globs` (array of strings, optional): Glob patterns to match
- `path_prefixes` (array of strings, optional): Path prefix matches
- `any` (boolean, optional): Match any file (catch-all rule)
- `size_min` (integer, optional): Minimum file size in bytes
- `size_max` (integer, optional): Maximum file size in bytes
- `age_days_min` (integer, optional): Minimum file age in days
- `age_days_max` (integer, optional): Maximum file age in days

#### [rules.action]

Action to perform:
- `type` (string): Action type
  - `"move"`: Move file to destination
  - `"copy"`: Copy file to destination
  - `"quarantine"`: Move to quarantine
  - `"skip"`: Skip this file
- `to` (string): Destination directory (relative to root or absolute)
- `use_layout` (boolean): Use date-based layout (creates date subdirectories)
- `conflict_strategy` (string, default: `"rename"`): How to handle destination conflicts
  - `"rename"`: Add incrementing suffix (`file-1.ext`, `file-2.ext`)
  - `"keep_newest"`: Keep file with newer modification time
  - `"keep_oldest"`: Keep file with older modification time
  - `"quarantine"`: Move conflicting file to quarantine
  - `"skip"`: Skip operation if conflict exists
  - `"overwrite"`: Overwrite existing file (unsafe, requires aggressive mode)

**Example Rules**:
```toml
# Images rule
[[rules]]
name = "Images"
enabled = true
priority = 10

[rules.match]
extensions = ["png", "jpg", "jpeg", "gif", "webp", "heic", "tiff"]

[rules.action]
type = "move"
to = "images"
use_layout = true
conflict_strategy = "rename"

# Videos rule
[[rules]]
name = "Videos"
enabled = true
priority = 20

[rules.match]
extensions = ["mp4", "mov", "mkv", "avi", "webm", "flv"]

[rules.action]
type = "move"
to = "videos"
use_layout = true
conflict_strategy = "rename"

# Documents rule
[[rules]]
name = "Documents"
enabled = true
priority = 30

[rules.match]
extensions = ["pdf", "doc", "docx", "ppt", "pptx", "xls", "xlsx", "txt", "md"]

[rules.action]
type = "move"
to = "documents"
use_layout = true
conflict_strategy = "rename"

# Large files rule (example with metadata constraints)
[[rules]]
name = "Large Files"
enabled = true
priority = 40

[rules.match]
size_min = 10485760  # 10 MB
globs = ["**/*"]

[rules.action]
type = "move"
to = "large_files"
use_layout = false
conflict_strategy = "rename"

# Fallback rule (catch-all)
[[rules]]
name = "Other"
enabled = true
priority = 999

[rules.match]
any = true

[rules.action]
type = "move"
to = "other"
use_layout = false
conflict_strategy = "rename"
```

## Configuration Validation

Validate your configuration with:
```bash
neatd validate
```

The validator checks:
- TOML syntax correctness
- Required fields are present
- Field types are correct
- Paths exist and are readable (optional)
- Glob patterns are valid
- Rule priorities are unique (warnings)
- Conflict strategies are valid
- Date formats are valid strftime strings

## Configuration Examples

### Minimal Configuration
```toml
version = 1
created_by = "neatd"

[general]
mode = "dry_run"
default_action = "move"
recursive = true

[paths]
roots = ["~/Downloads"]
state_dir = "~/.neatd/state"
quarantine = "~/.neatd/quarantine"

[ignore]
extensions = []
globs = []
ignore_hidden = false
directories = []

[safety]
require_within_roots = true
allow_delete = false
mode = "safe"

[[rules]]
name = "Everything"
enabled = true
priority = 1

[rules.match]
any = true

[rules.action]
type = "move"
to = "organized"
use_layout = false
```

### Comprehensive Configuration
See the default configuration generated by `neatd init` for a comprehensive example with multiple rules, ignore patterns, and safety settings.

## Best Practices

1. **Start with dry-run**: Always test with `mode = "dry_run"` first
2. **Use safe mode**: Keep `mode = "safe"` and `allow_delete = false` initially
3. **Organize rules by priority**: Lower numbers for specific rules, higher for fallbacks
4. **Test with explain**: Use `neatd explain <path>` to understand rule matching
5. **Validate before apply**: Always run `neatd validate` before `neatd apply`
6. **Backup important data**: Even with safety features, backup critical files
7. **Use quarantine**: Keep quarantine directory accessible for conflict resolution
8. **Document custom rules**: Add comments (in TOML) to explain complex rules

---

**Document Version**: 1.0  
**Last Updated**: 2025-12-16

