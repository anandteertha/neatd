# Story 0.1

**Epic**: 0  
**File**: EPIC0-1

---

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
