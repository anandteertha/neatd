# neatd - Architecture Overview

## System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Interface                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │   init   │  │ validate │  │  print-  │  │   run    │       │
│  │          │  │          │  │  config  │  │          │       │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘       │
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
│ Configuration  │ │   Execution   │ │   Utilities     │
│   Manager      │ │    Engine     │ │                 │
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
│                │ │  │      Rule Engine            │  │
│                │ │  │  ┌───────────────────────┐  │  │
│                │ │  │  │  Rule Matcher         │  │  │
│                │ │  │  │  (priority-based)     │  │  │
│                │ │  │  └───────────┬───────────┘  │  │
│                │ │  │              │              │  │
│                │ │  │  ┌───────────▼───────────┐  │  │
│                │ │  │  │  Action Executor      │  │  │
│                │ │  │  │  (move/copy/delete)   │  │  │
│                │ │  │  └───────────────────────┘  │  │
│                │ │  └──────────────────────────────┘  │
│                │ │                 │                 │
│                │ │  ┌──────────────▼──────────────┐  │
│                │ │  │   File System Layer         │  │
│                │ │  │  ┌───────────────────────┐  │  │
│                │ │  │  │     FsEntry           │  │  │
│                │ │  │  │  (file metadata)      │  │  │
│                │ │  │  └───────────────────────┘  │  │
│                │ │  │                             │  │
│                │ │  │  ┌───────────────────────┐  │  │
│                │ │  │  │   File Operations     │  │  │
│                │ │  │  │  (scan, move, copy)   │  │  │
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
    ↓
Print success/error
```

### File Organization Execution

```
User: neatd run --once
    ↓
[parse::read_config] → Config
    ↓
[run::policies::config_policy::ConfigPolicy::new]
    Extract include_roots (config.paths.roots)
    Extract exclude_roots (quarantine, state_dir, rule destinations)
    Normalize paths
    ↓
[run::policies::effective_policy::EffectivePolicy::set_effective_policy]
    Resolve path conflicts
    Build effective include/exclude sets
    ↓
For each root directory:
    │
    ├─ [Scan directory recursively]
    │     │
    │     ├─ For each file:
    │     │     │
    │     │     ├─ [EffectivePolicy::should_process]
    │     │     │   Check if path is in include set
    │     │     │   Check if path is in exclude set
    │     │     │   Return true/false
    │     │     │
    │     │     ├─ [Check ignore patterns]
    │     │     │   - globs
    │     │     │   - hidden files
    │     │     │   - extensions
    │     │     │
    │     │     ├─ [Rule Engine]
    │     │     │   Sort rules by priority
    │     │     │   For each rule:
    │     │     │     - Check if enabled
    │     │     │     - Match extensions or catch-all
    │     │     │     - If match: extract action
    │     │     │
    │     │     ├─ [Build destination path]
    │     │     │   - Apply rule destination
    │     │     │   - Apply date layout if enabled
    │     │     │
    │     │     └─ [Execute action]
    │     │         - Move/Copy/Delete
    │     │         - Or simulate in dry-run
    │     │
    │     └─ [Log result]
    │
    └─ [Generate report]
```

## Data Structures

### Configuration Hierarchy

```
Config
├── version: i64
├── created_by: String
├── created_at: String
├── general: General
│   ├── mode: ExecutionMode (dry_run | run)
│   ├── default_action: ActionType (move | copy | delete)
│   └── recursive: bool
├── paths: ConfigPaths
│   ├── roots: Vec<PathBuf>
│   ├── state_dir: PathBuf
│   └── quarantine: PathBuf
├── ignore: Ignore
│   ├── globs: Vec<PathBuf>
│   ├── ignore_hidden: bool
│   └── extensions: Vec<PathBuf>
├── naming: Naming
│   └── normalize_names: bool
├── layout: Layout
│   ├── date_source: String
│   └── date_format: String
├── log: Log
│   └── level: LogType (info | error | success)
├── report: Report
│   └── format: ReportType (text | spreadsheet | analytics)
├── safety: Safety
│   ├── require_within_roots: bool
│   └── allow_delete: bool
└── rules: Vec<Rule>
    ├── name: String
    ├── enabled: bool
    ├── priority: i64
    ├── match: Option<Match>
    │   ├── extensions: Option<Vec<String>>
    │   └── any: Option<bool>
    └── action: Option<Action>
        ├── type: ActionType
        ├── to: PathBuf
        └── use_layout: bool
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

## Rule Matching Algorithm

```
function match_file(file, rules):
    sort rules by priority (ascending)
    
    for rule in rules:
        if not rule.enabled:
            continue
        
        if rule.match is None:
            continue
        
        if rule.match.any == true:
            return rule.action  // Catch-all rule
        
        if file.extension in rule.match.extensions:
            return rule.action  // Extension match
    
    return None  // No match found
```

## Policy Resolution Algorithm

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

## Error Handling Model

```
FsEntry
├── path: PathBuf
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
    │   (ReadDir | Metadata | Canonicalize | Open | Move | Copy | Delete | ParsePath | MatchRule)
    ├── source: Error
    ├── severity: Severity (Warning | Error | Fatal)
    └── outcome: Outcome (Skipped | Aborted | Retried)
```

## State Management (Planned)

```
~/.neatd/
├── config.toml              # User configuration
└── state/
    ├── processed.db         # Processed file registry (future)
    ├── daemon.pid           # Daemon process ID (future)
    └── logs/
        └── neatd.log        # Execution logs (future)
```

## Security Considerations

1. **Path Validation**: All operations restricted to configured root directories
2. **Quarantine**: Files that can't be processed moved to safe location
3. **Delete Protection**: Explicit flag required for delete operations
4. **Exclusion Policy**: Prevents processing of destination directories
5. **Dry-Run Mode**: Safe testing without making changes

---

**Document Version**: 1.0  
**Last Updated**: 2025-12-16

