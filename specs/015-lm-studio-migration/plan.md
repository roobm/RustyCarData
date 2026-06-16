# Plan: Migrate from Ollama to LM Studio

## Overview
Replace the `ollama-rs` crate with an OpenAI-compatible HTTP client to use LM Studio (`localhost:1234`) as the LLM backend. This maintains the same analysis capabilities while providing better JSON output reliability and broader model support.

## Key Changes

### 1. **Architecture**
- **Current**: Ollama provider using `ollama-rs` crate → `localhost:11434`
- **New**: OpenAI-compatible provider using `reqwest` + `serde` → `localhost:1234`
- **Benefit**: Same abstraction layer (AnalysisAgent trait), but different backend

### 2. **Dependencies**
Remove:
- `ollama-rs = "0.3.2"`

Keep/verify already present:
- `reqwest` (HTTP client with async/await)
- `serde` + `serde_json` (JSON serialization)
- `tokio` (async runtime)

### 3. **File Changes**

| File | Action | Details |
|------|--------|---------|
| `src-tauri/src/agents/providers/mod.rs` | Update | Export new `LmStudioProvider`, remove/keep `OllamaProvider` as fallback |
| `src-tauri/src/agents/providers/ollama.rs` | Replace | Create `lm_studio.rs` with OpenAI-compatible client implementation |
| `src-tauri/Cargo.toml` | Update | Remove `ollama-rs`, ensure `reqwest` has required features |
| `.env.example` | Update | Change `OLLAMA_MODEL` → `LM_STUDIO_MODEL`, change port if needed |
| `src-tauri/src/agents/implementations/*.rs` | Update | Change provider initialization (Diagnostic, Fuel Coach, Safety Advisor) |

### 4. **New Provider Implementation**

**File**: `src-tauri/src/agents/providers/lm_studio.rs`

**Key methods**:
```rust
pub struct LmStudioProvider {
    client: reqwest::Client,
    model: String,
    base_url: String,  // default: http://localhost:1234/v1
    timeout: Duration,
}

// Public API:
impl LmStudioProvider {
    pub fn new() -> Self
    pub fn with_model(model: String) -> Self
    pub async fn health_check(&self) -> bool
    pub async fn generate(&self, prompt: String) -> Result<String, AgentError>
    pub async fn analyze_session(&self, request: &AnalysisRequest, system_prompt: &str) -> Result<String, AgentError>
    pub fn parse_json_or_text<T>(&self, response: &str) -> Result<ParsedResponse<T>, AgentError>
}
```

**OpenAI-compatible request format**:
```json
{
  "model": "qwen3.5-9b",
  "messages": [
    {"role": "system", "content": "..."},
    {"role": "user", "content": "..."}
  ],
  "temperature": 0.3,
  "max_tokens": 2000
}
```

### 5. **Configuration**

**Environment variables** (.env):
- `LM_STUDIO_MODEL` (default: "qwen3.5-9b")
- `LM_STUDIO_BASE_URL` (default: "http://localhost:1234/v1")
- `LM_STUDIO_TIMEOUT_SECONDS` (default: 60)

**LM Studio Settings**:
- Ensure server running on port 1234
- Load desired model (Qwen 3.5 9B)
- Model must support chat completions API

### 6. **JSON Response Handling**

LM Studio returns responses in OpenAI format:
```json
{
  "id": "chatcmpl-...",
  "object": "text_completion",
  "created": 1234567890,
  "model": "qwen3.5-9b",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "..."
      }
    }
  ]
}
```

**Extraction logic**: Parse JSON response and extract `choices[0].message.content`

### 7. **Error Handling**

| Scenario | Handling |
|----------|----------|
| LM Studio server not running | Health check fails, fallback to rule-based analysis |
| Invalid model name | HTTP 404 error, clear error message |
| Timeout (>60s) | Cancel request, return timeout error |
| Malformed JSON response | Use existing fallback logic (freeform text) |

### 8. **Testing Strategy**

**Unit tests** (mock responses):
- Parse OpenAI-format JSON response
- Extract message content correctly
- Handle JSON extraction edge cases
- Timeout handling

**Integration tests** (require LM Studio running):
- Health check with server running/down
- Full end-to-end analysis request
- Verify all three agents (Diagnostic, Fuel Coach, Safety Advisor) work

**Manual testing**:
1. Start LM Studio with a model
2. Run Tauri app
3. Verify health check detects server
4. Run analysis on sample OBD-II data
5. Confirm JSON output parsing works

### 9. **Migration Path**

**Phase 1: Create new provider**
- [ ] Create `lm_studio.rs` with full implementation
- [ ] Add unit tests
- [ ] Update `mod.rs` exports

**Phase 2: Update agents**
- [ ] Update Diagnostic agent to use LmStudioProvider
- [ ] Update Fuel Coach agent
- [ ] Update Safety Advisor agent
- [ ] Test individually

**Phase 3: Configuration**
- [ ] Update `Cargo.toml` (remove ollama-rs, verify reqwest)
- [ ] Update `.env.example`
- [ ] Update README with LM Studio setup instructions

**Phase 4: Validation**
- [ ] Run existing tests
- [ ] Manual integration test with LM Studio
- [ ] Verify all three agents work end-to-end
- [ ] Cleanup: Keep ollama.rs as optional fallback or remove entirely

### 10. **Backwards Compatibility**

**Option A** (recommended): Replace completely
- Remove `ollama-rs` dependency
- Remove `ollama.rs` provider
- Full migration to LM Studio

**Option B**: Keep both
- Add conditional compilation feature: `features = ["ollama", "lm-studio"]`
- Allow users to choose at runtime
- More complex but flexible

**Recommendation**: Go with Option A for simplicity. If needed, we can always add Ollama back later.

### 11. **Risks & Mitigations**

| Risk | Mitigation |
|------|-----------|
| LM Studio API differs from OpenAI spec | Test with actual LM Studio before full commit |
| JSON parsing fails differently | Use same fallback logic as Ollama provider |
| Users still have Ollama references | Update all docs and env examples clearly |
| Model naming incompatibility | Keep flexible model selection, document available models |

### 12. **Success Criteria**

- ✅ `ollama-rs` removed from Cargo.toml
- ✅ `lm_studio.rs` created with all required methods
- ✅ All three agents updated to use LmStudioProvider
- ✅ Health check works with LM Studio running
- ✅ Analysis generates correct JSON output
- ✅ Tests pass (unit + integration)
- ✅ Documentation updated (.env.example, README if needed)
- ✅ Verified end-to-end with sample OBD-II data

---

## Implementation Order

1. **Create the provider** → Most critical, everything depends on it
2. **Update agents** → Consume the new provider
3. **Update dependencies** → Clean up old references
4. **Test & validate** → Ensure nothing broke
5. **Document** → Update setup instructions
