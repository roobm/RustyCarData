# Unified AI Analysis Interface - UX Specification

## Problem Statement

Currently, the application has two separate pages for AI-powered analysis:
1. **AI Analysis** (`/analysis`) - Shows session data in a table with AI assistant chat
2. **AI Agents** (`/agents`) - Shows available AI agents for selection

**User Pain Points:**
- Split workflow: Users must navigate between two pages
- No clear connection: Not obvious how agents relate to analysis
- Missing integration: Can't select which agent analyzes which session
- Redundant navigation: Extra clicks to perform analysis

**User's Request:**
> "i think ai analysis and ai agents should be on same site. Like you have option to choose which data are we analysing and with what agent"

## UX Analysis

### Current State

**AI Analysis Page Features:**
- Session selection from list
- CSV import functionality
- Data table view (shows telemetry points)
- AI chat assistant (generic OpenAI integration)

**AI Agents Page Features:**
- Grid of available agents (Fuel Coach, Safety Advisor, Diagnostic)
- Agent selection (single or multiple)
- Agent metadata display (name, description, capabilities)

**Missing Link:** No way to connect selected sessions with selected agents.

### Proposed Unified Experience

#### Core User Flow

1. **Select Session(s)**
   - User chooses which driving session(s) to analyze
   - Support single or multiple session selection
   - Clear visual indication of selected sessions

2. **Choose Agent(s)**
   - User picks specialized AI agents
   - Support single or multiple agent selection
   - Show agent capabilities and what they analyze

3. **Run Analysis**
   - Trigger analysis with selected session(s) + agent(s)
   - Show progress/loading state
   - Display results organized by agent

4. **View Results**
   - Structured presentation of insights
   - Compare results from different agents
   - Option to re-analyze or adjust selections

### UX Best Practices Research

#### Multi-Step Workflow Patterns

**Option A: Wizard/Stepper Pattern**
```
[Step 1: Select Data] → [Step 2: Choose Agents] → [Step 3: Review & Run] → [Step 4: Results]
```
**Pros:** Clear progression, beginner-friendly, validates each step
**Cons:** More clicks, feels rigid, hard to go back and adjust

**Option B: Split-Panel Pattern** ⭐ RECOMMENDED
```
┌─────────────┬──────────────┬─────────────┐
│  Sessions   │    Agents    │   Results   │
│   (left)    │   (center)   │   (right)   │
└─────────────┴──────────────┴─────────────┘
```
**Pros:** All options visible, flexible, quick adjustments, familiar (email/file browsers)
**Cons:** Needs larger screen, can feel crowded on smaller displays

**Option C: Modal/Overlay Pattern**
```
[Session List] → (Click "Analyze") → [Modal: Choose Agents] → [Results View]
```
**Pros:** Focused flow, works on smaller screens, clear action
**Cons:** Hides context, extra modal management

**Option D: Collapsible Sections** ⭐ RECOMMENDED
```
▼ 1. Select Session (dropdown or expandable list)
▼ 2. Choose Agents (grid, collapsible)
▼ 3. Results (expandable, per agent)
```
**Pros:** Progressive disclosure, mobile-friendly, clean, scannable
**Cons:** More vertical scrolling

### Recommended Solution: Hybrid Approach

**Layout: Collapsible Sections with Side-by-Side Selection**

```
┌─────────────────────────────────────────────────┐
│  🤖 AI Analysis                        [Settings]│
├─────────────────────────────────────────────────┤
│                                                  │
│  ▼ Select Driving Session                       │
│  ┌─────────────────────────────────────────┐   │
│  │ [Grid of Sessions with Preview Cards]   │   │
│  │ • Session Name                           │   │
│  │ • Date/Time                              │   │
│  │ • Duration                               │   │
│  │ • Data points                            │   │
│  └─────────────────────────────────────────┘   │
│                                                  │
│  ▼ Choose AI Agents (2 selected)                │
│  ┌──────────────────────────────────────────┐  │
│  │ [Grid of Agent Cards - checkboxes]       │  │
│  │ ☑ Fuel Coach - Optimize fuel efficiency  │  │
│  │ ☑ Safety Advisor - Identify risky behavior│ │
│  │ ☐ Diagnostics - Detect issues             │ │
│  └──────────────────────────────────────────┘  │
│                                                  │
│  [Analyze Session] button - prominent           │
│                                                  │
│  ▼ Analysis Results                              │
│  ┌──────────────────────────────────────────┐  │
│  │ Fuel Coach Results                        │  │
│  │ • Key insights                            │  │
│  │ • Recommendations                         │  │
│  │ • Score: 85/100                           │  │
│  ├──────────────────────────────────────────┤  │
│  │ Safety Advisor Results                    │  │
│  │ • Key insights                            │  │
│  │ • Recommendations                         │  │
│  │ • Score: 92/100                           │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

### Key UX Features

#### 1. Session Selection
- **Format**: Card grid (not table)
- **Info shown**: Session name, date, duration, data point count
- **Interaction**: Click card to select, visual highlight
- **Multi-select**: Checkbox mode if analyzing trends across multiple sessions
- **Quick actions**: Preview data, delete, export

#### 2. Agent Selection
- **Format**: Card grid with checkboxes
- **Info shown**: Agent name, icon, description, capabilities tags
- **Interaction**: Check to select, supports multiple
- **Visual feedback**: Selected state with color/border
- **Info**: Tooltip or expand for detailed capabilities

#### 3. Action Button
- **Placement**: Sticky/prominent between agents and results
- **States**:
  - Disabled if no session or no agent selected
  - Loading during analysis
  - Success with checkmark
- **Text**: "Analyze with [N] agents" (dynamic)

#### 4. Results Display
- **Organization**: One section per agent
- **Format**: Expandable/collapsible cards
- **Content**:
  - Summary at top
  - Detailed insights
  - Scores/metrics
  - Recommendations
  - Visualizations (charts if applicable)
- **Actions**: Export, share, re-run

### Progressive Disclosure Strategy

**First Load:**
- Sessions section expanded
- Agents section expanded
- Results section collapsed (empty)

**After Selection:**
- Keep all sections visible but compact
- Highlight "Analyze" button

**After Analysis:**
- Auto-scroll to results
- Expand results section
- Collapse sessions/agents (but keep visible headers)

### Mobile Considerations

**Responsive Breakpoints:**
- Desktop (>1024px): Full layout as described
- Tablet (768-1023px): Stack sections vertically, larger cards
- Mobile (<768px): Full vertical stack, single column cards

**Mobile-specific UX:**
- Fixed "Analyze" button at bottom
- Swipeable session cards
- Full-screen agent selection modal
- Simplified results view

### Accessibility

- **Keyboard Navigation**: Tab through selections, Enter to select
- **Screen Readers**: ARIA labels for all interactive elements
- **Focus Indicators**: Clear visual focus states
- **Color Contrast**: WCAG AA compliant
- **Loading States**: Announced to screen readers

### Error States

1. **No sessions available**: Show empty state with "Import CSV" CTA
2. **Agent fetch failed**: Retry button with error message
3. **Analysis failed**: Clear error, option to retry
4. **Partial failure**: Show which agents succeeded/failed

### Success Metrics

- **Primary**: Time from landing to viewing results
- **Secondary**: Number of analyses run per session
- **Tertiary**: Agent selection diversity

## Technical Considerations

### Data Flow

```typescript
interface UnifiedAnalysisState {
  selectedSessionId: string | null;
  selectedAgentIds: string[];
  analysisResults: Record<string, AgentAnalysisResult>;
  isAnalyzing: boolean;
  error: string | null;
}
```

### API Integration

```typescript
// Combined analysis call
async analyzeSession(sessionId: string, agentIds: string[]): Promise<AnalysisResults>

// Returns results keyed by agent ID
interface AnalysisResults {
  [agentId: string]: {
    insights: string[];
    recommendations: string[];
    score?: number;
    metrics?: Record<string, number>;
  }
}
```

### Component Structure

```
UnifiedAnalysisPage/
├── SessionSelector (card grid, selection state)
├── AgentSelector (card grid, multi-select)
├── AnalyzeButton (sticky, context-aware)
└── ResultsDisplay (collapsible sections)
    ├── AgentResultCard (per agent)
    └── ResultsEmpty (when no results)
```

## Implementation Priority

### Phase 1: Core Functionality
1. Session selector with cards (from Supabase sessions)
2. Agent selector with checkboxes (from agent registry)
3. Analyze button with loading state
4. Basic results display

### Phase 2: Enhanced UX
5. Collapsible sections
6. Visual improvements (animations, transitions)
7. Error handling and empty states
8. Export/share functionality

### Phase 3: Advanced Features
9. Multi-session comparison
10. Historical results caching
11. Agent recommendations based on session type
12. Real-time analysis streaming

## Migration Strategy

### Remove:
- Separate `/agents` route
- Separate `/analysis` route

### Create:
- New `/ai-analysis` route (or use `/analysis` as primary)
- Unified component integrating both

### Keep:
- Agent backend logic (no changes)
- Session data fetching (reuse existing)
- AI assistant component (can be integrated)

### Navigation:
- Update sidebar: Single "AI Analysis" link instead of two
- Remove "AI Agents" link
- Update icon to something unified (BrainCircuit + Bot combo?)

## Conclusion

**Recommended Approach:** Collapsible sections with card-based selection.

**Why?**
- ✅ Natural progressive disclosure
- ✅ Mobile-friendly
- ✅ Clear user flow
- ✅ Flexible (add/remove agents easily)
- ✅ Familiar pattern (common in modern web apps)
- ✅ Accessible
- ✅ Scalable (works with 1 or 20 agents)

**Next Steps:**
1. Get user approval on this spec
2. Create wireframes/mockups if needed
3. Implement Phase 1 (core functionality)
4. User testing and iteration
