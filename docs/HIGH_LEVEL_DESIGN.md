# neatd - High-Level Design Document

## 1. Overview

**neatd** (neat daemon) is a safe, configurable, cross-platform file organizer that scans one or more root folders, classifies files using policy rules (extensions, globs, paths, metadata), and applies deterministic actions (move/copy/rename/tag/quarantine) with strong safety guarantees (dry-run, conflicts handling, undo, audit logs).

### 1.1 Product Vision

**Neatd** enables users to automatically organize their files with zero-surprise behavior: explicit policies, predictable results, and clear reports before changes. The system prioritizes safety first with dry-run by default, conflict-safe moves, quarantine, and undo capabilities.

### 1.2 Core Goals

* **Zero surprise behavior**: Explicit policies, predictable results, clear reports before changes
* **Safety first**: Dry-run by default (optional), conflict-safe moves, quarantine, undo
* **High performance**: Scalable on large directories (100k+ files) with efficient traversal and filtering
* **Cross-platform**: macOS, Linux, Windows (including WSL paths considerations)
* **Great UX**: Clear CLI, explainable decisions ("why this file matched rule X"), good error messages

### 1.3 Non-Goals (v1)

* Cloud storage organization (Drive/Dropbox) as a first-class target
* Content-based classification requiring heavy parsing (OCR, ML, video scanning)
* Full GUI (possible future extension; not required for v1)

### 1.4 Key Features

* Rule-based file organization with priority-based, deterministic matching
* TOML configuration with comprehensive validation
* Multiple execution modes: scan, plan, apply, undo
* Dry-run mode for safe testing (default behavior)
* Configurable file layouts (date-based organization, naming strategies)
* Advanced ignore patterns (globs, hidden files, extensions, directories)
* Safety controls (quarantine directory, conflict resolution, delete protection)
* Audit logging and run history with undo capability
* Explanatory output (`explain` command shows why files match rules)
* Watch mode for continuous monitoring (optional)

## 2. Architecture

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      CLI Interface                               │
│  init, validate, scan, plan, apply, undo, status, explain,      │
│  watch, report                                                  │
└────────────────────┬────────────────────────────────────────────┘
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
    │  Parse  │   │ Policy  │      │  Scanner  │
    │  Config │   │ Engine  │      │  (Entries)│
    └─────────┘   └────┬────┘      └─────┬─────┘
                       │                  │
                       │          ┌───────┴────────┐
                       │          │                │
                  ┌────▼────┐ ┌───▼────┐    ┌─────▼─────┐
                  │ Matcher │ │Planner │    │ Executor  │
                  └────┬────┘ └───┬────┘    └─────┬─────┘
                       │          │                │
                       └──────────┼────────────────┘
                                  │
                          ┌───────▼────────┐
                          │  State & Audit │
                          │  + Reporter    │
                          └────────────────┘
```

### 2.2 Component Layers

1. **CLI Layer** (`src/main.rs`, `src/args.rs`)
   - Command parsing and routing
   - User interaction and output formatting
   - Help text and error messages

2. **Configuration Layer** (`src/parse/`, `src/config_file_data.rs`, `src/init/`)
   - Configuration file parsing (TOML)
   - Comprehensive configuration validation
   - Default configuration generation
   - Schema validation and diagnostics

3. **Entries Layer** (`src/run/entries/`)
   - File system entry representation
   - Metadata collection (size, timestamps, permissions)
   - Error collection and tracking

4. **Scanner Layer** (`src/run/scanner/`)
   - Directory traversal with recursion control
   - Ignore rule application
   - Symlink policy handling
   - Robust error handling (permissions, broken links)

5. **Policy Engine** (`src/run/policies/`)
   - Effective policy computation (include/exclude resolution)
   - Path normalization and validation
   - Rule compilation and ordering

6. **Matcher Layer** (`src/run/matcher/`)
   - Rule matching logic (extensions, globs, paths)
   - Priority-based rule evaluation
   - Match explanation generation

7. **Planner Layer** (`src/run/planner/`)
   - Operation plan generation
   - Conflict detection and resolution
   - Operation ordering and grouping
   - Skip reasoning

8. **Executor Layer** (`src/run/executor/`)
   - Safe plan execution
   - Transactional moves (atomic where possible)
   - Quarantine handling
   - Audit log generation

9. **State & Audit Layer** (`src/run/state/`)
   - Run history management
   - Undo map storage
   - State snapshots
   - Audit log persistence

10. **Reporter Layer** (`src/run/reporter/`)
    - Human-readable console output
    - JSON report generation
    - Summary statistics
    - Error reporting with remediation hints

## 3. Core Components

### 3.1 Configuration System

**Location**: `src/run/config/`, `src/parse/`

The configuration system manages the TOML-based configuration file with comprehensive settings:

**Global Settings**:
- `roots`: List of paths to scan
- `recursive`: Enable recursive directory traversal
- `dry_run_default`: Default to dry-run mode
- `default_action`: move/copy/none (fallback action)
- `mode`: safe/aggressive (safe = quarantine on conflict, no deletes)
- `state_dir`: Where runs and logs are stored
- `quarantine_dir`: Where suspicious/conflicting files are moved
- `log.level`: error/warn/info/debug

**Ignore Block**:
- `ignore.extensions`: Case-insensitive extension list
- `ignore.globs`: Glob patterns (temporary files, build artifacts)
- `ignore.hidden`: Toggle to ignore hidden files
- `ignore.directories`: List of directory names to ignore (`.git`, `node_modules`)

**Rules Block** (each rule supports):
- `name`, `enabled`, `priority`
- `match`: extensions, globs, path prefixes, metadata constraints
- `action`: move/copy/quarantine/skip, destination template, naming strategy, conflict strategy

**Conflict Strategies**:
- `rename`: Add incrementing suffix
- `keep_newest`/`keep_oldest`: Keep based on modification time
- `quarantine`: Move conflicts to quarantine
- `skip`: Skip conflicting operation
- `overwrite`: Overwrite (unsafe, requires explicit enable)

**Key Structures**:
- `Config`: Root configuration structure
- `General`: Execution mode, default actions, safety mode
- `ConfigPaths`: Directory paths for roots, state, and quarantine
- `Rule`: Individual organization rule with comprehensive match criteria and actions
- `Match`: File matching criteria (extensions, globs, path patterns, metadata)
- `Action`: Actions to perform with destination, naming, and conflict resolution

### 3.2 Entries System

**Location**: `src/run/entries/`

**FsEntry** represents each filesystem object (file/dir/symlink/other) with:
- `path`: Absolute/normalized path
- `kind`: FileKind (File, Directory, Symlink, Other)
- `metadata`: Snapshot (size, timestamps, inode/file-id, permissions, MIME type)
- `errors`: Collection of errors collected during discovery

**EntryError** provides structured error handling:
- `path`: Path where error occurred
- `operation`: EntryOp (ReadDir, Metadata, Canonicalize, Open, Move, Copy, Delete, etc.)
- `source`: Underlying I/O error
- `severity`: Warning, Error, Fatal
- `outcome`: Skipped, Aborted, Retried(count)

### 3.3 Scanner (Traversal)

**Location**: `src/run/scanner/`

The scanner walks roots and yields `FsEntry` items with support for:

**Features**:
- Recursion toggle (respects `recursive` config)
- Ignore hidden files toggle
- Symlink policy (skip/follow/record)
- Exclude short-circuit (do not descend into excluded dirs)
- Robust error handling (permission denied, broken links) without crashing
- Efficient traversal with early pruning

**Performance**:
- Early pruning of excluded directories
- Reuse buffers where possible
- Parallel traversal (optional, future enhancement)

### 3.4 Policy Engine

**Location**: `src/run/policies/`

The policy engine reads config rules and produces an **EffectivePolicy**:

**ConfigPolicy**:
- Extracts include/exclude paths from configuration
- Normalizes paths (handles relative/absolute, resolves components)
- Builds include roots (user-specified root directories)
- Builds exclude roots (quarantine, state_dir, rule destinations)

**EffectivePolicy**:
- Computes effective include/exclude sets by:
  - Removing redundant ancestor paths
  - Sorting by depth
  - Resolving conflicts (excludes take precedence)
- Provides `should_process()` method for path filtering
- Supports short-circuit traversal (skip excluded subtrees)

### 3.5 Matcher

**Location**: `src/run/matcher/`

The matcher computes for each entry:

**Matching Process**:
1. Ignore decision (global ignores: extensions, globs, hidden, directories)
2. Rule match (first match by priority; deterministic)
3. Action output (operation details + destination + rename)

**Match Explanation**:
Every match result includes:
- Matched rule name + priority
- Exact conditions that matched (extension X, glob Y, path Z)
- Destination computed
- Any overrides applied

This powers the `explain` command.

**Deterministic Matching**:
- Rules evaluated in priority order (stable tie-breakers)
- Extensions matched case-insensitively
- Globs applied to filename and optionally relative path from root
- Path includes/excludes short-circuit traversal

### 3.6 Planner

**Location**: `src/run/planner/`

The planner converts matches into an ordered plan:

**Responsibilities**:
- Identify operations (source → destination)
- Detect collisions (two files → same destination)
- Detect cycles (rare, but possible if moving within roots)
- Detect duplicates (same file referenced twice due to symlink policy)
- Resolve conflicts according to strategy
- Group operations (safe ordering: creates before moves, etc.)
- Mark no-ops and skips with reasons

**Plan Structure**:
- Ordered steps with dependencies
- Explicit skip reasons
- Conflict resolution decisions
- Deterministic output (same filesystem state → same plan)

### 3.7 Executor

**Location**: `src/run/executor/`

The executor applies plan with transactional-ish safety:

**Safety Guarantees**:
- Never partially corrupt: if a move fails, record failure and continue safely
- Prefer atomic operations: atomic rename/move inside same filesystem
- Cross-device moves: copy then verify then delete (only if allowed)
- Never delete original unless:
  - Copy verification succeeded
  - Mode permits deletes
- Always log:
  - Before applying: write plan + intent
  - After applying: write results + undo mapping

**Operations**:
- Atomic move where possible (same filesystem)
- Fallback copy+delete only when safe
- Quarantine support for conflicts and errors
- Transaction-like behavior (best effort)

### 3.8 State & Audit

**Location**: `src/run/state/`

Stores run history and enables undo:

**Run Identification**:
Each apply produces a **run id** and stores:
- Timestamp
- Config hash (used for run)
- Roots scanned
- Statistics (scanned, matched, moved, copied, skipped, errors)
- Plan file
- Operations performed
- Undo info (source/destination pairs, timestamps)
- Errors and warnings

**Storage Structure**:
```
~/.neatd/state/
├── runs/
│   ├── run-{timestamp}-{id}/
│   │   ├── plan.json
│   │   ├── audit.json
│   │   ├── undo.json
│   │   └── report.json
│   └── ...
├── current-run -> run-{timestamp}-{id}
└── index.json
```

### 3.9 Reporter

**Location**: `src/run/reporter/`

Generates human and machine-readable output:

**Console Output** (human):
- Summary header: scanned, matched, planned, applied
- Top warnings (permissions, invalid config fields, skipped dirs)
- Conflicts section
- "Next steps" hints (e.g., run validate, run explain)

**JSON Report** (machine):
- List of operations with:
  - Source, destination
  - Rule name
  - Action type
  - Outcome (success/failure/skipped)
  - Error details if any

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
Check paths exist and are readable (optional)
    ↓
Report diagnostics (missing fields, invalid globs, bad paths)
```

### 4.2 Scan Flow

```
User runs 'scan' command
    ↓
Load and parse config.toml
    ↓
Build ConfigPolicy (include/exclude roots)
    ↓
Build EffectivePolicy (resolve path conflicts)
    ↓
Initialize Scanner
    ↓
For each root directory:
    ├─ Scanner.traverse()
    │   ├─ Check if directory should_process() (policy)
    │   ├─ If excluded → skip (short-circuit)
    │   ├─ For each entry:
    │   │   ├─ Create FsEntry
    │   │   ├─ Collect metadata
    │   │   ├─ Check ignore patterns
    │   │   └─ Yield entry
    │   └─ Handle errors (collect, continue)
    └─ Build inventory
    ↓
Output scan summary (counts, sizes, errors)
    ↓
Optionally save scan results to state
```

### 4.3 Plan Flow

```
User runs 'plan' command
    ↓
Load config and scan (or use cached scan)
    ↓
Build EffectivePolicy
    ↓
For each entry in inventory:
    ├─ Matcher.match(entry)
    │   ├─ Check ignore patterns
    │   ├─ Evaluate rules by priority
    │   ├─ Find first match
    │   └─ Generate match explanation
    ├─ Build operation (source → destination)
    └─ Add to plan
    ↓
Planner.process_plan()
    ├─ Detect collisions
    ├─ Detect cycles
    ├─ Resolve conflicts (according to strategy)
    ├─ Order operations (safe dependencies)
    └─ Mark skips with reasons
    ↓
Generate plan output (JSON + console)
```

### 4.4 Apply Flow

```
User runs 'apply' command
    ↓
Load plan (from file or generate live)
    ↓
Generate run ID
    ↓
Write plan to state (audit log start)
    ↓
Executor.execute(plan)
    For each operation:
    ├─ Pre-operation validation
    ├─ Execute operation:
    │   ├─ Move: atomic rename (or copy+delete if cross-device)
    │   ├─ Copy: copy with verification
    │   ├─ Quarantine: move to quarantine with metadata
    │   └─ Skip: record reason
    ├─ Handle errors (quarantine if needed)
    └─ Record result
    ↓
Write audit log (operations, results, undo map)
    ↓
Generate report (console + JSON)
    ↓
Update state index
```

### 4.5 Undo Flow

```
User runs 'undo' command [--run-id ID]
    ↓
Load run record (last run or specified ID)
    ↓
Load undo map (source/destination pairs)
    ↓
Planner.build_undo_plan(undo_map)
    ├─ Reverse operations
    ├─ Detect conflicts (if restore path exists)
    └─ Apply conflict strategy for undo
    ↓
Execute undo plan (same safety as apply)
    ↓
Report undo results
```

### 4.6 Explain Flow

```
User runs 'explain <path>' command
    ↓
Load config
    ↓
Resolve path (absolute, normalized)
    ↓
Create FsEntry for path
    ↓
Check ignore patterns
    ├─ If ignored → explain why (extension/glob/hidden)
    └─ If not ignored → continue
    ↓
Match against rules
    ├─ Evaluate rules by priority
    ├─ Find first match
    └─ Generate explanation:
        ├─ Rule name and priority
        ├─ Exact match conditions
        ├─ Destination computed
        └─ Any overrides applied
    ↓
Display explanation (formatted output)
```

## 5. User-Facing Commands

### 5.1 Core Commands

1. **`init`**
   - Creates default config and example rules
   - Generates folders (state dir, quarantine dir) if desired
   - Options: `--path`, `--force`

2. **`validate`**
   - Validates config schema and prints actionable diagnostics
   - Checks: missing fields, invalid globs, bad paths
   - Optionally validates that roots exist and are readable
   - Options: `--path`

3. **`scan`**
   - Traverses roots and builds an inventory (entries) respecting ignore rules
   - Outputs scan summary (counts, sizes, errors)
   - Can optionally save scan results into state for reuse
   - Options: `--save-state`, `--json`

4. **`plan`**
   - Produces a deterministic "plan" (list of operations) without executing
   - Shows rule matches, destinations, rename decisions, conflicts, and reasons
   - Supports filters (only show moves, only conflicts, only certain roots/rules)
   - Options: `--filter`, `--json`, `--output`

5. **`apply`**
   - Executes a plan (from `plan` output or generated live)
   - Writes an audit log and state snapshots to enable **undo**
   - Options: `--plan-file`, `--dry-run`, `--run-id`

6. **`undo`**
   - Reverts the last apply (or a selected run id)
   - Restores files to original paths, best-effort with conflict handling
   - Options: `--run-id`, `--dry-run`

7. **`status`**
   - Shows last run summary, state location, last errors, and current config in effect
   - Options: `--json`

8. **`report`**
   - Outputs a human-friendly report (text/JSON) including:
     - Operations performed
     - Skipped items and why
     - Conflicts and resolutions
     - Errors with remediation hints
   - Options: `--run-id`, `--format`, `--json`

### 5.2 Quality-of-Life Commands

9. **`explain <path>`**
   - Explains how a specific file would be classified:
     - Whether it's ignored and why
     - Which rule matched (or why none matched)
     - What action would occur and target path
     - Exact match conditions satisfied
   - Options: `--json`

10. **`watch`**
    - Watches roots for changes and applies rules continuously
    - Runs in safe mode by default (plan-only or limited actions unless explicitly enabled)
    - Options: `--mode`, `--interval`

## 6. Configuration Schema

See [CONFIG.md](./CONFIG.md) for comprehensive configuration documentation.

Key configuration sections:
- Global settings (roots, recursive, mode, state_dir, quarantine_dir)
- Ignore block (extensions, globs, hidden, directories)
- Rules block (name, enabled, priority, match, action)
- Conflict strategies (rename, keep_newest, quarantine, skip, overwrite)
- Layout settings (date_source, date_format)
- Naming strategies (normalize, slugify, preserve)
- Safety settings (dry_run_default, allow_delete, require_within_roots)

## 7. Safety Features

See [SAFETY.md](./SAFETY.md) for comprehensive safety documentation.

Key safety features:
1. **Dry-Run Mode**: Test configuration without making changes (default)
2. **Quarantine Directory**: Safe location for files that can't be processed
3. **Delete Protection**: Explicit flag required for delete operations
4. **Path Validation**: All operations restricted to configured root directories
5. **Exclusion Policy**: Prevents processing of destination directories
6. **Conflict Resolution**: Multiple strategies with safe defaults
7. **Audit Logging**: Complete record of all operations
8. **Undo Capability**: Revert operations with conflict handling
9. **Configuration Validation**: Syntax and semantic validation before execution
10. **Transaction-like Behavior**: Best-effort atomic operations

## 8. Performance Requirements

**Scalability Targets**:
- Handle 100k+ file scans efficiently
- Efficient traversal with early pruning (excluded dirs)
- O(1) match operations where possible (extensions as hash set, compiled glob matchers)
- Avoid repeated allocations (reuse buffers)

**Optimization Strategies**:
- Early pruning of excluded directories
- Compiled glob matchers (compile once, reuse)
- Hash sets for extension matching
- Parallel traversal (optional, future enhancement)
- Parallel matching (optional, future enhancement)
- Sequential execution (required for safety)

## 9. Cross-Platform Considerations

**Path Handling**:
- Use `PathBuf` for all paths (handles separators correctly)
- Path normalization (lexical: resolve `.` and `..`)
- Windows case-insensitivity considerations
- Unicode filename preservation

**Platform-Specific**:
- Windows: Case-insensitive paths, drive letters, UNC paths, junction points
- Unix: Case-sensitive paths, symlinks, permissions
- macOS: Case-insensitive by default (but can be case-sensitive)

**Symlink Policy**:
- Configurable: skip/follow/record
- Default: skip (safe)

**Permissions**:
- Robust error collection
- Never crash the run on permission errors
- Report permission issues clearly

## 10. Testing Requirements

### Unit Tests
- Extension matching (case-insensitive)
- Glob matching (file vs relative path)
- Include/exclude normalization logic
- Conflict resolution behavior
- Naming normalization behavior
- Rule priority evaluation

### Integration Tests
- Scan output correctness on temp directory fixture
- Plan determinism across runs
- Apply correctness: moved/copied files exist as expected
- Undo correctness: restores originals
- Conflict resolution scenarios
- Quarantine behavior

### Regression Tests
- Historical bugs (should_descend logic, exclude/include conflicts)
- Edge cases (symlinks, permissions, unicode filenames)

## 11. Logging and Observability

**Structured Logging**:
- Console output (human-readable)
- Run logs persisted to state directory
- Debug mode prints:
  - Traversal decisions
  - Match evaluation trace (optional, behind flag)

**Error Messages**:
- Clear error messages with remediation hints
- Context: path, operation, suggested fix
- Never crash silently

## 12. Module Structure

```
src/
├── main.rs                    # Entry point, command routing
├── args.rs                    # CLI argument definitions
├── config_file_data.rs        # Default configuration template
│
├── directory/                 # Directory utilities
│   └── mod.rs                # Home directory, path management
│
├── init/                      # Configuration initialization
│   └── mod.rs                # Create config file
│
├── parse/                     # Configuration parsing
│   └── mod.rs                # TOML parsing and validation
│
└── run/                       # Execution engine
    ├── mod.rs                # Run module exports
    │
    ├── config/               # Configuration structures
    │   ├── mod.rs
    │   ├── config.rs         # Config structs (Config, Rule, Action, etc.)
    │   └── display.rs        # Colored config display
    │
    ├── entries/              # File system entries
    │   ├── mod.rs
    │   └── fs_entry.rs       # FsEntry, EntryError structures
    │
    ├── scanner/              # Directory traversal
    │   ├── mod.rs
    │   └── traversal.rs      # Scanner implementation
    │
    ├── policies/             # Path policy system
    │   ├── mod.rs
    │   ├── config_policy.rs  # Extract include/exclude from config
    │   └── effective_policy.rs # Compute effective policy
    │
    ├── matcher/              # Rule matching
    │   ├── mod.rs
    │   ├── rule_matcher.rs   # Rule matching logic
    │   └── explanation.rs    # Match explanation generation
    │
    ├── planner/              # Plan generation
    │   ├── mod.rs
    │   ├── plan.rs           # Plan structures
    │   ├── collision.rs      # Collision detection
    │   └── resolver.rs       # Conflict resolution
    │
    ├── executor/             # Plan execution
    │   ├── mod.rs
    │   ├── execute.rs        # Execution logic
    │   └── operations.rs     # File operations (move, copy, etc.)
    │
    ├── state/                # State and audit
    │   ├── mod.rs
    │   ├── run.rs            # Run record management
    │   ├── audit.rs          # Audit log
    │   └── undo.rs           # Undo map management
    │
    └── reporter/             # Reporting
        ├── mod.rs
        ├── console.rs        # Console output
        └── json.rs           # JSON report generation
```

## 13. Future Considerations

See [ROADMAP.md](./ROADMAP.md) for detailed milestones and roadmap.

**Potential Enhancements**:
1. File watching: Real-time file system monitoring (inotify/fsevents)
2. Advanced matching: Regex patterns, file content inspection, MIME types
3. Action hooks: Pre/post-action scripts
4. Rule composition: Multiple match criteria (AND/OR logic)
5. Scheduling: Cron-like scheduling for periodic runs
6. Multi-threading: Parallel file processing (scan, match)
7. Progress reporting: Real-time progress for large operations
8. Configuration migration: Version management for config schema
9. Cloud storage: First-class support for cloud storage
10. Content-based classification: OCR, ML, video scanning

## 14. Dependencies

**Core Dependencies**:
- `clap`: CLI argument parsing with derive macros
- `toml`: TOML configuration parsing
- `serde`: Serialization/deserialization
- `chrono`: Date/time handling
- `dirs`: Platform-specific directory resolution
- `os_info`: Operating system detection
- `colored`: Terminal color output

**Future Dependencies** (potential):
- `notify`: File system watching (watch mode)
- `glob`: Advanced glob pattern matching
- `walkdir`: Enhanced directory traversal (if needed)
- `uuid`: Run ID generation

## 15. Definition of Done

A feature is considered done only if:
- It has clear CLI behavior and help text
- It logs meaningful output
- It includes tests (unit or integration)
- It is documented (at least one doc section updated)
- It is safe by default (no destructive surprises)

## 16. Implementation Progress

**Current Status**: Early Development (~15% Complete)

### Milestone Status
- **Milestone 0** (CLI + Config + Validation): ~40% Complete
- **Milestone 1** (Scanner + Entries): ~20% Complete
- **Milestone 2** (Policy Engine + Matching): ~15% Complete
- **Milestone 3-7**: Not Started

### Key Implemented Features
- ✅ `init` command (creates default config)
- ✅ `print-config` command (displays config)
- ✅ Basic configuration parsing (TOML)
- ✅ Policy system structures (ConfigPolicy, EffectivePolicy)
- ✅ FsEntry structure (basic implementation)
- ✅ Basic scanner foundation

### Key Missing Features
- ❌ Complete scanner implementation
- ❌ Rule matching (complete)
- ❌ Plan generation
- ❌ Execution engine
- ❌ State management and audit logging
- ❌ Undo functionality
- ❌ Reporting system
- ❌ Most CLI commands (scan, plan, apply, undo, explain, etc.)

For detailed progress tracking, see [PROGRESS.md](./PROGRESS.md) and [ROADMAP.md](./ROADMAP.md).

---

**Document Version**: 2.0  
**Last Updated**: 2025-12-16  
**Application Version**: 0.1.0
