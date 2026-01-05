# Story 3.1

**Epic**: 3  
**File**: EPIC3-1

---

### Story 3.1: Plan Structure Definition

**ID**: EPIC3-1  
**Title**: Define Plan Data Structures  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC2-1

**Description**:
Define the data structures for representing a plan of operations.

**Tasks**:
- [ ] Create plan module (`src/run/planner/`)
- [ ] Define Operation struct:
  - [ ] Source path
  - [ ] Destination path
  - [ ] Action type (move, copy, quarantine, skip)
  - [ ] Rule name
  - [ ] Conflict resolution
  - [ ] Skip reason (if skipped)
- [ ] Define Plan struct:
  - [ ] Operations list
  - [ ] Collisions list
  - [ ] Metadata (timestamp, config hash, roots)
- [ ] Define Collision struct:
  - [ ] Destination path
  - [ ] Conflicting operations
- [ ] Implement serialization (JSON)
- [ ] Implement deserialization (JSON)
- [ ] Test plan structures

**Acceptance Criteria**:
- Plan structures are well-defined
- Serialization/deserialization works correctly
- Structures support all required information

---
