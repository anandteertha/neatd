# Story 1.5

**Epic**: 1  
**File**: EPIC1-5

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
