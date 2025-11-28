// Type definitions for unified AI analysis interface

/**
 * Session metadata from Supabase
 */
export interface Session {
  id: string;
  session_name: string | null;
  started_at: string; // ISO 8601 timestamp
  ended_at: string | null;
  duration_seconds: number | null;
  user_id: string | null;
}

/**
 * Session data transformed for UI display
 */
export interface SessionCard {
  id: string;
  name: string; // Fallback to "Unnamed Session" if null
  date: string; // Formatted: "Nov 28, 2025"
  time: string; // Formatted: "10:30 AM"
  duration: string; // Formatted: "15m 30s"
  dataPoints: number; // Count of telemetry points
}

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

/**
 * Analysis request
 */
export interface AnalysisRequest {
  sessionId: string;
  agentIds: string[];
}

/**
 * Insight from agent analysis
 */
export interface Insight {
  id: string;
  type: 'positive' | 'neutral' | 'warning' | 'critical';
  title: string;
  description: string;
  timestamp?: string; // Related to specific time in session
}

/**
 * Recommendation from agent
 */
export interface Recommendation {
  id: string;
  priority: 'high' | 'medium' | 'low';
  category: string; // e.g., "fuel-efficiency", "safety", "maintenance"
  action: string;
  rationale: string;
  estimatedImpact?: string; // e.g., "10% fuel savings"
}

/**
 * Individual agent analysis result
 */
export interface AgentAnalysisResult {
  agentId: string;
  agentName: string;
  insights: Insight[];
  recommendations: Recommendation[];
  score?: number; // 0-100, optional
  metrics?: Record<string, number>; // Agent-specific metrics
  analyzedAt: string; // ISO 8601 timestamp
  error?: string; // If analysis failed
}

/**
 * Complete analysis result
 */
export interface AnalysisResult {
  sessionId: string;
  agentResults: AgentAnalysisResult[];
  completedAt: string;
}
