# neatd Documentation

This directory contains comprehensive design, architecture, and usage documentation for the neatd application.

## Documentation Index

### Getting Started

- **[HIGH_LEVEL_DESIGN.md](./HIGH_LEVEL_DESIGN.md)** - Comprehensive high-level design document covering:
  - Overview and product vision
  - System architecture and component layers
  - Core components and their responsibilities
  - Data flow diagrams
  - User-facing commands
  - Configuration schema overview
  - Safety features
  - Performance requirements
  - Cross-platform considerations
  - Testing requirements
  - Module structure
  - Future considerations

  **Recommended for**: New developers, architects, and anyone wanting a complete understanding of the system.

### Architecture and Design

- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Detailed architecture documentation with:
  - System architecture diagrams
  - Component interaction flows
  - Data structure hierarchies
  - Algorithms (rule matching, policy resolution, conflict resolution)
  - Error handling model
  - State management concepts
  - Performance considerations

  **Recommended for**: Developers implementing features, debugging, or understanding system behavior.

- **[DESIGN_DECISIONS.md](./DESIGN_DECISIONS.md)** - Architectural decision records (ADRs) explaining:
  - Why specific technologies were chosen (TOML, priority-based rules, etc.)
  - Design trade-offs and alternatives considered
  - Rationale behind key architectural choices
  - Implementation patterns and their reasoning

  **Recommended for**: Developers making changes, code reviewers, and future maintainers.

### Configuration and Usage

- **[CONFIG.md](./CONFIG.md)** - Complete configuration reference:
  - Configuration file structure and location
  - All configuration sections and fields
  - Configuration examples (minimal, comprehensive)
  - Configuration validation
  - Best practices

  **Recommended for**: Users configuring neatd, developers implementing configuration features.

- **[RULES.md](./RULES.md)** - Rules and matching logic documentation:
  - Rule structure and matching criteria
  - Matching logic (priority-based, deterministic)
  - Rule actions and destinations
  - Conflict resolution strategies
  - Best practices and examples
  - Troubleshooting guide

  **Recommended for**: Users creating rules, developers implementing matching logic.

- **[SAFETY.md](./SAFETY.md)** - Safety features and guarantees:
  - Core safety features (dry-run, quarantine, delete protection, etc.)
  - Safety guarantees and best practices
  - Recovery procedures
  - Safety checklist

  **Recommended for**: All users, especially before first use.

### Development

- **[ROADMAP.md](./ROADMAP.md)** - Development roadmap and milestones:
  - Development milestones (M0-M7)
  - Task breakdowns for each milestone
  - Acceptance criteria
  - Progress tracking
  - Future enhancements

  **Recommended for**: Developers, project managers, contributors.

- **[PROGRESS.md](./PROGRESS.md)** - Implementation progress tracking:
  - Current implementation status
  - Command implementation status
  - Milestone completion percentages
  - Component status
  - Code quality metrics
  - Next steps and priorities

  **Recommended for**: Developers, project managers, stakeholders.

## Quick Navigation

### For Users

1. **Getting Started**: 
   - Read [CONFIG.md](./CONFIG.md) to understand configuration
   - Read [SAFETY.md](./SAFETY.md) before first use
   - Read [RULES.md](./RULES.md) to create organization rules

2. **Understanding Behavior**:
   - Use `neatd explain <path>` to understand rule matching
   - Check [RULES.md](./RULES.md) for matching logic details
   - Review [SAFETY.md](./SAFETY.md) for safety features

3. **Troubleshooting**:
   - Check [CONFIG.md](./CONFIG.md) for configuration issues
   - Review [RULES.md](./RULES.md) troubleshooting section
   - Review [SAFETY.md](./SAFETY.md) recovery procedures

### For Developers

1. **Understanding the Codebase**:
   - Start with [HIGH_LEVEL_DESIGN.md](./HIGH_LEVEL_DESIGN.md) Section 12 (Module Structure)
   - Read [ARCHITECTURE.md](./ARCHITECTURE.md) for component interactions
   - Review [DESIGN_DECISIONS.md](./DESIGN_DECISIONS.md) for context

2. **Implementing Features**:
   - Review [ROADMAP.md](./ROADMAP.md) for current milestones
   - Check [ARCHITECTURE.md](./ARCHITECTURE.md) for data flows
   - Review [DESIGN_DECISIONS.md](./DESIGN_DECISIONS.md) for design patterns

3. **Making Changes**:
   - Review [DESIGN_DECISIONS.md](./DESIGN_DECISIONS.md) for existing decisions
   - Update relevant documentation when making changes
   - Follow the "Definition of Done" in [HIGH_LEVEL_DESIGN.md](./HIGH_LEVEL_DESIGN.md)

### For Architects

1. **System Overview**:
   - Read [HIGH_LEVEL_DESIGN.md](./HIGH_LEVEL_DESIGN.md) for complete system overview
   - Review [ARCHITECTURE.md](./ARCHITECTURE.md) for architectural patterns
   - Check [DESIGN_DECISIONS.md](./DESIGN_DECISIONS.md) for rationale

2. **Design Evaluation**:
   - Review architecture diagrams in [ARCHITECTURE.md](./ARCHITECTURE.md)
   - Evaluate design decisions in [DESIGN_DECISIONS.md](./DESIGN_DECISIONS.md)
   - Consider scalability in [HIGH_LEVEL_DESIGN.md](./HIGH_LEVEL_DESIGN.md) Section 8

## Document Maintenance

These documents should be updated when:

- **Major architectural changes**: Update [ARCHITECTURE.md](./ARCHITECTURE.md) and [HIGH_LEVEL_DESIGN.md](./HIGH_LEVEL_DESIGN.md)
- **New components added**: Update [ARCHITECTURE.md](./ARCHITECTURE.md) and module structure in [HIGH_LEVEL_DESIGN.md](./HIGH_LEVEL_DESIGN.md)
- **Significant design decisions**: Update [DESIGN_DECISIONS.md](./DESIGN_DECISIONS.md)
- **Configuration schema changes**: Update [CONFIG.md](./CONFIG.md)
- **Rule matching changes**: Update [RULES.md](./RULES.md)
- **Safety features added/changed**: Update [SAFETY.md](./SAFETY.md)
- **Milestones completed**: Update [ROADMAP.md](./ROADMAP.md)
- **New features added**: Update relevant documentation sections

## Documentation Standards

All documentation should:

- Be clear and accessible to the target audience
- Include examples where helpful
- Be kept up-to-date with code changes
- Follow the structure and style of existing documents
- Include version information and last updated dates

---

**Document Version**: 2.0  
**Last Updated**: 2025-12-16  
**Application Version**: 0.1.0
