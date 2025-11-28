// Analysis service layer for Tauri IPC

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
  _sessionId: string
): Promise<AgentAnalysisResult[] | null> {
  // For now, no backend caching
  // Results only cached in frontend Zustand store
  return null;
}
