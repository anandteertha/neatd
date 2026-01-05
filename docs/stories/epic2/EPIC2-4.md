# Story 2.4

**Epic**: 2  
**File**: EPIC2-4

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
