# Data Model: Sub-Agent Integration

**Feature**: 008-subagent-integration | **Date**: 2025-11-25 | **Phase**: 1 (Design)

## 1. Core Entities

### 1.1 Agent Metadata

Describes the static properties of an agent.

```rust
/// Metadata about a registered agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Unique identifier (e.g., "fuel-coach")
    pub id: String,

    /// Display name (e.g., "Fuel Efficiency Coach")
    pub name: String,

    /// Agent domain
    pub domain: AgentDomain,

    /// Brief description of agent's purpose
    pub description: String,

    /// Preferred model tier for this agent
    pub model_preference: ModelPreference,

    /// Privacy level (local-only, cloud-preferred, etc.)
    pub privacy_level: PrivacyLevel,

    /// Icon emoji for UI
    pub icon: String,

    /// Version of this agent implementation
    pub version: String,
}
```

**Example:**
```rust
AgentMetadata {
    id: "fuel-coach".into(),
    name: "Fuel Efficiency Coach".into(),
    domain: AgentDomain::FuelEfficiency,
    description: "Analyzes driving patterns to identify fuel-wasting behaviors".into(),
    model_preference: ModelPreference::Lightweight,
    privacy_level: PrivacyLevel::CloudPreferred,
    icon: "🛢️".into(),
    version: "1.0.0".into(),
}
```

---

### 1.2 Agent Domain

Categorizes agents by their area of expertise.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentDomain {
    // Automotive agents
    FuelEfficiency,
    Safety,
    Diagnostics,
    EcoRoute,
    Performance,
    DriverProfile,

    // Development agents
    CodeArchitecture,
    CodeReview,
    TestGeneration,
    Documentation,
    PerformanceOptimization,
    DependencyManagement,

    // Custom domain
    Custom(String),
}
```

---

### 1.3 Model Preference

Indicates which model tier an agent prefers.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelPreference {
    /// Lightweight models (Haiku, Llama 3 8B)
    Lightweight,

    /// Medium models (Sonnet, Mistral 7B)
    Medium,

    /// Powerful models (Opus, Claude Sonnet 4)
    Powerful,
}
```

**Cost Implications:**
- Lightweight: ~$0.25 per 1M tokens
- Medium: ~$3.00 per 1M tokens
- Powerful: ~$15.00 per 1M tokens

---

### 1.4 Privacy Level

Determines where an agent can execute.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrivacyLevel {
    /// Must run locally, never send data to cloud (e.g., Driver Profile, Code Reviewer)
    LocalOnly,

    /// Prefer cloud if available, fallback to local
    CloudPreferred,

    /// Require cloud (complex reasoning), fail if unavailable
    CloudRequired,
}
```

---

### 1.5 Agent Configuration

User-configurable settings for each agent.

```rust
/// Agent-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct AgentConfig {
    /// Whether this agent is enabled
    pub enabled: bool,

    /// Verbosity level for output
    pub verbosity: VerbosityLevel,

    /// Agent-specific thresholds (e.g., "rpm_warning": 6000.0)
    pub thresholds: HashMap<String, f32>,

    /// Language for output (future: i18n support)
    pub language: String,

    /// Custom settings (agent-specific)
    pub custom: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq)]
pub enum VerbosityLevel {
    /// Brief, actionable summary
    Concise,

    /// Moderate detail, explanations provided
    Detailed,

    /// In-depth analysis, technical terminology
    Expert,
}
```

**Example:**
```rust
AgentConfig {
    enabled: true,
    verbosity: VerbosityLevel::Detailed,
    thresholds: [
        ("rpm_warning".into(), 6000.0),
        ("fuel_efficiency_target".into(), 30.0),
    ].into_iter().collect(),
    language: "en".into(),
    custom: HashMap::new(),
}
```

---

### 1.6 Analysis Request

Input data sent to an agent for analysis.

```rust
/// Request for agent analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    /// Unique request ID
    pub request_id: String,

    /// Session data to analyze
    pub session_data: SessionSummary,

    /// Vehicle metadata
    pub vehicle_info: VehicleInfo,

    /// Optional user preferences
    pub user_preferences: Option<UserPreferences>,

    /// Optional historical context (previous sessions)
    pub historical_context: Option<Vec<PreviousSession>>,

    /// Force local execution (privacy override)
    pub force_local: bool,

    /// Timeout for analysis (seconds)
    pub timeout_seconds: u64,
}
```

**Field Descriptions:**
- `session_data`: Aggregated telemetry from a single driving session
- `vehicle_info`: Make, model, year, engine type, etc.
- `user_preferences`: Driving goals (fuel efficiency, performance, etc.)
- `historical_context`: Previous session summaries for trend analysis
- `force_local`: Override cloud preference, force local execution
- `timeout_seconds`: Max time agent can take (default: 10s)

---

### 1.7 Session Summary

Aggregated session data (from feature 006-data-recording).

```rust
/// Aggregated session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_seconds: u64,

    // Distance and fuel
    pub distance_miles: f32,
    pub fuel_consumed_gallons: f32,
    pub avg_fuel_efficiency_mpg: f32,

    // Speed metrics
    pub avg_speed_mph: f32,
    pub max_speed_mph: f32,
    pub speed_violations_count: u32,

    // Engine metrics
    pub avg_rpm: f32,
    pub max_rpm: f32,
    pub high_rpm_duration_seconds: u64, // Time above threshold

    // Temperature metrics
    pub avg_coolant_temp_f: f32,
    pub max_coolant_temp_f: f32,
    pub avg_oil_temp_f: f32,

    // Events
    pub hard_braking_events: Vec<Event>,
    pub rapid_acceleration_events: Vec<Event>,
    pub error_codes: Vec<ErrorCode>,

    // Sensor readings (for diagnostics)
    pub sensor_readings: HashMap<String, Vec<SensorReading>>,
}
```

---

### 1.8 Vehicle Info

Metadata about the vehicle being analyzed.

```rust
/// Vehicle metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleInfo {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub engine_type: EngineType,
    pub transmission: TransmissionType,
    pub fuel_type: FuelType,

    // Optional VIN (privacy-sensitive, not sent to cloud by default)
    pub vin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineType {
    GasolineNaturallyAspirated,
    GasolineTurbocharged,
    Diesel,
    Hybrid,
    Electric,
    PluginHybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransmissionType {
    Manual,
    Automatic,
    CVT,
    DualClutch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuelType {
    Regular87,
    Midgrade89,
    Premium91Plus,
    Diesel,
    Electric,
    E85,
}
```

---

### 1.9 Analysis Result

Output from an agent analysis.

```rust
/// Result of agent analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Agent that produced this result
    pub agent_id: String,

    /// When this analysis was performed
    pub timestamp: DateTime<Utc>,

    /// Overall score (0-100, higher is better)
    pub score: Option<f32>,

    /// Confidence in this analysis (0.0-1.0)
    pub confidence: f32,

    /// Insights discovered by agent
    pub insights: Vec<Insight>,

    /// Actionable recommendations
    pub recommendations: Vec<Recommendation>,

    /// Execution time (milliseconds)
    pub execution_time_ms: u64,

    /// Model used for analysis
    pub model_used: String,

    /// Metadata (for debugging, caching, etc.)
    pub metadata: ResultMetadata,
}
```

---

### 1.10 Insight

A single observation or finding from an agent.

```rust
/// Analytical observation from agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    /// Severity category
    pub category: InsightCategory,

    /// Short title (e.g., "High RPM cruising detected")
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Supporting data (quantitative evidence)
    pub supporting_data: HashMap<String, serde_json::Value>,

    /// Related insights (for correlation)
    pub related_insight_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InsightCategory {
    /// Informational observation
    Info,

    /// Potential issue, not critical
    Warning,

    /// Serious issue requiring attention
    Critical,

    /// Positive behavior observed
    Positive,
}
```

**Example:**
```rust
Insight {
    category: InsightCategory::Warning,
    title: "High RPM cruising detected".into(),
    description: "Average RPM of 3200 is 20% higher than optimal for highway speeds. This increases fuel consumption significantly.".into(),
    supporting_data: [
        ("avg_rpm".into(), json!(3200)),
        ("optimal_rpm".into(), json!(2500)),
        ("excess_percentage".into(), json!(28)),
    ].into_iter().collect(),
    related_insight_ids: vec!["fuel-efficiency-001".into()],
}
```

---

### 1.11 Recommendation

Actionable advice from an agent.

```rust
/// Actionable recommendation from agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Importance level
    pub priority: Priority,

    /// Action to take
    pub action: String,

    /// Expected benefit from taking this action
    pub expected_benefit: String,

    /// Effort required to implement
    pub effort_level: EffortLevel,

    /// Related insight that led to this recommendation
    pub related_insight_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EffortLevel {
    /// Easy to implement, immediate change
    Easy,

    /// Requires some adjustment or practice
    Moderate,

    /// Significant effort or cost required
    Significant,
}
```

**Example:**
```rust
Recommendation {
    priority: Priority::High,
    action: "Shift to higher gear earlier (below 2500 RPM)".into(),
    expected_benefit: "8-12% fuel savings (~$150/year based on 12,000 mi/year)".into(),
    effort_level: EffortLevel::Easy,
    related_insight_id: Some("fuel-efficiency-001".into()),
}
```

---

### 1.12 Result Metadata

Additional metadata for caching, debugging, and tracing.

```rust
/// Metadata about analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMetadata {
    /// Whether this result came from cache
    pub from_cache: bool,

    /// Cache key used (for debugging)
    pub cache_key: Option<String>,

    /// Tokens consumed (for cost tracking)
    pub tokens_consumed: Option<TokenUsage>,

    /// Provider used (OpenAI, Anthropic, Ollama)
    pub provider: String,

    /// Whether this was executed locally or in cloud
    pub execution_location: ExecutionLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionLocation {
    Local,
    Cloud,
}
```

---

### 1.13 Event (from existing schema)

Represents a driving event (hard braking, acceleration, etc.).

```rust
/// Driving event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub severity: f32, // 0.0-1.0
    pub value: f32,    // e.g., deceleration in G's
    pub location: Option<GpsCoordinate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    HardBraking,
    RapidAcceleration,
    SharpTurn,
    SpeedViolation,
    IdlingExcessive,
}
```

---

### 1.14 Error Code (from existing schema)

OBD-II error code.

```rust
/// OBD-II diagnostic trouble code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCode {
    pub code: String, // e.g., "P0420"
    pub description: String,
    pub severity: ErrorSeverity,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorSeverity {
    Informational,
    Warning,
    Critical,
}
```

---

### 1.15 Sensor Reading (from existing schema)

Time-series sensor data.

```rust
/// Sensor reading at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub timestamp: DateTime<Utc>,
    pub sensor_id: String,
    pub value: f32,
    pub unit: String,
}
```

---

## 2. Trait Definitions

### 2.1 AnalysisAgent Trait

The core abstraction for all agents.

```rust
use async_trait::async_trait;

/// Core trait that all agents must implement
#[async_trait]
pub trait AnalysisAgent: Send + Sync {
    /// Perform analysis on the given request
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError>;

    /// Get agent metadata
    fn metadata(&self) -> AgentMetadata;

    /// Get current configuration
    fn config(&self) -> &AgentConfig;

    /// Update configuration
    fn set_config(&mut self, config: AgentConfig) -> Result<(), AgentError>;

    /// Validate that this agent can handle the request
    fn can_handle(&self, request: &AnalysisRequest) -> bool {
        // Default implementation: all agents can handle all requests
        // Override for specialized agents
        true
    }

    /// Estimate token usage for this request (for cost prediction)
    fn estimate_tokens(&self, request: &AnalysisRequest) -> u32 {
        // Default: rough estimate based on session data size
        1000
    }
}
```

---

### 2.2 Agent Error

Error types for agent operations.

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Agent not found: {0}")]
    NotFound(String),

    #[error("Agent execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Failed to parse agent response: {0}")]
    ParseError(String),

    #[error("AI provider error: {0}")]
    ProviderError(String),

    #[error("Request timeout after {0} seconds")]
    Timeout(u64),

    #[error("No local provider available (required for LocalOnly agent)")]
    NoLocalProviderAvailable,

    #[error("API key not found for provider: {0}")]
    ApiKeyNotFound(String),

    #[error("Circuit breaker open (too many failures)")]
    CircuitOpen,

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Agent is disabled")]
    AgentDisabled,

    #[error("Internal error: {0}")]
    Internal(String),
}
```

---

## 3. Relationships

### 3.1 Entity Relationship Diagram

```
┌─────────────────┐
│ AgentMetadata   │
│ - id            │
│ - name          │
│ - domain        │◀────────────┐
│ - privacy       │             │ has
└─────────────────┘             │
                                │
┌─────────────────┐             │
│ AgentConfig     │─────────────┘
│ - enabled       │ 1:1
│ - verbosity     │
│ - thresholds    │
└─────────────────┘

┌─────────────────┐
│ AnalysisRequest │
│ - request_id    │
│ - session_data  │──────┐
│ - vehicle_info  │      │ contains
└─────────────────┘      │
                         │
┌─────────────────┐      │
│ SessionSummary  │◀─────┘
│ - session_id    │ 1:1
│ - metrics       │
│ - events        │
└─────────────────┘

┌─────────────────┐
│ AnalysisResult  │
│ - agent_id      │────────┐
│ - score         │        │ produces
│ - confidence    │        │
└─────────────────┘        │
        │                  │
        │ contains 1:N     │
        ▼                  │
┌─────────────────┐        │
│ Insight         │        │
│ - category      │        │
│ - title         │        │
│ - description   │        │
└─────────────────┘        │
                           │
┌─────────────────┐        │
│ Recommendation  │        │
│ - priority      │◀───────┘
│ - action        │ 1:N
│ - benefit       │
└─────────────────┘
```

---

## 4. State Transitions

### 4.1 Agent Lifecycle

```
Unregistered
    │
    │ register()
    ▼
Registered ──────┐
    │            │ disable()
    │ enable()   │
    ▼            │
Enabled ◀────────┘
    │
    │ analyze()
    ▼
Analyzing
    │
    ├─ success ──▶ Results Ready
    │
    └─ failure ──▶ Error State
                      │
                      │ retry()
                      └──▶ Analyzing
```

### 4.2 Analysis Request Lifecycle

```
Created
    │
    │ dispatch()
    ▼
Dispatched ──────┐
    │            │ timeout
    │            │
    ▼            ▼
Processing    Timeout
    │            (error)
    ├─ success
    ▼
Completed
    │
    └─ cache() ──▶ Cached
```

---

## 5. Data Flow

### 5.1 Single Agent Analysis

```
1. User triggers analysis
   │
   ▼
2. Orchestrator receives AnalysisRequest
   │
   ▼
3. Orchestrator looks up agent by ID
   │
   ▼
4. Check cache (session_id + agent_id + config_hash)
   │
   ├─ Cache hit ──▶ Return cached result
   │
   └─ Cache miss ──▶ Continue
                      │
                      ▼
5. Agent builds prompt from SessionSummary
   │
   ▼
6. Agent calls AI provider (OpenAI/Anthropic/Ollama)
   │
   ▼
7. Parse JSON response into AnalysisResult
   │
   ▼
8. Store result in cache
   │
   ▼
9. Return AnalysisResult to orchestrator
   │
   ▼
10. Orchestrator returns to frontend
```

### 5.2 Multi-Agent Analysis (Parallel)

```
1. User triggers analysis with multiple agent IDs
   │
   ▼
2. Orchestrator spawns N async tasks (tokio::spawn)
   │
   ├─▶ Task 1: Fuel Coach ──▶ Result 1
   │
   ├─▶ Task 2: Safety ────────▶ Result 2
   │
   └─▶ Task 3: Diagnostic ────▶ Result 3
   │
   │ (all run in parallel)
   │
   ▼
3. await all tasks (futures::join_all)
   │
   ▼
4. Aggregate results
   │
   ▼
5. Return Vec<AnalysisResult>
```

---

## 6. Data Validation

### 6.1 Input Validation

```rust
impl AnalysisRequest {
    pub fn validate(&self) -> Result<(), AgentError> {
        // Validate session data exists
        if self.session_data.session_id.is_empty() {
            return Err(AgentError::ConfigError("Session ID required".into()));
        }

        // Validate timeout is reasonable
        if self.timeout_seconds == 0 || self.timeout_seconds > 300 {
            return Err(AgentError::ConfigError("Timeout must be 1-300 seconds".into()));
        }

        Ok(())
    }
}
```

### 6.2 Output Validation

```rust
impl AnalysisResult {
    pub fn validate(&self) -> Result<(), AgentError> {
        // Validate score is in range
        if let Some(score) = self.score {
            if !(0.0..=100.0).contains(&score) {
                return Err(AgentError::ParseError(
                    format!("Score {} out of range 0-100", score)
                ));
            }
        }

        // Validate confidence is in range
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(AgentError::ParseError(
                format!("Confidence {} out of range 0.0-1.0", self.confidence)
            ));
        }

        // Validate at least one insight or recommendation
        if self.insights.is_empty() && self.recommendations.is_empty() {
            return Err(AgentError::ParseError(
                "Result must contain at least one insight or recommendation".into()
            ));
        }

        Ok(())
    }
}
```

---

## 7. Storage and Persistence

### 7.1 Agent Registry (In-Memory)

```rust
use std::collections::HashMap;
use std::sync::Arc;

pub struct AgentRegistry {
    agents: HashMap<String, Arc<dyn AnalysisAgent>>,
}

impl AgentRegistry {
    pub fn register(&mut self, agent: Arc<dyn AnalysisAgent>) {
        let metadata = agent.metadata();
        self.agents.insert(metadata.id.clone(), agent);
    }

    pub fn get(&self, agent_id: &str) -> Option<Arc<dyn AnalysisAgent>> {
        self.agents.get(agent_id).cloned()
    }

    pub fn list(&self) -> Vec<AgentMetadata> {
        self.agents.values()
            .map(|agent| agent.metadata())
            .collect()
    }
}
```

### 7.2 Agent Configuration (localStorage - Frontend)

```typescript
// src/store/agentStore.ts
interface AgentConfigStore {
  configs: Record<string, AgentConfig>;
  saveConfig: (agentId: string, config: AgentConfig) => void;
  loadConfig: (agentId: string) => AgentConfig | null;
}

export const useAgentStore = create<AgentConfigStore>((set, get) => ({
  configs: loadFromLocalStorage(),

  saveConfig: (agentId, config) => {
    set((state) => ({
      configs: { ...state.configs, [agentId]: config },
    }));
    saveToLocalStorage(get().configs);
  },

  loadConfig: (agentId) => {
    return get().configs[agentId] || null;
  },
}));
```

### 7.3 Analysis Results (Session Storage - Frontend)

```typescript
// Cache results in session storage (cleared on tab close)
const cacheKey = `analysis:${sessionId}:${agentId}`;
sessionStorage.setItem(cacheKey, JSON.stringify(result));
```

### 7.4 Historical Agent Insights (CSV - Optional)

Reuse existing CSV recording infrastructure (feature 006-data-recording):

```csv
timestamp,session_id,agent_id,score,category,insight_title
2025-11-25T10:30:00Z,sess-001,fuel-coach,72,Warning,"High RPM cruising"
2025-11-25T10:30:01Z,sess-001,safety,85,Info,"Good following distance"
```

---

## 8. Indexing and Lookup

### 8.1 Agent Registry Index

```rust
// Primary index: agent_id -> Agent
HashMap<String, Arc<dyn AnalysisAgent>>

// Secondary index: domain -> Vec<agent_id>
HashMap<AgentDomain, Vec<String>>

// Example: Get all automotive agents
registry.get_by_domain(AgentDomain::FuelEfficiency)
```

### 8.2 Cache Index

```rust
// Cache key: "{session_id}:{agent_id}:{config_hash}"
// Example: "sess-001:fuel-coach:a3f2d1"

pub fn generate_cache_key(
    session_id: &str,
    agent_id: &str,
    config: &AgentConfig,
) -> String {
    let config_hash = hash_config(config);
    format!("{}:{}:{}", session_id, agent_id, config_hash)
}
```

---

## 9. Summary

### Entities Created
- `AgentMetadata`: Static agent properties
- `AgentConfig`: User configuration
- `AnalysisRequest`: Input to agent
- `AnalysisResult`: Output from agent
- `Insight`: Observation from analysis
- `Recommendation`: Actionable advice
- `AgentError`: Error types

### Enums Created
- `AgentDomain`: Agent specialization area
- `ModelPreference`: Model tier selection
- `PrivacyLevel`: Execution location constraint
- `VerbosityLevel`: Output detail level
- `InsightCategory`: Insight severity
- `Priority`: Recommendation importance
- `EffortLevel`: Implementation difficulty
- `ExecutionLocation`: Local vs Cloud

### Traits Created
- `AnalysisAgent`: Core agent interface

### Relationships
- Agent 1:1 AgentConfig
- AnalysisRequest 1:1 SessionSummary
- AnalysisResult 1:N Insights
- AnalysisResult 1:N Recommendations

### State Transitions
- Agent: Unregistered → Registered → Enabled → Analyzing → Results Ready
- Request: Created → Dispatched → Processing → Completed/Cached

---

## Next Steps

1. ✅ Data model defined
2. ⏭️ Generate agent contracts in `/contracts/` (Phase 1)
3. ⏭️ Generate `quickstart.md` (Phase 1)
4. ⏭️ Run `/speckit.tasks` to generate task breakdown (Phase 2)
