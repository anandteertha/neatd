# Story 2.2

**Epic**: 2  
**File**: EPIC2-2

---

### Story 2.2: Compiled Glob Matchers

**ID**: EPIC2-2  
**Title**: Implement Compiled Glob Pattern Matching  
**Priority**: High  
**Status**: Not Started  
**Dependencies**: EPIC2-1

**Description**:
Implement efficient glob pattern matching using compiled matchers that are created once and reused.

**Tasks**:
- [ ] Add glob matching dependency (or implement simple glob matching)
- [ ] Create glob compiler (compile patterns once)
- [ ] Cache compiled globs (reuse across files)
- [ ] Match against filename
- [ ] Match against relative path from root (optional)
- [ ] Handle special glob patterns (`**`, `*`, `?`)
- [ ] Test glob matching with various patterns
- [ ] Performance test (compiled globs should be fast)
- [ ] Test edge cases (unicode, special chars)

**Acceptance Criteria**:
- Glob patterns are compiled once and reused
- Glob matching is efficient
- All glob patterns work correctly
- Edge cases handled correctly

---
