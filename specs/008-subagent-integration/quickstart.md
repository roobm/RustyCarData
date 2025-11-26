# Quickstart Guide: Adding New Agents

**Feature**: 008-subagent-integration | **Audience**: Developers

This guide walks you through implementing a new agent in the sub-agent framework, from trait implementation to frontend integration.

---

## Table of Contents

1. [Agent Implementation (Rust)](#1-agent-implementation-rust)
2. [Agent Registration](#2-agent-registration)
3. [Tauri Command Integration](#3-tauri-command-integration)
4. [Frontend Integration (TypeScript/React)](#4-frontend-integration-typescriptreact)
5. [Testing Your Agent](#5-testing-your-agent)
6. [Prompt Engineering Tips](#6-prompt-engineering-tips)
7. [Troubleshooting](#7-troubleshooting)

---

## 1. Agent Implementation (Rust)

### Step 1.1: Create Agent File

Create a new file in `src-tauri/src/agents/implementations/`:

```bash
touch src-tauri/src/agents/implementations/my_agent.rs
```

### Step 1.2: Implement the `AnalysisAgent` Trait

```rust
// src-tauri/src/agents/implementations/my_agent.rs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::agents::{
    AnalysisAgent, AgentConfig, AgentDomain, AgentError, AgentMetadata,
    AnalysisRequest, AnalysisResult, Insight, InsightCategory,
    ModelPreference, PrivacyLevel, Recommendation, Priority, EffortLevel,
};
use crate::ai_provider::AiProvider;

/// My custom agent implementation
pub struct MyAgent {
    config: AgentConfig,
    ai_provider: Arc<dyn AiProvider>,
}

impl MyAgent {
    pub fn new(config: AgentConfig, ai_provider: Arc<dyn AiProvider>) -> Self {
        Self {
            config,
            ai_provider,
        }
    }

    /// Build the system prompt for this agent
    fn build_system_prompt(&self) -> String {
        r#"
You are an expert [ROLE] specializing in [DOMAIN].

Your goal is to [PRIMARY OBJECTIVE].

Analyze the following data and provide:
1. [OUTPUT 1]
2. [OUTPUT 2]
3. [OUTPUT 3]

Guidelines:
- Be [TONE]
- Focus on [PRIORITY]
- Provide actionable recommendations

Output format: Valid JSON matching the schema below.
        "#.to_string()
    }

    /// Build the user prompt from session data
    fn build_user_prompt(&self, request: &AnalysisRequest) -> String {
        // Extract relevant fields from request
        let session = &request.session_data;

        // Convert to JSON for the AI model
        serde_json::to_string_pretty(&serde_json::json!({
            "session_id": session.session_id,
            "duration_seconds": session.duration_seconds,
            "distance_miles": session.distance_miles,
            // Add more fields as needed for your agent
        })).unwrap_or_else(|_| "{}".to_string())
    }

    /// Parse AI response into AnalysisResult
    fn parse_response(&self, response_text: &str, agent_id: &str) -> Result<AnalysisResult, AgentError> {
        // Define expected response structure
        #[derive(Deserialize)]
        struct AgentResponse {
            score: u8,
            confidence: f32,
            insights: Vec<InsightData>,
            recommendations: Vec<RecommendationData>,
        }

        #[derive(Deserialize)]
        struct InsightData {
            category: String,
            title: String,
            description: String,
            #[serde(default)]
            supporting_data: HashMap<String, serde_json::Value>,
        }

        #[derive(Deserialize)]
        struct RecommendationData {
            priority: String,
            action: String,
            expected_benefit: String,
            effort_level: String,
        }

        // Parse JSON response
        let parsed: AgentResponse = serde_json::from_str(response_text)
            .map_err(|e| AgentError::ParseError(format!("Invalid JSON: {}", e)))?;

        // Convert to AnalysisResult
        let insights = parsed.insights.into_iter()
            .map(|i| Insight {
                category: match i.category.as_str() {
                    "Info" => InsightCategory::Info,
                    "Warning" => InsightCategory::Warning,
                    "Critical" => InsightCategory::Critical,
                    "Positive" => InsightCategory::Positive,
                    _ => InsightCategory::Info,
                },
                title: i.title,
                description: i.description,
                supporting_data: i.supporting_data,
                related_insight_ids: Vec::new(),
            })
            .collect();

        let recommendations = parsed.recommendations.into_iter()
            .map(|r| Recommendation {
                priority: match r.priority.as_str() {
                    "Low" => Priority::Low,
                    "Medium" => Priority::Medium,
                    "High" => Priority::High,
                    _ => Priority::Medium,
                },
                action: r.action,
                expected_benefit: r.expected_benefit,
                effort_level: match r.effort_level.as_str() {
                    "Easy" => EffortLevel::Easy,
                    "Moderate" => EffortLevel::Moderate,
                    "Significant" => EffortLevel::Significant,
                    _ => EffortLevel::Moderate,
                },
                related_insight_id: None,
            })
            .collect();

        Ok(AnalysisResult {
            agent_id: agent_id.to_string(),
            timestamp: chrono::Utc::now(),
            score: Some(parsed.score as f32),
            confidence: parsed.confidence,
            insights,
            recommendations,
            execution_time_ms: 0, // Will be set by orchestrator
            model_used: self.ai_provider.model_name().to_string(),
            metadata: Default::default(),
        })
    }
}

#[async_trait]
impl AnalysisAgent for MyAgent {
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError> {
        let start_time = std::time::Instant::now();

        // Build prompts
        let system_prompt = self.build_system_prompt();
        let user_prompt = self.build_user_prompt(request);

        // Call AI provider
        let response = self.ai_provider
            .complete(&system_prompt, &user_prompt)
            .await
            .map_err(|e| AgentError::ProviderError(e.to_string()))?;

        // Parse response
        let mut result = self.parse_response(&response, &self.metadata().id)?;

        // Set execution time
        result.execution_time_ms = start_time.elapsed().as_millis() as u64;

        // Validate result
        result.validate()?;

        Ok(result)
    }

    fn metadata(&self) -> AgentMetadata {
        AgentMetadata {
            id: "my-agent".into(),
            name: "My Agent".into(),
            domain: AgentDomain::Custom("my-domain".into()),
            description: "Brief description of what this agent does".into(),
            model_preference: ModelPreference::Medium,
            privacy_level: PrivacyLevel::CloudPreferred,
            icon: "🤖".into(),
            version: "1.0.0".into(),
        }
    }

    fn config(&self) -> &AgentConfig {
        &self.config
    }

    fn set_config(&mut self, config: AgentConfig) -> Result<(), AgentError> {
        self.config = config;
        Ok(())
    }

    fn can_handle(&self, request: &AnalysisRequest) -> bool {
        // Optional: Add custom logic to determine if this agent can handle the request
        // Default: all agents can handle all requests
        true
    }

    fn estimate_tokens(&self, request: &AnalysisRequest) -> u32 {
        // Rough estimate: system prompt + user prompt + output
        let system_tokens = 300; // Typical system prompt size
        let user_tokens = 500;   // Estimate based on session data size
        let output_tokens = 500; // Expected output size

        system_tokens + user_tokens + output_tokens
    }
}
```

---

## 2. Agent Registration

### Step 2.1: Add to Module Exports

Edit `src-tauri/src/agents/implementations/mod.rs`:

```rust
// src-tauri/src/agents/implementations/mod.rs

pub mod fuel_coach;
pub mod safety_advisor;
pub mod diagnostic;
pub mod my_agent; // ADD THIS LINE

pub use fuel_coach::FuelCoachAgent;
pub use safety_advisor::SafetyAdvisorAgent;
pub use diagnostic::DiagnosticAgent;
pub use my_agent::MyAgent; // ADD THIS LINE
```

### Step 2.2: Register Agent with Orchestrator

Edit `src-tauri/src/agents/registry.rs`:

```rust
// src-tauri/src/agents/registry.rs

use crate::agents::implementations::{
    FuelCoachAgent,
    SafetyAdvisorAgent,
    DiagnosticAgent,
    MyAgent, // ADD THIS LINE
};

impl AgentRegistry {
    pub fn with_default_agents(ai_provider: Arc<dyn AiProvider>) -> Self {
        let mut registry = Self::new();

        // Register default agents
        registry.register(Arc::new(FuelCoachAgent::new(
            AgentConfig::default(),
            ai_provider.clone(),
        )));

        registry.register(Arc::new(SafetyAdvisorAgent::new(
            AgentConfig::default(),
            ai_provider.clone(),
        )));

        registry.register(Arc::new(DiagnosticAgent::new(
            AgentConfig::default(),
            ai_provider.clone(),
        )));

        // ADD YOUR AGENT HERE
        registry.register(Arc::new(MyAgent::new(
            AgentConfig::default(),
            ai_provider.clone(),
        )));

        registry
    }
}
```

---

## 3. Tauri Command Integration

Your agent is now automatically available through the existing Tauri commands:

```typescript
// Frontend can now invoke your agent like this:
const result = await invoke('analyze_session', {
  sessionId: 'sess-001',
  agentIds: ['my-agent'], // Use your agent ID
});
```

No additional Tauri command setup required! The orchestrator handles routing.

---

## 4. Frontend Integration (TypeScript/React)

### Step 4.1: Add Agent Type

Edit `src/types/agents.ts`:

```typescript
// src/types/agents.ts

export type AgentId =
  | 'fuel-coach'
  | 'safety-advisor'
  | 'diagnostic-technician'
  | 'my-agent'; // ADD THIS LINE

export interface AgentMetadata {
  id: AgentId;
  name: string;
  domain: string;
  description: string;
  icon: string;
  enabled: boolean;
}

export const AGENT_METADATA: Record<AgentId, AgentMetadata> = {
  'fuel-coach': {
    id: 'fuel-coach',
    name: 'Fuel Efficiency Coach',
    domain: 'FuelEfficiency',
    description: 'Analyzes fuel consumption patterns',
    icon: '🛢️',
    enabled: true,
  },
  // ... other agents ...

  // ADD YOUR AGENT HERE
  'my-agent': {
    id: 'my-agent',
    name: 'My Agent',
    domain: 'Custom',
    description: 'Brief description',
    icon: '🤖',
    enabled: true,
  },
};
```

### Step 4.2: Use Agent in Dashboard

Your agent will automatically appear in the `<AgentDashboard>` component:

```tsx
// src/components/agents/AgentDashboard.tsx

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api';
import { AgentCard } from './AgentCard';

export function AgentDashboard({ sessionId }: { sessionId: string }) {
  const [results, setResults] = useState<AnalysisResult[]>([]);
  const [loading, setLoading] = useState(false);

  const runAnalysis = async () => {
    setLoading(true);
    try {
      const agentIds = ['fuel-coach', 'safety-advisor', 'my-agent']; // Include your agent
      const analysisResults = await invoke('analyze_session', {
        sessionId,
        agentIds,
      });

      setResults(analysisResults);
    } catch (error) {
      console.error('Agent analysis failed:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="agent-dashboard">
      <button onClick={runAnalysis} disabled={loading}>
        {loading ? 'Analyzing...' : 'Run Analysis'}
      </button>

      <div className="agent-results">
        {results.map((result) => (
          <AgentCard key={result.agent_id} result={result} />
        ))}
      </div>
    </div>
  );
}
```

---

## 5. Testing Your Agent

### Step 5.1: Unit Tests

Create `tests/agents/my_agent_test.rs`:

```rust
// tests/agents/my_agent_test.rs

use your_crate::agents::*;
use your_crate::test_utils::MockAiProvider;

#[tokio::test]
async fn test_my_agent_normal_case() {
    // Setup mock AI provider with expected response
    let mock_response = r#"{
        "score": 85,
        "confidence": 0.9,
        "insights": [
            {
                "category": "Info",
                "title": "Test Insight",
                "description": "Test description",
                "supporting_data": {}
            }
        ],
        "recommendations": []
    }"#;

    let mock_provider = Arc::new(MockAiProvider::new(mock_response));
    let agent = MyAgent::new(AgentConfig::default(), mock_provider);

    // Create test request
    let request = AnalysisRequest {
        request_id: "test-001".into(),
        session_data: create_test_session(), // Helper function
        vehicle_info: create_test_vehicle(),
        user_preferences: None,
        historical_context: None,
        force_local: false,
        timeout_seconds: 10,
    };

    // Execute analysis
    let result = agent.analyze(&request).await;

    // Assert
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.score, Some(85.0));
    assert_eq!(result.insights.len(), 1);
}

#[tokio::test]
async fn test_my_agent_malformed_json() {
    let mock_provider = Arc::new(MockAiProvider::new("Invalid JSON"));
    let agent = MyAgent::new(AgentConfig::default(), mock_provider);

    let request = create_test_request();
    let result = agent.analyze(&request).await;

    assert!(matches!(result, Err(AgentError::ParseError(_))));
}

#[tokio::test]
async fn test_my_agent_timeout() {
    // Test timeout handling
    let mock_provider = Arc::new(MockAiProvider::with_delay(15_000)); // 15 second delay
    let agent = MyAgent::new(AgentConfig::default(), mock_provider);

    let request = AnalysisRequest {
        timeout_seconds: 5, // 5 second timeout
        ..create_test_request()
    };

    let result = agent.analyze(&request).await;

    assert!(matches!(result, Err(AgentError::Timeout(_))));
}
```

### Step 5.2: Integration Tests

Test with the orchestrator:

```rust
// tests/agents/orchestrator_test.rs

#[tokio::test]
async fn test_orchestrator_with_my_agent() {
    let mock_provider = Arc::new(MockAiProvider::new(/* response */));
    let orchestrator = AgentOrchestrator::new(mock_provider);

    let request = create_test_request();
    let results = orchestrator.analyze_session(
        &request,
        vec!["fuel-coach".into(), "my-agent".into()],
    ).await;

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|r| r.agent_id == "my-agent"));
}
```

### Step 5.3: Manual Testing

```bash
# Run unit tests
cargo test my_agent

# Run all agent tests
cargo test agents

# Run with real AI provider (requires API key)
OPENAI_API_KEY=your_key cargo test --features integration_tests
```

---

## 6. Prompt Engineering Tips

### Tip 1: Use Few-Shot Examples

Include example inputs/outputs in your system prompt:

```rust
fn build_system_prompt(&self) -> String {
    format!(
        r#"
You are an expert [ROLE].

Example Input:
{{
  "metric_a": 42,
  "metric_b": 100
}}

Example Output:
{{
  "score": 80,
  "insights": [
    {{
      "category": "Info",
      "title": "Metric A is within range",
      "description": "Value of 42 is optimal."
    }}
  ]
}}

Now analyze the following session:
        "#
    )
}
```

### Tip 2: Specify Output Format Explicitly

```rust
r#"
Output format: Valid JSON with the following structure:
{
  "score": <number 0-100>,
  "confidence": <number 0.0-1.0>,
  "insights": [
    {
      "category": "Info" | "Warning" | "Critical" | "Positive",
      "title": <string>,
      "description": <string>,
      "supporting_data": <object>
    }
  ],
  "recommendations": [/* similar structure */]
}
"#
```

### Tip 3: Tune Model Parameters

```rust
// For consistent outputs (diagnostic agents)
let config = AiConfig {
    temperature: 0.2,
    top_p: 0.9,
    max_tokens: 1000,
};

// For creative outputs (less structured agents)
let config = AiConfig {
    temperature: 0.7,
    top_p: 0.95,
    max_tokens: 1500,
};
```

### Tip 4: Validate Output Schema

```rust
fn parse_response(&self, response_text: &str, agent_id: &str) -> Result<AnalysisResult, AgentError> {
    let parsed: AgentResponse = serde_json::from_str(response_text)?;

    // Validate score range
    if parsed.score > 100 {
        return Err(AgentError::ParseError("Score exceeds 100".into()));
    }

    // Validate confidence range
    if !(0.0..=1.0).contains(&parsed.confidence) {
        return Err(AgentError::ParseError("Confidence out of range".into()));
    }

    // ... convert to AnalysisResult
}
```

---

## 7. Troubleshooting

### Issue 1: Agent Not Appearing in UI

**Symptom:** Agent registered but doesn't show in dashboard

**Solution:**
1. Check `src/types/agents.ts` has your agent ID
2. Verify `AGENT_METADATA` includes your agent
3. Clear browser cache / restart dev server

### Issue 2: JSON Parsing Fails

**Symptom:** `AgentError::ParseError`

**Solution:**
1. Add few-shot example to system prompt showing exact JSON format
2. Lower temperature (0.2-0.3) for more structured outputs
3. Add schema validation instructions
4. Check AI model supports JSON mode (use `response_format: { type: "json_object" }` for OpenAI)

### Issue 3: Analysis Takes Too Long

**Symptom:** Timeout errors

**Solution:**
1. Reduce prompt size (fewer tokens = faster)
2. Use lighter model (Haiku instead of Opus)
3. Increase timeout in request
4. Check AI provider API latency

### Issue 4: Low Confidence Scores

**Symptom:** Agent always returns low confidence

**Solution:**
1. Review `confidence_calculation` logic
2. Ensure all required data fields are present
3. Check if session duration is too short (<5 minutes)
4. Verify sensor data completeness

### Issue 5: Agent Always Returns Same Score

**Symptom:** Score doesn't vary with input

**Solution:**
1. Increase temperature (too low = deterministic)
2. Add more context to prompts (model may not have enough info)
3. Check scoring rubric logic in prompt
4. Verify input data is actually changing

---

## Example: Complete Minimal Agent

Here's a minimal working agent:

```rust
use async_trait::async_trait;
use std::sync::Arc;

use crate::agents::*;
use crate::ai_provider::AiProvider;

pub struct MinimalAgent {
    config: AgentConfig,
    ai_provider: Arc<dyn AiProvider>,
}

impl MinimalAgent {
    pub fn new(config: AgentConfig, ai_provider: Arc<dyn AiProvider>) -> Self {
        Self { config, ai_provider }
    }
}

#[async_trait]
impl AnalysisAgent for MinimalAgent {
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError> {
        let prompt = format!(
            "Analyze this driving session: {} miles in {} minutes. Provide JSON with score (0-100) and insights.",
            request.session_data.distance_miles,
            request.session_data.duration_seconds / 60,
        );

        let response = self.ai_provider
            .complete("You are an AI assistant.", &prompt)
            .await
            .map_err(|e| AgentError::ProviderError(e.to_string()))?;

        // Simplified parsing (production code should be more robust)
        let parsed: serde_json::Value = serde_json::from_str(&response)?;

        Ok(AnalysisResult {
            agent_id: "minimal-agent".into(),
            timestamp: chrono::Utc::now(),
            score: parsed["score"].as_f64().map(|s| s as f32),
            confidence: 0.8,
            insights: Vec::new(),
            recommendations: Vec::new(),
            execution_time_ms: 0,
            model_used: self.ai_provider.model_name().to_string(),
            metadata: Default::default(),
        })
    }

    fn metadata(&self) -> AgentMetadata {
        AgentMetadata {
            id: "minimal-agent".into(),
            name: "Minimal Agent".into(),
            domain: AgentDomain::Custom("minimal".into()),
            description: "A minimal agent example".into(),
            model_preference: ModelPreference::Lightweight,
            privacy_level: PrivacyLevel::CloudPreferred,
            icon: "🤖".into(),
            version: "1.0.0".into(),
        }
    }

    fn config(&self) -> &AgentConfig { &self.config }
    fn set_config(&mut self, config: AgentConfig) -> Result<(), AgentError> {
        self.config = config;
        Ok(())
    }
}
```

**Register it:**
```rust
registry.register(Arc::new(MinimalAgent::new(
    AgentConfig::default(),
    ai_provider.clone(),
)));
```

**Use it:**
```typescript
await invoke('analyze_session', {
  sessionId: 'sess-001',
  agentIds: ['minimal-agent'],
});
```

---

## Next Steps

- ✅ Agent implemented and registered
- ✅ Tests written and passing
- ⏭️ Add to production agent list in `AgentRegistry`
- ⏭️ Document in project README
- ⏭️ Create contract document in `/specs/008-subagent-integration/contracts/`

Happy agent building! 🚀
