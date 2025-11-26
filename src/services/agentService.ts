import { invoke } from "@tauri-apps/api/core";
import type { AgentMetadata, AnalysisResult } from "../types/agents";

/**
 * Service for interacting with the agent framework via Tauri commands
 */
export class AgentService {
  /**
   * Get list of all registered agents
   */
  static async listAgents(): Promise<AgentMetadata[]> {
    return await invoke<AgentMetadata[]>("list_agents");
  }

  /**
   * Analyze a session with specific agents
   */
  static async analyzeSession(
    sessionId: string,
    agentIds: string[]
  ): Promise<AnalysisResult[]> {
    return await invoke<AnalysisResult[]>("analyze_session_with_agents", {
      sessionId,
      agentIds,
    });
  }

  /**
   * Get detailed information about a specific agent
   */
  static async getAgentInfo(agentId: string): Promise<AgentMetadata> {
    return await invoke<AgentMetadata>("get_agent_info", { agentId });
  }

  /**
   * Clear all cached analysis results
   */
  static async clearCache(): Promise<void> {
    await invoke("clear_agent_cache");
  }

  /**
   * Invalidate cache for a specific session
   */
  static async invalidateSessionCache(sessionId: string): Promise<void> {
    await invoke("invalidate_session_cache", { sessionId });
  }
}
