# Story 3.11

**Epic**: 3  
**File**: EPIC3-11

---

### Story 3.11: Plan Command Implementation

**ID**: EPIC3-11  
**Title**: Implement Plan Command  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC3-1 through EPIC3-10

**Description**:
Implement the `plan` command that generates and displays a plan.

**Tasks**:
- [ ] Add `plan` command to CLI args
- [ ] Implement plan command handler
- [ ] Load config
- [ ] Scan directories (or use cached scan)
- [ ] Match files against rules
- [ ] Build operations
- [ ] Detect collisions
- [ ] Resolve conflicts
- [ ] Order operations
- [ ] Generate plan
- [ ] Display plan (console or JSON)
- [ ] Add `--filter` flag (filter operations)
- [ ] Add `--json` flag (JSON output)
- [ ] Add `--output` flag (save to file)
- [ ] Test plan command
- [ ] Test plan determinism (same filesystem → same plan)

**Acceptance Criteria**:
- Plan command works correctly
- Plans are deterministic
- Plan output is accurate
- Filters work correctly
- JSON output is valid

---
