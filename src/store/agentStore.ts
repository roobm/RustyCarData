import { create } from "zustand";
import type { AgentMetadata, AnalysisResult } from "../types/agents";
import { AgentService } from "../services/agentService";

interface AgentState {
  // Available agents
  agents: AgentMetadata[];
  loadingAgents: boolean;
  agentsError: string | null;

  // Analysis results
  analysisResults: Record<string, AnalysisResult[]>; // sessionId -> results
  analyzingSession: string | null;
  analysisError: string | null;

  // Actions
  fetchAgents: () => Promise<void>;
  analyzeSession: (sessionId: string, agentIds: string[]) => Promise<void>;
  clearCache: () => Promise<void>;
  invalidateSessionCache: (sessionId: string) => Promise<void>;
}

export const useAgentStore = create<AgentState>((set) => ({
  // Initial state
  agents: [],
  loadingAgents: false,
  agentsError: null,

  analysisResults: {},
  analyzingSession: null,
  analysisError: null,

  // Fetch all available agents
  fetchAgents: async () => {
    set({ loadingAgents: true, agentsError: null });
    try {
      const agents = await AgentService.listAgents();
      set({ agents, loadingAgents: false });
    } catch (error) {
      set({
        agentsError: error instanceof Error ? error.message : "Failed to load agents",
        loadingAgents: false,
      });
    }
  },

  // Analyze a session with selected agents
  analyzeSession: async (sessionId: string, agentIds: string[]) => {
    set({ analyzingSession: sessionId, analysisError: null });
    try {
      const results = await AgentService.analyzeSession(sessionId, agentIds);
      set((state) => ({
        analysisResults: {
          ...state.analysisResults,
          [sessionId]: results,
        },
        analyzingSession: null,
      }));
    } catch (error) {
      set({
        analysisError:
          error instanceof Error ? error.message : "Analysis failed",
        analyzingSession: null,
      });
    }
  },

  // Clear all cached analysis results
  clearCache: async () => {
    try {
      await AgentService.clearCache();
      // Optionally clear local results too
      set({ analysisResults: {} });
    } catch (error) {
      console.error("Failed to clear cache:", error);
    }
  },

  // Invalidate cache for a specific session
  invalidateSessionCache: async (sessionId: string) => {
    try {
      await AgentService.invalidateSessionCache(sessionId);
      // Remove from local state
      set((state) => {
        const newResults = { ...state.analysisResults };
        delete newResults[sessionId];
        return { analysisResults: newResults };
      });
    } catch (error) {
      console.error("Failed to invalidate session cache:", error);
    }
  },
}));
