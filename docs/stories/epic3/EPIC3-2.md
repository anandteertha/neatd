# Story 3.2

**Epic**: 3  
**File**: EPIC3-2

---

### Story 3.2: Operation Builder

**ID**: EPIC3-2  
**Title**: Build Operations from Matches  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC3-1, EPIC2-1

**Description**:
Build operations (source → destination) from rule matches, including destination path computation and layout application.

**Tasks**:
- [ ] Create operation builder
- [ ] Convert match result to operation
- [ ] Compute destination path:
  - [ ] Apply rule destination
  - [ ] Apply date layout if enabled
  - [ ] Apply naming strategy if enabled
- [ ] Handle relative vs absolute paths
- [ ] Validate destination paths
- [ ] Build operation with all metadata
- [ ] Test operation building
- [ ] Test destination path computation
- [ ] Test date layout application
- [ ] Test naming strategies

**Acceptance Criteria**:
- Operations are built correctly from matches
- Destination paths are computed correctly
- Date layouts are applied correctly
- Naming strategies work correctly
- Edge cases handled

---
