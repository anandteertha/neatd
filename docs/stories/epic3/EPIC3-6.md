# Story 3.6

**Epic**: 3  
**File**: EPIC3-6

---

### Story 3.6: Conflict Resolution

**ID**: EPIC3-6  
**Title**: Implement Conflict Resolution Strategies  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC3-3

**Description**:
Implement all conflict resolution strategies: rename, keep_newest, keep_oldest, quarantine, skip, overwrite.

**Tasks**:
- [ ] Implement rename strategy (incrementing suffix)
- [ ] Implement keep_newest strategy (compare timestamps)
- [ ] Implement keep_oldest strategy (compare timestamps)
- [ ] Implement quarantine strategy (move to quarantine)
- [ ] Implement skip strategy (mark as skipped)
- [ ] Implement overwrite strategy (requires aggressive mode)
- [ ] Apply strategy to collisions
- [ ] Update operations with resolved destinations
- [ ] Test all conflict strategies
- [ ] Test strategy combinations
- [ ] Test with various file scenarios

**Acceptance Criteria**:
- All conflict strategies work correctly
- Strategies respect config settings
- Overwrite requires aggressive mode
- Resolved operations are correct

---
