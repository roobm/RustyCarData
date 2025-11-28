// Analysis state management with Zustand

import { create } from 'zustand';
import { AgentAnalysisResult, AnalysisResult } from '../types/analysis';
import { analyzeSession } from '../services/analysisService';

interface AnalysisState {
  // ========== Selection State ==========
  selectedSessionId: string | null;
  selectedAgentIds: Set<string>;

  // ========== Analysis State ==========
  isAnalyzing: boolean;
  analysisProgress: number; // 0-100, for future progress tracking

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
