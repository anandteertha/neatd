# Story 1.2

**Epic**: 1  
**File**: EPIC1-2

---

### Story 1.2: Complete Metadata Collection

**ID**: EPIC1-2  
**Title**: Implement Complete File Metadata Collection  
**Priority**: Critical  
**Status**: Not Started  
**Dependencies**: EPIC1-1

**Description**:
Populate EntryMetaData with all available file metadata including size, timestamps, permissions, and MIME types.

**Tasks**:
- [ ] Implement metadata collection function
- [ ] Collect file size (bytes)
- [ ] Collect modification time
- [ ] Collect creation time (platform-specific)
- [ ] Collect access time
- [ ] Collect read-only status
- [ ] Collect MIME type (optional, using file extension or magic bytes)
- [ ] Collect canonical path (resolve symlinks)
- [ ] Handle metadata errors gracefully (collect as EntryError)
- [ ] Cache metadata to avoid repeated filesystem calls
- [ ] Update FsEntry creation to populate metadata
- [ ] Test metadata collection on all platforms
- [ ] Test metadata collection with various file types

**Acceptance Criteria**:
- All metadata fields are populated when available
- Metadata errors are collected as EntryError, not crashes
- Metadata collection works on Windows, macOS, and Linux
- Performance is acceptable (no excessive filesystem calls)

---
