# neatd - Development Roadmap

This document outlines the development milestones for neatd, ordered by priority and dependencies.

## Milestone 0: CLI + Config + Validation (Foundation)

**Status**: Partially Complete  
**Priority**: Critical

### Goals
- Complete CLI interface with all commands
- Full configuration schema implementation
- Comprehensive validation with actionable diagnostics
- Polished error messages

### Tasks
- [x] `init` command (create default config)
- [x] Basic config schema (TOML structures)
- [ ] Enhanced config schema (conflict strategies, metadata constraints)
- [ ] `validate` command with field-level diagnostics
- [ ] Path validation (existence, readability)
- [ ] Glob pattern validation
- [ ] Configuration schema documentation (CONFIG.md)

### Acceptance Criteria
- Invalid config fails with precise field-level diagnostics
- Valid config prints a clear summary of effective settings
- All config options documented and validated

---

## Milestone 1: Scanner + Entries (Inventory)

**Status**: Not Started  
**Priority**: Critical  
**Depends on**: Milestone 0

### Goals
- Complete file system scanning with robust error handling
- FsEntry representation with metadata
- Ignore rule implementation
- Scan command with inventory output

### Tasks
- [ ] FsEntry implementation (path, kind, metadata, errors)
- [ ] EntryError structured error handling
- [ ] Scanner/traversal implementation
- [ ] Recursion toggle support
- [ ] Ignore hidden files
- [ ] Ignore directories list
- [ ] Ignore extensions (case-insensitive)
- [ ] Ignore globs
- [ ] Symlink policy (skip/follow/record)
- [ ] Exclude short-circuit (do not descend into excluded dirs)
- [ ] Permission error handling (collect, continue)
- [ ] `scan` command implementation
- [ ] Scan summary output (counts, sizes, errors)
- [ ] Optional scan state persistence

### Acceptance Criteria
- Scanning a directory with permission errors completes and reports skipped items
- Ignore rules measurably reduce scanned set
- Scan output is deterministic and accurate
- Large directories (10k+ files) scan efficiently

---

## Milestone 2: Policy Engine + Matching

**Status**: Partially Complete  
**Priority**: Critical  
**Depends on**: Milestone 1

### Goals
- Effective policy computation (include/exclude resolution)
- Rule matching with priority-based evaluation
- Match explanation generation
- Explain command

### Tasks
- [x] ConfigPolicy (extract include/exclude from config)
- [x] EffectivePolicy (path resolution)
- [ ] Enhanced policy (path prefixes, metadata constraints)
- [ ] Rule matcher implementation
- [ ] Extension matching (case-insensitive, hash set)
- [ ] Glob matching (compiled matchers)
- [ ] Path pattern matching
- [ ] Metadata constraints (size ranges, age ranges)
- [ ] Priority-based rule evaluation
- [ ] First-match semantics (deterministic)
- [ ] Match explanation generation
- [ ] `explain` command implementation

### Acceptance Criteria
- For any file, `explain` produces:
  - Ignored? Why?
  - Matched rule? Why?
  - Resulting destination/action
- Rule matching is deterministic (same file → same match)
- Performance: O(1) extension matching, efficient glob matching

---

## Milestone 3: Planner (Dry-Run Plan Output)

**Status**: Not Started  
**Priority**: High  
**Depends on**: Milestone 2

### Goals
- Deterministic plan generation
- Conflict detection and resolution
- Plan output (JSON + console)
- Plan command

### Tasks
- [ ] Plan structure definition
- [ ] Operation representation (source → destination)
- [ ] Collision detection (destination conflicts)
- [ ] Cycle detection (move within roots)
- [ ] Duplicate detection (symlink policy)
- [ ] Conflict resolution strategies:
  - [ ] Rename with suffix (incrementing)
  - [ ] Keep newest/keep oldest
  - [ ] Quarantine conflicts
  - [ ] Skip conflicting operation
  - [ ] Overwrite (unsafe, explicit)
- [ ] Operation ordering (safe dependencies)
- [ ] Skip reasoning
- [ ] Plan serialization (JSON)
- [ ] Plan console output (human-readable)
- [ ] `plan` command implementation
- [ ] Plan filters (only moves, only conflicts, etc.)
- [ ] Plan determinism (same filesystem → same plan)

### Acceptance Criteria
- Running `plan` twice produces identical output (for same filesystem state)
- Collisions are detected and reported
- Conflicts are resolved according to config strategy
- Plan is human-readable and machine-parseable

---

## Milestone 4: Apply + Audit + Undo

**Status**: Not Started  
**Priority**: High  
**Depends on**: Milestone 3

### Goals
- Safe plan execution
- Audit logging and state management
- Undo capability
- Apply and undo commands

### Tasks
- [ ] Executor implementation
- [ ] Atomic move operations (same filesystem)
- [ ] Cross-device moves (copy + verify + delete)
- [ ] Copy operations with verification
- [ ] Quarantine integration
- [ ] Transaction-like behavior (best effort)
- [ ] Error handling (never partially corrupt)
- [ ] Run ID generation
- [ ] Audit log format
- [ ] State storage structure
- [ ] Undo map generation
- [ ] Run record management
- [ ] `apply` command implementation
- [ ] `undo` command implementation
- [ ] Undo conflict handling
- [ ] State directory management
- [ ] Run history index

### Acceptance Criteria
- Apply produces a run record and a report
- All operations are logged to audit file
- Undo restores files to original locations (best-effort with explicit conflict handling)
- State is persisted correctly
- Cross-device moves work correctly

---

## Milestone 5: Reporting and Status

**Status**: Not Started  
**Priority**: Medium  
**Depends on**: Milestone 4

### Goals
- Comprehensive reporting (console + JSON)
- Status command
- Report command
- Clear error messages with remediation

### Tasks
- [ ] Console reporter (human-readable)
- [ ] JSON reporter (machine-readable)
- [ ] Summary statistics
- [ ] Operations list
- [ ] Errors and warnings section
- [ ] Conflicts section
- [ ] "Next steps" hints
- [ ] `status` command implementation
- [ ] `report` command implementation
- [ ] Report formats (text, JSON)
- [ ] Error message improvements
- [ ] Remediation hints

### Acceptance Criteria
- Reports are clear and actionable
- JSON reports are parseable and complete
- Status shows relevant information
- Error messages include remediation hints

---

## Milestone 6: Hardening and Polish

**Status**: Not Started  
**Priority**: Medium  
**Depends on**: Milestones 1-5

### Goals
- Performance improvements for large trees
- Extensive test coverage
- CI/CD setup
- Release packaging
- Documentation completion

### Tasks
- [ ] Performance profiling
- [ ] Optimizations (traversal, matching)
- [ ] Unit test coverage (>80%)
- [ ] Integration test suite
- [ ] Regression test suite
- [ ] CI/CD pipeline (GitHub Actions)
- [ ] Cross-platform testing
- [ ] Release packaging (binaries)
- [ ] Documentation completion:
  - [ ] README.md
  - [ ] CONFIG.md
  - [ ] RULES.md
  - [ ] SAFETY.md
  - [ ] ARCHITECTURE.md
  - [ ] CONTRIBUTING.md
  - [ ] CHANGELOG.md
- [ ] Example configurations
- [ ] Migration guides (if needed)

### Acceptance Criteria
- Handles 100k file scans without excessive slowdown
- Test suite covers core logic and prevents regressions
- CI/CD runs on all PRs
- Cross-platform binaries available
- Documentation is complete and accurate

---

## Milestone 7: Watch Mode (Optional)

**Status**: Not Started  
**Priority**: Low  
**Depends on**: Milestone 6

### Goals
- File system watching
- Continuous monitoring
- Watch command

### Tasks
- [ ] File system watching (notify crate)
- [ ] Watch mode implementation
- [ ] Safe mode by default (plan-only)
- [ ] Configurable intervals
- [ ] Watch command
- [ ] Performance considerations (debouncing, batching)
- [ ] Watch mode documentation

### Acceptance Criteria
- Watch mode monitors roots for changes
- Safe mode prevents accidental changes
- Performance is acceptable (no excessive CPU usage)

---

## Future Enhancements (Post-v1)

These features are considered for future versions but not required for v1:

1. **Advanced Matching**
   - Regex patterns
   - File content inspection
   - MIME type detection
   - Advanced metadata constraints

2. **Action Hooks**
   - Pre-action scripts
   - Post-action scripts
   - Conditional actions

3. **Rule Composition**
   - Multiple match criteria (AND/OR logic)
   - Rule dependencies
   - Rule groups

4. **Scheduling**
   - Cron-like scheduling
   - Periodic runs
   - Event-based triggers

5. **Performance Enhancements**
   - Parallel traversal
   - Parallel matching
   - Optimized state storage

6. **Cloud Storage**
   - First-class cloud storage support
   - Sync capabilities
   - Cloud-native features

7. **Content-Based Classification**
   - OCR for images
   - ML-based classification
   - Video scanning
   - Audio metadata extraction

8. **GUI**
   - Desktop application
   - Web interface
   - Configuration editor

---

## Progress Tracking

### Overall Progress

**Current Status**: Early Development (Milestone 0 - Partial)
- **Milestone 0**: ~40% Complete
- **Milestone 1**: ~20% Complete (foundation started)
- **Milestone 2**: ~15% Complete (partial foundation)
- **Milestone 3-7**: Not Started
- **Total Implementation**: ~15% Complete

See [PROGRESS.md](./PROGRESS.md) for detailed progress tracking.

### Completed Milestones
- None (project in early stages)

### In Progress
- **Milestone 0**: CLI + Config + Validation (~40% complete)
  - ✅ `init` command
  - ✅ Basic config schema
  - ✅ `print-config` command
  - 🟡 `validate` command (basic parsing, needs field-level diagnostics)
  - ❌ Enhanced validation features

- **Milestone 1**: Scanner + Entries (~20% complete)
  - ✅ FsEntry structure
  - ✅ Basic scanner foundation
  - 🟡 Scanner implementation (exists but has bugs)
  - ❌ Complete scanner, metadata collection, ignore patterns

- **Milestone 2**: Policy Engine + Matching (~15% complete)
  - ✅ Policy system structures (ConfigPolicy, EffectivePolicy)
  - 🟡 Rule matching foundation (started but incomplete)
  - ❌ Complete matching logic, explain command

### Next Up
1. **Fix Scanner Bugs** (Milestone 1) - Critical blocker
2. **Complete Configuration Validation** (Milestone 0)
3. **Implement Metadata Collection** (Milestone 1)
4. **Implement Ignore Patterns** (Milestone 1)
5. **Complete Rule Matching** (Milestone 2)

---

**Document Version**: 1.0  
**Last Updated**: 2025-12-16

