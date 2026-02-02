# Story 0.1

**Epic**: 0  
**File**: EPIC0-1

## 📚 Learning Resources

**New to Rust?** Check out these implementation guides:
1. **[Quick Start Guide](./EPIC0-1-QUICK-START.md)** ⭐ **START HERE!** - Get started in 5 steps
2. **[Full Implementation Guide](./EPIC0-1-IMPLEMENTATION-GUIDE.md)** - Complete guide with all concepts and examples
3. **[Code Template](./EPIC0-1-CODE-TEMPLATE.rs)** - Copy-paste starter code

**Recommended Order**:
1. Read Quick Start Guide (15 minutes)
2. Study Rust basics from Quick Start (30-60 minutes)
3. Copy Code Template and adapt it
4. Refer to Full Guide when you need more details

---

### Story 0.1: Enhanced Configuration Validation

**ID**: EPIC0-1  
**Title**: Comprehensive Configuration Validation with Field-Level Diagnostics  
**Priority**: Critical  
**Status**: In Progress  
**Dependencies**: None

**Description**:
Implement comprehensive configuration validation that checks all fields, provides actionable error messages, and validates paths and patterns.

**Tasks**:
- [x] Create validation module (`src/validate/`)
- [ ] Implement field presence validation (required fields)
- [ ] Implement field type validation (string, integer, boolean, array, etc.)
- [ ] Implement enum value validation (ExecutionMode, ActionType, LogType, etc.)
- [ ] Implement path validation:
  - [x] Check if paths exist (optional flag)
  - [ ] Check if paths are readable/writable
  - [x] Check if paths are directories vs files
  - [ ] Validate path format (absolute vs relative)
- [ ] Implement glob pattern validation:
  - [ ] Check glob syntax correctness
  - [ ] Warn about potentially problematic patterns
- [ ] Implement rule validation:
  - [x] Check rule priorities are unique (warn on duplicates)
  - [x] Validate rule match criteria (at least one required)
  - [x] Validate rule actions (destination must be set)
  - [ ] Check for conflicting rules
- [ ] Implement date format validation (strftime format)
- [ ] Generate actionable error messages with:
  - [x] Field name and location
  - [x] Error type and description
  - [ ] Suggested fix
- [x] Update `validate` command to use new validation module
- [ ] Add `--strict` flag for additional checks
- [x] Add `--check-paths` flag for path existence validation

**Acceptance Criteria**:
- [x] Invalid config fails with precise field-level diagnostics
- [x] Error messages include field name, error type, and suggested fix (partial: field name and error type implemented, suggested fix pending)
- [ ] Valid config prints a clear summary of effective settings
- [x] Path validation works correctly (when enabled)
- [ ] Glob pattern validation catches syntax errors
- [x] Rule validation catches priority conflicts and missing fields

---
