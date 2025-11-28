# Component Interface Contracts

## Overview

This document defines the prop interfaces, callbacks, and contracts for all components in the unified AI analysis interface.

## Page Component

### UnifiedAnalysisPage

**File**: `src/pages/UnifiedAnalysisPage.tsx`

**Props**: None (root page component)

**Responsibilities**:
- Layout and coordination of all sections
- Scroll management
- Section expand/collapse state

**Contract**:
```typescript
export const UnifiedAnalysisPage: React.FC = () => {
  // Manages overall layout and section states
  // No props - gets data from stores
}
```

---

## Selection Components

### SessionSelector

**File**: `src/components/analysis/SessionSelector.tsx`

**Props**:
```typescript
interface SessionSelectorProps {
  selectedId: string | null;
  onSelect: (sessionId: string) => void;
  className?: string;
}
```

**Behavior**:
- Fetches sessions from Supabase on mount
- Displays as card grid
- Highlights selected session
- Shows loading/error states

**States**:
- Loading: Skeleton cards
- Error: Error banner with retry button
- Empty: "No sessions found" with import CTA
- Success: Grid of session cards

**Contract**:
```typescript
export const SessionSelector: React.FC<SessionSelectorProps> = ({
  selectedId,
  onSelect,
  className
}) => {
  // MUST call onSelect when user clicks a session
  // MUST show loading state during fetch
  // MUST handle fetch errors gracefully
  // MUST support keyboard navigation
}
```

### SessionCard

**File**: `src/components/analysis/SessionCard.tsx`

**Props**:
```typescript
interface SessionCardProps {
  session: SessionCard;  // From data-model.md
  selected: boolean;
  onClick: () => void;
  className?: string;
}
```

**Contract**:
```typescript
export const SessionCard: React.FC<SessionCardProps> = ({
  session,
  selected,
  onClick,
  className
}) => {
  // MUST show visual selected state (border + background)
  // MUST be keyboard accessible (Enter/Space)
  // MUST show all session metadata
}
```

**Visual States**:
- Default: Gray border, hover effect
- Selected: Blue border, blue background tint
- Hover: Slightly elevated, cursor pointer
- Focus: Clear focus ring

---

### AgentSelector

**File**: `src/components/analysis/AgentSelector.tsx`

**Props**:
```typescript
interface AgentSelectorProps {
  selectedIds: Set<string>;
  onToggle: (agentId: string) => void;
  onSelectAll: () => void;
  onDeselectAll: () => void;
  className?: string;
}
```

**Behavior**:
- Fetches agents from agentStore on mount
- Displays as card grid with checkboxes
- Shows selection count
- "Select All" / "Deselect All" toggle

**Contract**:
```typescript
export const AgentSelector: React.FC<AgentSelectorProps> = ({
  selectedIds,
  onToggle,
  onSelectAll,
  onDeselectAll,
  className
}) => {
  // MUST call onToggle when agent is clicked
  // MUST show agent capabilities
  // MUST support keyboard checkbox interaction
  // MUST display selection count
}
```

### AgentCard

**File**: `src/components/AgentCard.tsx` (REUSE existing)

**Props**:
```typescript
interface AgentCardProps {
  agent: AgentMetadata;
  selected: boolean;
  onSelect: (agentId: string) => void;
  showCheckbox?: boolean;  // NEW: Enable checkbox mode
}
```

**Modifications Needed**:
- Add checkbox visual (currently just card selection)
- Keep existing hover/selected styles

---

## Action Component

### AnalyzeButton

**File**: `src/components/analysis/AnalyzeButton.tsx`

**Props**:
```typescript
interface AnalyzeButtonProps {
  disabled: boolean;
  loading: boolean;
  agentCount: number;
  onClick: () => void;
  className?: string;
}
```

**Behavior**:
- Prominent CTA between selection and results
- Dynamic text based on agent count
- Loading spinner during analysis
- Disabled state with tooltip

**Text States**:
- No agents: "Select agents to analyze"
- 1 agent: "Analyze with Fuel Coach"
- Multiple: "Analyze with 3 agents"
- Loading: "Analyzing..." + spinner

**Contract**:
```typescript
export const AnalyzeButton: React.FC<AnalyzeButtonProps> = ({
  disabled,
  loading,
  agentCount,
  onClick,
  className
}) => {
  // MUST show dynamic text based on agentCount
  // MUST show loading state during analysis
  // MUST be disabled when selections incomplete
  // MUST show tooltip explaining why disabled
}
```

---

## Results Components

### ResultsDisplay

**File**: `src/components/analysis/ResultsDisplay.tsx`

**Props**:
```typescript
interface ResultsDisplayProps {
  results: AgentAnalysisResult[];
  loading: boolean;
  error: string | null;
  onClearResults: () => void;
  className?: string;
}
```

**Behavior**:
- Collapsible sections per agent
- Empty state when no results
- Error state with retry
- Export/share actions (future)

**Contract**:
```typescript
export const ResultsDisplay: React.FC<ResultsDisplayProps> = ({
  results,
  loading,
  error,
  onClearResults,
  className
}) => {
  // MUST show empty state when results.length === 0
  // MUST handle partial failures (some agents succeeded)
  // MUST allow expanding/collapsing per agent
  // MUST auto-scroll to results on mount
}
```

### AgentResultCard

**File**: `src/components/analysis/AgentResultCard.tsx`

**Props**:
```typescript
interface AgentResultCardProps {
  result: AgentAnalysisResult;
  defaultExpanded?: boolean;
  onToggle?: (expanded: boolean) => void;
}
```

**Sections**:
1. Header: Agent name, score, timestamp
2. Summary: Key highlights
3. Insights: Grouped by type (critical, warning, positive)
4. Recommendations: Sorted by priority
5. Actions: Export, retry (if failed)

**Contract**:
```typescript
export const AgentResultCard: React.FC<AgentResultCardProps> = ({
  result,
  defaultExpanded = false,
  onToggle
}) => {
  // MUST show error state if result.error exists
  // MUST group insights by type
  // MUST sort recommendations by priority
  // MUST show score if available
}
```

---

## Utility Components

### CollapsibleSection

**File**: `src/components/analysis/CollapsibleSection.tsx`

**Props**:
```typescript
interface CollapsibleSectionProps {
  title: string;
  subtitle?: string;
  icon?: React.ReactNode;
  defaultExpanded?: boolean;
  children: React.ReactNode;
  className?: string;
}
```

**Contract**:
```typescript
export const CollapsibleSection: React.FC<CollapsibleSectionProps> = ({
  title,
  subtitle,
  icon,
  defaultExpanded = false,
  children,
  className
}) => {
  // MUST animate expand/collapse
  // MUST show chevron icon (up/down)
  // MUST be keyboard accessible
  // MUST announce state changes to screen readers
}
```

### LoadingSpinner

**File**: `src/components/LoadingSpinner.tsx` (use existing or create)

**Props**:
```typescript
interface LoadingSpinnerProps {
  size?: 'small' | 'medium' | 'large';
  text?: string;
}
```

### ErrorBanner

**File**: `src/components/ErrorBanner.tsx` (create)

**Props**:
```typescript
interface ErrorBannerProps {
  message: string;
  onRetry?: () => void;
  onDismiss?: () => void;
}
```

---

## Event Flow

### Selection Flow

```
User clicks SessionCard
  → SessionCard.onClick()
  → SessionSelector.onSelect(sessionId)
  → Store.setSelectedSession(sessionId)
  → Re-render with new selection
```

```
User clicks AgentCard checkbox
  → AgentCard.onSelect(agentId)
  → AgentSelector.onToggle(agentId)
  → Store.toggleAgent(agentId)
  → Re-render with new selection
```

### Analysis Flow

```
User clicks AnalyzeButton
  → AnalyzeButton.onClick()
  → Store.analyze()
  → API call via analysisService
  → Store updates results
  → ResultsDisplay re-renders with data
  → Auto-scroll to results
```

### Error Flow

```
API call fails
  → Store.error = "error message"
  → ErrorBanner displays
  → User clicks retry
  → ErrorBanner.onRetry()
  → Store.analyze() again
```

---

## Accessibility Contracts

### Keyboard Navigation

All interactive components MUST support:
- **Tab**: Focus navigation
- **Enter/Space**: Activate/select
- **Escape**: Close/cancel
- **Arrow keys**: Navigate within grids (optional enhancement)

### Screen Reader

All components MUST:
- Have proper ARIA labels
- Announce state changes
- Describe interactive elements
- Support ARIA live regions for async updates

### Focus Management

- Focus MUST be visible (2px outline)
- Focus MUST move logically
- Modals MUST trap focus
- Analyze button MUST be reachable

---

## Error Handling Contracts

### Component-Level Errors

Each component MUST:
- Catch render errors with ErrorBoundary
- Display user-friendly error messages
- Provide retry mechanisms
- Log errors to console

### Data Fetch Errors

Components that fetch data MUST:
- Show loading state during fetch
- Show error state on failure
- Provide retry button
- Not block other components

### Partial Failures

When some agents fail:
- Show successful results
- Mark failed agents clearly
- Allow retry for failed agents only
- Explain what went wrong

---

## Testing Contracts

### Unit Test Coverage

Each component MUST have tests for:
- Default render
- All prop variations
- User interactions (click, keyboard)
- Loading states
- Error states
- Accessibility (ARIA, keyboard)

### Integration Tests

Full flow MUST be tested:
1. Select session
2. Select agents
3. Click analyze
4. View results
5. Handle errors

---

## Performance Contracts

### Rendering

Components MUST:
- Use React.memo for cards
- Avoid unnecessary re-renders
- Use keys for lists
- Lazy load heavy content

### Data Fetching

Components MUST:
- Cache fetched data
- Debounce rapid selections
- Cancel in-flight requests on unmount
- Handle race conditions

---

**Contracts complete. Ready for implementation.**
