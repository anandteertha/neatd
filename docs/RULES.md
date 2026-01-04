# neatd - Rules and Matching Logic

This document explains how rule matching works in neatd, including priority, matching criteria, and best practices.

## Rule Overview

Rules are the core mechanism for organizing files in neatd. Each rule defines:
1. **Matching criteria**: What files should match this rule
2. **Action**: What to do with matching files
3. **Priority**: Order of evaluation (lower = first)

## Rule Structure

```toml
[[rules]]
name = "Rule Name"
enabled = true
priority = 10

[rules.match]
# Matching criteria (at least one required)
extensions = ["png", "jpg"]
globs = ["**/*.photo.*"]
path_prefixes = ["/special/"]
any = false

[rules.action]
# Action to perform
type = "move"
to = "images"
use_layout = true
conflict_strategy = "rename"
```

## Matching Criteria

### Extensions

Match files by file extension (case-insensitive).

```toml
[rules.match]
extensions = ["png", "jpg", "jpeg", "gif"]
```

**Characteristics**:
- Case-insensitive matching (`PNG`, `png`, `PnG` all match)
- Matches exact extension only (`.jpg` matches, but not `.jpg.backup`)
- Fast matching (uses hash set for O(1) lookup)

**Example**:
- `photo.jpg` → matches
- `photo.JPG` → matches
- `photo.jpg.backup` → does not match (extension is `.backup`)

### Globs

Match files using glob patterns (supports `**` for recursive matching).

```toml
[rules.match]
globs = ["**/*.photo.*", "**/screenshot-*"]
```

**Characteristics**:
- Supports `**` for recursive directory matching
- Supports `*` for single-segment wildcard
- Supports `?` for single character
- Can match against filename or relative path from root
- Compiled for performance (compile once, reuse)

**Pattern Examples**:
- `**/*.jpg`: All `.jpg` files in any subdirectory
- `**/screenshot-*`: Files starting with `screenshot-` in any directory
- `*.tmp`: `.tmp` files in root only
- `backup/**`: All files in `backup` directory and subdirectories

### Path Prefixes

Match files by path prefix (useful for organizing by source location).

```toml
[rules.match]
path_prefixes = ["/Downloads/special/", "/Desktop/important/"]
```

**Characteristics**:
- Exact prefix matching (case-sensitive on Unix, case-insensitive on Windows)
- Useful for organizing files from specific locations
- Can combine with other criteria (AND logic)

### Metadata Constraints

Match files by metadata (size, age).

```toml
[rules.match]
size_min = 1048576  # 1 MB
size_max = 10485760  # 10 MB
age_days_min = 30
age_days_max = 365
```

**Characteristics**:
- `size_min`/`size_max`: File size in bytes
- `age_days_min`/`age_days_max`: File age in days (based on modification time)
- Can combine with other criteria (AND logic)

### Catch-All (Any)

Match any file (useful for fallback rules).

```toml
[rules.match]
any = true
```

**Characteristics**:
- Matches all files that haven't been matched by previous rules
- Typically used for fallback rules with high priority (e.g., `priority = 999`)

## Matching Logic

### Priority-Based Evaluation

Rules are evaluated in **priority order** (lower number = higher priority, evaluated first).

**Evaluation Process**:
1. Rules are sorted by priority (ascending)
2. For each file, rules are evaluated in order
3. **First match wins** (evaluation stops after first match)
4. If no rule matches, `default_action` from `[general]` is used

**Example**:
```toml
[[rules]]
name = "Images"
priority = 10  # Evaluated first
[rules.match]
extensions = ["png", "jpg"]

[[rules]]
name = "Large Images"
priority = 5  # Evaluated even earlier (lower = first)
[rules.match]
extensions = ["png", "jpg"]
size_min = 10485760  # 10 MB
```

In this example, large images (≥10 MB) match the "Large Images" rule first (priority 5), while smaller images match "Images" (priority 10).

### Deterministic Matching

Matching is **deterministic**: the same file always matches the same rule (given the same configuration).

**Guarantees**:
- Same filesystem state → same matches
- Same configuration → same matches
- Stable tie-breakers (rule name, if priorities are equal)

### Match Explanation

Every match includes an explanation:
- Which rule matched (name and priority)
- Exact conditions that matched (extension X, glob Y, path Z)
- Destination computed
- Any overrides applied

Use `neatd explain <path>` to see the explanation for a specific file.

## Combining Matching Criteria

Within a single rule, matching criteria use **AND logic** (all specified criteria must match).

**Example**:
```toml
[rules.match]
extensions = ["jpg", "png"]
size_min = 1048576  # 1 MB
globs = ["**/photo*"]
```

This rule matches files that:
- Have extension `.jpg` OR `.png` (extensions use OR)
- AND are at least 1 MB in size
- AND match the glob pattern `**/photo*`

## Rule Actions

### Action Types

- **`move`**: Move file to destination (default)
- **`copy`**: Copy file to destination (original remains)
- **`quarantine`**: Move to quarantine directory (for suspicious files)
- **`skip`**: Skip this file (no action)

### Destination

The `to` field specifies where files should be moved/copied:
- **Relative path**: Relative to the root directory where the file was found
  - Example: `to = "images"` → `{root}/images/`
- **Absolute path**: Absolute path (must be within roots if `require_within_roots = true`)
  - Example: `to = "/Users/you/Pictures"`

### Date Layout

When `use_layout = true`, files are organized by date:

```
images/
  2025/
    12/
      photo.jpg
```

Date format is controlled by `[layout]` settings:
- `date_source`: Which timestamp to use (`modified`, `created`, `accessed`)
- `date_format`: strftime format (`%Y/%m`, `%Y/%m/%d`, etc.)

### Conflict Strategies

When multiple files would go to the same destination:

- **`rename`** (default): Add incrementing suffix
  - `file.jpg` → `file-1.jpg` (if `file.jpg` exists)
  - `file-2.jpg` (if `file-1.jpg` also exists)
- **`keep_newest`**: Keep file with newer modification time
- **`keep_oldest`**: Keep file with older modification time
- **`quarantine`**: Move conflicting file to quarantine
- **`skip`**: Skip operation if conflict exists
- **`overwrite`**: Overwrite existing file (unsafe, requires `mode = "aggressive"`)

## Best Practices

### Rule Priority

1. **Specific rules first**: Lower priority numbers for specific rules
2. **Fallback rules last**: High priority numbers (e.g., 999) for catch-all rules
3. **Leave gaps**: Use priorities like 10, 20, 30 to allow inserting rules later

**Example**:
```toml
priority = 10   # Images
priority = 20   # Videos
priority = 30   # Documents
priority = 40   # Archives
priority = 999  # Fallback (catch-all)
```

### Rule Organization

1. **One rule per file type**: Avoid overlapping rules when possible
2. **Use extensions for common types**: Fast and clear
3. **Use globs for patterns**: Complex matching patterns
4. **Use metadata for filtering**: Size, age constraints
5. **Test with explain**: Use `neatd explain <path>` to verify rules

### Performance

1. **Extensions first**: Extension matching is fastest (O(1))
2. **Compile globs**: Glob patterns are compiled once (efficient)
3. **Avoid complex metadata queries**: Size/age checks are slower
4. **Order rules by frequency**: Most common matches should have lower priority

### Safety

1. **Test with dry-run**: Always test rules with `mode = "dry_run"` first
2. **Use safe conflict strategies**: Prefer `rename` over `overwrite`
3. **Enable rules gradually**: Start with `enabled = false`, test, then enable
4. **Keep fallback rules**: Always have a catch-all rule to handle unexpected files

## Examples

### Simple Extension-Based Rule

```toml
[[rules]]
name = "Images"
enabled = true
priority = 10

[rules.match]
extensions = ["png", "jpg", "jpeg", "gif", "webp"]

[rules.action]
type = "move"
to = "images"
use_layout = true
conflict_strategy = "rename"
```

### Complex Rule with Multiple Criteria

```toml
[[rules]]
name = "Large Photos"
enabled = true
priority = 5

[rules.match]
extensions = ["jpg", "jpeg", "png"]
size_min = 5242880  # 5 MB
globs = ["**/photo*", "**/IMG_*"]

[rules.action]
type = "move"
to = "photos/large"
use_layout = true
conflict_strategy = "keep_newest"
```

### Path-Based Rule

```toml
[[rules]]
name = "Downloads Organizer"
enabled = true
priority = 1

[rules.match]
path_prefixes = ["/Users/you/Downloads/"]
extensions = ["pdf", "doc", "docx"]

[rules.action]
type = "move"
to = "documents/downloads"
use_layout = false
conflict_strategy = "rename"
```

### Age-Based Rule

```toml
[[rules]]
name = "Old Files"
enabled = true
priority = 50

[rules.match]
any = true
age_days_min = 365  # Older than 1 year

[rules.action]
type = "move"
to = "archive"
use_layout = true
conflict_strategy = "rename"
```

### Fallback Rule

```toml
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

## Troubleshooting

### Rule Not Matching

1. Check rule is `enabled = true`
2. Verify priority (lower rules evaluated first)
3. Use `neatd explain <path>` to see why file doesn't match
4. Check ignore patterns (files ignored before rule matching)
5. Verify matching criteria (extensions case-insensitive, globs correct)

### Multiple Rules Matching

Remember: **first match wins**. If a file matches multiple rules, only the first (lowest priority) rule's action is applied.

To debug:
1. Check rule priorities
2. Use `neatd explain <path>` to see which rule matched
3. Reorder rules if needed

### Unexpected Behavior

1. Validate configuration: `neatd validate`
2. Test with dry-run: `neatd plan`
3. Use explain command: `neatd explain <path>`
4. Check conflict strategy (may affect behavior)
5. Review ignore patterns (may exclude files before matching)

---

**Document Version**: 1.0  
**Last Updated**: 2025-12-16

