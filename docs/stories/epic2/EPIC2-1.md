# Story 2.1

**Epic**: 2  
**File**: EPIC2-1

---

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
