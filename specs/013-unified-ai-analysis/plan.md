# Implementation Plan: Unified AI Analysis Interface

**Branch**: `013-unified-ai-analysis` | **Date**: 2025-11-28 | **Spec**: [spec.md](./spec.md)

## Summary

Merge the separate AI Analysis (`/analysis`) and AI Agents (`/agents`) pages into a single unified interface where users can:
1. Select which driving session(s) to analyze
2. Choose which AI agent(s) to use for analysis
3. View results organized by agent

**Technical Approach**: Collapsible sections with card-based selection, progressive disclosure UX pattern, leveraging existing agent store and Supabase session data.

## Technical Context

**Language/Version**: TypeScript 5.8.3 (Frontend), Rust 1.75+ (Backend)
**Primary Dependencies**: React 19.1.0, Zustand 5.0.8, Recharts 3.4.1, Tauri 2.x, Axios 1.13.2
**Storage**: Supabase PostgreSQL (sessions, telemetry), Local state (Zustand)
**Testing**: Vitest 4.0.14 + React Testing Library (Frontend), cargo test (Backend)
**Target Platform**: Desktop (macOS, Windows, Linux) via Tauri
**Project Type**: Web application (React frontend + Tauri Rust backend)
**Performance Goals**: <100ms UI response, <3s for analysis results fetch
**Constraints**: Tauri IPC communication, Agent API rate limits, Supabase connection limits
**Scale/Scope**: ~5 specialized AI agents, 100s of sessions, single unified page (~400 LOC)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### ✅ I. Branch Management
- **Status**: PASS
- **Evidence**: Working on feature branch `013-unified-ai-analysis`

### ✅ II. Documentation
- **Status**: PASS
- **Evidence**: Comprehensive spec.md created with UX analysis, will update with implementation details

### ✅ III. Rust Idioms and Code Quality
- **Status**: PASS (Backend unchanged)
- **Evidence**: No Rust backend changes required for this feature. Existing agent infrastructure already follows constitution principles.

### ✅ IV. Architecture and Modularity
- **Status**: PASS
- **Evidence**:
  - Frontend: New unified component will use existing stores (agentStore, session data hooks)
  - Backend: Reuses existing Tauri commands (no new coupling)
  - Clear separation: UI layer change only, business logic unchanged

### ✅ V. Error Handling
- **Status**: PASS
- **Evidence**: Will follow existing error handling patterns (try/catch in React, display user-friendly messages)

**Overall**: ✅ All gates passed. No constitution violations.

## Project Structure

### Documentation (this feature)

```text
specs/013-unified-ai-analysis/
├── spec.md              # UX specification (completed)
├── plan.md              # This file
├── research.md          # Phase 0 output (to be created)
├── data-model.md        # Phase 1 output (to be created)
├── quickstart.md        # Phase 1 output (to be created)
├── contracts/           # Phase 1 output (to be created)
└── tasks.md             # Phase 2 output (NOT created by /speckit-plan)
```

### Source Code (repository root)

**Current Structure** (Tauri + React):
```text
src-tauri/                  # Rust backend (minimal changes)
├── src/
│   ├── agents/            # Existing agent infrastructure (no changes)
│   ├── commands.rs        # Tauri commands (no changes)
│   └── storage/           # Supabase integration (no changes)

src/                        # React frontend (PRIMARY WORK AREA)
├── pages/
│   ├── AnalysisPage.tsx          # REMOVE (merge into unified)
│   ├── AgentDashboard.tsx        # REMOVE (merge into unified)
│   └── UnifiedAnalysisPage.tsx   # CREATE (new unified interface)
├── components/
│   ├── analysis/                  # CREATE (new directory)
│   │   ├── SessionSelector.tsx   # Card-based session selection
│   │   ├── AgentSelector.tsx     # Checkbox-based agent selection
│   │   ├── AnalyzeButton.tsx     # Context-aware action button
│   │   └── ResultsDisplay.tsx    # Collapsible agent result cards
│   ├── AgentCard.tsx             # REUSE (existing)
│   └── AiAssistant.tsx           # REUSE (existing)
├── store/
│   ├── agentStore.ts             # MODIFY (add unified state)
│   └── sessionStore.ts           # CREATE (if needed for session state)
├── services/
│   ├── agentService.ts           # REUSE (existing)
│   └── api.ts                    # MODIFY (add Supabase session queries)
├── types/
│   └── analysis.ts               # CREATE (unified analysis types)
└── App.tsx                        # MODIFY (update routing)

tests/                      # Test files
├── components/
│   └── analysis/          # Unit tests for new components
└── integration/           # Integration tests for unified flow
```

**Structure Decision**: Tauri web application structure. Frontend changes only, reusing existing backend infrastructure. New `/analysis` route will host unified page, old `/agents` route will be removed.

## Complexity Tracking

> No violations - all gates passed.

---

## Phase 0: Research (PENDING)

**Unknowns to resolve:**
1. ✅ Session data source: Confirmed Supabase via existing `get_supabase_sessions` command
2. ✅ Agent selection persistence: Use Zustand store (in-memory, per-session)
3. ⚠️ **NEEDS CLARIFICATION**: Multi-session analysis - Is this required for v1?
4. ⚠️ **NEEDS CLARIFICATION**: Results caching strategy - Store in Zustand or Supabase?
5. ✅ UX patterns: Completed in spec.md (collapsible sections recommended)

**Research tasks:**
- [ ] Best practices for progressive disclosure in React apps
- [ ] Supabase session query patterns for dropdown/card views
- [ ] State management patterns for multi-step selections
- [ ] Error boundary patterns for analysis failures

**Output**: `research.md` with all clarifications resolved

## Phase 1: Design & Contracts (PENDING)

**Prerequisites**: research.md complete

**Deliverables:**
1. **data-model.md**: Define unified state shape, session/agent types
2. **contracts/**: Define component interfaces (props, callbacks)
3. **quickstart.md**: Component integration guide

**Key decisions needed:**
- State shape for unified analysis flow
- Component prop interfaces
- Event flow (selection → analysis → results)

---

## Phase 2: Task Breakdown (NOT IN THIS COMMAND)

*Use `/speckit-tasks` command after Phase 1 complete*

