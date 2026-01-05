# neatd - User Stories for 100% Completion

This document contains detailed user stories for completing the neatd project from current state (~15% complete) to 100% completion.

**Last Updated**: 2025-12-16  
**Current Progress**: ~15% Complete  
**Target**: 100% Complete (All Milestones)

---

## Story Organization

Stories are organized by:
- **Epic**: Major feature area (Milestone)
- **Story**: Individual feature/requirement
- **Tasks**: Specific implementation tasks
- **Acceptance Criteria**: Definition of done

**Story Format**:
- **ID**: Unique identifier (EPIC-STORY)
- **Title**: Brief description
- **Priority**: Critical / High / Medium / Low
- **Status**: Not Started / In Progress / Complete
- **Dependencies**: Other stories that must be completed first
- **Description**: Detailed requirements
- **Tasks**: Implementation checklist
- **Acceptance Criteria**: How to verify completion

---

## Epic 0: CLI + Config + Validation (Foundation)

**Current Status**: ~40% Complete  
**Target**: 100% Complete

### Story 0.1: Enhanced Configuration Validation

**ID**: EPIC0-1  
**Title**: Comprehensive Configuration Validation with Field-Level Diagnostics  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: None

**Description**:
Implement comprehensive configuration validation that checks all fields, provides actionable error messages, and validates paths and patterns.

**Tasks**:
- [ ] Create validation module (`src/validate/`)
- [ ] Implement field presence validation (required fields)
- [ ] Implement field type validation (string, integer, boolean, array, etc.)
- [ ] Implement enum value validation (ExecutionMode, ActionType, LogType, etc.)
- [ ] Implement path validation:
  - [ ] Check if paths exist (optional flag)
  - [ ] Check if paths are readable/writable
  - [ ] Check if paths are directories vs files
  - [ ] Validate path format (absolute vs relative)
- [ ] Implement glob pattern validation:
  - [ ] Check glob syntax correctness
  - [ ] Warn about potentially problematic patterns
- [ ] Implement rule validation:
  - [ ] Check rule priorities are unique (warn on duplicates)
  - [ ] Validate rule match criteria (at least one required)
  - [ ] Validate rule actions (destination must be set)
  - [ ] Check for conflicting rules
- [ ] Implement date format validation (strftime format)
- [ ] Generate actionable error messages with:
  - [ ] Field name and location
  - [ ] Error type and description
  - [ ] Suggested fix
- [ ] Update `validate` command to use new validation module
- [ ] Add `--strict` flag for additional checks
- [ ] Add `--check-paths` flag for path existence validation

**Acceptance Criteria**:
- Invalid config fails with precise field-level diagnostics
- Error messages include field name, error type, and suggested fix
- Valid config prints a clear summary of effective settings
- Path validation works correctly (when enabled)
- Glob pattern validation catches syntax errors
- Rule validation catches priority conflicts and missing fields

---

### Story 0.2: Enhanced Configuration Schema

**ID**: EPIC0-2  
**Title**: Complete Configuration Schema with All Features  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: None

**Description**:
Extend configuration schema to support all planned features including conflict strategies, metadata constraints, and advanced options.

**Tasks**:
- [ ] Add conflict strategy enum to Action:
  - [ ] `rename` (default)
  - [ ] `keep_newest`
  - [ ] `keep_oldest`
  - [ ] `quarantine`
  - [ ] `skip`
  - [ ] `overwrite` (requires aggressive mode)
- [ ] Add metadata constraints to Match:
  - [ ] `size_min` (integer, bytes)
  - [ ] `size_max` (integer, bytes)
  - [ ] `age_days_min` (integer)
  - [ ] `age_days_max` (integer)
- [ ] Add path prefix matching to Match:
  - [ ] `path_prefixes` (array of strings)
- [ ] Add naming strategy options:
  - [ ] `slugify` (boolean)
  - [ ] `preserve_case` (boolean)
- [ ] Add ignore directories list:
  - [ ] `directories` (array of strings) in Ignore block
- [ ] Add safety mode enum:
  - [ ] `safe` (default)
  - [ ] `aggressive`
- [ ] Update default config template with new fields
- [ ] Update config display to show new fields
- [ ] Update validation to check new fields

**Acceptance Criteria**:
- All new configuration fields are supported
- Default config includes examples of new features
- Config display shows all new fields
- Validation validates new fields correctly
- Backward compatibility maintained (old configs still work)

---

### Story 0.3: Configuration Documentation Validation

**ID**: EPIC0-3  
**Title**: Ensure Configuration Documentation Matches Implementation  
**Priority**: Medium  
**Status**: Not Started  
**Dependencies**: EPIC0-2

**Description**:
Verify that CONFIG.md documentation accurately reflects the implemented configuration schema.

**Tasks**:
- [ ] Compare CONFIG.md with actual Config struct
- [ ] Update CONFIG.md with any missing fields
- [ ] Add examples for all new configuration options
- [ ] Verify all examples in CONFIG.md are valid
- [ ] Add migration guide for config changes
- [ ] Test all documented examples

**Acceptance Criteria**:
- CONFIG.md is 100% accurate
- All configuration options are documented
- All examples are valid and tested
- Migration guide exists for breaking changes

---

## Epic 1: Scanner + Entries (Inventory)

**Current Status**: ~20% Complete  
**Target**: 100% Complete

### Story 1.1: Fix Scanner Bugs

**ID**: EPIC1-1  
**Title**: Fix Critical Bugs in Scanner Implementation  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: None

**Description**:
Fix existing bugs in scanner implementation including wrong variable usage, recursion issues, and error handling problems.

**Tasks**:
- [ ] Fix variable usage in `recurse_dirs` (using `path` instead of `entry_path`)
- [ ] Fix recursion logic (should recurse into `entry_path`, not `path`)
- [ ] Fix FsEntry creation (should create new entry for each file, not reuse)
- [ ] Fix error handling (don't use `expect`, use proper error propagation)
- [ ] Fix Log return values (should return proper status, not exit early)
- [ ] Add proper error collection (collect errors, continue processing)
- [ ] Test scanner with various directory structures
- [ ] Test scanner with permission errors
- [ ] Test scanner with symlinks

**Acceptance Criteria**:
- Scanner correctly traverses all directories
- Scanner handles errors gracefully (collects and continues)
- Scanner correctly identifies file types
- Scanner doesn't crash on permission errors
- Scanner correctly handles symlinks
- All tests pass

---

### Story 1.2: Complete Metadata Collection

**ID**: EPIC1-2  
**Title**: Implement Complete File Metadata Collection  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC1-1

**Description**:
Populate EntryMetaData with all available file metadata including size, timestamps, permissions, and MIME types.

**Tasks**:
- [ ] Implement metadata collection function
- [ ] Collect file size (bytes)
- [ ] Collect modification time
- [ ] Collect creation time (platform-specific)
- [ ] Collect access time
- [ ] Collect read-only status
- [ ] Collect MIME type (optional, using file extension or magic bytes)
- [ ] Collect canonical path (resolve symlinks)
- [ ] Handle metadata errors gracefully (collect as EntryError)
- [ ] Cache metadata to avoid repeated filesystem calls
- [ ] Update FsEntry creation to populate metadata
- [ ] Test metadata collection on all platforms
- [ ] Test metadata collection with various file types

**Acceptance Criteria**:
- All metadata fields are populated when available
- Metadata errors are collected as EntryError, not crashes
- Metadata collection works on Windows, macOS, and Linux
- Performance is acceptable (no excessive filesystem calls)

---

### Story 1.3: Implement Ignore Patterns

**ID**: EPIC1-3  
**Title**: Complete Ignore Pattern Implementation  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC1-1

**Description**:
Implement all ignore pattern types: extensions, globs, hidden files, and directories.

**Tasks**:
- [ ] Create ignore filter module (`src/run/matcher/ignore_filter.rs`)
- [ ] Implement extension ignore (case-insensitive hash set lookup)
- [ ] Implement glob ignore (compile globs, match against filename and path)
- [ ] Implement hidden file ignore (check if filename starts with `.`)
- [ ] Implement directory ignore (exact name match, case-sensitive)
- [ ] Combine ignore checks (file must pass all ignore filters)
- [ ] Add ignore explanation (why file was ignored)
- [ ] Integrate ignore filter into scanner
- [ ] Test ignore patterns with various files
- [ ] Test ignore patterns with edge cases (unicode, special chars)
- [ ] Performance test (ignore patterns should be fast)

**Acceptance Criteria**:
- All ignore patterns work correctly
- Ignore patterns are applied before rule matching
- Ignore explanation is available for `explain` command
- Performance is acceptable (O(1) for extensions, compiled globs)
- Edge cases handled correctly

---

### Story 1.4: Implement Symlink Policy

**ID**: EPIC1-4  
**Title**: Symlink Handling with Configurable Policy  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC1-1

**Description**:
Implement configurable symlink policy (skip, follow, or record) with proper handling.

**Tasks**:
- [ ] Add symlink policy to config:
  - [ ] `symlink_policy` enum: `skip`, `follow`, `record`
- [ ] Implement symlink detection
- [ ] Implement skip policy (don't process symlinks)
- [ ] Implement follow policy (resolve symlinks, process target)
- [ ] Implement record policy (process symlink itself, record target)
- [ ] Handle broken symlinks gracefully
- [ ] Prevent symlink loops (detect cycles)
- [ ] Update scanner to respect symlink policy
- [ ] Test all symlink policies
- [ ] Test with broken symlinks
- [ ] Test with symlink cycles

**Acceptance Criteria**:
- All symlink policies work correctly
- Broken symlinks are handled gracefully
- Symlink cycles are detected and prevented
- Policy is configurable and respected

---

### Story 1.5: Implement Recursion Control

**ID**: EPIC1-5  
**Title**: Configurable Recursive Directory Traversal  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC1-1

**Description**:
Implement proper recursion control based on config setting, with depth limiting and exclude short-circuit.

**Tasks**:
- [ ] Respect `recursive` config setting
- [ ] Implement depth limiting (optional, for safety)
- [ ] Implement exclude short-circuit (don't descend into excluded dirs)
- [ ] Track recursion depth
- [ ] Add recursion depth to FsEntry (optional)
- [ ] Test with recursive enabled/disabled
- [ ] Test with deep directory structures
- [ ] Test exclude short-circuit performance

**Acceptance Criteria**:
- Recursion respects config setting
- Exclude short-circuit works correctly
- Performance is good (doesn't traverse excluded dirs)
- Deep structures handled correctly

---

### Story 1.6: Implement Scan Command

**ID**: EPIC1-6  
**Title**: Complete Scan Command Implementation  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC1-1, EPIC1-2, EPIC1-3

**Description**:
Implement the `scan` command that traverses roots, builds inventory, and outputs summary.

**Tasks**:
- [ ] Add `scan` command to CLI args
- [ ] Implement scan command handler
- [ ] Build inventory (collection of FsEntry)
- [ ] Generate scan summary:
  - [ ] Total files scanned
  - [ ] Total directories scanned
  - [ ] Total size (bytes, human-readable)
  - [ ] Files by type (extensions)
  - [ ] Errors encountered
  - [ ] Ignored files count
- [ ] Implement `--save-state` flag (save scan results)
- [ ] Implement `--json` flag (JSON output)
- [ ] Format console output (human-readable)
- [ ] Add progress indicator (optional, for large scans)
- [ ] Test scan command with various directories
- [ ] Test scan command with large directories (10k+ files)
- [ ] Test scan state persistence

**Acceptance Criteria**:
- Scan command works correctly
- Scan summary is accurate and informative
- Scan state can be saved and loaded
- JSON output is valid and parseable
- Performance is acceptable (handles 100k+ files)

---

### Story 1.7: Robust Error Handling

**ID**: EPIC1-7  
**Title**: Comprehensive Error Handling in Scanner  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC1-1

**Description**:
Implement robust error handling that collects errors, continues processing, and provides clear error messages.

**Tasks**:
- [ ] Replace all `expect()` calls with proper error handling
- [ ] Collect permission errors (don't crash)
- [ ] Collect broken symlink errors
- [ ] Collect I/O errors
- [ ] Continue processing after errors
- [ ] Categorize errors by severity
- [ ] Include error context (path, operation, reason)
- [ ] Report errors in scan summary
- [ ] Test error handling with various error conditions
- [ ] Test error handling doesn't slow down processing

**Acceptance Criteria**:
- Scanner never crashes on errors
- All errors are collected and reported
- Error messages are clear and actionable
- Processing continues after errors
- Error reporting doesn't significantly impact performance

---

## Epic 2: Policy Engine + Matching

**Current Status**: ~15% Complete  
**Target**: 100% Complete

### Story 2.1: Complete Rule Matching Implementation

**ID**: EPIC2-1  
**Title**: Finish Rule Matching with Priority-Based Evaluation  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC1-3

**Description**:
Complete the rule matching implementation with priority-based evaluation, first-match semantics, and all matching criteria.

**Tasks**:
- [ ] Fix `apply_rules_to_file` function
- [ ] Implement priority-based rule sorting
- [ ] Implement first-match semantics (stop after first match)
- [ ] Complete extension matching (case-insensitive, hash set)
- [ ] Implement glob matching (compile globs, match against filename/path)
- [ ] Implement path prefix matching
- [ ] Implement metadata constraint matching:
  - [ ] Size range matching
  - [ ] Age range matching
- [ ] Implement catch-all matching (`any = true`)
- [ ] Handle disabled rules (skip)
- [ ] Generate match result with:
  - [ ] Matched rule name and priority
  - [ ] Match conditions satisfied
  - [ ] Action to perform
  - [ ] Destination path
- [ ] Test rule matching with various files
- [ ] Test rule priority ordering
- [ ] Test first-match semantics
- [ ] Performance test (matching should be fast)

**Acceptance Criteria**:
- Rule matching works correctly for all criteria types
- Priority-based evaluation is correct
- First-match semantics is deterministic
- Performance is acceptable (O(1) extensions, compiled globs)
- All edge cases handled

---

### Story 2.2: Compiled Glob Matchers

**ID**: EPIC2-2  
**Title**: Implement Compiled Glob Pattern Matching  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC2-1

**Description**:
Implement efficient glob pattern matching using compiled matchers that are created once and reused.

**Tasks**:
- [ ] Add glob matching dependency (or implement simple glob matching)
- [ ] Create glob compiler (compile patterns once)
- [ ] Cache compiled globs (reuse across files)
- [ ] Match against filename
- [ ] Match against relative path from root (optional)
- [ ] Handle special glob patterns (`**`, `*`, `?`)
- [ ] Test glob matching with various patterns
- [ ] Performance test (compiled globs should be fast)
- [ ] Test edge cases (unicode, special chars)

**Acceptance Criteria**:
- Glob patterns are compiled once and reused
- Glob matching is efficient
- All glob patterns work correctly
- Edge cases handled correctly

---

### Story 2.3: Match Explanation Generation

**ID**: EPIC2-3  
**Title**: Generate Detailed Match Explanations  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC2-1

**Description**:
Generate detailed explanations for why files match or don't match rules, including all conditions satisfied.

**Tasks**:
- [ ] Create match explanation structure
- [ ] Generate explanation for ignored files:
  - [ ] Which ignore pattern matched
  - [ ] Why it was ignored
- [ ] Generate explanation for matched rules:
  - [ ] Rule name and priority
  - [ ] Exact conditions that matched (extension, glob, path, metadata)
  - [ ] Destination computed
  - [ ] Any overrides applied
- [ ] Generate explanation for unmatched files:
  - [ ] Why no rule matched
  - [ ] Which rules were evaluated
  - [ ] Why each rule didn't match
- [ ] Format explanation for display
- [ ] Test explanation generation
- [ ] Test explanation accuracy

**Acceptance Criteria**:
- Match explanations are accurate and detailed
- Explanations are human-readable
- Explanations include all relevant information
- Explanations help users understand rule behavior

---

### Story 2.4: Implement Explain Command

**ID**: EPIC2-4  
**Title**: Explain Command for File Classification  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC2-3

**Description**:
Implement the `explain` command that shows how a specific file would be classified.

**Tasks**:
- [ ] Add `explain <path>` command to CLI args
- [ ] Implement explain command handler
- [ ] Resolve and normalize path
- [ ] Create FsEntry for path
- [ ] Check ignore patterns
- [ ] Match against rules
- [ ] Generate explanation
- [ ] Display formatted explanation
- [ ] Add `--json` flag for JSON output
- [ ] Test explain command with various files
- [ ] Test explain command with edge cases

**Acceptance Criteria**:
- Explain command works correctly
- Explanation is clear and helpful
- JSON output is valid
- Edge cases handled correctly

---

### Story 2.5: Enhanced Policy System

**ID**: EPIC2-5  
**Title**: Enhance Policy System with Advanced Features  
**Priority**: Medium  
**Status**: Not Started  
**Dependencies**: None

**Description**:
Enhance the policy system to support path prefixes, metadata constraints, and more sophisticated include/exclude logic.

**Tasks**:
- [ ] Add path prefix matching to policy
- [ ] Add metadata constraints to policy
- [ ] Enhance include/exclude resolution
- [ ] Support complex path patterns
- [ ] Optimize policy evaluation
- [ ] Test enhanced policy system
- [ ] Performance test

**Acceptance Criteria**:
- Enhanced policy features work correctly
- Performance is acceptable
- Backward compatibility maintained

---

## Epic 3: Planner (Dry-Run Plan Output)

**Current Status**: 0% Complete  
**Target**: 100% Complete

### Story 3.1: Plan Structure Definition

**ID**: EPIC3-1  
**Title**: Define Plan Data Structures  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC2-1

**Description**:
Define the data structures for representing a plan of operations.

**Tasks**:
- [ ] Create plan module (`src/run/planner/`)
- [ ] Define Operation struct:
  - [ ] Source path
  - [ ] Destination path
  - [ ] Action type (move, copy, quarantine, skip)
  - [ ] Rule name
  - [ ] Conflict resolution
  - [ ] Skip reason (if skipped)
- [ ] Define Plan struct:
  - [ ] Operations list
  - [ ] Collisions list
  - [ ] Metadata (timestamp, config hash, roots)
- [ ] Define Collision struct:
  - [ ] Destination path
  - [ ] Conflicting operations
- [ ] Implement serialization (JSON)
- [ ] Implement deserialization (JSON)
- [ ] Test plan structures

**Acceptance Criteria**:
- Plan structures are well-defined
- Serialization/deserialization works correctly
- Structures support all required information

---

### Story 3.2: Operation Builder

**ID**: EPIC3-2  
**Title**: Build Operations from Matches  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC3-1, EPIC2-1

**Description**:
Build operations (source → destination) from rule matches, including destination path computation and layout application.

**Tasks**:
- [ ] Create operation builder
- [ ] Convert match result to operation
- [ ] Compute destination path:
  - [ ] Apply rule destination
  - [ ] Apply date layout if enabled
  - [ ] Apply naming strategy if enabled
- [ ] Handle relative vs absolute paths
- [ ] Validate destination paths
- [ ] Build operation with all metadata
- [ ] Test operation building
- [ ] Test destination path computation
- [ ] Test date layout application
- [ ] Test naming strategies

**Acceptance Criteria**:
- Operations are built correctly from matches
- Destination paths are computed correctly
- Date layouts are applied correctly
- Naming strategies work correctly
- Edge cases handled

---

### Story 3.3: Collision Detection

**ID**: EPIC3-3  
**Title**: Detect Destination Collisions  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC3-2

**Description**:
Detect when multiple operations would result in the same destination path.

**Tasks**:
- [ ] Implement collision detection algorithm
- [ ] Group operations by destination
- [ ] Identify collisions (multiple operations → same destination)
- [ ] Create collision records
- [ ] Include all conflicting operations in collision
- [ ] Test collision detection
- [ ] Test with various collision scenarios
- [ ] Performance test (should be efficient)

**Acceptance Criteria**:
- All collisions are detected
- Collision records are accurate
- Performance is acceptable
- Edge cases handled

---

### Story 3.4: Cycle Detection

**ID**: EPIC3-4  
**Title**: Detect Circular Move Operations  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC3-2

**Description**:
Detect cycles in move operations (e.g., moving file A to location B, and file B to location A).

**Tasks**:
- [ ] Implement cycle detection algorithm
- [ ] Build dependency graph of operations
- [ ] Detect cycles in graph
- [ ] Mark operations involved in cycles
- [ ] Report cycles in plan
- [ ] Test cycle detection
- [ ] Test with various cycle scenarios
- [ ] Test with complex cycles

**Acceptance Criteria**:
- All cycles are detected
- Cycle reports are accurate
- Performance is acceptable

---

### Story 3.5: Duplicate Detection

**ID**: EPIC3-5  
**Title**: Detect Duplicate File References  
**Priority**: Medium  
**Status**: Not Started  
**Dependencies**: EPIC3-2

**Description**:
Detect when the same file is referenced multiple times (e.g., due to symlink following).

**Tasks**:
- [ ] Implement duplicate detection
- [ ] Track files by canonical path
- [ ] Identify duplicate references
- [ ] Mark duplicates in plan
- [ ] Test duplicate detection
- [ ] Test with symlinks
- [ ] Test with hard links

**Acceptance Criteria**:
- Duplicates are detected correctly
- Symlink scenarios handled
- Hard link scenarios handled

---

### Story 3.6: Conflict Resolution

**ID**: EPIC3-6  
**Title**: Implement Conflict Resolution Strategies  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC3-3

**Description**:
Implement all conflict resolution strategies: rename, keep_newest, keep_oldest, quarantine, skip, overwrite.

**Tasks**:
- [ ] Implement rename strategy (incrementing suffix)
- [ ] Implement keep_newest strategy (compare timestamps)
- [ ] Implement keep_oldest strategy (compare timestamps)
- [ ] Implement quarantine strategy (move to quarantine)
- [ ] Implement skip strategy (mark as skipped)
- [ ] Implement overwrite strategy (requires aggressive mode)
- [ ] Apply strategy to collisions
- [ ] Update operations with resolved destinations
- [ ] Test all conflict strategies
- [ ] Test strategy combinations
- [ ] Test with various file scenarios

**Acceptance Criteria**:
- All conflict strategies work correctly
- Strategies respect config settings
- Overwrite requires aggressive mode
- Resolved operations are correct

---

### Story 3.7: Operation Ordering

**ID**: EPIC3-7  
**Title**: Order Operations for Safe Execution  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC3-6

**Description**:
Order operations to ensure safe execution (e.g., create directories before moving files into them).

**Tasks**:
- [ ] Implement operation ordering algorithm
- [ ] Identify dependencies (directory creation before moves)
- [ ] Topological sort operations
- [ ] Handle cycles (shouldn't happen after resolution, but handle)
- [ ] Group operations by type
- [ ] Order groups appropriately
- [ ] Test operation ordering
- [ ] Test with complex scenarios
- [ ] Verify ordering is correct

**Acceptance Criteria**:
- Operations are ordered correctly
- Dependencies are respected
- Execution order is safe
- Performance is acceptable

---

### Story 3.8: Skip Reasoning

**ID**: EPIC3-8  
**Title**: Generate Skip Reasons for Operations  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC3-6

**Description**:
Generate clear reasons for why operations are skipped (ignored, no match, conflict, etc.).

**Tasks**:
- [ ] Identify skip reasons:
  - [ ] File ignored
  - [ ] No rule matched
  - [ ] Conflict resolution (skip strategy)
  - [ ] Invalid destination
  - [ ] Permission error
- [ ] Generate skip reason messages
- [ ] Attach skip reasons to operations
- [ ] Include skip reasons in plan output
- [ ] Test skip reasoning
- [ ] Test with various skip scenarios

**Acceptance Criteria**:
- All skip reasons are accurate
- Skip reasons are clear and helpful
- Skip reasons are included in plan output

---

### Story 3.9: Plan Serialization

**ID**: EPIC3-9  
**Title**: Serialize and Deserialize Plans  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC3-1

**Description**:
Implement JSON serialization and deserialization for plans.

**Tasks**:
- [ ] Implement JSON serialization for Plan
- [ ] Implement JSON serialization for Operation
- [ ] Implement JSON serialization for Collision
- [ ] Implement JSON deserialization
- [ ] Include all metadata in serialization
- [ ] Test serialization/deserialization
- [ ] Test with various plans
- [ ] Verify round-trip (serialize → deserialize → same)

**Acceptance Criteria**:
- Plans serialize correctly to JSON
- Plans deserialize correctly from JSON
- Round-trip works correctly
- JSON is valid and parseable

---

### Story 3.10: Plan Console Output

**ID**: EPIC3-10  
**Title**: Human-Readable Plan Console Output  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC3-1

**Description**:
Generate human-readable console output for plans.

**Tasks**:
- [ ] Create plan formatter
- [ ] Format operations list:
  - [ ] Source → destination
  - [ ] Action type
  - [ ] Rule name
  - [ ] Skip reason (if skipped)
- [ ] Format collisions section
- [ ] Format summary statistics
- [ ] Use colors for clarity (optional)
- [ ] Add pagination for large plans (optional)
- [ ] Test console output
- [ ] Test with various plans

**Acceptance Criteria**:
- Console output is clear and readable
- All information is included
- Formatting is consistent
- Large plans are handled gracefully

---

### Story 3.11: Plan Command Implementation

**ID**: EPIC3-11  
**Title**: Implement Plan Command  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC3-1 through EPIC3-10

**Description**:
Implement the `plan` command that generates and displays a plan.

**Tasks**:
- [ ] Add `plan` command to CLI args
- [ ] Implement plan command handler
- [ ] Load config
- [ ] Scan directories (or use cached scan)
- [ ] Match files against rules
- [ ] Build operations
- [ ] Detect collisions
- [ ] Resolve conflicts
- [ ] Order operations
- [ ] Generate plan
- [ ] Display plan (console or JSON)
- [ ] Add `--filter` flag (filter operations)
- [ ] Add `--json` flag (JSON output)
- [ ] Add `--output` flag (save to file)
- [ ] Test plan command
- [ ] Test plan determinism (same filesystem → same plan)

**Acceptance Criteria**:
- Plan command works correctly
- Plans are deterministic
- Plan output is accurate
- Filters work correctly
- JSON output is valid

---

### Story 3.12: Plan Filters

**ID**: EPIC3-12  
**Title**: Implement Plan Filtering Options  
**Priority**: Medium  
**Status**: Not Started  
**Dependencies**: EPIC3-11

**Description**:
Implement filtering options for plan output (only moves, only conflicts, only certain roots/rules).

**Tasks**:
- [ ] Implement filter by action type (move, copy, quarantine, skip)
- [ ] Implement filter by conflict status
- [ ] Implement filter by root directory
- [ ] Implement filter by rule name
- [ ] Implement filter combinations
- [ ] Add filter options to plan command
- [ ] Test filters
- [ ] Test filter combinations

**Acceptance Criteria**:
- All filters work correctly
- Filter combinations work
- Filtered output is accurate

---

## Epic 4: Apply + Audit + Undo

**Current Status**: 0% Complete  
**Target**: 100% Complete

### Story 4.1: Executor Foundation

**ID**: EPIC4-1  
**Title**: Create Executor Module Structure  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC3-11

**Description**:
Create the executor module structure and basic operation execution framework.

**Tasks**:
- [ ] Create executor module (`src/run/executor/`)
- [ ] Define Executor struct
- [ ] Define operation execution interface
- [ ] Implement basic execution loop
- [ ] Implement operation validation
- [ ] Implement error handling framework
- [ ] Test executor structure

**Acceptance Criteria**:
- Executor module structure is in place
- Basic execution framework works
- Error handling is in place

---

### Story 4.2: Atomic Move Operations

**ID**: EPIC4-2  
**Title**: Implement Atomic Move Operations  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC4-1

**Description**:
Implement atomic move operations for same-filesystem moves.

**Tasks**:
- [ ] Implement atomic rename (same filesystem)
- [ ] Detect filesystem boundaries
- [ ] Use atomic rename when possible
- [ ] Verify move success
- [ ] Handle move errors
- [ ] Test atomic moves
- [ ] Test cross-filesystem detection
- [ ] Test error handling

**Acceptance Criteria**:
- Atomic moves work correctly
- Filesystem detection is accurate
- Errors are handled gracefully
- Moves are verified

---

### Story 4.3: Cross-Device Moves

**ID**: EPIC4-3  
**Title**: Implement Cross-Device Move Operations  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC4-2

**Description**:
Implement cross-device moves using copy + verify + delete pattern.

**Tasks**:
- [ ] Detect cross-device moves
- [ ] Implement copy operation
- [ ] Verify copy (checksum or size comparison)
- [ ] Delete original (only if copy verified and delete allowed)
- [ ] Handle copy errors
- [ ] Handle verification failures
- [ ] Rollback on failure
- [ ] Test cross-device moves
- [ ] Test error scenarios
- [ ] Test rollback

**Acceptance Criteria**:
- Cross-device moves work correctly
- Copy verification is reliable
- Deletes only occur when safe
- Rollback works on failure

---

### Story 4.4: Copy Operations

**ID**: EPIC4-4  
**Title**: Implement Copy Operations with Verification  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC4-1

**Description**:
Implement copy operations with verification to ensure data integrity.

**Tasks**:
- [ ] Implement file copy
- [ ] Implement directory copy (recursive)
- [ ] Verify copy (checksum or size comparison)
- [ ] Preserve metadata (timestamps, permissions)
- [ ] Handle copy errors
- [ ] Handle verification failures
- [ ] Test copy operations
- [ ] Test with large files
- [ ] Test with directories
- [ ] Test error handling

**Acceptance Criteria**:
- Copy operations work correctly
- Verification is reliable
- Metadata is preserved
- Errors are handled gracefully

---

### Story 4.5: Quarantine Integration

**ID**: EPIC4-5  
**Title**: Integrate Quarantine for Conflicts and Errors  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC4-1

**Description**:
Integrate quarantine directory for handling conflicts and errors.

**Tasks**:
- [ ] Create quarantine directory structure
- [ ] Implement quarantine file move
- [ ] Preserve original path in quarantine structure
- [ ] Generate quarantine metadata
- [ ] Handle quarantine errors
- [ ] Integrate quarantine into conflict resolution
- [ ] Integrate quarantine into error handling
- [ ] Test quarantine integration
- [ ] Test quarantine structure
- [ ] Test error scenarios

**Acceptance Criteria**:
- Quarantine works correctly
- Quarantine structure is organized
- Original paths are preserved
- Errors are handled gracefully

---

### Story 4.6: Transaction-like Behavior

**ID**: EPIC4-6  
**Title**: Implement Transaction-like Safety Guarantees  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC4-2, EPIC4-3, EPIC4-4

**Description**:
Implement transaction-like behavior to ensure operations are safe and can be rolled back.

**Tasks**:
- [ ] Implement operation pre-validation
- [ ] Implement operation logging (before execution)
- [ ] Implement rollback mechanism
- [ ] Implement checkpoint system
- [ ] Handle partial failures
- [ ] Ensure no partial corruption
- [ ] Test transaction behavior
- [ ] Test rollback scenarios
- [ ] Test partial failure handling

**Acceptance Criteria**:
- Operations are safe
- Rollback works correctly
- No partial corruption occurs
- Partial failures are handled gracefully

---

### Story 4.7: Run ID Generation

**ID**: EPIC4-7  
**Title**: Generate Unique Run IDs  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: None

**Description**:
Generate unique run IDs for each execution.

**Tasks**:
- [ ] Implement run ID generation (UUID or timestamp-based)
- [ ] Ensure uniqueness
- [ ] Format run IDs consistently
- [ ] Include run ID in all logs
- [ ] Test run ID generation
- [ ] Test uniqueness

**Acceptance Criteria**:
- Run IDs are unique
- Run IDs are formatted consistently
- Run IDs are included in all logs

---

### Story 4.8: Audit Log Format

**ID**: EPIC4-8  
**Title**: Define and Implement Audit Log Format  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC4-7

**Description**:
Define and implement the audit log format for recording all operations.

**Tasks**:
- [ ] Define audit log structure:
  - [ ] Run metadata (ID, timestamp, config hash, roots)
  - [ ] Operations list (source, destination, action, rule, outcome)
  - [ ] Errors and warnings
  - [ ] Statistics
- [ ] Implement audit log writer
- [ ] Implement audit log reader
- [ ] Serialize to JSON
- [ ] Include all required information
- [ ] Test audit log format
- [ ] Test serialization/deserialization

**Acceptance Criteria**:
- Audit log format is well-defined
- All operations are logged
- Audit logs are readable and parseable
- Serialization works correctly

---

### Story 4.9: State Storage Structure

**ID**: EPIC4-9  
**Title**: Implement State Directory Structure  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC4-7

**Description**:
Implement the state directory structure for storing runs, audit logs, and undo maps.

**Tasks**:
- [ ] Create state directory structure:
  - [ ] `runs/run-{timestamp}-{id}/` directories
  - [ ] `plan.json`, `audit.json`, `undo.json`, `report.json` files
  - [ ] `current-run` symlink
  - [ ] `index.json` (run history)
- [ ] Implement state directory creation
- [ ] Implement run directory creation
- [ ] Implement file writing
- [ ] Implement file reading
- [ ] Test state structure
- [ ] Test file operations
- [ ] Test cross-platform compatibility

**Acceptance Criteria**:
- State structure is correct
- Files are written and read correctly
- Cross-platform compatibility works
- Structure is organized and maintainable

---

### Story 4.10: Undo Map Generation

**ID**: EPIC4-10  
**Title**: Generate Undo Maps for Operations  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC4-8

**Description**:
Generate undo maps that record source/destination pairs for all operations.

**Tasks**:
- [ ] Define undo map structure
- [ ] Generate undo map during execution:
  - [ ] Record source → destination pairs
  - [ ] Record timestamps
  - [ ] Record metadata (optional)
- [ ] Save undo map to state
- [ ] Include undo map in audit log
- [ ] Test undo map generation
- [ ] Test undo map accuracy

**Acceptance Criteria**:
- Undo maps are generated correctly
- All operations are recorded
- Undo maps are saved correctly
- Undo maps are accurate

---

### Story 4.11: Run Record Management

**ID**: EPIC4-11  
**Title**: Manage Run Records and History  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC4-9

**Description**:
Implement run record management including creation, storage, and retrieval.

**Tasks**:
- [ ] Define run record structure
- [ ] Create run records
- [ ] Store run records
- [ ] Update run history index
- [ ] Retrieve run records
- [ ] List run history
- [ ] Test run record management
- [ ] Test run history

**Acceptance Criteria**:
- Run records are managed correctly
- Run history is accurate
- Run records are retrievable

---

### Story 4.12: Apply Command Implementation

**ID**: EPIC4-12  
**Title**: Implement Apply Command  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC4-1 through EPIC4-11

**Description**:
Implement the `apply` command that executes plans safely.

**Tasks**:
- [ ] Add `apply` command to CLI args
- [ ] Implement apply command handler
- [ ] Load plan (from file or generate live)
- [ ] Generate run ID
- [ ] Create run record
- [ ] Write plan to state
- [ ] Execute plan (using executor)
- [ ] Write audit log
- [ ] Generate undo map
- [ ] Finalize run record
- [ ] Generate report
- [ ] Add `--plan-file` flag (load from file)
- [ ] Add `--dry-run` flag (simulate)
- [ ] Add `--run-id` flag (custom run ID)
- [ ] Test apply command
- [ ] Test with various plans
- [ ] Test error scenarios

**Acceptance Criteria**:
- Apply command works correctly
- Plans are executed safely
- Audit logs are created
- Undo maps are generated
- Reports are generated

---

### Story 4.13: Undo Command Implementation

**ID**: EPIC4-13  
**Title**: Implement Undo Command  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC4-12

**Description**:
Implement the `undo` command that reverts previous operations.

**Tasks**:
- [ ] Add `undo` command to CLI args
- [ ] Implement undo command handler
- [ ] Load run record (last or specified ID)
- [ ] Load undo map
- [ ] Build undo plan (reverse operations)
- [ ] Detect conflicts (if restore path exists)
- [ ] Apply conflict strategy for undo
- [ ] Execute undo plan
- [ ] Generate undo report
- [ ] Add `--run-id` flag (specify run to undo)
- [ ] Add `--dry-run` flag (simulate)
- [ ] Test undo command
- [ ] Test with various scenarios
- [ ] Test conflict handling

**Acceptance Criteria**:
- Undo command works correctly
- Files are restored correctly
- Conflicts are handled
- Undo reports are generated

---

### Story 4.14: Undo Conflict Handling

**ID**: EPIC4-14  
**Title**: Handle Conflicts During Undo Operations  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC4-13

**Description**:
Handle conflicts when undoing operations (e.g., restore path already exists).

**Tasks**:
- [ ] Detect conflicts during undo
- [ ] Apply conflict strategy:
  - [ ] Rename restored file
  - [ ] Quarantine existing file
  - [ ] Skip undo operation
- [ ] Report conflicts
- [ ] Test conflict handling
- [ ] Test with various conflict scenarios

**Acceptance Criteria**:
- Conflicts are detected
- Conflict strategies work correctly
- Conflicts are reported clearly

---

## Epic 5: Reporting and Status

**Current Status**: 0% Complete  
**Target**: 100% Complete

### Story 5.1: Console Reporter

**ID**: EPIC5-1  
**Title**: Implement Human-Readable Console Reporting  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC4-12

**Description**:
Implement human-readable console reporting for operations, errors, and statistics.

**Tasks**:
- [ ] Create console reporter module (`src/run/reporter/console.rs`)
- [ ] Implement summary header (scanned, matched, planned, applied)
- [ ] Implement operations list formatting
- [ ] Implement errors and warnings section
- [ ] Implement conflicts section
- [ ] Implement statistics section
- [ ] Add "Next steps" hints
- [ ] Use colors for clarity (optional)
- [ ] Format output nicely
- [ ] Test console reporting
- [ ] Test with various scenarios

**Acceptance Criteria**:
- Console reports are clear and readable
- All information is included
- Formatting is consistent
- Reports are helpful

---

### Story 5.2: JSON Reporter

**ID**: EPIC5-2  
**Title**: Implement Machine-Readable JSON Reporting  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC4-12

**Description**:
Implement JSON reporting for machine consumption and integration.

**Tasks**:
- [ ] Create JSON reporter module (`src/run/reporter/json.rs`)
- [ ] Define JSON report structure
- [ ] Implement JSON serialization
- [ ] Include all operations
- [ ] Include errors and warnings
- [ ] Include statistics
- [ ] Include conflicts
- [ ] Test JSON reporting
- [ ] Test JSON validity
- [ ] Test with various scenarios

**Acceptance Criteria**:
- JSON reports are valid
- All information is included
- JSON is parseable
- Reports are complete

---

### Story 5.3: Summary Statistics

**ID**: EPIC5-3  
**Title**: Generate Summary Statistics for Reports  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC4-12

**Description**:
Generate comprehensive summary statistics for reports.

**Tasks**:
- [ ] Calculate statistics:
  - [ ] Files scanned
  - [ ] Files matched
  - [ ] Files moved
  - [ ] Files copied
  - [ ] Files quarantined
  - [ ] Files skipped
  - [ ] Errors encountered
  - [ ] Conflicts resolved
- [ ] Format statistics
- [ ] Include in reports
- [ ] Test statistics calculation
- [ ] Test statistics accuracy

**Acceptance Criteria**:
- Statistics are accurate
- Statistics are comprehensive
- Statistics are formatted clearly

---

### Story 5.4: Status Command Implementation

**ID**: EPIC5-4  
**Title**: Implement Status Command  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC4-11, EPIC5-1

**Description**:
Implement the `status` command that shows current state and last run information.

**Tasks**:
- [ ] Add `status` command to CLI args
- [ ] Implement status command handler
- [ ] Load last run record
- [ ] Display last run summary:
  - [ ] Run ID, timestamp
  - [ ] Mode (dry-run/run)
  - [ ] Statistics
  - [ ] Errors
- [ ] Display state location
- [ ] Display current config summary
- [ ] Add `--json` flag (JSON output)
- [ ] Test status command
- [ ] Test with various states

**Acceptance Criteria**:
- Status command works correctly
- All information is displayed
- JSON output is valid

---

### Story 5.5: Report Command Implementation

**ID**: EPIC5-5  
**Title**: Implement Report Command  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC5-1, EPIC5-2

**Description**:
Implement the `report` command that generates reports for previous runs.

**Tasks**:
- [ ] Add `report` command to CLI args
- [ ] Implement report command handler
- [ ] Load run record (last or specified ID)
- [ ] Load audit log
- [ ] Generate report (console or JSON)
- [ ] Include operations, errors, conflicts
- [ ] Add `--run-id` flag (specify run)
- [ ] Add `--format` flag (text/json)
- [ ] Add `--json` flag (JSON output)
- [ ] Test report command
- [ ] Test with various runs

**Acceptance Criteria**:
- Report command works correctly
- Reports are accurate and complete
- JSON output is valid

---

### Story 5.6: Error Message Improvements

**ID**: EPIC5-6  
**Title**: Improve Error Messages with Remediation Hints  
**Priority**: Medium  
**Status**: Not Started  
**Dependencies**: EPIC5-1

**Description**:
Improve error messages to include clear descriptions and remediation hints.

**Tasks**:
- [ ] Review all error messages
- [ ] Add context to error messages
- [ ] Add remediation hints
- [ ] Format error messages clearly
- [ ] Test error messages
- [ ] Ensure errors are actionable

**Acceptance Criteria**:
- Error messages are clear
- Remediation hints are helpful
- Errors are actionable

---

## Epic 6: Hardening and Polish

**Current Status**: 0% Complete  
**Target**: 100% Complete

### Story 6.1: Performance Profiling

**ID**: EPIC6-1  
**Title**: Profile and Optimize Performance  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC1-6, EPIC3-11, EPIC4-12

**Description**:
Profile the application and optimize performance bottlenecks.

**Tasks**:
- [ ] Profile scanner performance
- [ ] Profile matcher performance
- [ ] Profile planner performance
- [ ] Profile executor performance
- [ ] Identify bottlenecks
- [ ] Optimize hot paths
- [ ] Test performance improvements
- [ ] Verify 100k+ file handling

**Acceptance Criteria**:
- Performance meets requirements (100k+ files)
- Bottlenecks are identified and fixed
- Performance improvements are measurable

---

### Story 6.2: Unit Test Coverage

**ID**: EPIC6-2  
**Title**: Achieve 80%+ Unit Test Coverage  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: All previous epics

**Description**:
Write comprehensive unit tests for all core functionality.

**Tasks**:
- [ ] Write tests for configuration parsing
- [ ] Write tests for policy system
- [ ] Write tests for scanner
- [ ] Write tests for matcher
- [ ] Write tests for planner
- [ ] Write tests for executor
- [ ] Write tests for state management
- [ ] Write tests for reporter
- [ ] Achieve 80%+ coverage
- [ ] Run tests in CI

**Acceptance Criteria**:
- 80%+ test coverage achieved
- All critical paths are tested
- Tests run in CI
- Tests are maintainable

---

### Story 6.3: Integration Test Suite

**ID**: EPIC6-3  
**Title**: Create Comprehensive Integration Test Suite  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: All previous epics

**Description**:
Create integration tests that test end-to-end workflows.

**Tasks**:
- [ ] Create test fixtures (temp directories)
- [ ] Test scan → plan → apply workflow
- [ ] Test undo workflow
- [ ] Test conflict resolution
- [ ] Test error handling
- [ ] Test with various file types
- [ ] Test with large directories
- [ ] Test cross-platform compatibility
- [ ] Run tests in CI

**Acceptance Criteria**:
- Integration tests cover all workflows
- Tests are reliable
- Tests run in CI
- Tests catch regressions

---

### Story 6.4: Regression Test Suite

**ID**: EPIC6-4  
**Title**: Create Regression Test Suite  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC6-2, EPIC6-3

**Description**:
Create regression tests for known bugs and edge cases.

**Tasks**:
- [ ] Document known bugs
- [ ] Create tests for each bug
- [ ] Test edge cases:
  - [ ] Unicode filenames
  - [ ] Very long paths
  - [ ] Special characters
  - [ ] Symlinks
  - [ ] Hard links
  - [ ] Permission errors
- [ ] Run regression tests
- [ ] Ensure tests prevent regressions

**Acceptance Criteria**:
- All known bugs have tests
- Edge cases are covered
- Tests prevent regressions

---

### Story 6.5: CI/CD Pipeline

**ID**: EPIC6-5  
**Title**: Set Up CI/CD Pipeline  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC6-2, EPIC6-3

**Description**:
Set up continuous integration and deployment pipeline.

**Tasks**:
- [ ] Set up GitHub Actions (or similar)
- [ ] Configure test runs
- [ ] Configure linting
- [ ] Configure formatting checks
- [ ] Configure cross-platform testing
- [ ] Configure release builds
- [ ] Configure artifact publishing
- [ ] Test CI/CD pipeline

**Acceptance Criteria**:
- CI runs on all PRs
- Tests run automatically
- Linting and formatting are checked
- Cross-platform testing works
- Releases are automated

---

### Story 6.6: Cross-Platform Testing

**ID**: EPIC6-6  
**Title**: Test on All Target Platforms  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC6-3

**Description**:
Test the application on Windows, macOS, and Linux.

**Tasks**:
- [ ] Test on Windows
- [ ] Test on macOS
- [ ] Test on Linux
- [ ] Test path handling on each platform
- [ ] Test symlink handling on each platform
- [ ] Test permissions on each platform
- [ ] Fix platform-specific issues
- [ ] Document platform differences

**Acceptance Criteria**:
- Application works on all platforms
- Platform-specific issues are fixed
- Platform differences are documented

---

### Story 6.7: Release Packaging

**ID**: EPIC6-7  
**Title**: Create Release Packaging System  
**Priority**: Medium  
**Status**: Not Started  
**Dependencies**: EPIC6-5

**Description**:
Create system for packaging and distributing releases.

**Tasks**:
- [ ] Create release build scripts
- [ ] Build binaries for all platforms
- [ ] Create release archives
- [ ] Create installation instructions
- [ ] Create release notes template
- [ ] Test release packages
- [ ] Automate release process

**Acceptance Criteria**:
- Release packages are created
- Binaries work on all platforms
- Installation instructions are clear
- Release process is automated

---

### Story 6.8: Documentation Completion

**ID**: EPIC6-8  
**Title**: Complete All Documentation  
**Priority**: Medium  
**Status**: Not Started  
**Dependencies**: All previous epics

**Description**:
Ensure all documentation is complete and accurate.

**Tasks**:
- [ ] Review all documentation
- [ ] Update documentation with final implementation
- [ ] Add missing documentation
- [ ] Create example configurations
- [ ] Create troubleshooting guide
- [ ] Create FAQ
- [ ] Test all documentation examples
- [ ] Ensure documentation is accurate

**Acceptance Criteria**:
- All documentation is complete
- Documentation is accurate
- Examples work correctly
- Documentation is helpful

---

### Story 6.9: Example Configurations

**ID**: EPIC6-9  
**Title**: Create Example Configuration Files  
**Priority**: Low  
**Status**: Not Started  
**Dependencies**: EPIC0-2

**Description**:
Create example configuration files for common use cases.

**Tasks**:
- [ ] Create basic example
- [ ] Create advanced example
- [ ] Create examples for different file types
- [ ] Create examples for different workflows
- [ ] Document examples
- [ ] Test examples

**Acceptance Criteria**:
- Examples are comprehensive
- Examples are documented
- Examples work correctly

---

## Epic 7: Watch Mode (Optional)

**Current Status**: 0% Complete  
**Target**: 100% Complete (Optional)

### Story 7.1: File System Watching

**ID**: EPIC7-1  
**Title**: Implement File System Watching  
**Priority**: Low  
**Status**: Not Started  
**Dependencies**: EPIC4-12, EPIC6-1

**Description**:
Implement file system watching using notify crate or similar.

**Tasks**:
- [ ] Add file system watching dependency (notify)
- [ ] Implement file system watcher
- [ ] Watch root directories
- [ ] Handle file system events
- [ ] Debounce events
- [ ] Batch events
- [ ] Test file system watching
- [ ] Test performance

**Acceptance Criteria**:
- File system watching works correctly
- Events are handled efficiently
- Performance is acceptable

---

### Story 7.2: Watch Mode Implementation

**ID**: EPIC7-2  
**Title**: Implement Watch Mode with Safe Defaults  
**Priority**: Low  
**Status**: Not Started  
**Dependencies**: EPIC7-1

**Description**:
Implement watch mode that continuously monitors and applies rules.

**Tasks**:
- [ ] Implement watch mode
- [ ] Run in safe mode by default (plan-only)
- [ ] Apply rules on file changes
- [ ] Handle watch errors
- [ ] Add configurable intervals
- [ ] Add watch command
- [ ] Test watch mode
- [ ] Test with various scenarios

**Acceptance Criteria**:
- Watch mode works correctly
- Safe mode prevents accidental changes
- Performance is acceptable

---

### Story 7.3: Watch Mode Documentation

**ID**: EPIC7-3  
**Title**: Document Watch Mode Features  
**Priority**: Low  
**Status**: Not Started  
**Dependencies**: EPIC7-2

**Description**:
Document watch mode features and usage.

**Tasks**:
- [ ] Document watch mode
- [ ] Document configuration options
- [ ] Document safety features
- [ ] Add examples
- [ ] Add troubleshooting

**Acceptance Criteria**:
- Watch mode is documented
- Examples are provided
- Troubleshooting is included

---

## Summary

### Total Stories by Epic

- **Epic 0**: 3 stories
- **Epic 1**: 7 stories
- **Epic 2**: 5 stories
- **Epic 3**: 12 stories
- **Epic 4**: 14 stories
- **Epic 5**: 6 stories
- **Epic 6**: 9 stories
- **Epic 7**: 3 stories (optional)

**Total**: 59 stories (56 required + 3 optional)

### Priority Breakdown

- **Critical**: 25 stories
- **High**: 22 stories
- **Medium**: 9 stories
- **Low**: 3 stories

### Estimated Completion

Based on story complexity and dependencies:
- **Epic 0**: ~2-3 weeks
- **Epic 1**: ~4-6 weeks
- **Epic 2**: ~3-4 weeks
- **Epic 3**: ~6-8 weeks
- **Epic 4**: ~8-10 weeks
- **Epic 5**: ~2-3 weeks
- **Epic 6**: ~4-6 weeks
- **Epic 7**: ~2-3 weeks (optional)

**Total Estimated Time**: ~31-43 weeks (7-10 months) for full completion

---

**Document Version**: 1.0  
**Last Updated**: 2025-12-16

