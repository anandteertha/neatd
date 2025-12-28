# neatd - Design Decisions

This document captures key architectural and design decisions made during the development of neatd, along with their rationale and trade-offs.

## 1. Configuration Format: TOML

**Decision**: Use TOML (Tom's Obvious Minimal Language) for configuration files.

**Rationale**:
- Human-readable and easy to edit manually
- Supports nested structures (arrays of tables for rules)
- Better for configuration than JSON (no trailing commas, comments allowed)
- Widely adopted in Rust ecosystem
- Better UX than YAML for simple configurations

**Trade-offs**:
- Less flexible than JSON/YAML for complex structures
- TOML parser may be less forgiving than JSON
- Limited support for advanced features (but sufficient for this use case)

**Alternatives Considered**:
- JSON: Too verbose, no comments
- YAML: More complex, indentation-sensitive
- INI: Too limited for nested structures

## 2. Rule Priority System (First-Match-Wins)

**Decision**: Rules are evaluated by priority (lower number = higher priority), first matching rule wins.

**Rationale**:
- Simple mental model: rules evaluated top-to-bottom by priority
- Predictable behavior: users can control order explicitly
- Efficient: stop evaluation after first match
- Supports fallback patterns (catch-all rule with highest priority number)

**Trade-offs**:
- No rule composition (can't combine multiple rules)
- No "all matching rules" behavior
- Users must be aware of priority ordering

**Example**:
```toml
[[rules]]
priority = 10  # Evaluated first
name = "Images"
match.extensions = ["png", "jpg"]

[[rules]]
priority = 999  # Evaluated last (fallback)
name = "Other"
match.any = true
```

**Alternatives Considered**:
- All matching rules: More complex, harder to reason about
- Rule composition (AND/OR): Too complex for initial version

## 3. Policy System: Include/Exclude with Effective Resolution

**Decision**: Separate include/exclude paths with effective policy resolution that removes redundant ancestors.

**Rationale**:
- Safety: Prevents processing destination directories
- Performance: Efficient path checking with hash sets
- Correctness: Resolves overlapping path configurations
- Flexibility: Users can specify multiple root directories

**Trade-offs**:
- Added complexity in policy resolution algorithm
- Path normalization required
- Lexical normalization (no symlink resolution) - may miss some cases

**Algorithm Complexity**:
- O(n log n) for sorting paths by depth
- O(n * d) for ancestor checking where d is average depth
- O(1) for should_process() checks (hash set lookup)

**Alternatives Considered**:
- Simple include list: Would process destination directories
- Whitelist only: Less flexible for complex directory structures

## 4. Path Normalization: Lexical Only

**Decision**: Use lexical path normalization (resolve `.` and `..`) without symlink resolution.

**Rationale**:
- Performance: No file system access required
- Cross-platform: Works consistently on all platforms
- Sufficient for most use cases (paths in config are typically absolute)
- Faster than canonical path resolution

**Trade-offs**:
- May not resolve symlinks (e.g., `/var/www` → `/home/user/www`)
- Relative paths in config require base directory context
- May have issues with junction points on Windows

**Alternatives Considered**:
- Canonical path resolution: Too slow, requires file system access
- No normalization: Would break with relative paths and `..`

## 5. Structured Error Handling: FsEntry + EntryError

**Decision**: Use structured error types (EntryError) within FsEntry to capture detailed error context.

**Rationale**:
- Better debugging: Know exactly what operation failed
- Error reporting: Can generate detailed error reports
- Error recovery: Can implement retry logic based on error type
- Type safety: Compiler ensures error handling

**Trade-offs**:
- More verbose than simple Result<T, E>
- Requires more upfront design of error types
- May be overkill for simple operations

**Design Pattern**:
```rust
pub struct FsEntry {
    path: PathBuf,
    kind: FileKind,
    metadata: Option<EntryMetaData>,
    errors: Vec<EntryError>,  // Non-fatal errors tracked here
}

pub struct EntryError {
    path: PathBuf,
    operation: EntryOp,
    source: Error,
    severity: Severity,
    outcome: Outcome,
}
```

**Alternatives Considered**:
- Simple Result types: Less context, harder to debug
- Panic on errors: Not appropriate for daemon/CLI tool

## 6. Module Organization: Feature-Based Structure

**Decision**: Organize code by feature/domain (config, policies, entries) rather than by layer (models, services, controllers).

**Rationale**:
- Clear boundaries: Each module is self-contained
- Easy to navigate: Related code is together
- Scalable: Easy to add new features
- Rust-friendly: Matches Rust's module system

**Structure**:
```
src/
├── run/
│   ├── config/      # Configuration structures
│   ├── policies/    # Policy system
│   └── entries/     # File system entries
```

**Trade-offs**:
- Some duplication possible (e.g., path utilities)
- Cross-module dependencies need careful management

**Alternatives Considered**:
- Layer-based (models, services, controllers): More traditional but less cohesive
- Flat structure: Would become unwieldy as code grows

## 7. CLI Design: Subcommand-Based

**Decision**: Use subcommands (init, run, validate, etc.) rather than flags or positional arguments.

**Rationale**:
- Clear separation of operations
- Extensible: Easy to add new commands
- Familiar pattern (git-style)
- Good tooling support (clap with derive)

**Command Structure**:
```bash
neatd init [--path] [--force]
neatd run [--once] [--daemon]
neatd validate [--path]
neatd print-config [--path]
neatd dry-run
neatd status
```

**Trade-offs**:
- More verbose than single command with flags
- Requires typing command name each time

**Alternatives Considered**:
- Single command with flags: Less clear, harder to extend
- Interactive mode: Good for beginners but adds complexity

## 8. Configuration Defaults: Embedded Template

**Decision**: Generate default configuration from embedded Rust template string rather than external file.

**Rationale**:
- Single binary: No external template file needed
- Version control: Template code is versioned with application
- Customizable: Can inject dynamic values (e.g., timestamps)
- Simple: No file reading/parsing needed

**Implementation**:
```rust
pub fn config_file_data() -> String {
    let now = Local::now();
    format!(r#"version = 1
created_at = {:?}
...
"#, now.format(...))
}
```

**Trade-offs**:
- Template is code (not a template file)
- Less flexible than external template system
- Requires recompilation to change defaults

**Alternatives Considered**:
- External template file: More flexible but requires bundling
- Minimal config: Would require more user configuration

## 9. Rule Actions: Move, Copy, Delete (Explicit)

**Decision**: Support three explicit action types (move, copy, delete) with explicit configuration.

**Rationale**:
- Clear intent: Users explicitly choose action
- Safety: Delete requires explicit enable flag
- Flexibility: Different rules can use different actions
- Simple: No complex action composition

**Trade-offs**:
- Can't compose actions (e.g., move then delete)
- No conditional actions based on file properties

**Alternatives Considered**:
- Single default action: Less flexible
- Action chains: Too complex for initial version

## 10. Date-Based Layout: Optional Per-Rule

**Decision**: Date-based directory layout is optional and configured per-rule via `use_layout` flag.

**Rationale**:
- Flexible: Some rules use layouts, others don't
- User control: Users decide which files get organized by date
- Simple: Binary choice (use layout or not)
- Performance: Can skip date computation for rules that don't need it

**Example**:
```toml
[[rules]]
name = "Images"
action.use_layout = true  # → images/2025/12/file.jpg

[[rules]]
name = "Archives"
action.use_layout = false  # → archives/file.zip
```

**Trade-offs**:
- Can't mix layouts (e.g., year/month vs year/week)
- Layout format is global (not per-rule)

**Alternatives Considered**:
- Always use layout: Less flexible
- Per-rule layout format: More complex configuration

## 11. State Directory: Separate from Configuration

**Decision**: Store daemon state in separate directory (`~/.neatd/state/`) from configuration.

**Rationale**:
- Clean separation: Config is user-editable, state is application-managed
- Clear ownership: State is internal, config is user-facing
- Backups: Users can backup config without state
- Upgrade safety: Can migrate state separately from config

**Structure**:
```
~/.neatd/
├── config.toml       # User configuration
└── state/            # Application state (future)
    ├── processed.db
    └── daemon.pid
```

**Trade-offs**:
- Two directories to manage
- Need to handle missing state directory

**Alternatives Considered**:
- Same directory: Simpler but mixes concerns
- No state directory: Would need state elsewhere or in-memory only

## 12. Quarantine Directory: Explicit Configuration

**Decision**: Quarantine directory is explicitly configured in config file, not hardcoded.

**Rationale**:
- User control: Users choose where problematic files go
- Flexibility: Can be anywhere (same drive, different location, etc.)
- Visibility: Users know where to look for quarantined files
- Safety: Explicit configuration reduces surprises

**Trade-offs**:
- Users must configure it (though default is provided)
- Could be misconfigured (e.g., pointing to a slow network drive)

**Alternatives Considered**:
- Hardcoded location: Less flexible
- Auto-generate: Less user control

## 13. No External Dependencies for Core Logic

**Decision**: Keep core logic (rules, policies, matching) in pure Rust code without heavy external dependencies.

**Rationale**:
- Performance: No external process overhead
- Reliability: Fewer failure points
- Portability: Single binary
- Simplicity: Easier to understand and debug

**Current Dependencies**:
- `clap`: CLI parsing (essential)
- `toml`: Configuration parsing (essential)
- `serde`: Serialization (essential)
- `chrono`: Date handling (essential)
- `dirs`: Home directory (essential)
- `os_info`: OS detection (useful)
- `colored`: Terminal output (convenience)

**Trade-offs**:
- More code to maintain
- Less use of battle-tested libraries
- May need to reimplement features

**Alternatives Considered**:
- Heavy frameworks: Would add complexity and dependencies

## 14. Cross-Platform Design: PathBuf and dirs Crate

**Decision**: Use `PathBuf` for all paths and `dirs` crate for platform-specific directory resolution.

**Rationale**:
- `PathBuf`: Handles path separators correctly on all platforms
- `dirs`: Provides correct home directory on all platforms
- Minimal platform-specific code
- Works on Windows, macOS, Linux

**Trade-offs**:
- Path semantics differ slightly between platforms (e.g., case sensitivity)
- Need to test on all platforms
- Some platform-specific features may not be available everywhere

**Platform Considerations**:
- Windows: Case-insensitive paths, drive letters, UNC paths
- Unix: Case-sensitive, symlinks, permissions
- macOS: Case-insensitive by default (but can be case-sensitive)

---

**Document Version**: 1.0  
**Last Updated**: 2025-12-16

