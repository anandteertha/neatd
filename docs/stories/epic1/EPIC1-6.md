# Story 1.6

**Epic**: 1  
**File**: EPIC1-6

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
