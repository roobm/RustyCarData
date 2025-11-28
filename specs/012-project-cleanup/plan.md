# Project Cleanup and Modernization - Plan

## Overview
Comprehensive cleanup and modernization of the carDataProject codebase to improve maintainability, testability, and code quality.

## Objectives
1. **Unit Testing (PRIORITY)**: Implement comprehensive unit tests for all major components
2. **Code Cleanup**: Remove unused files, functions, and imports
3. **Warning Resolution**: Clear all compiler warnings
4. **Architecture Modernization**: Apply modern best practices for React/TypeScript frontend and Tauri Rust backend
5. **Project Structure**: Improve organization and naming conventions

## Phase 0: Research & Assessment

### Research Tasks
- [ ] Modern React/TypeScript frontend architecture best practices (2024-2025)
- [ ] Tauri Rust backend best practices and project structure
- [ ] Rust unit testing frameworks (`cargo test`, `mockall`, `rstest`)
- [ ] React testing frameworks (Jest, React Testing Library, Vitest)
- [ ] Current state assessment: warnings, unused code, test coverage

### Assessment Deliverables
- Document current compiler warnings (Rust + TypeScript)
- List all unused files and functions
- Identify untested modules and components
- Architecture recommendations based on research

## Phase 1: Unit Testing Implementation (PRIORITY)

User requirement: *"Make sure we have unit tests for everything. Start with that so we can use them later when we are refactoring."*

### Backend Testing (Rust)
- [ ] Set up testing infrastructure (`#[cfg(test)]` modules)
- [ ] Unit tests for `simulation` module
  - [ ] Test `mod.rs` simulation loop logic
  - [ ] Test `state.rs` state management
  - [ ] Test dataset playback functionality
- [ ] Unit tests for `storage` module
  - [ ] Test `supabase.rs` client operations (mocked)
  - [ ] Test data serialization/deserialization
- [ ] Unit tests for `commands.rs` Tauri commands
- [ ] Integration tests for full workflows

### Frontend Testing (React/TypeScript)
- [ ] Set up testing infrastructure (Jest/Vitest + React Testing Library)
- [ ] Component tests
  - [ ] `Gauge.tsx`
  - [ ] `CanMonitor.tsx`
  - [ ] `TelemetryChart.tsx`
  - [ ] `PlaybackControls.tsx`
  - [ ] `SessionNameDialog.tsx`
- [ ] Page tests
  - [ ] `DashboardPage.tsx`
  - [ ] `SessionHistoryPage.tsx`
  - [ ] `SessionDetailPage.tsx`
- [ ] Store tests (`useStore.ts`)
- [ ] API service tests (`api.ts`)

## Phase 2: Code Cleanup

### Remove Unused Code
- [ ] Scan for unused imports (Rust: `cargo +nightly udeps`, TypeScript: `ts-prune`)
- [ ] Remove dead code and functions
- [ ] Clean up obsolete files
- [ ] Remove commented-out code

### Clear Warnings
- [ ] Address all Rust compiler warnings
- [ ] Address all TypeScript/ESLint warnings
- [ ] Fix any clippy suggestions (`cargo clippy`)

## Phase 3: Architecture Improvements

### Backend (Rust/Tauri)
Based on research findings:
- [ ] Improve module organization
- [ ] Apply Rust best practices (error handling, async patterns)
- [ ] Enhance code documentation
- [ ] Consider separating concerns (business logic vs Tauri commands)

### Frontend (React/TypeScript)
Based on research findings:
- [ ] Improve component organization
- [ ] Apply modern React patterns (hooks, composition)
- [ ] Enhance type safety
- [ ] Consider folder structure improvements

## Phase 4: Project Structure & Naming

### Speckit Files Organization
- [ ] Review and organize all spec files in `specs/` directory
- [ ] Ensure consistent naming and structure
- [ ] Archive completed features properly

### Directory Naming
- [ ] Rename `src-tauri/` to more recognizable backend directory name (if applicable)
- [ ] Improve frontend directory structure clarity
- [ ] Update documentation to reflect new structure

## Success Criteria
- [ ] 80%+ code coverage with unit tests
- [ ] Zero compiler warnings (Rust + TypeScript)
- [ ] Zero unused files or dead code
- [ ] All speckit files properly organized
- [ ] Architecture follows modern best practices
- [ ] Clear, logical project structure
- [ ] Documentation updated

## Risks & Considerations
- **Testing overhead**: Comprehensive testing will take significant time
- **Breaking changes**: Refactoring may introduce bugs (mitigated by tests)
- **Scope creep**: Need to balance cleanup vs new features
- **Build performance**: Additional tests may slow CI/CD

## Timeline Approach
- Phase 0: Research and assessment
- Phase 1: Unit testing (priority - complete before refactoring)
- Phase 2: Code cleanup (safe with tests in place)
- Phase 3: Architecture improvements (incremental)
- Phase 4: Project structure finalization

## Notes
- User priority: Unit tests first, then refactoring
- All changes should be incremental and tested
- Keep main branch stable - merge frequently in small chunks
- Document architectural decisions
