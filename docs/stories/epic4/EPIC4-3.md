# Story 4.3

**Epic**: 4  
**File**: EPIC4-3

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
