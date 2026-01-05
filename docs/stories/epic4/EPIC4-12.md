# Story 4.12

**Epic**: 4  
**File**: EPIC4-12

---

### Story 4.12: Apply Command Implementation

**ID**: EPIC4-12  
**Title**: Implement Apply Command  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC4-1 through EPIC4-11

**Description**:
Implement the `apply` command that executes plans safely.

**Tasks**:
- [ ] Add `apply` command to CLI args
- [ ] Implement apply command handler
- [ ] Load plan (from file or generate live)
- [ ] Generate run ID
- [ ] Create run record
- [ ] Write plan to state
- [ ] Execute plan (using executor)
- [ ] Write audit log
- [ ] Generate undo map
- [ ] Finalize run record
- [ ] Generate report
- [ ] Add `--plan-file` flag (load from file)
- [ ] Add `--dry-run` flag (simulate)
- [ ] Add `--run-id` flag (custom run ID)
- [ ] Test apply command
- [ ] Test with various plans
- [ ] Test error scenarios

**Acceptance Criteria**:
- Apply command works correctly
- Plans are executed safely
- Audit logs are created
- Undo maps are generated
- Reports are generated

---
