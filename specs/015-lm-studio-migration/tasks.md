# Tasks: Migrate from Ollama to LM Studio

## Phase 1: Create LM Studio Provider

### Task 1.1: Create lm_studio.rs provider module
**Description**: Implement the core LmStudioProvider struct with OpenAI-compatible HTTP client

**Acceptance Criteria**:
- [ ] File `src-tauri/src/agents/providers/lm_studio.rs` created
- [ ] `LmStudioProvider` struct defined with:
  - `client: reqwest::Client`
  - `model: String`
  - `base_url: String` (default: "http://localhost:1234/v1")
  - `timeout: Duration` (default: 60s)
- [ ] Constructor methods:
  - `pub fn new() -> Self` (uses env vars for config)
  - `pub fn with_model(model: impl Into<String>) -> Self`
  - `pub fn with_base_url(base_url: impl Into<String>) -> Self`
- [ ] Implements `Default` trait
- [ ] Compiles without errors

**Implementation Notes**:
- Use `reqwest::Client::new()`
- Support environment variables: `LM_STUDIO_MODEL`, `LM_STUDIO_BASE_URL`, `LM_STUDIO_TIMEOUT_SECONDS`
- Use Serde for JSON serialization

---

### Task 1.2: Implement health_check method
**Description**: Detect if LM Studio server is running and accessible

**Acceptance Criteria**:
- [ ] Method signature: `pub async fn health_check(&self) -> bool`
- [ ] Makes HTTP request to LM Studio
- [ ] Returns `true` if server responds successfully
- [ ] Returns `false` if timeout or connection error
- [ ] Timeout: 5 seconds (not affected by model timeout)
- [ ] Can be called without a specific model loaded

**Implementation Notes**:
- Use `GET /v1/models` or similar health endpoint
- Wrap in `tokio::time::timeout()`
- Return `true` for 2xx status codes

---

### Task 1.3: Implement generate method
**Description**: Send a text prompt to LM Studio and get response

**Acceptance Criteria**:
- [ ] Method signature: `pub async fn generate(&self, prompt: String) -> Result<String, AgentError>`
- [ ] Constructs OpenAI-compatible request:
  - `model`: from self.model
  - `messages`: single user message with prompt
  - `temperature`: 0.3 (for consistent output)
  - `max_tokens`: 2000
- [ ] Sends POST to `/v1/chat/completions`
- [ ] Extracts message content from response
- [ ] Returns content as String
- [ ] Handles timeout error (returns `AgentError::Timeout`)
- [ ] Handles HTTP errors (returns `AgentError::ProviderError`)

**Implementation Notes**:
- Define OpenAI request/response structs (use serde)
- Use `tokio::time::timeout()` for timeout handling
- Error message format: "LM Studio generation failed: {error}"

---

### Task 1.4: Implement analyze_session method
**Description**: Analyze OBD-II session with domain-specific system prompt

**Acceptance Criteria**:
- [ ] Method signature: `pub async fn analyze_session(&self, request: &AnalysisRequest, system_prompt: &str) -> Result<String, AgentError>`
- [ ] Formats session data using `format_telemetry()`
- [ ] Constructs messages array with:
  - System message: provided system_prompt
  - User message: formatted telemetry + instructions
- [ ] Calls `generate()` with full prompt
- [ ] Returns LLM response as String

**Implementation Notes**:
- Reuse telemetry formatting from OllamaProvider
- Maintain identical prompt structure
- Keep instruction about JSON output

---

### Task 1.5: Implement JSON parsing methods
**Description**: Parse LLM responses and extract JSON with fallback to freeform

**Acceptance Criteria**:
- [ ] Copy `parse_json_or_text<T>()` from OllamaProvider
- [ ] Copy `extract_json_from_markdown()` helper
- [ ] Copy `extract_json_object()` helper
- [ ] Return `ParsedResponse<T>` enum (Structured or Freeform)
- [ ] Works with OpenAI-format responses

**Implementation Notes**:
- Copy existing implementation from ollama.rs
- No changes needed, compatible with LM Studio output

---

### Task 1.6: Add telemetry formatting
**Description**: Format session data into structured text prompt

**Acceptance Criteria**:
- [ ] Implement `format_telemetry(session: &SessionSummary) -> String`
- [ ] Identical output to OllamaProvider
- [ ] Includes all telemetry fields (fuel, speed, RPM, temperature, events)

**Implementation Notes**:
- Copy from OllamaProvider implementation

---

### Task 1.7: Update providers/mod.rs
**Description**: Export new LmStudioProvider

**Acceptance Criteria**:
- [ ] Add `pub mod lm_studio;`
- [ ] Add `pub use lm_studio::LmStudioProvider;`
- [ ] Keep ollama module (for now, mark as deprecated in comment)

---

## Phase 2: Add Unit Tests

### Task 2.1: Test OpenAI response parsing
**Description**: Unit tests for JSON response parsing from LM Studio

**Acceptance Criteria**:
- [ ] Test valid OpenAI chat completion response
- [ ] Test extraction of message content
- [ ] Test response with choices[0].message.content
- [ ] Test error responses (non-2xx status)
- [ ] All tests pass

**Test Cases**:
```
test_parse_openai_response()
test_extract_message_from_openai_format()
test_handle_malformed_response()
test_timeout_error_handling()
```

---

### Task 2.2: Test JSON fallback logic
**Description**: Unit tests for JSON extraction fallback

**Acceptance Criteria**:
- [ ] Test markdown code block extraction
- [ ] Test raw JSON object extraction
- [ ] Test freeform text fallback
- [ ] All existing tests pass

**Implementation Notes**:
- Reuse tests from ollama.rs (should be identical)

---

## Phase 3: Update Agents

### Task 3.1: Update Diagnostic Agent
**Description**: Change from OllamaProvider to LmStudioProvider

**Acceptance Criteria**:
- [ ] File: `src-tauri/src/agents/implementations/diagnostic.rs`
- [ ] Change: `OllamaProvider::new()` → `LmStudioProvider::new()`
- [ ] Change: system prompt remains identical
- [ ] Change: prompt engineering unchanged
- [ ] Compiles without errors
- [ ] Tests updated if any agent-specific tests exist

---

### Task 3.2: Update Fuel Coach Agent
**Description**: Change from OllamaProvider to LmStudioProvider

**Acceptance Criteria**:
- [ ] File: `src-tauri/src/agents/implementations/fuel_coach.rs` (or similar)
- [ ] Change: `OllamaProvider::new()` → `LmStudioProvider::new()`
- [ ] Compiles without errors
- [ ] Tests updated if any agent-specific tests exist

---

### Task 3.3: Update Safety Advisor Agent
**Description**: Change from OllamaProvider to LmStudioProvider

**Acceptance Criteria**:
- [ ] File: `src-tauri/src/agents/implementations/safety_advisor.rs` (or similar)
- [ ] Change: `OllamaProvider::new()` → `LmStudioProvider::new()`
- [ ] Compiles without errors
- [ ] Tests updated if any agent-specific tests exist

---

## Phase 4: Update Dependencies & Configuration

### Task 4.1: Update Cargo.toml
**Description**: Remove ollama-rs, verify reqwest configuration

**Acceptance Criteria**:
- [ ] Remove line: `ollama-rs = "0.3.2"`
- [ ] Verify `reqwest` present with features:
  - [ ] `features = ["json"]` (for serde_json support)
- [ ] Verify other dependencies present:
  - [ ] `serde` with derive feature
  - [ ] `serde_json`
  - [ ] `tokio` with full features
- [ ] `cargo check` passes (no compilation errors)

---

### Task 4.2: Update .env.example
**Description**: Replace Ollama variables with LM Studio variables

**Acceptance Criteria**:
- [ ] Remove or comment out:
  - `OLLAMA_MODEL=llama3.2:3b`
  - `OLLAMA_TIMEOUT_SECONDS=60`
- [ ] Add:
  - `LM_STUDIO_MODEL=qwen3.5-9b`
  - `LM_STUDIO_BASE_URL=http://localhost:1234/v1`
  - `LM_STUDIO_TIMEOUT_SECONDS=60`
- [ ] Add comments explaining LM Studio setup

---

### Task 4.3: Update README (if applicable)
**Description**: Update setup instructions to reference LM Studio

**Acceptance Criteria**:
- [ ] Find and update Ollama setup instructions
- [ ] Add LM Studio setup steps:
  - Download from lmstudio.ai
  - Load Qwen 3.5 9B
  - Start server on port 1234
- [ ] Update model information
- [ ] Update port information (if documented)

---

## Phase 5: Testing & Validation

### Task 5.1: Run unit tests
**Description**: Verify all tests pass

**Acceptance Criteria**:
- [ ] `cargo test --lib` passes all tests
- [ ] No warnings related to LmStudioProvider
- [ ] All provider tests pass

---

### Task 5.2: Integration test with LM Studio
**Description**: Manual test with actual LM Studio running

**Acceptance Criteria**:
- [ ] LM Studio running with Qwen 3.5 9B on localhost:1234
- [ ] Health check succeeds (detects running server)
- [ ] Can generate text (simple prompt)
- [ ] Can analyze a session (full request with telemetry)
- [ ] JSON output parsed correctly
- [ ] Response format matches expected agent output

**Manual Test Steps**:
1. Start LM Studio with Qwen 3.5 9B
2. Verify server ready on port 1234
3. Run Tauri app
4. Check health check passes
5. Run analysis on sample OBD-II data
6. Verify JSON parsing works
7. Check fallback to freeform text if JSON invalid

---

### Task 5.3: Test health check offline
**Description**: Verify health check works when server is down

**Acceptance Criteria**:
- [ ] Stop LM Studio server
- [ ] Health check returns `false`
- [ ] App detects unavailability
- [ ] Agents fall back to rule-based analysis
- [ ] No crashes or panics

---

### Task 5.4: Verify agent outputs
**Description**: Compare Qwen 3.5 9B output quality vs Llama 3.2 3B

**Acceptance Criteria**:
- [ ] Run 3+ analyses with same telemetry data
- [ ] JSON output is valid (no parsing errors)
- [ ] Insights are relevant to the data
- [ ] Recommendations are actionable
- [ ] Output format matches expected structure

---

## Phase 6: Cleanup & Documentation

### Task 6.1: Decision on ollama.rs
**Description**: Keep or remove old Ollama provider

**Options**:
- **Option A** (Recommended): Delete `src-tauri/src/agents/providers/ollama.rs`
  - Cleaner codebase
  - Can add Ollama back later if needed
- **Option B**: Keep and mark deprecated
  - `#[deprecated]` attribute
  - Comment: "Use LmStudioProvider instead"
  - Useful as reference

**Acceptance Criteria**:
- [ ] One option selected and documented
- [ ] If deleted: no references remain in code
- [ ] If kept: marked as deprecated

---

### Task 6.2: Update internal documentation
**Description**: Add inline comments explaining LM Studio integration

**Acceptance Criteria**:
- [ ] `LmStudioProvider` struct documented with example
- [ ] Methods documented with parameters and errors
- [ ] Configuration environment variables documented
- [ ] Example usage comments

**Documentation Template**:
```rust
/// LM Studio LLM provider using OpenAI-compatible API
///
/// Connects to LM Studio server (default: localhost:1234)
/// and uses configured model for analysis.
///
/// # Environment Variables
/// - `LM_STUDIO_MODEL`: Model name (default: "qwen3.5-9b")
/// - `LM_STUDIO_BASE_URL`: API base URL (default: "http://localhost:1234/v1")
/// - `LM_STUDIO_TIMEOUT_SECONDS`: Timeout in seconds (default: 60)
///
/// # Example
/// ```
/// let provider = LmStudioProvider::new();
/// let result = provider.health_check().await;
/// ```
pub struct LmStudioProvider { ... }
```

---

### Task 6.3: Final validation
**Description**: Comprehensive final check

**Acceptance Criteria**:
- [ ] `cargo build` succeeds (no errors/warnings)
- [ ] `cargo test` passes all tests
- [ ] All three agents work with LM Studio
- [ ] Documentation complete and accurate
- [ ] No Ollama references in non-deprecated code
- [ ] .env.example reflects LM Studio setup

---

## Success Checklist

- [x] Plan created
- [ ] LmStudioProvider implemented (Phase 1)
- [ ] Unit tests added (Phase 2)
- [ ] All agents updated (Phase 3)
- [ ] Dependencies cleaned up (Phase 4)
- [ ] Testing completed (Phase 5)
- [ ] Documentation updated (Phase 6)
- [ ] Ready for production use
