# Research: Sub-Agent Integration Technology Decisions

**Feature**: 008-subagent-integration | **Date**: 2025-11-25 | **Phase**: 0 (Research)

## Executive Summary

This document records technology choices, best practices, and architectural patterns for implementing a multi-agent AI system with 12 specialized agents (6 automotive, 6 development). Key decisions include parallel execution with Tokio, JSON-structured agent outputs, model selection per agent type, in-memory LRU caching, and privacy-first routing.

---

## 1. Agent Orchestration Patterns

### Decision: Parallel Execution with Tokio

**Options Evaluated:**
1. **Sequential Execution**: Simple but slow (3s × 3 agents = 9s)
2. **Parallel Execution with `tokio::spawn`**: Fast, idiomatic Rust
3. **Thread Pool**: Overkill for I/O-bound AI calls
4. **Actor Model (e.g., Actix)**: Over-engineered for this use case

**Chosen**: **Parallel Execution with `tokio::spawn`**

**Rationale:**
- AI provider calls are I/O-bound (HTTP requests), perfect for async
- Tokio is already used in the project (Tauri requirement)
- `tokio::spawn` allows true concurrent execution of independent agents
- Each agent can fail independently without blocking others

**Implementation Pattern:**
```rust
pub async fn analyze_session(
    &self,
    request: &AnalysisRequest,
    agent_ids: Vec<String>,
) -> Vec<Result<AnalysisResult, AgentError>> {
    let mut handles = vec![];

    for agent_id in agent_ids {
        let agent = self.registry.get(&agent_id)?;
        let req = request.clone();

        // Spawn independent task per agent
        let handle = tokio::spawn(async move {
            agent.analyze(&req).await
        });

        handles.push(handle);
    }

    // Wait for all agents in parallel
    let results = futures::future::join_all(handles).await;

    // Convert JoinHandles to Results
    results.into_iter()
        .map(|r| r.unwrap_or_else(|e| Err(AgentError::ExecutionFailed(e.to_string()))))
        .collect()
}
```

**Performance Target**: 3 agents in parallel should complete in ~3 seconds (vs 9s sequential)

---

## 2. Prompt Engineering for Specialized Agents

### Decision: Structured System Prompts + JSON Output Format

**Best Practices Identified:**

#### A. Role Definition (System Prompt)
```markdown
You are a [ROLE] with [EXPERTISE] specializing in [DOMAIN].

Your goal is to [PRIMARY OBJECTIVE].

Analyze the following data and provide:
1. [OUTPUT 1]
2. [OUTPUT 2]
3. [OUTPUT 3]

Guidelines:
- Be [TONE] (specific, encouraging, direct)
- Focus on [PRIORITY]
- Reference [STANDARDS] where applicable
```

**Example (Fuel Coach):**
```rust
const SYSTEM_PROMPT: &str = r#"
You are an expert Fuel Efficiency Coach specializing in automotive telemetry analysis.
Your goal is to help drivers reduce fuel consumption through actionable driving behavior changes.

Analyze the following driving session data and provide:
1. Fuel efficiency score (0-100, 100 = optimal)
2. 3-5 specific fuel-wasting behaviors observed
3. Actionable recommendations with estimated fuel savings
4. Comparison to optimal driving for this vehicle type

Be specific, quantitative, and encouraging. Focus on the top 3 highest-impact improvements.
"#;
```

#### B. Context Window Optimization

**Constraints:**
- Total prompt < 4000 tokens (support for smaller models)
- Session data summarization: Include only aggregates, not raw CAN frames
- Few-shot examples: 1-2 examples max (avoid bloating prompt)

**Token Budget Allocation:**
- System prompt: ~300 tokens
- Session data (JSON): ~500 tokens
- Vehicle metadata: ~100 tokens
- Few-shot examples: ~200 tokens
- **Total**: ~1100 tokens (leaves room for model response)

#### C. Output Format Standardization

**Decision**: **JSON-structured outputs for parsing, Markdown for display**

**Agent Response Format:**
```json
{
  "score": 72,
  "confidence": 0.85,
  "insights": [
    {
      "category": "Warning",
      "title": "High RPM cruising detected",
      "description": "Average RPM of 3200 is 20% higher than optimal for highway speeds. This increases fuel consumption significantly.",
      "supporting_data": {
        "avg_rpm": 3200,
        "optimal_rpm": 2500,
        "excess_percentage": 28
      }
    }
  ],
  "recommendations": [
    {
      "priority": "High",
      "action": "Shift to higher gear earlier (below 2500 RPM)",
      "expected_benefit": "8-12% fuel savings (~$150/year based on 12,000 mi/year)",
      "effort_level": "Easy"
    }
  ]
}
```

**Parsing Strategy:**
```rust
// Agent implementations parse JSON, handle malformed responses gracefully
let response_text = ai_provider.complete(&prompt).await?;

let result: AnalysisResult = serde_json::from_str(&response_text)
    .map_err(|e| AgentError::ParseError(format!("Invalid JSON: {}", e)))?;

// Frontend renders recommendations as Markdown
```

#### D. Few-Shot Examples for Automotive Domain

**When to Use:**
- Complex diagnostic reasoning (Diagnostic Agent)
- Domain-specific terminology (e.g., "understeer", "valve float")
- Specific output format enforcement

**Example (Diagnostic Agent):**
```rust
const FEW_SHOT_EXAMPLE: &str = r#"
Example Input:
{
  "coolant_temp": [85, 88, 92, 97, 103],
  "rpm": [2000, 2200, 2400, 2600, 2800],
  "engine_load": [45, 48, 52, 55, 58]
}

Example Output:
{
  "score": 65,
  "insights": [{
    "category": "Warning",
    "title": "Coolant temperature rising faster than RPM",
    "description": "Temperature increased 21% while RPM increased 40%. This suggests thermostat may be stuck partially closed or coolant flow restriction."
  }],
  "recommendations": [{
    "priority": "High",
    "action": "Inspect thermostat and coolant system within 500 miles",
    "expected_benefit": "Prevent overheating and potential engine damage"
  }]
}
"#;
```

---

## 3. Model Selection Strategy

### Decision: Model Mapping by Agent Complexity

**Model Tiers:**

| Tier | Models | Cost (per 1M tokens) | Use Case |
|------|--------|---------------------|----------|
| **Lightweight** | Haiku, Llama 3 8B | $0.25 | Simple pattern matching, rule-based advice |
| **Medium** | Sonnet, Mistral 7B | $3.00 | Multi-factor analysis, contextual reasoning |
| **Powerful** | Opus, Claude Sonnet 4 | $15.00 | Deep diagnostics, complex correlations |

**Agent-to-Model Mapping:**

| Agent | Model Tier | Rationale |
|-------|-----------|-----------|
| Fuel Efficiency Coach | **Lightweight** (Haiku) | Simple calculations (RPM, speed, fuel rate) |
| Safety Advisor | **Lightweight** (Haiku) | Count events, compare to thresholds |
| Diagnostic Technician | **Powerful** (Opus) | Complex sensor correlations, failure mode reasoning |
| Eco-Route Planner | **Medium** (Sonnet) | Route comparison, multi-variable optimization |
| Performance Analyst | **Medium** (Sonnet) | Physics-based reasoning (G-forces, lap times) |
| Driver Profile Learner | **Powerful** (Opus) | Deep pattern recognition over time |
| Code Reviewer | **Medium** (Sonnet) | Syntax + logic + constitution checks |
| Test Generator | **Medium** (Sonnet) | Code understanding + edge case generation |
| Documentation Writer | **Medium** (Sonnet) | Natural language generation with technical accuracy |
| Code Architect | **Powerful** (Opus) | System design, tradeoff analysis |
| Performance Optimizer | **Powerful** (Opus) | Algorithm complexity, profiling analysis |
| Dependency Manager | **Lightweight** (Haiku) | Version comparison, security advisory matching |

**Local Model Support:**
- Ollama with Llama 3.1 8B or Mistral 7B for lightweight/medium agents
- Cloud required for Opus-tier agents (too large to run locally on consumer hardware)

**Cost Estimation (per session with 6 automotive agents):**
- Fuel Coach (Haiku): ~1000 tokens × $0.25/1M = $0.00025
- Safety (Haiku): ~1000 tokens × $0.25/1M = $0.00025
- Diagnostic (Opus): ~2000 tokens × $15/1M = $0.03
- Eco-Route (Sonnet): ~1500 tokens × $3/1M = $0.0045
- Performance (Sonnet): ~1500 tokens × $3/1M = $0.0045
- Driver Profile (Opus): ~2500 tokens × $15/1M = $0.0375
- **Total per analysis**: ~$0.08 (acceptable for per-session analysis)

**Optimization Strategies:**
1. **Cache results** for same session (don't re-run if data unchanged)
2. **User preference**: Allow users to force local-only (lower quality, zero cost)
3. **Hybrid mode**: Run lightweight agents first, only invoke expensive agents if anomalies detected

---

## 4. Privacy-Preserving Agent Design

### Decision: Local-First with Privacy Flags

**Privacy Levels:**

```rust
pub enum PrivacyLevel {
    /// Must run locally, never send to cloud
    LocalOnly,
    /// Can run cloud if available, local fallback
    CloudPreferred,
    /// Cloud required (complex reasoning), fail if unavailable
    CloudRequired,
}
```

**Agent Privacy Classification:**

| Agent | Privacy Level | Rationale |
|-------|--------------|-----------|
| Fuel Coach | `CloudPreferred` | Session aggregates are not sensitive |
| Safety Advisor | `CloudPreferred` | Driving events are anonymized |
| Diagnostic | `CloudPreferred` | Sensor data is technical, not personal |
| Eco-Route Planner | `LocalOnly` | **Location data is sensitive** |
| Performance Analyst | `CloudPreferred` | Track data is not sensitive |
| Driver Profile | `LocalOnly` | **Behavioral patterns are personal** |
| Code Reviewer | `LocalOnly` | **Source code is proprietary** |
| Test Generator | `LocalOnly` | **Source code is proprietary** |
| Documentation Writer | `LocalOnly` | **Source code is proprietary** |
| Code Architect | `LocalOnly` | **Source code is proprietary** |
| Performance Optimizer | `LocalOnly` | **Source code is proprietary** |
| Dependency Manager | `CloudPreferred` | Dependency info is public |

**Enforcement Mechanism:**
```rust
impl AgentOrchestrator {
    async fn dispatch_agent(
        &self,
        agent: &dyn AnalysisAgent,
        request: &AnalysisRequest,
    ) -> Result<AnalysisResult, AgentError> {
        let metadata = agent.metadata();

        // Check privacy constraint
        if metadata.privacy_level == PrivacyLevel::LocalOnly {
            // Force local provider
            if !self.has_local_provider() {
                return Err(AgentError::NoLocalProviderAvailable);
            }
            // Override provider selection to local
            request.force_local = true;
        }

        agent.analyze(request).await
    }
}
```

**Data Minimization Techniques:**
1. **Aggregate before sending**: Send "average RPM: 3200" not raw CAN frames
2. **Anonymize identifiers**: Strip VIN, user ID from cloud requests
3. **Selective field inclusion**: Only send fields required by agent
4. **User transparency**: UI clearly shows which agents run locally vs cloud

**User Consent:**
- Default: All automotive agents run cloud (with aggregation)
- Opt-in required for: Raw telemetry data, location data, VIN
- Clear banner: "🌐 Cloud agents enabled (data sent to OpenAI) | 🔒 Switch to Local"

---

## 5. Agent Result Caching

### Decision: In-Memory LRU Cache with 100-Entry Limit

**Cache Key Generation:**
```rust
pub fn generate_cache_key(
    session_id: &str,
    agent_id: &str,
    config_hash: &str,
) -> String {
    format!("{}:{}:{}", session_id, agent_id, config_hash)
}

// Config hash includes agent settings that affect output
pub fn hash_config(config: &AgentConfig) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    config.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
```

**TTL Strategies:**

| Agent Type | TTL | Rationale |
|-----------|-----|-----------|
| Automotive (session-based) | **Until session changes** | Session data is immutable after recording |
| Development (code-based) | **Until file changes** | File modification timestamp invalidates cache |

**Invalidation Triggers:**
1. Session data modified/re-recorded
2. Agent configuration changed (thresholds, verbosity)
3. User explicitly requests "Force Re-analyze"
4. Cache size limit reached (LRU eviction)

**Implementation:**
```rust
use lru::LruCache;
use std::num::NonZeroUsize;

pub struct AgentCache {
    cache: LruCache<String, AnalysisResult>,
}

impl AgentCache {
    pub fn new() -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(100).unwrap()),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&AnalysisResult> {
        self.cache.get(key)
    }

    pub fn insert(&mut self, key: String, result: AnalysisResult) {
        self.cache.put(key, result);
    }

    pub fn invalidate_session(&mut self, session_id: &str) {
        // Remove all entries matching session_id:*:*
        self.cache.retain(|k, _| !k.starts_with(&format!("{}:", session_id)));
    }
}
```

**Memory vs Disk Caching:**
- **Decision**: In-memory only (no disk persistence)
- **Rationale**:
  - Sessions are ephemeral (analysis happens once after recording)
  - Disk I/O adds complexity and latency
  - 100 entries × ~5KB per result = ~500KB memory (negligible)
  - Cache invalidation is simpler (just restart app to clear)

**Cache Hit Rate Estimation:**
- User tweaks agent settings: Cache miss, re-analyze
- User views same session multiple times: Cache hit
- Expected hit rate: ~40% (acceptable for cost savings)

---

## 6. Frontend Agent Visualization

### Decision: Card-Based Dashboard with Progressive Disclosure

**UI Pattern:**

```
┌─────────────────────────────────────────────────────────────┐
│  Agent Analysis Results                         [⚙️ Config]  │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────┐│
│  │ 🛢️ Fuel Coach    │  │ 🚦 Safety        │  │ 🔧 Diag    ││
│  │ Score: 72/100    │  │ Score: 85/100    │  │ Score: 88  ││
│  │                  │  │                  │  │            ││
│  │ "Shift earlier   │  │ "3 hard braking  │  │ "Coolant   ││
│  │ to save 8% fuel" │  │ events detected" │  │ rising..."  ││
│  │                  │  │                  │  │            ││
│  │ [Expand ▼]       │  │ [Expand ▼]       │  │ [Expand ▼] ││
│  └──────────────────┘  └──────────────────┘  └────────────┘│
└─────────────────────────────────────────────────────────────┘
```

**Component Hierarchy:**
```tsx
<AgentDashboard>
  <AgentCard agent="fuel-coach" result={fuelResult}>
    <AgentIcon type="fuel" />
    <AgentScore value={72} />
    <AgentSummary text="Shift earlier to save 8% fuel" />
    <AgentDetails collapsed>
      <InsightList insights={fuelResult.insights} />
      <RecommendationList recommendations={fuelResult.recommendations} />
    </AgentDetails>
  </AgentCard>

  <AgentCard agent="safety" result={safetyResult}>
    {/* Similar structure */}
  </AgentCard>
</AgentDashboard>
```

**Agent-Specific Iconography:**
- Fuel Coach: 🛢️ (oil drum)
- Safety Advisor: 🚦 (traffic light)
- Diagnostic: 🔧 (wrench)
- Eco-Route: 🗺️ (map)
- Performance: 🏁 (checkered flag)
- Driver Profile: 🧠 (brain)
- Code Reviewer: 🔍 (magnifying glass)
- Test Generator: 🧪 (test tube)
- Documentation: 📝 (memo)
- Code Architect: 🏗️ (building construction)
- Performance Optimizer: ⚡ (lightning)
- Dependency Manager: 📦 (package)

**Color Coding (Score-Based):**
```typescript
const getScoreColor = (score: number): string => {
  if (score >= 90) return 'text-green-600';  // Excellent
  if (score >= 75) return 'text-blue-600';   // Good
  if (score >= 60) return 'text-yellow-600'; // Fair
  return 'text-red-600';                     // Needs improvement
};
```

**Progressive Disclosure:**
1. **Collapsed (default)**: Show icon, score, 1-sentence summary
2. **Expanded (click)**: Show all insights, recommendations with details
3. **Detail view (click insight)**: Show supporting data, charts, trends

**Confidence Score Visualization:**
```tsx
<ConfidenceBadge confidence={0.85}>
  {confidence >= 0.9 ? '🟢 High confidence' :
   confidence >= 0.7 ? '🟡 Medium confidence' :
   '🔴 Low confidence'}
</ConfidenceBadge>
```

**Multi-Agent Result Aggregation:**
- Show all agents in single dashboard
- Sort by priority (Critical insights first)
- Highlight conflicting recommendations (e.g., Safety says "Brake earlier" vs Performance says "Brake later")
- Allow filtering by agent type (Automotive only, Development only)

---

## 7. Inter-Agent Communication (Phase 3 - Future)

**Not Implemented in P1/P2**, but researched for future:

**Pattern: Agent Consultation**
```rust
// Example: Diagnostic agent detects fuel system issue,
// consults Fuel Coach for detailed analysis

impl DiagnosticAgent {
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError> {
        let mut result = self.run_diagnostics(request).await?;

        // If fuel-related anomaly detected, consult Fuel Coach
        if result.insights.iter().any(|i| i.title.contains("fuel")) {
            let fuel_coach = self.orchestrator.get_agent("fuel-coach")?;
            let fuel_analysis = fuel_coach.analyze(request).await?;

            // Merge insights
            result.insights.extend(fuel_analysis.insights);
            result.recommendations.extend(fuel_analysis.recommendations);
        }

        Ok(result)
    }
}
```

**Challenges Identified:**
- Circular dependencies (Agent A calls Agent B calls Agent A)
- Latency multiplication (3s per agent × 3 consultations = 9s)
- Prompt context explosion (need to pass prior agent results)

**Mitigation (when implemented):**
- Limit consultation depth (max 1 level)
- Timeout for consultations (fail gracefully)
- Explicit consultation graph (no cycles)

---

## 8. Development Agent Integration with Speckit

**Code Reviewer Integration (Pre-Commit Hook):**
```bash
# .git/hooks/pre-commit
#!/bin/bash
echo "🔍 Running Code Reviewer Agent..."

# Get staged files
FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$')

if [ -z "$FILES" ]; then
    echo "No Rust files to review."
    exit 0
fi

# Invoke Code Reviewer agent via Tauri command
cargo tauri dev --command review-code -- --files "$FILES"

if [ $? -ne 0 ]; then
    echo "❌ Code review failed. Fix issues or use --no-verify to bypass."
    exit 1
fi

echo "✅ Code review passed."
exit 0
```

**Test Generator Integration:**
```bash
# Custom cargo command: cargo test-gen
# .cargo/config.toml
[alias]
test-gen = "run --bin test-generator --"

# Usage:
cargo test-gen src/agents/fuel_coach.rs
# → Generates tests/agents/fuel_coach_test.rs automatically
```

**Documentation Writer Integration:**
```bash
# Pre-push hook generates docs
# .git/hooks/pre-push
#!/bin/bash
echo "📝 Generating documentation..."

cargo doc --no-deps
cargo run --bin doc-generator

git add docs/
git commit --amend --no-edit --no-verify

exit 0
```

---

## 9. Error Handling and Fallback Strategies

**Agent Failure Scenarios:**

| Scenario | Fallback Strategy |
|----------|------------------|
| Cloud API unavailable | Use local model if available, else skip agent |
| Agent timeout (>10s) | Cancel request, return partial results from other agents |
| Malformed JSON response | Log error, return "Agent failed to parse response" insight |
| Model hallucination | Validate output against schema, reject if invalid |
| Rate limit exceeded | Queue request, retry with exponential backoff |

**Circuit Breaker Pattern:**
```rust
pub struct CircuitBreaker {
    failure_count: AtomicU32,
    last_failure: Mutex<Option<Instant>>,
    threshold: u32,
    timeout: Duration,
}

impl CircuitBreaker {
    pub fn call<F, T>(&self, f: F) -> Result<T, AgentError>
    where
        F: FnOnce() -> Result<T, AgentError>,
    {
        // If too many recent failures, fail fast
        if self.failure_count.load(Ordering::Relaxed) >= self.threshold {
            if let Some(last) = *self.last_failure.lock().unwrap() {
                if last.elapsed() < self.timeout {
                    return Err(AgentError::CircuitOpen);
                }
            }
        }

        match f() {
            Ok(result) => {
                self.failure_count.store(0, Ordering::Relaxed);
                Ok(result)
            }
            Err(e) => {
                self.failure_count.fetch_add(1, Ordering::Relaxed);
                *self.last_failure.lock().unwrap() = Some(Instant::now());
                Err(e)
            }
        }
    }
}
```

---

## 10. Testing Strategies

**Unit Tests:**
- Test each agent in isolation with mock AI provider
- Verify prompt construction
- Validate JSON parsing (malformed responses)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::MockAiProvider;

    #[tokio::test]
    async fn test_fuel_coach_normal_case() {
        let mock_provider = MockAiProvider::new(r#"{
            "score": 72,
            "insights": [{"category": "Warning", "title": "Test"}],
            "recommendations": []
        }"#);

        let agent = FuelCoachAgent::new(mock_provider);
        let request = AnalysisRequest { /* ... */ };

        let result = agent.analyze(&request).await.unwrap();
        assert_eq!(result.score, Some(72.0));
    }

    #[tokio::test]
    async fn test_fuel_coach_malformed_json() {
        let mock_provider = MockAiProvider::new("Invalid JSON");
        let agent = FuelCoachAgent::new(mock_provider);
        let request = AnalysisRequest { /* ... */ };

        let result = agent.analyze(&request).await;
        assert!(matches!(result, Err(AgentError::ParseError(_))));
    }
}
```

**Integration Tests:**
- Test orchestrator with 3+ agents in parallel
- Verify cache hit/miss behavior
- Test privacy enforcement (LocalOnly agents reject cloud provider)

**End-to-End Tests:**
- Full session recording → AI analysis → frontend rendering
- Test with real AI providers (use cheap Haiku model)
- Verify latency < 8 seconds for 6 agents

---

## 11. Performance Optimization

**Identified Bottlenecks:**
1. **HTTP Request Latency**: 2-3 seconds per agent (network-bound)
2. **JSON Parsing**: Negligible (<1ms per response)
3. **Prompt Construction**: ~5ms per agent (string formatting)

**Optimization Techniques:**

**A. Request Batching (Future Enhancement)**
```rust
// Instead of N HTTP requests, batch multiple agent prompts
// (Only works for same provider + model)
let batched_request = BatchRequest {
    prompts: vec![fuel_prompt, safety_prompt, diagnostic_prompt],
};

let responses = provider.batch_complete(batched_request).await?;
```

**B. Streaming Responses (Future Enhancement)**
```rust
// Start rendering agent insights as they arrive (UX improvement)
let stream = provider.complete_stream(&prompt).await?;

while let Some(chunk) = stream.next().await {
    // Send partial result to frontend via Tauri event
    app.emit_all("agent-chunk", chunk)?;
}
```

**C. Prefetching (Future Enhancement)**
```rust
// Start agent analysis as soon as session recording stops
// (Don't wait for user to click "Analyze" button)
#[tauri::command]
async fn stop_recording(session_id: String, orchestrator: State<'_, AgentOrchestrator>) {
    // Save session data
    save_session(&session_id).await?;

    // Immediately start agent analysis in background
    tokio::spawn(async move {
        orchestrator.analyze_session(&session_id, vec!["fuel-coach", "safety"]).await;
    });
}
```

---

## 12. Deployment Considerations

**Local Model Setup (Ollama):**
```bash
# User setup instructions in README
curl -fsSL https://ollama.com/install.sh | sh
ollama pull llama3.1:8b
ollama pull mistral:7b

# App auto-detects Ollama availability
# Fallback to cloud if not available
```

**API Key Management:**
```rust
// Securely store API keys in OS keychain
use keyring::Entry;

pub fn store_api_key(service: &str, key: &str) -> Result<(), AgentError> {
    let entry = Entry::new(service, "user")?;
    entry.set_password(key)?;
    Ok(())
}

pub fn load_api_key(service: &str) -> Result<String, AgentError> {
    let entry = Entry::new(service, "user")?;
    entry.get_password()
        .map_err(|_| AgentError::ApiKeyNotFound)
}
```

**Graceful Degradation:**
```
Priority 1: Try local model (Ollama)
Priority 2: Try cloud model (OpenAI/Anthropic)
Priority 3: Use rule-based fallback
Priority 4: Show "Agent unavailable" message
```

---

## 13. Decisions Summary

| Topic | Decision | Rationale |
|-------|----------|-----------|
| **Orchestration** | Parallel execution with `tokio::spawn` | Fast, idiomatic, independent failures |
| **Prompt Format** | JSON output, Markdown display | Structured parsing + human-readable |
| **Model Selection** | Tiered (Haiku/Sonnet/Opus) by agent | Balance cost vs accuracy |
| **Privacy** | Local-first with privacy flags | User control, data minimization |
| **Caching** | In-memory LRU (100 entries) | Simple, fast, ephemeral sessions |
| **UI Pattern** | Card-based dashboard, progressive disclosure | Scannable, detailed on demand |
| **Error Handling** | Circuit breaker, graceful degradation | Resilience, user transparency |
| **Testing** | Unit + Integration + E2E | Coverage across all layers |
| **Deployment** | Ollama local, cloud fallback | Privacy + convenience |

---

## Next Steps

1. ✅ Research complete
2. ⏭️ Generate `data-model.md` (Phase 1)
3. ⏭️ Generate agent contracts in `/contracts/` (Phase 1)
4. ⏭️ Generate `quickstart.md` (Phase 1)
5. ⏭️ Run `/speckit.tasks` to generate task breakdown (Phase 2)
