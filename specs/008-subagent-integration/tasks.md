# Implementation Tasks: Sub-Agent Integration

**Feature**: 008-subagent-integration | **Branch**: `008-subagent-integration`
**Generated**: 2025-11-25

This document provides a dependency-ordered task breakdown for implementing the sub-agent framework with 12 specialized AI agents.

---

## Task Format

```
- [ ] [TaskID] [Priority] [Story] Description
      File: path/to/file.rs
      Dependencies: T001, T002
      Parallel Group: A
```

- **TaskID**: Unique identifier (T001, T002, etc.)
- **Priority**: P1 (core), P2 (important), P3 (enhancement), P4 (advanced)
- **Story**: Which user story this task satisfies
- **Dependencies**: Must complete these tasks first
- **Parallel Group**: Tasks in the same group can run in parallel

---

## Phase 1: Project Setup

**Objective**: Prepare project structure and dependencies

- [ ] **T001** [P1] [Setup] Add required dependencies to Cargo.toml
      File: src-tauri/Cargo.toml
      Dependencies: None
      Parallel Group: A

      ```toml
      [dependencies]
      async-trait = "0.1"
      tokio = { version = "1.35", features = ["full"] }
      serde = { version = "1.0", features = ["derive"] }
      serde_json = "1.0"
      chrono = { version = "0.4", features = ["serde"] }
      thiserror = "1.0"
      lru = "0.12"
      futures = "0.3"
      ```

- [ ] **T002** [P1] [Setup] Create agent module directory structure
      File: src-tauri/src/agents/
      Dependencies: None
      Parallel Group: A

      ```bash
      mkdir -p src-tauri/src/agents/implementations
      touch src-tauri/src/agents/mod.rs
      touch src-tauri/src/agents/traits.rs
      touch src-tauri/src/agents/types.rs
      touch src-tauri/src/agents/error.rs
      touch src-tauri/src/agents/registry.rs
      touch src-tauri/src/agents/orchestrator.rs
      touch src-tauri/src/agents/cache.rs
      touch src-tauri/src/agents/implementations/mod.rs
      ```

- [ ] **T003** [P1] [Setup] Add frontend dependencies to package.json
      File: package.json
      Dependencies: None
      Parallel Group: A

      ```json
      {
        "dependencies": {
          "zustand": "^4.4.0"
        }
      }
      ```

---

## Phase 2: Core Agent Framework (Blocking Prerequisites)

**Objective**: Implement foundational agent abstractions

- [ ] **T004** [P1] [Core] Define AgentError enum with thiserror
      File: src-tauri/src/agents/error.rs
      Dependencies: T001, T002
      Parallel Group: B

      ```rust
      use thiserror::Error;

      #[derive(Debug, Error)]
      pub enum AgentError {
          #[error("Agent not found: {0}")]
          NotFound(String),
          // ... (see data-model.md for full enum)
      }
      ```

- [ ] **T005** [P1] [Core] Define core data types (AgentMetadata, AgentConfig, etc.)
      File: src-tauri/src/agents/types.rs
      Dependencies: T001, T002, T004
      Parallel Group: B

      Implement all types from data-model.md:
      - AgentMetadata
      - AgentDomain enum
      - ModelPreference enum
      - PrivacyLevel enum
      - AgentConfig
      - VerbosityLevel enum
      - AnalysisRequest
      - AnalysisResult
      - Insight, InsightCategory
      - Recommendation, Priority, EffortLevel
      - ResultMetadata, TokenUsage, ExecutionLocation

- [ ] **T006** [P1] [Core] Define AnalysisAgent trait with async_trait
      File: src-tauri/src/agents/traits.rs
      Dependencies: T004, T005
      Parallel Group: B

      ```rust
      use async_trait::async_trait;

      #[async_trait]
      pub trait AnalysisAgent: Send + Sync {
          async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError>;
          fn metadata(&self) -> AgentMetadata;
          fn config(&self) -> &AgentConfig;
          fn set_config(&mut self, config: AgentConfig) -> Result<(), AgentError>;
          fn can_handle(&self, request: &AnalysisRequest) -> bool;
          fn estimate_tokens(&self, request: &AnalysisRequest) -> u32;
      }
      ```

- [ ] **T007** [P1] [Core] Implement AgentRegistry with HashMap storage
      File: src-tauri/src/agents/registry.rs
      Dependencies: T005, T006
      Parallel Group: C

      - `register()`: Add agent to registry
      - `get()`: Retrieve agent by ID
      - `list()`: List all registered agents
      - `get_by_domain()`: Filter agents by domain

- [ ] **T008** [P1] [Core] Implement AgentCache with LRU eviction
      File: src-tauri/src/agents/cache.rs
      Dependencies: T001, T005
      Parallel Group: C

      - `get()`: Retrieve cached result
      - `insert()`: Store result
      - `invalidate_session()`: Clear session-specific cache
      - `generate_cache_key()`: Create cache key from session+agent+config

- [ ] **T009** [P1] [Core] Implement AgentOrchestrator with parallel execution
      File: src-tauri/src/agents/orchestrator.rs
      Dependencies: T006, T007, T008
      Parallel Group: None (blocks P1 agents)

      - `analyze_session()`: Dispatch multiple agents in parallel
      - `dispatch_agent()`: Execute single agent with privacy enforcement
      - Circuit breaker pattern for fault tolerance
      - Cache integration

- [ ] **T010** [P1] [Core] Create agent module exports
      File: src-tauri/src/agents/mod.rs
      Dependencies: T004, T005, T006, T007, T008, T009
      Parallel Group: D

      ```rust
      pub mod error;
      pub mod types;
      pub mod traits;
      pub mod registry;
      pub mod cache;
      pub mod orchestrator;
      pub mod implementations;

      pub use error::*;
      pub use types::*;
      pub use traits::*;
      pub use registry::*;
      pub use cache::*;
      pub use orchestrator::*;
      ```

---

## Phase 3: P1 Agents (Core Automotive Agents)

**Objective**: Implement 3 core automotive agents

- [ ] **T011** [P1] [Agent] Implement FuelCoachAgent
      File: src-tauri/src/agents/implementations/fuel_coach.rs
      Dependencies: T009
      Parallel Group: E

      - Build system prompt (see contracts/fuel-coach.md)
      - Extract fuel metrics from SessionSummary
      - Parse JSON response into AnalysisResult
      - Calculate fuel efficiency score (0-100)
      - Generate fuel-saving recommendations

- [ ] **T012** [P1] [Agent] Implement SafetyAdvisorAgent
      File: src-tauri/src/agents/implementations/safety_advisor.rs
      Dependencies: T009
      Parallel Group: E

      - Build system prompt with NHTSA references
      - Extract safety events (hard braking, speeding, following distance)
      - Calculate safety score (0-100)
      - Generate risk warnings and defensive driving recommendations

- [ ] **T013** [P1] [Agent] Implement DiagnosticAgent
      File: src-tauri/src/agents/implementations/diagnostic.rs
      Dependencies: T009
      Parallel Group: E

      - Build system prompt with diagnostic patterns
      - Extract sensor time-series data (coolant temp, voltage, fuel trim)
      - Correlate multiple sensors (e.g., temp rise vs RPM)
      - Calculate vehicle health score (0-100)
      - Predict maintenance needs with urgency timeline

- [ ] **T014** [P1] [Agent] Register P1 agents in implementations module
      File: src-tauri/src/agents/implementations/mod.rs
      Dependencies: T011, T012, T013
      Parallel Group: F

      ```rust
      pub mod fuel_coach;
      pub mod safety_advisor;
      pub mod diagnostic;

      pub use fuel_coach::FuelCoachAgent;
      pub use safety_advisor::SafetyAdvisorAgent;
      pub use diagnostic::DiagnosticAgent;
      ```

- [ ] **T015** [P1] [Agent] Create agent registry with P1 agents
      File: src-tauri/src/agents/registry.rs (extend)
      Dependencies: T014
      Parallel Group: F

      ```rust
      impl AgentRegistry {
          pub fn with_default_agents(ai_provider: Arc<dyn AiProvider>) -> Self {
              let mut registry = Self::new();
              registry.register(Arc::new(FuelCoachAgent::new(...)));
              registry.register(Arc::new(SafetyAdvisorAgent::new(...)));
              registry.register(Arc::new(DiagnosticAgent::new(...)));
              registry
          }
      }
      ```

---

## Phase 4: Tauri Command Layer

**Objective**: Expose agents to frontend via Tauri commands

- [ ] **T016** [P1] [Integration] Add agent commands to Tauri
      File: src-tauri/src/commands.rs
      Dependencies: T015
      Parallel Group: G

      ```rust
      #[tauri::command]
      async fn analyze_session(
          session_id: String,
          agent_ids: Vec<String>,
          orchestrator: State<'_, AgentOrchestrator>,
      ) -> Result<Vec<AnalysisResult>, String> {
          // ... implementation
      }

      #[tauri::command]
      async fn list_agents(
          orchestrator: State<'_, AgentOrchestrator>,
      ) -> Result<Vec<AgentMetadata>, String> {
          // ... implementation
      }

      #[tauri::command]
      async fn update_agent_config(
          agent_id: String,
          config: AgentConfig,
          orchestrator: State<'_, AgentOrchestrator>,
      ) -> Result<(), String> {
          // ... implementation
      }
      ```

- [ ] **T017** [P1] [Integration] Register agent commands in main.rs
      File: src-tauri/src/main.rs
      Dependencies: T016
      Parallel Group: G

      ```rust
      fn main() {
          tauri::Builder::default()
              .manage(agent_orchestrator) // Initialize orchestrator
              .invoke_handler(tauri::generate_handler![
                  analyze_session,
                  list_agents,
                  update_agent_config,
                  // ... existing commands
              ])
              .run(tauri::generate_context!())
              .expect("error while running tauri application");
      }
      ```

- [ ] **T018** [P1] [Integration] Initialize AgentOrchestrator in Tauri setup
      File: src-tauri/src/main.rs
      Dependencies: T017
      Parallel Group: None

      - Load AI provider from existing infrastructure (005-real-ai)
      - Create AgentRegistry with default agents
      - Initialize AgentOrchestrator with registry and provider
      - Register as Tauri state

---

## Phase 5: Frontend Agent UI (P1)

**Objective**: Display agent results in user interface

- [ ] **T019** [P1] [Frontend] Define TypeScript types for agents
      File: src/types/agents.ts
      Dependencies: None
      Parallel Group: H

      ```typescript
      export type AgentId = 'fuel-coach' | 'safety-advisor' | 'diagnostic-technician';

      export interface AnalysisResult {
          agent_id: string;
          timestamp: string;
          score?: number;
          confidence: number;
          insights: Insight[];
          recommendations: Recommendation[];
          // ...
      }
      ```

- [ ] **T020** [P1] [Frontend] Create AgentCard component
      File: src/components/agents/AgentCard.tsx
      Dependencies: T019
      Parallel Group: H

      - Display agent icon, name, score
      - Show 1-sentence summary
      - Expand/collapse for full insights
      - Color-code by score (green/yellow/red)
      - Show confidence badge

- [ ] **T021** [P1] [Frontend] Create AgentDashboard component
      File: src/components/agents/AgentDashboard.tsx
      Dependencies: T020
      Parallel Group: H

      - Grid layout of AgentCard components
      - "Run Analysis" button
      - Loading states during analysis
      - Handle errors gracefully

- [ ] **T022** [P1] [Frontend] Create Zustand store for agent state
      File: src/store/agentStore.ts
      Dependencies: T019
      Parallel Group: H

      ```typescript
      interface AgentStore {
          results: Record<string, AnalysisResult>;
          configs: Record<string, AgentConfig>;
          loading: boolean;
          runAnalysis: (sessionId: string, agentIds: string[]) => Promise<void>;
          updateConfig: (agentId: string, config: AgentConfig) => void;
      }
      ```

- [ ] **T023** [P1] [Frontend] Integrate AgentDashboard into main Dashboard
      File: src/components/Dashboard.tsx
      Dependencies: T021, T022
      Parallel Group: I

      - Add "Agent Analysis" tab/section
      - Pass current sessionId to AgentDashboard
      - Display results alongside existing dashboard widgets

- [ ] **T024** [P1] [Frontend] Create AgentConfig component for settings
      File: src/components/agents/AgentConfig.tsx
      Dependencies: T022
      Parallel Group: I

      - Enable/disable individual agents
      - Adjust verbosity level (Concise/Detailed/Expert)
      - Configure agent-specific thresholds
      - Save settings to localStorage

---

## Phase 6: Testing (P1)

**Objective**: Ensure core functionality works correctly

- [ ] **T025** [P1] [Testing] Write unit tests for AgentError
      File: tests/agents/error_test.rs
      Dependencies: T004
      Parallel Group: J

- [ ] **T026** [P1] [Testing] Write unit tests for AgentCache
      File: tests/agents/cache_test.rs
      Dependencies: T008
      Parallel Group: J

      - Test cache hit/miss
      - Test LRU eviction
      - Test session invalidation

- [ ] **T027** [P1] [Testing] Write unit tests for FuelCoachAgent
      File: tests/agents/fuel_coach_test.rs
      Dependencies: T011
      Parallel Group: J

      - Test normal driving session
      - Test aggressive driving (low score)
      - Test optimal driving (high score)
      - Test malformed JSON response

- [ ] **T028** [P1] [Testing] Write unit tests for SafetyAdvisorAgent
      File: tests/agents/safety_advisor_test.rs
      Dependencies: T012
      Parallel Group: J

      - Test safe driving (high score)
      - Test risky driving (low score)
      - Test edge cases (short trip, missing data)

- [ ] **T029** [P1] [Testing] Write unit tests for DiagnosticAgent
      File: tests/agents/diagnostic_test.rs
      Dependencies: T013
      Parallel Group: J

      - Test healthy vehicle (high score)
      - Test thermostat failure pattern
      - Test lean fuel condition (P0171)
      - Test weak alternator pattern

- [ ] **T030** [P1] [Testing] Write integration tests for AgentOrchestrator
      File: tests/agents/orchestrator_test.rs
      Dependencies: T009, T015
      Parallel Group: K

      - Test parallel agent execution
      - Test privacy enforcement (LocalOnly agents)
      - Test timeout handling
      - Test circuit breaker

- [ ] **T031** [P1] [Testing] Create MockAiProvider for testing
      File: tests/test_utils/mock_ai_provider.rs
      Dependencies: None
      Parallel Group: J

      - Return predefined responses
      - Simulate timeouts
      - Simulate malformed responses

---

## Phase 7: P2 Agents (Additional Automotive Agents)

**Objective**: Implement 3 additional automotive agents

- [ ] **T032** [P2] [Agent] Implement EcoRoutePlannerAgent
      File: src-tauri/src/agents/implementations/eco_route.rs
      Dependencies: T015
      Parallel Group: L

      - Analyze historical trip data
      - Compare fuel efficiency by route
      - Provide route recommendations
      - Privacy: LocalOnly (location data)

- [ ] **T033** [P2] [Agent] Implement PerformanceAnalystAgent
      File: src-tauri/src/agents/implementations/performance.rs
      Dependencies: T015
      Parallel Group: L

      - Analyze G-forces, lap times, cornering
      - Compare to optimal racing line
      - Provide performance improvement tips

- [ ] **T034** [P2] [Agent] Implement DriverProfileLearnerAgent
      File: src-tauri/src/agents/implementations/driver_profile.rs
      Dependencies: T015
      Parallel Group: L

      - Aggregate patterns over multiple sessions
      - Classify driver personality (e.g., "Aggressive Commuter")
      - Provide personalized recommendations
      - Privacy: LocalOnly (behavioral data)

- [ ] **T035** [P2] [Agent] Register P2 agents in registry
      File: src-tauri/src/agents/implementations/mod.rs, registry.rs
      Dependencies: T032, T033, T034
      Parallel Group: M

- [ ] **T036** [P2] [Agent] Write unit tests for P2 agents
      File: tests/agents/
      Dependencies: T032, T033, T034
      Parallel Group: M

- [ ] **T037** [P2] [Frontend] Add P2 agents to TypeScript types and UI
      File: src/types/agents.ts, src/components/agents/
      Dependencies: T035
      Parallel Group: M

---

## Phase 8: P3 Agents (Core Development Agents)

**Objective**: Implement 3 core development/meta-agents

- [ ] **T038** [P3] [DevAgent] Implement CodeReviewerAgent
      File: src-tauri/src/agents/implementations/code_reviewer.rs
      Dependencies: T015
      Parallel Group: N

      - Accept code diffs as input
      - Check for constitution violations (unwrap(), clippy warnings)
      - Detect potential bugs
      - Suggest refactorings
      - Privacy: LocalOnly (source code)

- [ ] **T039** [P3] [DevAgent] Implement TestGeneratorAgent
      File: src-tauri/src/agents/implementations/test_generator.rs
      Dependencies: T015
      Parallel Group: N

      - Accept source code as input
      - Generate unit tests
      - Generate integration tests
      - Create test fixtures
      - Privacy: LocalOnly (source code)

- [ ] **T040** [P3] [DevAgent] Implement DocumentationWriterAgent
      File: src-tauri/src/agents/implementations/documentation.rs
      Dependencies: T015
      Parallel Group: N

      - Accept source code as input
      - Generate Rustdoc comments
      - Create API documentation
      - Generate usage examples
      - Privacy: LocalOnly (source code)

- [ ] **T041** [P3] [DevAgent] Integrate CodeReviewerAgent with git pre-commit hook
      File: .git/hooks/pre-commit
      Dependencies: T038
      Parallel Group: O

      ```bash
      #!/bin/bash
      FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$')
      cargo tauri dev --command review-code -- --files "$FILES"
      ```

- [ ] **T042** [P3] [DevAgent] Create cargo alias for test generation
      File: .cargo/config.toml
      Dependencies: T039
      Parallel Group: O

      ```toml
      [alias]
      test-gen = "run --bin test-generator --"
      ```

- [ ] **T043** [P3] [DevAgent] Register P3 agents in registry
      File: src-tauri/src/agents/implementations/mod.rs, registry.rs
      Dependencies: T038, T039, T040
      Parallel Group: P

- [ ] **T044** [P3] [DevAgent] Write unit tests for P3 agents
      File: tests/agents/
      Dependencies: T038, T039, T040
      Parallel Group: P

---

## Phase 9: P4 Agents (Advanced Development Agents)

**Objective**: Implement 3 advanced development agents

- [ ] **T045** [P4] [DevAgent] Implement CodeArchitectAgent
      File: src-tauri/src/agents/implementations/code_architect.rs
      Dependencies: T043
      Parallel Group: Q

      - Accept feature requirements as input
      - Design system architecture
      - Suggest file structure and module design
      - Privacy: LocalOnly (design documents)

- [ ] **T046** [P4] [DevAgent] Implement PerformanceOptimizerAgent
      File: src-tauri/src/agents/implementations/performance_optimizer.rs
      Dependencies: T043
      Parallel Group: Q

      - Accept profiling data as input
      - Identify performance bottlenecks
      - Suggest algorithmic improvements
      - Estimate performance gains

- [ ] **T047** [P4] [DevAgent] Implement DependencyManagerAgent
      File: src-tauri/src/agents/implementations/dependency_manager.rs
      Dependencies: T043
      Parallel Group: Q

      - Parse Cargo.toml and package.json
      - Check for security vulnerabilities
      - Suggest dependency updates
      - Detect unused dependencies

- [ ] **T048** [P4] [DevAgent] Register P4 agents in registry
      File: src-tauri/src/agents/implementations/mod.rs, registry.rs
      Dependencies: T045, T046, T047
      Parallel Group: R

- [ ] **T049** [P4] [DevAgent] Write unit tests for P4 agents
      File: tests/agents/
      Dependencies: T045, T046, T047
      Parallel Group: R

---

## Phase 10: Multi-Agent Collaboration (P3 - Future Enhancement)

**Objective**: Enable agents to consult each other

- [ ] **T050** [P3] [Collaboration] Extend AnalysisAgent trait with consultation method
      File: src-tauri/src/agents/traits.rs
      Dependencies: T015
      Parallel Group: S

      ```rust
      async fn consult(
          &self,
          other_agent_id: &str,
          request: &AnalysisRequest,
      ) -> Result<AnalysisResult, AgentError>;
      ```

- [ ] **T051** [P3] [Collaboration] Implement consultation graph (prevent cycles)
      File: src-tauri/src/agents/orchestrator.rs
      Dependencies: T050
      Parallel Group: S

      - Track consultation depth
      - Limit to max 1 level of consultation
      - Detect circular dependencies

- [ ] **T052** [P3] [Collaboration] Example: DiagnosticAgent consults FuelCoach for lean condition
      File: src-tauri/src/agents/implementations/diagnostic.rs
      Dependencies: T051
      Parallel Group: T

---

## Phase 11: Polish & Documentation

**Objective**: Production readiness and developer experience

- [ ] **T053** [P1] [Docs] Update project README with agent framework overview
      File: README.md
      Dependencies: T018, T023
      Parallel Group: U

      - Architecture diagram
      - List of available agents
      - Usage examples
      - Performance benchmarks

- [ ] **T054** [P1] [Docs] Create agent API documentation
      File: docs/agents-api.md
      Dependencies: T018
      Parallel Group: U

      - Trait reference
      - Type reference
      - Example agent implementation
      - Testing guide

- [ ] **T055** [P1] [Polish] Add clippy lints and run cargo fmt
      File: All Rust files
      Dependencies: T018
      Parallel Group: V

      ```bash
      cargo fmt
      cargo clippy -- -D warnings
      ```

- [ ] **T056** [P1] [Polish] Handle all TODOs and unwrap() calls
      File: All Rust files
      Dependencies: T055
      Parallel Group: V

      - Replace `unwrap()` with proper error handling
      - Resolve all `todo!()` macros
      - Ensure constitution compliance

- [ ] **T057** [P2] [Polish] Add loading animations and error toasts to frontend
      File: src/components/agents/
      Dependencies: T023
      Parallel Group: V

- [ ] **T058** [P2] [Polish] Implement agent result caching in frontend (sessionStorage)
      File: src/store/agentStore.ts
      Dependencies: T022
      Parallel Group: V

---

## Phase 12: End-to-End Testing

**Objective**: Validate complete workflow

- [ ] **T059** [P1] [E2E] Test full workflow: Record session → Run agents → Display results
      File: tests/e2e/
      Dependencies: T023, T030
      Parallel Group: W

- [ ] **T060** [P1] [E2E] Test multi-agent parallel execution (measure latency)
      File: tests/e2e/
      Dependencies: T030
      Parallel Group: W

      Expected: 3 agents complete in <8 seconds

- [ ] **T061** [P1] [E2E] Test privacy enforcement (LocalOnly agents reject cloud)
      File: tests/e2e/
      Dependencies: T030
      Parallel Group: W

- [ ] **T062** [P1] [E2E] Test with real AI providers (OpenAI, Anthropic, Ollama)
      File: tests/integration/
      Dependencies: T018
      Parallel Group: W

      Requires API keys in environment variables

---

## Completion Checklist

**Phase 1: Setup**
- [ ] T001: Dependencies added
- [ ] T002: Directory structure created
- [ ] T003: Frontend dependencies added

**Phase 2: Core Framework**
- [ ] T004-T010: All core agent framework types and traits implemented

**Phase 3: P1 Agents**
- [ ] T011-T015: FuelCoach, SafetyAdvisor, Diagnostic agents implemented and registered

**Phase 4: Tauri Integration**
- [ ] T016-T018: Tauri commands exposed and orchestrator initialized

**Phase 5: Frontend**
- [ ] T019-T024: Agent UI components and state management complete

**Phase 6: Testing**
- [ ] T025-T031: Unit and integration tests passing

**Phase 7-9: P2/P3/P4 Agents**
- [ ] T032-T049: All 12 agents implemented (optional phases)

**Phase 10-12: Polish**
- [ ] T050-T062: Collaboration, docs, E2E tests complete

---

## Estimated Effort

| Phase | Tasks | Est. Hours | Priority |
|-------|-------|-----------|----------|
| 1. Setup | 3 | 1h | P1 |
| 2. Core Framework | 7 | 12h | P1 |
| 3. P1 Agents | 5 | 16h | P1 |
| 4. Tauri Integration | 3 | 4h | P1 |
| 5. Frontend | 6 | 10h | P1 |
| 6. Testing | 7 | 12h | P1 |
| 7. P2 Agents | 6 | 12h | P2 |
| 8. P3 Dev Agents | 7 | 14h | P3 |
| 9. P4 Dev Agents | 5 | 10h | P4 |
| 10. Collaboration | 3 | 6h | P3 |
| 11. Polish | 6 | 8h | P1-P2 |
| 12. E2E Testing | 4 | 6h | P1 |
| **Total** | **62 tasks** | **~111 hours** | Mixed |

**Minimum Viable Product (P1 only)**: ~55 hours (Tasks T001-T062 filtered for P1)

---

## Parallel Execution Groups

Tasks within the same group can be executed in parallel:

- **Group A**: T001, T002, T003 (setup)
- **Group B**: T004, T005, T006 (types/traits)
- **Group C**: T007, T008 (registry/cache)
- **Group E**: T011, T012, T013 (P1 agents)
- **Group F**: T014, T015 (agent registration)
- **Group G**: T016, T017 (Tauri commands)
- **Group H**: T019, T020, T021, T022 (frontend components)
- **Group J**: T025, T026, T027, T028, T029, T031 (unit tests)
- **Group L**: T032, T033, T034 (P2 agents)
- **Group N**: T038, T039, T040 (P3 agents)
- **Group Q**: T045, T046, T047 (P4 agents)

---

## Next Steps

1. Review this task breakdown
2. Confirm priorities (P1 agents are MVP, P2-P4 are optional)
3. Run `/speckit.implement` to start execution
4. Track progress by marking tasks with [X]

**Ready to implement!** 🚀
