# neatd - Safety Features

This document describes the safety features and guarantees provided by neatd to prevent data loss and unexpected behavior.

## Safety Philosophy

neatd is designed with **safety first**: zero-surprise behavior, explicit policies, predictable results, and clear reports before changes. The system prioritizes protecting your data over convenience.

## Core Safety Features

### 1. Dry-Run Mode (Default)

**What it does**: Simulates operations without making any changes to your filesystem.

**Default behavior**: `dry_run_default = true` means dry-run is the default mode.

**Usage**:
```bash
# Dry-run (default)
neatd plan

# Explicit dry-run
neatd apply --dry-run

# Actual execution (requires explicit flag)
neatd apply
```

**Benefits**:
- Test configurations safely
- Preview changes before applying
- No risk of data loss during testing

### 2. Quarantine Directory

**What it does**: Provides a safe location for files that can't be processed normally (conflicts, errors, suspicious files).

**Configuration**:
```toml
[paths]
quarantine = "/Users/you/.neatd/quarantine"
```

**When files are quarantined**:
- Destination conflicts (if conflict strategy is `quarantine`)
- Permission errors
- Invalid filenames
- Cross-device move failures
- Unsafe operations (in safe mode)

**Quarantine structure**:
```
quarantine/
  run-{timestamp}-{id}/
    {original-path-encoded}/
      file.ext
```

**Recovery**:
- Quarantined files are preserved with metadata
- Use `neatd status` to see recent quarantines
- Manually review and restore quarantined files

### 3. Delete Protection

**What it does**: Prevents accidental file deletion.

**Configuration**:
```toml
[safety]
allow_delete = false  # Default: deletions disabled
```

**Behavior**:
- Delete operations are disabled by default
- Must explicitly set `allow_delete = true` to enable
- Even with delete enabled, deletions only occur in specific scenarios (see below)

**When deletions occur** (only if `allow_delete = true`):
- Cross-device moves (copy + verify + delete)
- Explicit delete actions (if supported in future)
- Cleanup operations (if explicitly configured)

**Best practice**: Keep `allow_delete = false` unless you have a specific need and have tested thoroughly.

### 4. Path Validation

**What it does**: Ensures all operations stay within configured root directories.

**Configuration**:
```toml
[safety]
require_within_roots = true  # Default: enabled
```

**Behavior**:
- All file operations must stay within `roots` directories
- Prevents accidental moves outside of monitored directories
- Validates destinations before operations

**Example**:
```toml
[paths]
roots = ["/Users/you/Downloads"]
```

With `require_within_roots = true`, files can only be moved within `/Users/you/Downloads` or its subdirectories.

### 5. Exclusion Policy

**What it does**: Automatically excludes destination directories and system paths from processing.

**Automatic exclusions**:
- Quarantine directory
- State directory
- All rule destination directories
- Prevents infinite loops and circular moves

**Behavior**:
- Excluded directories are not scanned
- Short-circuit traversal (do not descend into excluded dirs)
- Efficient (avoids unnecessary file system access)

### 6. Conflict Resolution

**What it does**: Provides multiple strategies for handling destination conflicts safely.

**Conflict strategies** (from safest to least safe):

1. **`rename`** (default): Add incrementing suffix
   - Safest: Never overwrites
   - Example: `file.jpg` → `file-1.jpg` if conflict exists

2. **`keep_newest`**: Keep file with newer modification time
   - Safe: Preserves newer data
   - Loses older file (but can be recovered from quarantine if configured)

3. **`keep_oldest`**: Keep file with older modification time
   - Safe: Preserves older data
   - Loses newer file (but can be recovered from quarantine if configured)

4. **`quarantine`**: Move conflicting file to quarantine
   - Safe: Preserves all files
   - Requires manual review

5. **`skip`**: Skip operation if conflict exists
   - Safe: No changes made
   - File remains in original location

6. **`overwrite`** (unsafe): Overwrite existing file
   - **Requires `mode = "aggressive"`**
   - **Dangerous**: Permanent data loss possible
   - Use with extreme caution

**Configuration**:
```toml
[rules.action]
conflict_strategy = "rename"  # Default: safe
```

### 7. Safe Mode vs Aggressive Mode

**Safe Mode** (default):
```toml
[safety]
mode = "safe"
```

**Restrictions**:
- No overwrites (conflict strategy `overwrite` is disabled)
- No deletes (unless explicitly enabled and safe)
- Quarantine on conflicts (if strategy allows)
- Strict path validation
- Additional safety checks

**Aggressive Mode**:
```toml
[safety]
mode = "aggressive"
```

**Allowed**:
- Overwrite operations (if conflict strategy is `overwrite`)
- More lenient validation
- Faster execution (fewer safety checks)

**Best practice**: Always start with `mode = "safe"`. Only use aggressive mode after thorough testing and when you understand the risks.

### 8. Audit Logging

**What it does**: Records all operations in an audit log for review and undo.

**Location**: `~/.neatd/state/runs/{run-id}/audit.json`

**Contents**:
- Run metadata (timestamp, config hash, roots)
- All operations (source, destination, rule, action, outcome)
- Errors and warnings
- Undo map (source/destination pairs for undo)

**Benefits**:
- Complete history of changes
- Enables undo functionality
- Debugging and troubleshooting
- Compliance and accountability

### 9. Undo Capability

**What it does**: Allows reverting operations from a previous run.

**Usage**:
```bash
# Undo last run
neatd undo

# Undo specific run
neatd undo --run-id {run-id}
```

**How it works**:
1. Loads undo map from audit log
2. Reverses operations (destination → source)
3. Handles conflicts (if restore path exists)
4. Applies conflict strategy for undo

**Limitations**:
- Best-effort restoration (may not be perfect)
- Conflicts are handled (renamed if needed)
- Metadata restoration is best-effort
- Files moved outside roots cannot be restored

**Best practice**: Test undo functionality with non-critical files first.

### 10. Configuration Validation

**What it does**: Validates configuration before execution to catch errors early.

**Usage**:
```bash
neatd validate
```

**Validates**:
- TOML syntax correctness
- Required fields present
- Field types correct
- Paths exist and are readable (optional)
- Glob patterns valid
- Rule priorities unique (warnings)
- Conflict strategies valid
- Date formats valid

**Benefits**:
- Catch errors before execution
- Clear error messages with remediation hints
- Prevents runtime failures

## Safety Guarantees

### Execution Safety

1. **Never partially corrupt**: If a move fails, record failure and continue safely
2. **Prefer atomic operations**: Atomic rename/move inside same filesystem
3. **Cross-device safety**: Copy then verify then delete (only if allowed)
4. **Never delete original unless**:
   - Copy verification succeeded
   - Mode permits deletes
5. **Always log**: Before and after operations are logged

### Data Protection

1. **No silent failures**: All errors are logged and reported
2. **Quarantine on uncertainty**: When in doubt, quarantine
3. **Preserve originals**: Copy operations preserve originals
4. **Audit trail**: Complete record of all operations

### Operational Safety

1. **Dry-run by default**: Must explicitly enable execution
2. **Validation before execution**: Configuration validated before operations
3. **Clear error messages**: Errors include remediation hints
4. **Explanatory output**: Use `explain` to understand decisions

## Safety Best Practices

### Before First Use

1. **Backup important data**: Even with safety features, backup critical files
2. **Start with dry-run**: Test with `mode = "dry_run"` first
3. **Use safe mode**: Keep `mode = "safe"` and `allow_delete = false`
4. **Validate configuration**: Run `neatd validate` before operations
5. **Test with explain**: Use `neatd explain <path>` to understand rules

### During Use

1. **Review plans**: Always review `neatd plan` output before applying
2. **Start small**: Test with a small directory first
3. **Monitor quarantine**: Check quarantine directory regularly
4. **Review audit logs**: Periodically review audit logs for issues
5. **Keep backups**: Maintain backups of important data

### Troubleshooting

1. **Check audit logs**: Review audit logs for errors
2. **Review quarantine**: Check quarantine directory for problematic files
3. **Use explain**: Use `neatd explain <path>` to understand rule matching
4. **Validate configuration**: Run `neatd validate` to check for issues
5. **Test with dry-run**: Test changes with dry-run before applying

## Recovery Procedures

### Recovering Quarantined Files

1. Check quarantine directory: `~/.neatd/quarantine/`
2. Review quarantine metadata (if available)
3. Manually restore files to desired locations
4. Use `neatd status` to see recent quarantines

### Undoing Operations

1. List recent runs: `neatd status`
2. Identify run to undo: Note the run ID
3. Undo run: `neatd undo --run-id {run-id}`
4. Verify restoration: Check that files are restored
5. Handle conflicts: If conflicts exist, manually resolve

### Recovering from Errors

1. Check audit logs for error details
2. Review quarantine directory
3. Validate configuration: `neatd validate`
4. Fix configuration issues
5. Re-run with corrected configuration

## Safety Checklist

Before running `neatd apply`:

- [ ] Configuration validated (`neatd validate`)
- [ ] Tested with dry-run (`neatd plan`)
- [ ] Reviewed plan output
- [ ] Safe mode enabled (`mode = "safe"`)
- [ ] Delete disabled (`allow_delete = false`)
- [ ] Quarantine directory configured
- [ ] Important data backed up
- [ ] Tested with small directory first
- [ ] Understand conflict strategy
- [ ] Reviewed ignore patterns

---

**Document Version**: 1.0  
**Last Updated**: 2025-12-16

