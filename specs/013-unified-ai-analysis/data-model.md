# Data Model: Unified AI Analysis

## Overview

This document defines the data structures, types, and state management for the unified AI analysis interface.

## Type Definitions

### Session Types

```typescript
// types/analysis.ts

/**
 * Session metadata from Supabase
 */
export interface Session {
  id: string;
  session_name: string | null;
  started_at: string;  // ISO 8601 timestamp
  ended_at: string | null;
  duration_seconds: number | null;
  user_id: string | null;
}

/**
 * Session data transformed for UI display
 */
export interface SessionCard {
  id: string;
  name: string;  // Fallback to "Unnamed Session" if null
  date: string;  // Formatted: "Nov 28, 2025"
  time: string;  // Formatted: "10:30 AM"
  duration: string;  // Formatted: "15m 30s"
  dataPoints: number;  // Count of telemetry points
}
```

### Agent Types

```typescript
/**
 * Agent metadata from backend
 */
export interface AgentMetadata {
  id: string;
  name: string;
  description: string;
  capabilities: string[];
  version: string;
}

/**
 * Agent with selection state
 */
export interface SelectableAgent extends AgentMetadata {
  selected: boolean;
}
```

### Analysis Types

```typescript
/**
 * Analysis request
 */
export interface AnalysisRequest {
  sessionId: string;
  agentIds: string[];
}

/**
 * Individual agent analysis result
 */
export interface AgentAnalysisResult {
  agentId: string;
  agentName: string;
  insights: Insight[];
  recommendations: Recommendation[];
  score?: number;  // 0-100, optional
  metrics?: Record<string, number>;  // Agent-specific metrics
  analyzedAt: string;  // ISO 8601 timestamp
  error?: string;  // If analysis failed
}

/**
 * Insight from agent analysis
 */
export interface Insight {
  id: string;
  type: 'positive' | 'neutral' | 'warning' | 'critical';
  title: string;
  description: string;
  timestamp?: string;  // Related to specific time in session
}

/**
 * Recommendation from agent
 */
export interface Recommendation {
  id: string;
  priority: 'high' | 'medium' | 'low';
  category: string;  // e.g., "fuel-efficiency", "safety", "maintenance"
  action: string;
  rationale: string;
  estimatedImpact?: string;  // e.g., "10% fuel savings"
}

/**
 * Complete analysis result
 */
export interface AnalysisResult {
  sessionId: string;
  agentResults: AgentAnalysisResult[];
  completedAt: string;
}
```

## State Management

### Unified Analysis Store

```typescript
// store/analysisStore.ts

import { create } from 'zustand';

interface AnalysisState {
  // ========== Selection State ==========
  selectedSessionId: string | null;
  selectedAgentIds: Set<string>;

  // ========== Analysis State ==========
  isAnalyzing: boolean;
  analysisProgress: number;  // 0-100, for future progress tracking

  // ========== Results State ==========
  // Results keyed by sessionId
  resultsCache: Record<string, AnalysisResult>;
  currentResults: AgentAnalysisResult[] | null;

  // ========== Error State ==========
  error: string | null;

  // ========== Actions ==========
  // Selection
  setSelectedSession: (sessionId: string | null) => void;
  toggleAgent: (agentId: string) => void;
  selectAllAgents: (agentIds: string[]) => void;
  deselectAllAgents: () => void;

  // Analysis
  analyze: () => Promise<void>;
  cancelAnalysis: () => void;

  // Results
  clearResults: (sessionId?: string) => void;
  getResultsForSession: (sessionId: string) => AgentAnalysisResult[] | null;

  // Utility
  reset: () => void;
}

export const useAnalysisStore = create<AnalysisState>((set, get) => ({
  // Initial state
  selectedSessionId: null,
  selectedAgentIds: new Set<string>(),
  isAnalyzing: false,
  analysisProgress: 0,
  resultsCache: {},
  currentResults: null,
  error: null,

  // Actions
  setSelectedSession: (sessionId) =>
    set({ selectedSessionId: sessionId, error: null }),

  toggleAgent: (agentId) =>
    set((state) => {
      const newSet = new Set(state.selectedAgentIds);
      if (newSet.has(agentId)) {
        newSet.delete(agentId);
      } else {
        newSet.add(agentId);
      }
      return { selectedAgentIds: newSet, error: null };
    }),

  selectAllAgents: (agentIds) =>
    set({ selectedAgentIds: new Set(agentIds) }),

  deselectAllAgents: () =>
    set({ selectedAgentIds: new Set<string>() }),

  analyze: async () => {
    const { selectedSessionId, selectedAgentIds } = get();

    if (!selectedSessionId || selectedAgentIds.size === 0) {
      set({ error: 'Please select a session and at least one agent' });
      return;
    }

    set({ isAnalyzing: true, error: null, analysisProgress: 0 });

    try {
      // Call agent service
      const results = await analyzeSession(
        selectedSessionId,
        Array.from(selectedAgentIds)
      );

      // Update cache and current results
      set((state) => ({
        resultsCache: {
          ...state.resultsCache,
          [selectedSessionId]: {
            sessionId: selectedSessionId,
            agentResults: results,
            completedAt: new Date().toISOString(),
          },
        },
        currentResults: results,
        isAnalyzing: false,
        analysisProgress: 100,
      }));
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : 'Analysis failed',
        isAnalyzing: false,
        analysisProgress: 0,
      });
    }
  },

  cancelAnalysis: () =>
    set({ isAnalyzing: false, analysisProgress: 0 }),

  clearResults: (sessionId) =>
    set((state) => {
      if (sessionId) {
        const newCache = { ...state.resultsCache };
        delete newCache[sessionId];
        return { resultsCache: newCache };
      } else {
        return { resultsCache: {}, currentResults: null };
      }
    }),

  getResultsForSession: (sessionId) => {
    const result = get().resultsCache[sessionId];
    return result?.agentResults || null;
  },

  reset: () =>
    set({
      selectedSessionId: null,
      selectedAgentIds: new Set<string>(),
      isAnalyzing: false,
      analysisProgress: 0,
      resultsCache: {},
      currentResults: null,
      error: null,
    }),
}));
```

### Agent Store Extension

```typescript
// store/agentStore.ts (existing, extend if needed)

// Keep existing agentStore for agent metadata
// analysisStore handles the unified workflow
```

## Data Transformations

### Session Transformation

```typescript
// utils/sessionTransform.ts

export function transformSessionToCard(session: Session): SessionCard {
  const startDate = new Date(session.started_at);

  return {
    id: session.id,
    name: session.session_name || 'Unnamed Session',
    date: startDate.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    }),
    time: startDate.toLocaleTimeString('en-US', {
      hour: 'numeric',
      minute: '2-digit',
    }),
    duration: formatDuration(session.duration_seconds),
    dataPoints: 0,  // TODO: Fetch from telemetry count
  };
}

function formatDuration(seconds: number | null): string {
  if (!seconds) return 'Unknown';

  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;

  if (mins === 0) return `${secs}s`;
  if (secs === 0) return `${mins}m`;
  return `${mins}m ${secs}s`;
}
```

### Result Transformation

```typescript
// utils/resultTransform.ts

/**
 * Group insights by type for display
 */
export function groupInsightsByType(insights: Insight[]) {
  return {
    critical: insights.filter(i => i.type === 'critical'),
    warning: insights.filter(i => i.type === 'warning'),
    positive: insights.filter(i => i.type === 'positive'),
    neutral: insights.filter(i => i.type === 'neutral'),
  };
}

/**
 * Sort recommendations by priority
 */
export function sortRecommendationsByPriority(recommendations: Recommendation[]) {
  const priorityOrder = { high: 0, medium: 1, low: 2 };
  return [...recommendations].sort((a, b) =>
    priorityOrder[a.priority] - priorityOrder[b.priority]
  );
}
```

## Validation

### Selection Validation

```typescript
// utils/validation.ts

export interface ValidationResult {
  valid: boolean;
  errors: string[];
}

export function validateAnalysisRequest(
  sessionId: string | null,
  agentIds: Set<string>
): ValidationResult {
  const errors: string[] = [];

  if (!sessionId) {
    errors.push('Please select a session to analyze');
  }

  if (agentIds.size === 0) {
    errors.push('Please select at least one AI agent');
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
```

## API Integration

### Service Layer

```typescript
// services/analysisService.ts

import { invoke } from '@tauri-apps/api/core';
import { AgentAnalysisResult } from '../types/analysis';

/**
 * Analyze a session with multiple agents
 */
export async function analyzeSession(
  sessionId: string,
  agentIds: string[]
): Promise<AgentAnalysisResult[]> {
  try {
    // Call Tauri command (existing backend)
    const results = await invoke<AgentAnalysisResult[]>(
      'analyze_session_with_agents',
      { sessionId, agentIds }
    );

    return results;
  } catch (error) {
    console.error('Analysis failed:', error);
    throw new Error(
      error instanceof Error
        ? error.message
        : 'Failed to analyze session'
    );
  }
}

/**
 * Get cached results for a session (if available)
 */
export async function getCachedResults(
  sessionId: string
): Promise<AgentAnalysisResult[] | null> {
  // For now, no backend caching
  // Results only cached in frontend Zustand store
  return null;
}
```

## Constraints & Invariants

### Invariants

1. **Single Session Selection**: Only one session can be selected at a time (v1)
2. **Non-empty Agent Selection**: At least one agent must be selected to analyze
3. **Results Keyed by Session**: Each session's results stored separately in cache
4. **No Duplicate Agents**: Agent IDs in selection are unique (Set enforces this)

### Constraints

1. **Session List**: Limited to 500 most recent sessions (pagination future)
2. **Agent Count**: 3-10 agents expected (UI designed for this range)
3. **Result Size**: Each agent result should be < 100KB (for performance)
4. **Cache Size**: Store max 20 analysis results (LRU eviction future)

## State Lifecycle

```
App Start
  ↓
Fetch Sessions (Supabase) → Display Cards
Fetch Agents (Backend) → Display Cards
  ↓
User Selects Session → Update selectedSessionId
  ↓
User Selects Agents → Update selectedAgentIds
  ↓
User Clicks "Analyze" → Validate → analyze()
  ↓
Analysis Running → isAnalyzing: true
  ↓
Results Received → Store in cache → Display
  ↓
User Changes Selection → Results persist in cache
```

## Error Scenarios

| Scenario | State Update | UI Feedback |
|----------|--------------|-------------|
| Session fetch fails | `error: "Failed to load sessions"` | Error banner with retry |
| Agent fetch fails | Via agentStore error | Error banner with retry |
| No session selected | `error: "Please select..."` | Disabled analyze button |
| No agent selected | `error: "Please select..."` | Disabled analyze button |
| Analysis times out | `error: "Analysis timed out"` | Retry button |
| Partial agent failure | Results with `error` field | Show successful, mark failed |
| Network error | `error: "Network error"` | Toast + retry |

## Future Enhancements

### Multi-Session Analysis

```typescript
// Future: Support multiple sessions
interface AnalysisState {
  selectedSessionIds: Set<string>;  // Multiple
  // ... rest same
}

// Results structure changes
interface ComparisonResult {
  sessions: string[];
  agentResults: AgentComparison[];
}
```

### Result Persistence

```typescript
// Future: Save to Supabase
interface SavedAnalysis {
  id: string;
  session_id: string;
  agent_ids: string[];
  results: AgentAnalysisResult[];
  created_at: string;
}
```

### Progress Tracking

```typescript
// Future: Track per-agent progress
interface AnalysisProgress {
  total: number;
  completed: number;
  currentAgent: string;
}
```

---

**Data model complete. Ready for contract definitions.**
