# Story 0.2

**Epic**: 0  
**File**: EPIC0-2

---

### Story 0.2: Enhanced Configuration Schema

**ID**: EPIC0-2  
**Title**: Complete Configuration Schema with All Features  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: None

**Description**:
Extend configuration schema to support all planned features including conflict strategies, metadata constraints, and advanced options.

**Tasks**:
- [ ] Add conflict strategy enum to Action:
  - [ ] `rename` (default)
  - [ ] `keep_newest`
  - [ ] `keep_oldest`
  - [ ] `quarantine`
  - [ ] `skip`
  - [ ] `overwrite` (requires aggressive mode)
- [ ] Add metadata constraints to Match:
  - [ ] `size_min` (integer, bytes)
  - [ ] `size_max` (integer, bytes)
  - [ ] `age_days_min` (integer)
  - [ ] `age_days_max` (integer)
- [ ] Add path prefix matching to Match:
  - [ ] `path_prefixes` (array of strings)
- [ ] Add naming strategy options:
  - [ ] `slugify` (boolean)
  - [ ] `preserve_case` (boolean)
- [ ] Add ignore directories list:
  - [ ] `directories` (array of strings) in Ignore block
- [ ] Add safety mode enum:
  - [ ] `safe` (default)
  - [ ] `aggressive`
- [ ] Update default config template with new fields
- [ ] Update config display to show new fields
- [ ] Update validation to check new fields

**Acceptance Criteria**:
- All new configuration fields are supported
- Default config includes examples of new features
- Config display shows all new fields
- Validation validates new fields correctly
- Backward compatibility maintained (old configs still work)

---
