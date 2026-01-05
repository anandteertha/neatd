# Story 2.3

**Epic**: 2  
**File**: EPIC2-3

---

### Story 2.3: Match Explanation Generation

**ID**: EPIC2-3  
**Title**: Generate Detailed Match Explanations  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC2-1

**Description**:
Generate detailed explanations for why files match or don't match rules, including all conditions satisfied.

**Tasks**:
- [ ] Create match explanation structure
- [ ] Generate explanation for ignored files:
  - [ ] Which ignore pattern matched
  - [ ] Why it was ignored
- [ ] Generate explanation for matched rules:
  - [ ] Rule name and priority
  - [ ] Exact conditions that matched (extension, glob, path, metadata)
  - [ ] Destination computed
  - [ ] Any overrides applied
- [ ] Generate explanation for unmatched files:
  - [ ] Why no rule matched
  - [ ] Which rules were evaluated
  - [ ] Why each rule didn't match
- [ ] Format explanation for display
- [ ] Test explanation generation
- [ ] Test explanation accuracy

**Acceptance Criteria**:
- Match explanations are accurate and detailed
- Explanations are human-readable
- Explanations include all relevant information
- Explanations help users understand rule behavior

---
