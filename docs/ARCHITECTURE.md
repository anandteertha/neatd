# neatd - Architecture Overview

## System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Interface                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │   init   │  │ validate │  │  scan    │  │   plan   │       │
│  │          │  │          │  │          │  │          │       │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘       │
│       │             │              │             │             │
│  ┌────▼─────┐  ┌────▼─────┐  ┌────▼─────┐  ┌────▼─────┐     │
│  │  apply   │  │   undo   │  │  status  │  │ explain  │     │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘     │
│       │             │              │             │             │
└───────┼─────────────┼──────────────┼─────────────┼─────────────┘
        │             │              │             │
        └─────────────┴──────────────┴─────────────┘
                           │
                  ┌────────▼─────────┐
                  │   CLI Router     │
                  │   (main.rs)      │
                  └────────┬─────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
┌───────▼────────┐ ┌───────▼───────┐ ┌───────▼─────────┐
│ Configuration  │ │   Execution   │ │   Utilities      │
│   Manager      │ │    Engine     │ │                  │
└───────┬────────┘ └───────┬───────┘ └─────────────────┘
        │                  │
        │                  │
┌───────▼────────┐ ┌───────▼───────────────────────────┐
│                │ │                                   │
│  • Parse       │ │  ┌─────────────────────────────┐  │
│  • Validate    │ │  │      Policy System          │  │
│  • Generate    │ │  │  ┌───────────────────────┐  │  │
│  • Display     │ │  │  │   ConfigPolicy        │  │  │
│                │ │  │  │  (include/exclude)    │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  EffectivePolicy      │  │  │
│                │ │  │  │  (path resolution)    │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  └──────────────┼──────────────┘  │
│                │ │                 │                 │
│                │ │  ┌──────────────▼──────────────┐  │
│                │ │  │      Scanner (Traversal)    │  │
│                │ │  │  ┌───────────────────────┐  │  │
│                │ │  │  │   Directory Walker    │  │  │
│                │ │  │  │   (recursive/toggle)  │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │   FsEntry Creation    │  │  │
│                │ │  │  │   (metadata, errors)  │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  └──────────────┼──────────────┘  │
│                │ │                 │                 │
│                │ │  ┌──────────────▼──────────────┐  │
│                │ │  │      Matcher                │  │
│                │ │  │  ┌───────────────────────┐  │  │
│                │ │  │  │  Ignore Filter        │  │  │
│                │ │  │  │  (extensions, globs)  │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Rule Matcher         │  │  │
│                │ │  │  │  (priority-based)     │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Match Explanation    │  │  │
│                │ │  │  │  (rule, conditions)   │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  └──────────────┼──────────────┘  │
│                │ │                 │                 │
│                │ │  ┌──────────────▼──────────────┐  │
│                │ │  │      Planner                │  │
│                │ │  │  ┌───────────────────────┐  │  │
│                │ │  │  │  Operation Builder    │  │  │
│                │ │  │  │  (source→destination) │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Collision Detection  │  │  │
│                │ │  │  │  (destination conf.)  │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Conflict Resolver    │  │  │
│                │ │  │  │  (rename, keep, etc.) │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Operation Ordering   │  │  │
│                │ │  │  │  (safe dependencies)  │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  └──────────────┼──────────────┘  │
│                │ │                 │                 │
│                │ │  ┌──────────────▼──────────────┐  │
│                │ │  │      Executor               │  │
│                │ │  │  ┌───────────────────────┐  │  │
│                │ │  │  │  Operation Executor   │  │  │
│                │ │  │  │  (move, copy, etc.)   │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Atomic Operations    │  │  │
│                │ │  │  │  (rename, verify)     │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Quarantine Handler   │  │  │
│                │ │  │  │  (conflicts, errors)  │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  └──────────────┼──────────────┘  │
│                │ │                 │                 │
│                │ │  ┌──────────────▼──────────────┐  │
│                │ │  │   State & Audit             │  │
│                │ │  │  ┌───────────────────────┐  │  │
│                │ │  │  │  Run Record Manager   │  │  │
│                │ │  │  │  (run ID, metadata)   │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Audit Log Writer     │  │  │
│                │ │  │  │  (operations, results)│  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Undo Map Builder     │  │  │
│                │ │  │  │  (source→dest pairs)  │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  └──────────────┼──────────────┘  │
│                │ │                 │                 │
│                │ │  ┌──────────────▼──────────────┐  │
│                │ │  │      Reporter               │  │
│                │ │  │  ┌───────────────────────┐  │  │
│                │ │  │  │  Console Reporter     │  │  │
│                │ │  │  │  (human-readable)     │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  JSON Reporter        │  │  │
│                │ │  │  │  (machine-readable)   │  │  │
│                │ │  │  └───────────────────────┘  │  │
│                │ │  └──────────────────────────────┘  │
│                │ └───────────────────────────────────┘
└────────────────┴───────────────────────────────────────┘
```

## Component Interaction Flow

### Configuration Initialization

```
User: neatd init
    ↓
[init::create_or_override_config_file]
    ↓
[directory::create_neatd_directory]
    ↓
Create ~/.neatd/config.toml
    ↓
Write default config (from config_file_data)
    ↓
Success message
```

### Configuration Validation

```
User: neatd validate
    ↓
[parse::read_config]
    ↓
Read TOML file
    ↓
[serde::Deserialize] → Config struct
    ↓
Validate structure
    ├─ Check required fields
    ├─ Validate field types
    ├─ Check paths exist (optional)
    ├─ Validate glob patterns
    └─ Check rule priorities
    ↓
Print diagnostics (errors, warnings)
```

### Scan Flow

```
User: neatd scan
    ↓
[parse::read_config] → Config
    ↓
[policies::config_policy::ConfigPolicy::new]
    Extract include_roots (config.paths.roots)
    Extract exclude_roots (quarantine, state_dir, rule destinations)
    Normalize paths
    ↓
[policies::effective_policy::EffectivePolicy::set_effective_policy]
    Resolve path conflicts
    Build effective include/exclude sets
    ↓
[scanner::traversal::Scanner::new]
    Initialize scanner with policy
    ↓
For each root directory:
    │
    ├─ [scanner::traversal::Scanner::traverse]
    │     │
    │     ├─ Check if directory should_process() (policy)
    │     ├─ If excluded → skip (short-circuit)
    │     │
    │     ├─ For each entry:
    │     │     │
    │     │     ├─ [entries::fs_entry::create_entry]
    │     │     │   Create FsEntry
    │     │     │   Collect metadata (size, timestamps)
    │     │     │   Handle errors (collect, continue)
    │     │     │
    │     │     └─ Yield entry
    │     │
    │     └─ Handle directory errors (permissions, etc.)
    │
    └─ Build inventory (collection of FsEntry)
    ↓
[reporter::console::report_scan]
    Generate scan summary
    ├─ Counts (files, directories, errors)
    ├─ Sizes (total, by type)
    └─ Errors (permission denied, broken links)
    ↓
Optionally save scan results to state
```

### Plan Flow

```
User: neatd plan
    ↓
Load config and scan (or use cached scan)
    ↓
Build EffectivePolicy
    ↓
For each entry in inventory:
    │
    ├─ [matcher::rule_matcher::Matcher::match]
    │     │
    │     ├─ [matcher::ignore_filter::check_ignore]
    │     │   Check ignore patterns:
    │     │   ├─ Extensions (hash set lookup)
    │     │   ├─ Globs (compiled matchers)
    │     │   ├─ Hidden files
    │     │   └─ Directory names
    │     │
    │     ├─ If ignored → skip to next entry
    │     │
    │     ├─ [matcher::rule_matcher::evaluate_rules]
    │     │   Sort rules by priority
    │     │   For each rule (priority order):
    │     │     ├─ Check if enabled
    │     │     ├─ Match extensions (hash set)
    │     │     ├─ Match globs (compiled)
    │     │     ├─ Match path prefixes
    │     │     ├─ Match metadata constraints
    │     │     └─ If match: return rule + action
    │     │
    │     └─ [matcher::explanation::generate_explanation]
    │         Build match explanation:
    │         ├─ Rule name + priority
    │         ├─ Exact match conditions
    │         ├─ Destination computed
    │         └─ Any overrides
    │
    ├─ [planner::plan::build_operation]
    │   Create operation:
    │   ├─ Source path
    │   ├─ Destination path (with layout if enabled)
    │   ├─ Action type (move/copy/quarantine/skip)
    │   └─ Rule name
    │
    └─ Add to plan (operations list)
    ↓
[planner::collision::detect_collisions]
    Find destination conflicts
    ↓
[planner::resolver::resolve_conflicts]
    For each conflict:
    ├─ Apply conflict strategy:
    │   ├─ rename: Add incrementing suffix
    │   ├─ keep_newest/keep_oldest: Compare timestamps
    │   ├─ quarantine: Move to quarantine
    │   ├─ skip: Mark as skip
    │   └─ overwrite: Mark as overwrite (if allowed)
    └─ Update operation destination
    ↓
[planner::plan::order_operations]
    Order operations for safe execution:
    ├─ Creates before moves (directory creation)
    ├─ Dependencies resolved
    └─ Cycles detected and handled
    ↓
[planner::plan::mark_skips]
    Mark operations with skip reasons
    ↓
Generate plan output:
    ├─ JSON format (machine-readable)
    └─ Console format (human-readable)
```

### Apply Flow

```
User: neatd apply [--plan-file path]
    ↓
Load plan (from file or generate live)
    ↓
[state::run::generate_run_id]
    Generate unique run ID
    ↓
[state::run::create_run_record]
    Create run record:
    ├─ Run ID
    ├─ Timestamp
    ├─ Config hash
    ├─ Roots
    └─ Initial stats
    ↓
[state::audit::start_audit_log]
    Write plan to audit log
    ↓
[executor::execute::Executor::execute]
    For each operation in plan:
    │
    ├─ [executor::operations::validate_operation]
    │   Pre-operation validation:
    │   ├─ Source exists
    │   ├─ Destination writable
    │   ├─ Within roots (if required)
    │   └─ Mode allows operation
    │
    ├─ [executor::operations::execute_move]
    │   Execute move:
    │   ├─ Try atomic rename (same filesystem)
    │   ├─ If cross-device:
    │   │   ├─ Copy file
    │   │   ├─ Verify copy
    │   │   └─ Delete original (if allowed)
    │   └─ Record result
    │
    ├─ [executor::operations::execute_copy]
    │   Execute copy:
    │   ├─ Copy file
    │   ├─ Verify copy
    │   └─ Record result
    │
    ├─ [executor::quarantine::quarantine_file]
    │   Quarantine (if needed):
    │   ├─ Move to quarantine directory
    │   ├─ Preserve metadata
    │   └─ Record in audit log
    │
    └─ [state::audit::log_operation]
        Log operation result
        ↓
[state::undo::build_undo_map]
    Build undo map (source/destination pairs)
    ↓
[state::audit::finalize_audit_log]
    Write final audit log:
    ├─ Operations performed
    ├─ Results (success/failure/skipped)
    ├─ Undo map
    └─ Errors and warnings
    ↓
[state::run::finalize_run_record]
    Update run record with final stats
    ↓
[reporter::console::report_apply]
    Generate apply report (console)
    ↓
[reporter::json::report_apply]
    Generate apply report (JSON, optional)
```

### Undo Flow

```
User: neatd undo [--run-id ID]
    ↓
[state::run::load_run_record]
    Load run record (last run or specified ID)
    ↓
[state::undo::load_undo_map]
    Load undo map from audit log
    ↓
[planner::plan::build_undo_plan]
    Build undo plan:
    ├─ Reverse operations (destination → source)
    ├─ Detect conflicts (if restore path exists)
    └─ Apply conflict strategy for undo
    ↓
[executor::execute::Executor::execute]
    Execute undo plan (same safety as apply)
    ↓
[reporter::console::report_undo]
    Report undo results
```

### Explain Flow

```
User: neatd explain <path>
    ↓
Load config
    ↓
[directory::resolve_path]
    Resolve path (absolute, normalized)
    ↓
[entries::fs_entry::create_entry]
    Create FsEntry for path
    ↓
[matcher::ignore_filter::check_ignore]
    Check ignore patterns
    ├─ If ignored → explain why:
    │   ├─ Extension match
    │   ├─ Glob match
    │   ├─ Hidden file
    │   └─ Directory match
    └─ If not ignored → continue
    ↓
[matcher::rule_matcher::Matcher::match]
    Match against rules:
    ├─ Evaluate rules by priority
    ├─ Find first match
    └─ Generate explanation:
        ├─ Rule name and priority
        ├─ Exact match conditions
        ├─ Destination computed
        └─ Any overrides applied
    ↓
[reporter::console::explain_match]
    Display explanation (formatted output)
```

## Data Structures

### Configuration Hierarchy

See [CONFIG.md](./CONFIG.md) for detailed configuration schema.

### Entry Structure

```
FsEntry
├── path: PathBuf (absolute, normalized)
├── kind: FileKind (File | Directory | Symlink | Other)
├── metadata: Option<EntryMetaData>
│   ├── size_bytes: Option<u64>
│   ├── modified: Option<SystemTime>
│   ├── created: Option<SystemTime>
│   ├── accessed: Option<SystemTime>
│   ├── readonly: Option<bool>
│   ├── mime: Option<String>
│   └── canonical_path: Option<PathBuf>
└── errors: Vec<EntryError>
    ├── path: PathBuf
    ├── operation: EntryOp
    ├── source: Error
    ├── severity: Severity (Warning | Error | Fatal)
    └── outcome: Outcome (Skipped | Aborted | Retried)
```

### Policy System Data Flow

```
Config
    ↓
ConfigPolicy
    ├── include_roots: Vec<PathBuf>  (from config.paths.roots)
    └── exclude_roots: Vec<PathBuf>  (quarantine + state_dir + rule destinations)
    ↓
EffectivePolicy
    ├── effective_includes: Vec<PathBuf>
    ├── effective_excludes: Vec<PathBuf>
    ├── effective_includes_hash: HashSet<PathBuf>
    └── effective_excludes_hash: HashSet<PathBuf>
    ↓
should_process(path) → bool
```

### Plan Structure

```
Plan
├── operations: Vec<Operation>
│   ├── source: PathBuf
│   ├── destination: PathBuf
│   ├── action: ActionType (move | copy | quarantine | skip)
│   ├── rule_name: String
│   ├── conflict_resolution: Option<ConflictResolution>
│   └── skip_reason: Option<String>
├── collisions: Vec<Collision>
│   ├── destination: PathBuf
│   └── operations: Vec<Operation>
└── metadata
    ├── generated_at: DateTime
    ├── config_hash: String
    └── roots: Vec<PathBuf>
```

### Run Record Structure

```
RunRecord
├── run_id: String
├── timestamp: DateTime
├── config_hash: String
├── roots: Vec<PathBuf>
├── mode: ExecutionMode (dry_run | run)
├── stats
│   ├── scanned: u64
│   ├── matched: u64
│   ├── moved: u64
│   ├── copied: u64
│   ├── skipped: u64
│   └── errors: u64
└── files
    ├── plan: PathBuf
    ├── audit: PathBuf
    ├── undo: PathBuf
    └── report: PathBuf
```

## State Management

### State Directory Structure

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

### Audit Log Structure

```
AuditLog
├── run_id: String
├── timestamp: DateTime
├── config_hash: String
├── plan: Plan
├── operations: Vec<OperationResult>
│   ├── operation: Operation
│   ├── outcome: Outcome (success | failure | skipped)
│   ├── error: Option<Error>
│   └── timestamp: DateTime
└── undo_map: UndoMap
    └── mappings: Vec<UndoMapping>
        ├── source: PathBuf
        └── destination: PathBuf
```

## Algorithms

### Rule Matching Algorithm

See [RULES.md](./RULES.md) for detailed rule matching logic.

### Policy Resolution Algorithm

```
function resolve_effective_policy(include_roots, exclude_roots):
    // Sort by depth (shallow first)
    sort include_roots by component count
    sort exclude_roots by component count
    
    // Build effective includes (remove redundant ancestors)
    effective_includes = []
    for path in include_roots:
        if no ancestor in effective_includes:
            effective_includes.add(path)
    
    // Filter excludes to only those within includes
    filtered_excludes = []
    for exclude in exclude_roots:
        if any ancestor of exclude in effective_includes:
            filtered_excludes.add(exclude)
    
    // Build effective excludes (remove redundant ancestors)
    sort filtered_excludes by component count
    effective_excludes = []
    for path in filtered_excludes:
        if no ancestor in effective_excludes:
            effective_excludes.add(path)
    
    return (effective_includes, effective_excludes)

function should_process(path, effective_includes, effective_excludes):
    for ancestor in path.ancestors():
        if ancestor in effective_excludes:
            return false  // Excluded
        if ancestor in effective_includes:
            return true   // Included
    
    return false  // Not in any include root
```

### Conflict Resolution Algorithm

```
function resolve_conflicts(operations):
    collisions = detect_collisions(operations)
    
    for collision in collisions:
        strategy = get_conflict_strategy(collision.operations[0])
        
        switch strategy:
            case "rename":
                for op in collision.operations[1:]:
                    op.destination = add_suffix(op.destination, increment)
            case "keep_newest":
                ops = sort_by_timestamp(collision.operations)
                for op in ops[1:]:
                    op.skip_reason = "conflict: keep_newest"
            case "keep_oldest":
                ops = sort_by_timestamp(collision.operations, reverse=true)
                for op in ops[1:]:
                    op.skip_reason = "conflict: keep_oldest"
            case "quarantine":
                for op in collision.operations[1:]:
                    op.action = quarantine
                    op.destination = quarantine_path(op.source)
            case "skip":
                for op in collision.operations[1:]:
                    op.skip_reason = "conflict: skip"
            case "overwrite":
                if mode != "aggressive":
                    error("overwrite requires aggressive mode")
                for op in collision.operations[1:]:
                    op.skip_reason = "conflict: overwrite (will overwrite)"
    
    return operations
```

## Performance Considerations

### Optimization Strategies

1. **Early Pruning**: Excluded directories are not traversed (short-circuit)
2. **Hash Sets**: Extension matching uses hash sets (O(1) lookup)
3. **Compiled Matchers**: Glob patterns are compiled once and reused
4. **Parallel Traversal**: Optional parallel directory traversal (future)
5. **Parallel Matching**: Optional parallel rule matching (future)
6. **Sequential Execution**: Operations execute sequentially for safety

### Scalability Targets

- Handle 100k+ file scans efficiently
- O(1) extension matching
- Efficient glob matching (compiled)
- Minimal memory footprint (streaming where possible)

---

**Document Version**: 2.0  
**Last Updated**: 2025-12-16
