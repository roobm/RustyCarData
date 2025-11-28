# Tasks: Unified AI Analysis Interface

**Branch**: `013-unified-ai-analysis` | **Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

## Overview

Merge AI Analysis and AI Agents pages into a single unified interface where users can:
1. Select a driving session
2. Choose AI agent(s) for analysis
3. View results organized by agent

**Implementation Strategy**: Incremental delivery, each user story is independently testable.

---

## Phase 1: Setup & Foundation

**Goal**: Establish type definitions, state management, and service layer.

### Tasks

- [X] T001 Create type definitions file at src/types/analysis.ts
- [X] T002 [P] Implement Session types (Session, SessionCard) in src/types/analysis.ts
- [X] T003 [P] Implement Agent types (AgentMetadata, SelectableAgent) in src/types/analysis.ts
- [X] T004 [P] Implement Analysis types (AnalysisRequest, AgentAnalysisResult, Insight, Recommendation) in src/types/analysis.ts
- [X] T005 Create analysisStore with Zustand at src/store/analysisStore.ts
- [X] T006 Implement selection state (selectedSessionId, selectedAgentIds) in src/store/analysisStore.ts
- [X] T007 Implement analysis state (isAnalyzing, resultsCache, currentResults) in src/store/analysisStore.ts
- [X] T008 Implement actions (setSelectedSession, toggleAgent, selectAllAgents, analyze) in src/store/analysisStore.ts
- [X] T009 Create analysisService at src/services/analysisService.ts
- [X] T010 Implement analyzeSession function calling Tauri 'analyze_session_with_agents' in src/services/analysisService.ts
- [X] T011 [P] Create utility functions for session transformation at src/utils/sessionTransform.ts
- [X] T012 [P] Create validation utilities at src/utils/validation.ts

**Completion Criteria**:
- ✅ All types compile without errors
- ✅ Store actions update state correctly
- ✅ Service can call backend (mock test)

---

## Phase 2: User Story 1 - Session Selection

**Goal**: User can view and select a driving session for analysis.

**User Story**: "As a user, I want to select a driving session so that I can choose what data to analyze."

### Tasks

- [X] T013 [US1] Create SessionCard component at src/components/analysis/SessionCard.tsx
- [X] T014 [US1] Implement SessionCard props interface (session, selected, onClick) in src/components/analysis/SessionCard.tsx
- [X] T015 [US1] Add visual selected state (border, background) to SessionCard in src/components/analysis/SessionCard.tsx
- [X] T016 [US1] Add keyboard support (Enter/Space) to SessionCard in src/components/analysis/SessionCard.tsx
- [X] T017 [US1] Create SessionSelector component at src/components/analysis/SessionSelector.tsx
- [X] T018 [US1] Implement session fetching via 'get_supabase_sessions' command in src/components/analysis/SessionSelector.tsx
- [X] T019 [US1] Implement card grid layout in SessionSelector in src/components/analysis/SessionSelector.tsx
- [X] T020 [US1] Add loading state (skeleton cards) to SessionSelector in src/components/analysis/SessionSelector.tsx
- [X] T021 [US1] Add error state with retry button to SessionSelector in src/components/analysis/SessionSelector.tsx
- [X] T022 [US1] Add empty state with import CTA to SessionSelector in src/components/analysis/SessionSelector.tsx
- [X] T023 [US1] Connect SessionSelector to analysisStore in src/components/analysis/SessionSelector.tsx

**Independent Test Criteria**:
- ✅ User can see list of sessions as cards
- ✅ Clicking a session highlights it
- ✅ Selected session shows in store state
- ✅ Loading state displays during fetch
- ✅ Error state shows with retry option
- ✅ Empty state shows when no sessions
- ✅ Keyboard navigation works (Tab, Enter)

---

## Phase 3: User Story 2 - Agent Selection

**Goal**: User can choose one or more AI agents to perform the analysis.

**User Story**: "As a user, I want to select AI agents so that I can choose which analysis I want performed."

### Tasks

- [X] T024 [US2] Modify existing AgentCard component at src/components/AgentCard.tsx
- [X] T025 [US2] Add showCheckbox prop to AgentCard in src/components/AgentCard.tsx
- [X] T026 [US2] Implement checkbox visual state in AgentCard in src/components/AgentCard.tsx
- [X] T027 [US2] Create AgentSelector component at src/components/analysis/AgentSelector.tsx
- [X] T028 [US2] Implement agent grid layout with checkboxes in src/components/analysis/AgentSelector.tsx
- [X] T029 [US2] Add "Select All" / "Deselect All" toggle in src/components/analysis/AgentSelector.tsx
- [X] T030 [US2] Display selection count in src/components/analysis/AgentSelector.tsx
- [X] T031 [US2] Connect AgentSelector to agentStore (fetch agents) in src/components/analysis/AgentSelector.tsx
- [X] T032 [US2] Connect AgentSelector to analysisStore (selection state) in src/components/analysis/AgentSelector.tsx
- [X] T033 [US2] Add loading state to AgentSelector in src/components/analysis/AgentSelector.tsx
- [X] T034 [US2] Add error state with retry to AgentSelector in src/components/analysis/AgentSelector.tsx

**Independent Test Criteria**:
- ✅ User can see grid of available agents
- ✅ Agents display with checkboxes
- ✅ Clicking checkbox toggles agent selection
- ✅ "Select All" selects all agents
- ✅ Selection count updates correctly
- ✅ Selected agents stored in analysisStore
- ✅ Keyboard navigation works for checkboxes

---

## Phase 4: User Story 3 - Trigger Analysis

**Goal**: User can initiate analysis with selected session and agents.

**User Story**: "As a user, I want to click an 'Analyze' button so that I can start the analysis of my selected data with chosen agents."

### Tasks

- [X] T035 [US3] Create AnalyzeButton component at src/components/analysis/AnalyzeButton.tsx
- [X] T036 [US3] Implement dynamic button text based on agent count in src/components/analysis/AnalyzeButton.tsx
- [X] T037 [US3] Add disabled state when selection incomplete in src/components/analysis/AnalyzeButton.tsx
- [X] T038 [US3] Add loading state with spinner during analysis in src/components/analysis/AnalyzeButton.tsx
- [X] T039 [US3] Add tooltip explaining why button is disabled in src/components/analysis/AnalyzeButton.tsx
- [X] T040 [US3] Implement onClick handler calling analysisStore.analyze in src/components/analysis/AnalyzeButton.tsx
- [X] T041 [US3] Add validation before triggering analysis in src/store/analysisStore.ts analyze action

**Independent Test Criteria**:
- ✅ Button is disabled when no session selected
- ✅ Button is disabled when no agents selected
- ✅ Button shows dynamic text ("Analyze with 2 agents")
- ✅ Clicking button triggers analysis
- ✅ Loading spinner shows during analysis
- ✅ Tooltip explains disabled state
- ✅ Keyboard accessible (Enter/Space)

---

## Phase 5: User Story 4 - View Results

**Goal**: User can view analysis results organized by agent.

**User Story**: "As a user, I want to see analysis results so that I can understand the insights from each AI agent."

### Tasks

- [X] T042 [US4] Create AgentResultCard component at src/components/analysis/AgentResultCard.tsx
- [X] T043 [US4] Implement collapsible card UI in AgentResultCard in src/components/analysis/AgentResultCard.tsx
- [X] T044 [US4] Add header section (agent name, score, timestamp) to AgentResultCard in src/components/analysis/AgentResultCard.tsx
- [X] T045 [US4] Add insights section grouped by type (critical, warning, positive) in AgentResultCard in src/components/analysis/AgentResultCard.tsx
- [X] T046 [US4] Add recommendations section sorted by priority in AgentResultCard in src/components/analysis/AgentResultCard.tsx
- [X] T047 [US4] Add error state if agent analysis failed in AgentResultCard in src/components/analysis/AgentResultCard.tsx
- [X] T048 [US4] Create ResultsDisplay component at src/components/analysis/ResultsDisplay.tsx
- [X] T049 [US4] Map results array to AgentResultCards in src/components/analysis/ResultsDisplay.tsx
- [X] T050 [US4] Add empty state when no results in src/components/analysis/ResultsDisplay.tsx
- [X] T051 [US4] Add loading state during analysis in src/components/analysis/ResultsDisplay.tsx
- [X] T052 [US4] Add error banner with retry in src/components/analysis/ResultsDisplay.tsx
- [X] T053 [US4] Implement auto-scroll to results on completion in src/components/analysis/ResultsDisplay.tsx
- [X] T054 [US4] Add clear results action in src/components/analysis/ResultsDisplay.tsx

**Independent Test Criteria**:
- ✅ Results display after analysis completes
- ✅ Each agent has its own result card
- ✅ Cards are collapsible/expandable
- ✅ Insights grouped by type with icons
- ✅ Recommendations sorted by priority
- ✅ Failed agents show error state
- ✅ Empty state shows when no results
- ✅ Auto-scrolls to results section
- ✅ User can clear results

---

## Phase 6: User Story 5 - Unified Page Integration

**Goal**: Bring all components together in a single unified page.

**User Story**: "As a user, I want a single page for AI analysis so that I don't have to navigate between multiple pages."

### Tasks

- [ ] T055 [P] [US5] Create CollapsibleSection utility component at src/components/analysis/CollapsibleSection.tsx
- [ ] T056 [P] [US5] Add expand/collapse animation to CollapsibleSection in src/components/analysis/CollapsibleSection.tsx
- [ ] T057 [P] [US5] Add keyboard support (Enter to toggle) to CollapsibleSection in src/components/analysis/CollapsibleSection.tsx
- [ ] T058 [US5] Create UnifiedAnalysisPage at src/pages/UnifiedAnalysisPage.tsx
- [ ] T059 [US5] Add SessionSelector in collapsible section to UnifiedAnalysisPage in src/pages/UnifiedAnalysisPage.tsx
- [ ] T060 [US5] Add AgentSelector in collapsible section to UnifiedAnalysisPage in src/pages/UnifiedAnalysisPage.tsx
- [ ] T061 [US5] Add AnalyzeButton between selection and results in src/pages/UnifiedAnalysisPage.tsx
- [ ] T062 [US5] Add ResultsDisplay in collapsible section to UnifiedAnalysisPage in src/pages/UnifiedAnalysisPage.tsx
- [ ] T063 [US5] Implement section expand/collapse state management in src/pages/UnifiedAnalysisPage.tsx
- [ ] T064 [US5] Auto-expand results section after analysis in src/pages/UnifiedAnalysisPage.tsx
- [ ] T065 [US5] Auto-collapse selection sections after analysis in src/pages/UnifiedAnalysisPage.tsx
- [ ] T066 [US5] Add page header with title and subtitle in src/pages/UnifiedAnalysisPage.tsx

**Independent Test Criteria**:
- ✅ Page displays all three sections (sessions, agents, results)
- ✅ Sections are collapsible/expandable
- ✅ Progressive disclosure works (sessions/agents expanded, results collapsed initially)
- ✅ After analysis, results expand and selections collapse
- ✅ Full workflow works end-to-end
- ✅ Page is keyboard navigable
- ✅ Layout is responsive

---

## Phase 7: User Story 6 - Navigation & Cleanup

**Goal**: Update routing and remove old pages.

**User Story**: "As a user, I want a single 'AI Analysis' menu item so that the navigation is simpler."

### Tasks

- [ ] T067 [US6] Update route '/analysis' to UnifiedAnalysisPage in src/App.tsx
- [ ] T068 [US6] Remove route '/agents' from src/App.tsx
- [ ] T069 [US6] Remove 'AI Agents' navigation link from src/components/Layout.tsx
- [ ] T070 [US6] Update 'AI Analysis' link to use combined icon in src/components/Layout.tsx
- [ ] T071 [US6] Delete old AnalysisPage.tsx from src/pages/
- [ ] T072 [US6] Delete old AgentDashboard.tsx from src/pages/
- [ ] T073 [US6] Delete old AgentDashboard.css from src/pages/
- [ ] T074 [US6] Update any remaining imports of deleted pages
- [ ] T075 [US6] Test all navigation links still work

**Independent Test Criteria**:
- ✅ '/analysis' route displays UnifiedAnalysisPage
- ✅ '/agents' route no longer exists (404 or redirect)
- ✅ Navigation shows single "AI Analysis" link
- ✅ No broken imports or console errors
- ✅ Old files removed from codebase

---

## Phase 8: Polish & Accessibility

**Goal**: Add final polish, error handling, and accessibility features.

### Tasks

- [ ] T076 [P] Create LoadingSpinner component at src/components/LoadingSpinner.tsx
- [ ] T077 [P] Create ErrorBanner component at src/components/ErrorBanner.tsx
- [ ] T078 Add ARIA labels to all interactive elements in SessionCard
- [ ] T079 Add ARIA labels to all interactive elements in AgentSelector
- [ ] T080 Add ARIA labels to all interactive elements in AnalyzeButton
- [ ] T081 Add ARIA labels to all interactive elements in ResultsDisplay
- [ ] T082 Add ARIA live region for analysis progress announcements in UnifiedAnalysisPage
- [ ] T083 Implement error boundary for SessionSelector in src/components/analysis/SessionSelector.tsx
- [ ] T084 Implement error boundary for AgentSelector in src/components/analysis/AgentSelector.tsx
- [ ] T085 Implement error boundary for ResultsDisplay in src/components/analysis/ResultsDisplay.tsx
- [ ] T086 Add focus management (move focus to results after analysis) in UnifiedAnalysisPage
- [ ] T087 [P] Test keyboard navigation through entire workflow
- [ ] T088 [P] Test screen reader compatibility
- [ ] T089 [P] Verify color contrast meets WCAG AA standards
- [ ] T090 [P] Add loading skeletons for session cards during fetch

**Completion Criteria**:
- ✅ All components have proper ARIA labels
- ✅ Keyboard navigation works throughout
- ✅ Screen reader announces state changes
- ✅ Focus moves logically through workflow
- ✅ Error boundaries catch and display errors gracefully
- ✅ Loading states are clear and informative
- ✅ Color contrast meets accessibility standards

---

## Dependencies

### User Story Completion Order

```
Phase 1 (Setup) → Must complete first
         ↓
Phase 2 (US1: Session Selection) → Independent
         ↓
Phase 3 (US2: Agent Selection) → Independent (can parallel with US1)
         ↓
Phase 4 (US3: Trigger Analysis) → Depends on US1 + US2
         ↓
Phase 5 (US4: View Results) → Depends on US3
         ↓
Phase 6 (US5: Page Integration) → Depends on US1 + US2 + US3 + US4
         ↓
Phase 7 (US6: Navigation) → Depends on US5
         ↓
Phase 8 (Polish) → Can happen anytime after US5
```

### Parallel Execution Opportunities

**Phase 1** (Foundation - Sequential):
- T001-T004: Type definitions (can be parallel per type group)
- T005-T008: Store implementation (sequential)
- T009-T010: Service layer (sequential)
- T011-T012: Utilities (parallel with each other)

**Phase 2** (US1 - Some parallel):
- T013-T016: SessionCard (sequential)
- T017-T023: SessionSelector (sequential, depends on SessionCard)

**Phase 3** (US2 - Can parallel with Phase 2):
- T024-T026: AgentCard mods (sequential)
- T027-T034: AgentSelector (sequential, depends on AgentCard)
- **Can work simultaneously with Phase 2 if 2 developers**

**Phase 4** (US3 - Small, sequential):
- T035-T041: AnalyzeButton (mostly sequential)

**Phase 5** (US4 - Some parallel):
- T042-T047: AgentResultCard (sequential)
- T048-T054: ResultsDisplay (sequential, depends on AgentResultCard)

**Phase 6** (US5 - Utility parallel):
- T055-T057: CollapsibleSection (parallel)
- T058-T066: Page integration (sequential, uses CollapsibleSection)

**Phase 7** (US6 - Sequential):
- T067-T075: Navigation and cleanup (mostly sequential)

**Phase 8** (Polish - Mostly parallel):
- T076-T077: Utility components (parallel)
- T078-T086: Accessibility (can be parallel across components)
- T087-T090: Testing and polish (parallel)

---

## Implementation Strategy

### MVP Scope (Minimal Viable Product)

**MVP = Phase 1 + Phase 2 + Phase 3 + Phase 4 + Phase 6 (core only)**

Minimal workflow:
1. User selects session (US1)
2. User selects agent(s) (US2)
3. User clicks analyze (US3)
4. User sees basic results (US4 - simplified)
5. Basic page layout (US5 - no collapsible, just stacked)

**Excludes for MVP**:
- Collapsible sections (use simple stacked layout)
- Advanced error handling
- Full accessibility features
- Navigation cleanup (keep both pages initially)

**Time Estimate**: ~2-3 days for MVP

### Full Implementation

**Time Estimate**: 4-6 days for complete feature

**Recommended Approach**:
1. Complete MVP first
2. Test end-to-end workflow
3. Add progressive disclosure (collapsible sections)
4. Add polish and accessibility
5. Clean up old pages

---

## Task Statistics

- **Total Tasks**: 90
- **Phase 1 (Setup)**: 12 tasks
- **Phase 2 (US1)**: 11 tasks
- **Phase 3 (US2)**: 11 tasks
- **Phase 4 (US3)**: 7 tasks
- **Phase 5 (US4)**: 13 tasks
- **Phase 6 (US5)**: 12 tasks
- **Phase 7 (US6)**: 9 tasks
- **Phase 8 (Polish)**: 15 tasks

**Parallel Tasks**: 31 tasks marked [P] can be done in parallel

**MVP Tasks**: ~40 tasks (Phases 1, 2, 3, 4, + simplified Phase 6)

---

## Testing Checklist

### Manual Testing

After each phase, verify:

**Phase 2 (US1)**:
- [ ] Can see session list
- [ ] Can select a session
- [ ] Selection persists in state

**Phase 3 (US2)**:
- [ ] Can see agent grid
- [ ] Can select multiple agents
- [ ] Selection count updates

**Phase 4 (US3)**:
- [ ] Button disables correctly
- [ ] Analysis triggers
- [ ] Loading state shows

**Phase 5 (US4)**:
- [ ] Results display
- [ ] Cards are expandable
- [ ] Insights/recommendations formatted correctly

**Phase 6 (US5)**:
- [ ] Full workflow works
- [ ] Sections expand/collapse
- [ ] Progressive disclosure works

**Phase 7 (US6)**:
- [ ] Old pages removed
- [ ] Navigation updated
- [ ] No broken links

**Phase 8 (Polish)**:
- [ ] Keyboard navigation works
- [ ] Screen reader works
- [ ] No console errors

### Automated Testing (Optional)

If adding tests, create after MVP:
- Unit tests for each component
- Integration test for full workflow
- Accessibility tests (axe-core)

---

**Ready for implementation! Start with Phase 1 (T001-T012).**
