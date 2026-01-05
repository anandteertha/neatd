# Story 1.4

**Epic**: 1  
**File**: EPIC1-4

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
