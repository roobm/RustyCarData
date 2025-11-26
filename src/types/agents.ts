// Agent Framework Types (mirrors Rust types)

export type AgentDomain =
  | "FuelEfficiency"
  | "Safety"
  | "Diagnostics"
  | "EcoRoute"
  | "Performance"
  | "DriverProfile"
  | "CodeArchitecture"
  | "CodeReview"
  | "TestGeneration"
  | "Documentation"
  | "PerformanceOptimization"
  | "DependencyManagement"
  | { Custom: string };

export type ModelPreference = "Lightweight" | "Medium" | "Powerful";

export type PrivacyLevel = "LocalOnly" | "CloudPreferred" | "CloudRequired";

export type VerbosityLevel = "Concise" | "Detailed" | "Expert";

export type InsightCategory = "Info" | "Warning" | "Critical" | "Positive";

export type Priority = "Low" | "Medium" | "High";

export type EffortLevel = "Easy" | "Moderate" | "Significant";

export type ExecutionLocation = "Local" | "Cloud" | "Hybrid";

export interface AgentMetadata {
  id: string;
  name: string;
  domain: AgentDomain;
  description: string;
  model_preference: ModelPreference;
  privacy_level: PrivacyLevel;
  icon: string;
  version: string;
}

export interface Insight {
  category: InsightCategory;
  title: string;
  description: string;
  supporting_data: Record<string, any>;
  related_insight_ids: string[];
}

export interface Recommendation {
  priority: Priority;
  action: string;
  expected_benefit: string;
  effort_level: EffortLevel;
  related_insight_id: string | null;
}

export interface ResultMetadata {
  from_cache: boolean;
  cache_key: string | null;
  tokens_consumed: TokenUsage | null;
  provider: string;
  execution_location: ExecutionLocation;
}

export interface TokenUsage {
  input_tokens: number;
  output_tokens: number;
  total_tokens: number;
}

export interface AnalysisResult {
  agent_id: string;
  timestamp: string;
  score: number | null;
  confidence: number;
  insights: Insight[];
  recommendations: Recommendation[];
  execution_time_ms: number;
  model_used: string;
  metadata: ResultMetadata;
}
