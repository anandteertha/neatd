# Story 1.1

**Epic**: 1  
**File**: EPIC1-1

---

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
