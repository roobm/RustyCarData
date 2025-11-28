# Quickstart: Unified AI Analysis Implementation

## Overview

This guide provides step-by-step instructions for implementing the unified AI analysis interface.

## Prerequisites

- Feature branch: `013-unified-ai-analysis`
- Existing codebase with:
  - Agent infrastructure (backend)
  - Supabase session storage
  - React + TypeScript + Tauri setup
  - Vitest testing configured

## Implementation Phases

### Phase 1: Type Definitions (Day 1, 2 hours)

**Create type file**:
```bash
touch src/types/analysis.ts
```

**Copy types from** [data-model.md](./data-model.md):
- Session types
- Agent types
- Analysis types

**Test**: TypeScript compiles without errors

---

### Phase 2: State Management (Day 1, 3 hours)

**Create analysis store**:
```bash
touch src/store/analysisStore.ts
```

**Implement**:
1. Copy store structure from data-model.md
2. Import types from `../types/analysis`
3. Implement all actions
4. Add validation logic

**Test**:
```typescript
// src/store/analysisStore.test.ts
describe('analysisStore', () => {
  it('toggles agent selection', () => {
    // Test toggle logic
  });

  it('validates analysis request', () => {
    // Test validation
  });
});
```

---

### Phase 3: Service Layer (Day 1, 1 hour)

**Create service**:
```bash
touch src/services/analysisService.ts
```

**Implement**:
```typescript
export async function analyzeSession(
  sessionId: string,
  agentIds: string[]
): Promise<AgentAnalysisResult[]> {
  return await invoke('analyze_session_with_agents', {
    sessionId,
    agentIds
  });
}
```

**Test with mock**:
```typescript
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));
```

---

### Phase 4: Utility Components (Day 2, 4 hours)

**Create components**:
```bash
mkdir -p src/components/analysis
touch src/components/analysis/CollapsibleSection.tsx
touch src/components/analysis/LoadingSpinner.tsx
touch src/components/ErrorBanner.tsx
```

**Implement in order**:
1. **LoadingSpinner**: Simple spinner with text
2. **ErrorBanner**: Error message + retry button
3. **CollapsibleSection**: Animated expand/collapse

**Test each**:
```bash
npm run test -- CollapsibleSection
```

---

### Phase 5: Selection Components (Day 3, 6 hours)

#### SessionCard (2 hours)
```bash
touch src/components/analysis/SessionCard.tsx
```

**Implement**:
- Display session metadata
- Selected state styling
- Click handler
- Keyboard support

**Test**:
- Renders all fields
- Handles click
- Shows selected state
- Keyboard accessible

#### SessionSelector (2 hours)
```bash
touch src/components/analysis/SessionSelector.tsx
```

**Implement**:
- Fetch sessions from Supabase
- Grid layout
- Loading/error states
- Selection management

**Test**:
- Fetches on mount
- Handles errors
- Manages selection

#### AgentSelector (2 hours)
```bash
touch src/components/analysis/AgentSelector.tsx
```

**Implement**:
- Fetch agents from store
- Multi-select with checkboxes
- Select all/none
- Selection count display

**Modify existing AgentCard**:
```typescript
// Add checkbox mode
<AgentCard
  agent={agent}
  selected={selected}
  onSelect={onSelect}
  showCheckbox={true}  // NEW
/>
```

---

### Phase 6: Action Component (Day 4, 2 hours)

**Create**:
```bash
touch src/components/analysis/AnalyzeButton.tsx
```

**Implement**:
- Dynamic text based on selection
- Disabled/loading states
- Tooltip for disabled state
- Click handler

**Test**:
- Shows correct text
- Disabled when invalid
- Calls onClick
- Shows loading

---

### Phase 7: Results Components (Day 4-5, 8 hours)

#### AgentResultCard (4 hours)
```bash
touch src/components/analysis/AgentResultCard.tsx
```

**Implement**:
- Collapsible card
- Score display
- Insights grouped by type
- Recommendations sorted by priority
- Error state

**Sections**:
1. Header (agent name, score)
2. Summary
3. Insights (with icons by type)
4. Recommendations (with priority badges)

#### ResultsDisplay (4 hours)
```bash
touch src/components/analysis/ResultsDisplay.tsx
```

**Implement**:
- Map results to AgentResultCards
- Empty state
- Loading state
- Error handling
- Auto-scroll on mount

---

### Phase 8: Main Page (Day 5, 4 hours)

**Create**:
```bash
touch src/pages/UnifiedAnalysisPage.tsx
```

**Implement layout**:
```typescript
export const UnifiedAnalysisPage: React.FC = () => {
  const store = useAnalysisStore();
  const [expanded, setExpanded] = useState({
    sessions: true,
    agents: true,
    results: false
  });

  return (
    <div className="p-6 space-y-6">
      <CollapsibleSection
        title="Select Session"
        expanded={expanded.sessions}
        onToggle={(e) => setExpanded({...expanded, sessions: e})}
      >
        <SessionSelector
          selectedId={store.selectedSessionId}
          onSelect={store.setSelectedSession}
        />
      </CollapsibleSection>

      <CollapsibleSection
        title="Choose AI Agents"
        expanded={expanded.agents}
      >
        <AgentSelector
          selectedIds={store.selectedAgentIds}
          onToggle={store.toggleAgent}
          onSelectAll={() => {/* ... */}}
          onDeselectAll={() => {/* ... */}}
        />
      </CollapsibleSection>

      <AnalyzeButton
        disabled={!store.selectedSessionId || store.selectedAgentIds.size === 0}
        loading={store.isAnalyzing}
        agentCount={store.selectedAgentIds.size}
        onClick={store.analyze}
      />

      <CollapsibleSection
        title="Analysis Results"
        expanded={expanded.results}
      >
        <ResultsDisplay
          results={store.currentResults || []}
          loading={store.isAnalyzing}
          error={store.error}
          onClearResults={() => store.clearResults()}
        />
      </CollapsibleSection>
    </div>
  );
};
```

---

### Phase 9: Routing & Navigation (Day 6, 2 hours)

**Update App.tsx**:
```typescript
import { UnifiedAnalysisPage } from './pages/UnifiedAnalysisPage';

// Replace:
<Route path="/analysis" element={<AnalysisPage />} />
<Route path="/agents" element={<AgentDashboard />} />

// With:
<Route path="/analysis" element={<UnifiedAnalysisPage />} />
```

**Update Layout.tsx**:
```typescript
// Remove:
<Link to="/agents">AI Agents</Link>

// Keep only:
<Link to="/analysis">AI Analysis</Link>
```

---

### Phase 10: Cleanup (Day 6, 2 hours)

**Remove old files**:
```bash
git rm src/pages/AnalysisPage.tsx
git rm src/pages/AgentDashboard.tsx
git rm src/pages/AgentDashboard.css
```

**Update imports**:
- Search for imports of removed files
- Update to use UnifiedAnalysisPage

---

### Phase 11: Testing (Day 7, Full day)

#### Unit Tests
```bash
npm run test -- components/analysis
```

Test all new components:
- [ ] SessionCard
- [ ] SessionSelector
- [ ] AgentSelector
- [ ] AnalyzeButton
- [ ] AgentResultCard
- [ ] ResultsDisplay
- [ ] CollapsibleSection

#### Integration Test
```typescript
// tests/integration/unified-analysis.test.tsx
describe('Unified Analysis Flow', () => {
  it('completes full workflow', async () => {
    render(<UnifiedAnalysisPage />);

    // 1. Select session
    const sessionCard = await screen.findByText(/Session 1/);
    await userEvent.click(sessionCard);

    // 2. Select agents
    const fuelCoach = await screen.findByLabelText(/Fuel Coach/);
    await userEvent.click(fuelCoach);

    // 3. Analyze
    const analyzeBtn = screen.getByRole('button', { name: /Analyze/ });
    await userEvent.click(analyzeBtn);

    // 4. Wait for results
    expect(await screen.findByText(/Analysis Results/)).toBeInTheDocument();
  });
});
```

---

## Development Checklist

### Before Starting
- [ ] Pull latest from main
- [ ] Create feature branch `013-unified-ai-analysis`
- [ ] Read spec.md, data-model.md, contracts

### During Development
- [ ] Write tests FIRST (TDD)
- [ ] Follow TypeScript strict mode
- [ ] Use existing components where possible
- [ ] Keep commits small and focused
- [ ] Run tests after each component

### Code Quality
- [ ] No `any` types
- [ ] All props typed
- [ ] Error states handled
- [ ] Loading states shown
- [ ] Accessible (ARIA labels, keyboard)

### Before PR
- [ ] All tests passing
- [ ] No console errors/warnings
- [ ] Tested in browser manually
- [ ] Code formatted (Prettier)
- [ ] TypeScript compiles clean
- [ ] Update documentation

---

## Commit Strategy

**Pattern**: Feature-based commits

```bash
# Phase 1-3
git commit -m "feat(analysis): Add type definitions and store"

# Phase 4
git commit -m "feat(analysis): Add utility components"

# Phase 5
git commit -m "feat(analysis): Add session and agent selectors"

# Phase 6
git commit -m "feat(analysis): Add analyze button with validation"

# Phase 7
git commit -m "feat(analysis): Add results display components"

# Phase 8
git commit -m "feat(analysis): Create unified analysis page"

# Phase 9
git commit -m "feat(analysis): Update routing and navigation"

# Phase 10
git commit -m "refactor(analysis): Remove old analysis and agent pages"

# Phase 11
git commit -m "test(analysis): Add integration tests for unified flow"
```

---

## Troubleshooting

### Common Issues

**Issue**: Tauri command not found
```bash
# Check backend has command registered
grep "analyze_session_with_agents" src-tauri/src/lib.rs
```

**Issue**: Store not updating
```bash
# Check Zustand DevTools
# Add logging to store actions
```

**Issue**: Tests failing with Tauri mocks
```typescript
// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue([])
}));
```

**Issue**: Styles not applying
```bash
# Check Tailwind classes compile
npm run build
```

---

## Performance Optimization

### After Basic Implementation

1. **Memoize expensive components**:
```typescript
export const SessionCard = React.memo(SessionCardComponent);
```

2. **Virtualize long lists** (if >100 sessions):
```bash
npm install react-window
```

3. **Lazy load page**:
```typescript
const UnifiedAnalysisPage = lazy(() => import('./pages/UnifiedAnalysisPage'));
```

4. **Debounce selections** (if needed):
```typescript
const debouncedAnalyze = useMemo(
  () => debounce(store.analyze, 300),
  []
);
```

---

## Timeline Estimate

| Phase | Duration | Can Parallelize? |
|-------|----------|------------------|
| 1-3: Foundation | 6 hours | No |
| 4: Utilities | 4 hours | No |
| 5: Selectors | 6 hours | Yes (2 developers) |
| 6: Button | 2 hours | Yes (parallel to 5) |
| 7: Results | 8 hours | Yes (2 developers) |
| 8: Page | 4 hours | No (needs 5-7) |
| 9-10: Migration | 4 hours | No |
| 11: Testing | 8 hours | Partially |

**Total**: 42 hours (~5-6 days for 1 developer, 3-4 days for 2)

---

## Success Criteria

### Functional
- [ ] User can select a session
- [ ] User can select multiple agents
- [ ] User can trigger analysis
- [ ] Results display correctly
- [ ] Errors handled gracefully
- [ ] Old pages removed

### Non-Functional
- [ ] Page loads in <1s
- [ ] Analysis completes in <5s
- [ ] All interactions <100ms
- [ ] Accessible (WCAG AA)
- [ ] Works on macOS/Windows/Linux
- [ ] Tests cover >80% of code

### User Experience
- [ ] Clear visual feedback for all actions
- [ ] No confusing states
- [ ] Helpful error messages
- [ ] Smooth animations
- [ ] Intuitive workflow

---

**Ready to implement! Start with Phase 1.**
