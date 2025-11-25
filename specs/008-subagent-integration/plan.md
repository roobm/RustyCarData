# Implementation Plan: Sub-Agent Integration for Automotive Intelligence & Development

**Branch**: `008-subagent-integration` | **Date**: 2025-11-25 | **Spec**: [spec.md](spec.md)

## Summary

Implement a modular sub-agent framework that supports **12 specialized AI agents** in two categories:

1. **Automotive Agents** (6): Fuel Efficiency Coach, Safety Advisor, Diagnostic Technician, Eco-Route Planner, Performance Analyst, Driver Profile Learner
2. **Development/Meta-Agents** (6): Code Architect, Code Reviewer, Test Generator, Documentation Writer, Performance Optimizer, Dependency Manager

This creates a **self-improving system** where agents analyze both vehicle telemetry AND help build/maintain the application itself. Agents can run locally or in the cloud, operate independently or collaboratively, and provide domain-specific insights.

**Technical Approach**: Extend the existing AI provider infrastructure (005-real-ai) with an agent orchestration layer in Rust, implement agent trait abstractions supporting both automotive and development domains, add parallel agent execution, and create agent-specific UI components in the React frontend. Development agents integrate with Speckit workflow.

## Technical Context

**Language/Version**: Rust 1.75+ (backend), TypeScript 5.x (frontend)
**Primary Dependencies**:
- Backend: `tokio` (async runtime), `serde` (serialization), `reqwest` (HTTP), existing AI provider clients
- Frontend: React 19, `zustand` (state management), existing UI components

**Storage**:
- Agent configurations in localStorage (frontend)
- Agent results cached in memory/session storage
- Historical agent insights in CSV (reuse 006-data-recording)

**Testing**: `cargo test` (Rust unit tests), Jest/React Testing Library (frontend)

**Target Platform**: Desktop (Tauri app) - macOS, Linux, Windows

**Project Type**: Desktop application with Rust backend and React frontend (existing Tauri structure)

**Performance Goals**:
- Single agent analysis: < 3 seconds
- Multi-agent parallel execution: < 8 seconds
- Agent registration/lookup: < 10ms

**Constraints**:
- Must reuse existing AI provider infrastructure (OpenAI, Anthropic, Ollama clients)
- Agent prompts < 4000 tokens (model compatibility)
- Local agents must run on consumer hardware (8GB RAM minimum)
- Privacy-sensitive agents (Driver Profile) must never send data to cloud
- Graceful degradation if cloud agents unavailable

**Scale/Scope**:
- 12 specialized agents total:
  - P1: 3 core automotive agents (Fuel, Safety, Diagnostic)
  - P2: 3 additional automotive agents (Eco-Route, Performance, Driver Profile)
  - P3: 3 core development agents (Code Reviewer, Test Generator, Documentation Writer)
  - P4: 3 advanced development agents (Code Architect, Performance Optimizer, Dependency Manager)
- Support for future agent plugins
- 50-100 lines of prompt engineering per agent
- ~1500 LOC Rust backend, ~700 LOC TypeScript frontend
- Integration with Speckit workflow for development agents

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### ✅ I. Branch Management
- Feature developed on `008-subagent-integration` branch
- Will merge to main only after testing complete

### ✅ II. Documentation
- Comprehensive spec.md created
- This plan.md documents technical approach
- Will create research.md, data-model.md, quickstart.md in subsequent phases
- Agent prompts will be documented in code comments

### ✅ III. Rust Idioms and Code Quality
- All Rust code will pass `cargo clippy` with no warnings
- Will use `cargo fmt` for consistent formatting
- No `unwrap()` or `expect()` in production code - all errors handled with `Result<T, E>`
- Agent trait design prioritizes clarity and simplicity

### ✅ IV. Architecture and Modularity
- Agent framework cleanly separated from existing AI provider layer
- `AnalysisAgent` trait provides abstraction for all agent implementations
- Agent orchestrator coordinates agents without tight coupling
- Frontend components modular and reusable

### ✅ V. Error Handling
- Custom `AgentError` enum using `thiserror` for typed errors
- All agent operations return `Result<T, AgentError>`
- Agent failures handled gracefully with fallbacks
- No panics during agent execution

**Status**: ✅ All principles satisfied. No violations to justify.

## Project Structure

### Documentation (this feature)

```text
specs/008-subagent-integration/
├── spec.md              # Feature specification (✅ created)
├── plan.md              # This file (✅ created)
├── research.md          # Phase 0: Technology research and decisions
├── data-model.md        # Phase 1: Agent data structures and traits
├── contracts/           # Phase 1: Agent API contracts (prompts, inputs/outputs)
│   ├── fuel-coach.md
│   ├── safety-advisor.md
│   └── diagnostic-agent.md
├── quickstart.md        # Phase 1: Developer guide for adding new agents
└── tasks.md             # Phase 2: Implementation task breakdown
```

### Source Code (repository root)

**Structure Decision**: Reuse existing Tauri monorepo structure. Backend agents in `src-tauri/src/agents/`, frontend components in `src/components/agents/`.

```text
src-tauri/src/
├── agents/                      # NEW: Agent framework
│   ├── mod.rs                   # Agent module exports
│   ├── registry.rs              # Agent registry and factory
│   ├── orchestrator.rs          # Multi-agent coordination
│   ├── traits.rs                # AnalysisAgent trait definition
│   ├── types.rs                 # Agent-related types (Domain, Config, etc.)
│   ├── error.rs                 # AgentError enum
│   ├── cache.rs                 # Agent result caching
│   └── implementations/         # Concrete agent implementations
│       ├── mod.rs
│       ├── fuel_coach.rs        # Fuel Efficiency Coach
│       ├── safety_advisor.rs    # Safety Advisor
│       ├── diagnostic.rs        # Diagnostic Technician
│       ├── eco_route.rs         # Eco-Route Planner (P2)
│       ├── performance.rs       # Performance Analyst (P2)
│       └── driver_profile.rs    # Driver Profile Learner (P2)
├── commands.rs                  # ADD: Agent-related Tauri commands
└── main.rs                      # MODIFY: Register agent commands

src/
├── components/
│   ├── agents/                  # NEW: Agent UI components
│   │   ├── AgentDashboard.tsx   # Main agent results display
│   │   ├── AgentCard.tsx        # Individual agent insight card
│   │   ├── AgentConfig.tsx      # Agent enable/disable, settings
│   │   └── AgentIcon.tsx        # Agent-specific icons
│   ├── Dashboard.tsx            # MODIFY: Integrate AgentDashboard
│   └── AiAnalysis.tsx           # MODIFY: Use new agent system
├── store/
│   └── agentStore.ts            # NEW: Zustand store for agent state
└── types/
    └── agents.ts                # NEW: TypeScript types for agents

tests/
└── agents/                      # NEW: Agent unit tests
    ├── fuel_coach_test.rs
    ├── safety_advisor_test.rs
    └── orchestrator_test.rs
```

## Complexity Tracking

**No violations** - All code follows constitution principles. Agent framework adds reasonable complexity justified by modularity and extensibility requirements.

---

## Phase 0: Research & Technology Decisions

**Objective**: Research and document technology choices, best practices, and integration patterns for multi-agent systems.

**Output**: `research.md` with decisions on:

### Research Topics

1. **Agent Orchestration Patterns**
   - Sequential vs parallel execution models
   - Inter-agent communication mechanisms
   - Error handling and fallback strategies
   - Best practices for multi-agent coordination

2. **Prompt Engineering for Specialized Agents**
   - Role definition techniques (system prompts)
   - Context window optimization
   - Output format standardization (JSON vs Markdown)
   - Few-shot examples for automotive domain

3. **Model Selection Strategy**
   - Haiku vs Sonnet vs Opus for different agent types
   - Local model capabilities (Llama 3, Mistral)
   - Cost-performance tradeoffs
   - Token usage estimation per agent

4. **Privacy-Preserving Agent Design**
   - Data minimization techniques
   - Local-first agent architecture
   - Sensitive data detection and routing
   - User consent and transparency patterns

5. **Agent Result Caching**
   - Cache key generation (session + agent + config hash)
   - TTL strategies for different agent types
   - Invalidation on config changes
   - Memory vs disk caching tradeoffs

6. **Frontend Agent Visualization**
   - Agent-specific iconography and branding
   - Progressive disclosure of detailed insights
   - Confidence score visualization
   - Multi-agent result aggregation UI patterns

### Expected Decisions

Document in `research.md`:
- **Orchestrator Pattern**: Parallel execution with `tokio::spawn` for independent agents
- **Prompt Format**: JSON-structured outputs for parsing, Markdown for display
- **Model Mapping**: Haiku for Fuel/Safety, Sonnet for Diagnostics/Profile, Opus for complex reasoning
- **Cache Strategy**: In-memory LRU cache with 100-entry limit, no disk persistence (sessions are ephemeral)
- **Privacy Routing**: Agent metadata includes `requires_local: bool` flag, enforced by orchestrator

---

## Phase 1: Design & Contracts

**Prerequisites**: `research.md` complete

**Objective**: Design agent data models, trait abstractions, and define agent contracts (inputs, outputs, prompts).

### 1.1 Data Model (`data-model.md`)

Define core entities and relationships:

**Entities:**

1. **Agent** - Metadata about each agent
   - `id`: String (e.g., "fuel-coach")
   - `name`: String (display name)
   - `domain`: AgentDomain enum
   - `description`: String
   - `model_preference`: ModelPreference enum
   - `requires_local`: bool (privacy flag)
   - `enabled`: bool (user config)
   - `config`: AgentConfig struct

2. **AgentConfig** - Agent-specific settings
   - `thresholds`: HashMap<String, f32> (e.g., "rpm_warning": 6000.0)
   - `verbosity`: VerbosityLevel enum (Concise, Detailed, Expert)
   - `language`: String (future: i18n support)

3. **AnalysisRequest** - Input to agent
   - `session_data`: SessionSummary (from 006-data-recording)
   - `vehicle_metadata`: VehicleInfo
   - `user_preferences`: Option<UserPreferences>
   - `historical_context`: Option<Vec<PreviousSession>>

4. **AnalysisResult** - Output from agent
   - `agent_id`: String
   - `timestamp`: DateTime
   - `insights`: Vec<Insight>
   - `recommendations`: Vec<Recommendation>
   - `score`: Option<f32> (0-100 scale)
   - `confidence`: f32 (0.0-1.0)
   - `execution_time_ms`: u64

5. **Insight** - Single analytical observation
   - `category`: InsightCategory enum (Info, Warning, Critical)
   - `title`: String
   - `description`: String
   - `supporting_data`: HashMap<String, Value>

6. **Recommendation** - Actionable advice
   - `priority`: Priority enum (Low, Medium, High)
   - `action`: String
   - `expected_benefit`: String
   - `effort_level`: EffortLevel enum (Easy, Moderate, Significant)

**Relationships:**
- Agent 1:N AgentConfig (one config per agent)
- AnalysisRequest 1:N SessionData (can analyze single session or trends)
- AnalysisResult 1:N Insights 1:N Recommendations

**State Transitions:**
- Agent: Registered → Configured → Enabled → Analyzing → Results Ready
- AnalysisRequest: Created → Dispatched → Processing → Completed/Failed

### 1.2 Agent Contracts (`contracts/`)

For each P1 agent, create a contract document defining:

#### `contracts/fuel-coach.md` - Fuel Efficiency Coach

**Inputs:**
```rust
struct FuelCoachInput {
    session: SessionSummary,
    vehicle: VehicleInfo,
    avg_rpm: f32,
    avg_speed_mph: f32,
    fuel_consumed: f32,
    distance_miles: f32,
    acceleration_events: Vec<Event>,
}
```

**System Prompt** (excerpt):
```
You are an expert Fuel Efficiency Coach specializing in automotive telemetry analysis.
Your goal is to help drivers reduce fuel consumption through actionable driving behavior changes.

Analyze the following driving session data and provide:
1. Fuel efficiency score (0-100, 100 = optimal)
2. 3-5 specific fuel-wasting behaviors observed
3. Actionable recommendations with estimated fuel savings
4. Comparison to optimal driving for this vehicle type

Be specific, quantitative, and encouraging. Focus on the top 3 highest-impact improvements.
```

**Output Format:**
```json
{
  "score": 72,
  "insights": [
    {
      "category": "Warning",
      "title": "High RPM cruising detected",
      "description": "Average RPM of 3200 is 20% higher than optimal..."
    }
  ],
  "recommendations": [
    {
      "priority": "High",
      "action": "Shift to higher gear earlier (below 2500 RPM)",
      "expected_benefit": "8-12% fuel savings (~$150/year)",
      "effort_level": "Easy"
    }
  ]
}
```

#### `contracts/safety-advisor.md` - Safety Advisor

**Inputs:**
```rust
struct SafetyAdvisorInput {
    session: SessionSummary,
    hard_braking_events: Vec<Event>,
    rapid_acceleration_events: Vec<Event>,
    speed_violations: Vec<SpeedEvent>,
    max_speed_mph: f32,
    avg_following_time_sec: Option<f32>,
}
```

**System Prompt** (excerpt):
```
You are a certified Defensive Driving Instructor analyzing telemetry data to improve driver safety.
Your goal is to identify risky behaviors and provide evidence-based safety recommendations.

Analyze the session and provide:
1. Safety score (0-100, 100 = safest)
2. Identified risk factors with severity levels
3. Specific behavior modifications to reduce accident risk
4. Positive reinforcement for safe behaviors observed

Be empathetic but direct about risks. Reference NHTSA guidelines where applicable.
```

**Output Format:** Similar JSON structure with safety-specific insights.

#### `contracts/diagnostic-agent.md` - Diagnostic Technician

**Inputs:**
```rust
struct DiagnosticInput {
    session: SessionSummary,
    sensor_readings: HashMap<String, Vec<SensorReading>>,
    error_codes: Vec<ErrorCode>,
    temperature_anomalies: Vec<TempAnomaly>,
    voltage_readings: Vec<f32>,
}
```

**System Prompt** (excerpt):
```
You are a Master Automotive Technician with 20+ years of diagnostic experience.
Your expertise is in correlating sensor data to predict mechanical failures before they occur.

Analyze the sensor data and provide:
1. Vehicle health score (0-100)
2. Anomalies detected with failure risk assessment
3. Predicted maintenance needs with urgency timeline
4. Correlation analysis (e.g., "coolant temp rise + RPM pattern suggests...")

Be specific about failure modes. Cite common patterns (e.g., "This pattern typically indicates...")
```

**Output Format:** Similar JSON structure with diagnostic-specific insights.

### 1.3 Quickstart Guide (`quickstart.md`)

Create developer documentation:

**Topics:**
1. How to add a new agent (implement `AnalysisAgent` trait)
2. How to register an agent with the orchestrator
3. How to test agents locally
4. How to configure agent prompts
5. Example agent implementation walkthrough
6. Frontend integration guide
7. Testing strategies for agents

**Example Code Snippets:**
```rust
// Example: Implementing a new agent
pub struct MyNewAgent {
    config: AgentConfig,
    client: Arc<dyn AiProvider>,
}

#[async_trait]
impl AnalysisAgent for MyNewAgent {
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError> {
        // Build prompt, call AI provider, parse response
        todo!()
    }

    fn metadata(&self) -> AgentMetadata {
        AgentMetadata {
            id: "my-new-agent".into(),
            name: "My New Agent".into(),
            domain: AgentDomain::Custom("my-domain".into()),
            ...
        }
    }
}
```

### 1.4 Frontend Component Wireframes

Include visual mockups in `contracts/ui-mockups.md`:
- Agent Dashboard layout (grid of agent cards)
- Individual Agent Card design (icon, score, insights, expand/collapse)
- Agent Configuration modal
- Multi-agent loading states

---

## Phase 2: Implementation Planning

**This phase is handled by the `/speckit.tasks` command and NOT part of `/speckit.plan` output.**

The tasks.md file will break down implementation into:
- Phase 1: Setup (dependencies, module structure)
- Phase 2: Core Agent Framework (traits, registry, orchestrator)
- Phase 3: P1 Agents (Fuel Coach, Safety Advisor, Diagnostic)
- Phase 4: Frontend Integration (components, state management)
- Phase 5: Testing & Polish

---

## Next Steps

1. ✅ Spec created (`spec.md`)
2. ✅ Implementation plan created (`plan.md`)
3. ⏭️ Generate `research.md` (Phase 0)
4. ⏭️ Generate `data-model.md` and `contracts/` (Phase 1)
5. ⏭️ Generate `quickstart.md` (Phase 1)
6. ⏭️ Run `/speckit.tasks` to create implementation task breakdown
7. ⏭️ Run `/speckit.implement` to execute tasks

**Ready for Phase 0 research!** 🚀
