# Research: Local LLM Integration with Ollama

## Decision: Use Ollama with Local Llama Models

**Selected Approach**: Run Llama models locally using Ollama (no cloud APIs)

### Rationale

1. **Privacy**: All telemetry data stays on user's computer
2. **Zero Cost**: No API fees, unlimited usage
3. **No Rate Limits**: Limited only by local hardware
4. **Offline Capability**: Works without internet connection
5. **Learning-Friendly**: Perfect for personal/learning projects

### Alternatives Considered

**Cloud APIs (Rejected)**:
- ❌ Google Gemini: Good free tier (1,500 RPD) but data sent to cloud
- ❌ OpenAI: Only $5 one-time credit, not sustainable
- ❌ Anthropic Claude: No free tier at all

**Why Local Wins**:
- User's OBD-II data contains personal driving patterns and locations
- No recurring costs vs. $6-60/month for cloud APIs
- Better for development (unlimited testing)
- Simpler setup (no API key management)

---

## Technical Decisions

### 1. Ollama Platform

**What is Ollama**: Open-source tool that runs LLMs locally via simple REST API

**Installation**:
- macOS: `brew install ollama`
- Windows: `winget install Ollama.Ollama`
- Linux: `curl -fsSL https://ollama.com/install.sh | sh`

**API**: HTTP REST API on `localhost:11434` (OpenAI-compatible)

---

### 2. Model Selection

**Primary Model: Llama 3.2 3B**
- **Parameters**: 3 billion
- **RAM Required**: 4-8 GB
- **Storage**: ~2 GB
- **Speed**: 10-50 tokens/sec (M1/M2 Mac)
- **Quality**: Excellent for structured data analysis
- **Installation**: `ollama pull llama3.2:3b`

**Why This Model**:
- Fast enough for real-time analysis
- Runs on most laptops (even 8GB RAM machines)
- Optimized for numeric data and reasoning
- Perfect balance of speed vs. quality

**Alternative for Power Users: Llama 3.3 70B**
- Better quality (GPT-4 level) but requires 48GB+ RAM
- Installation: `ollama pull llama3.3:70b`
- Use case: Detailed post-drive analysis

---

### 3. Rust Integration Strategy

**Selected Crate: `ollama-rs`**

```toml
[dependencies]
ollama-rs = "0.3.2"
```

**Advantages**:
- High-level, idiomatic Rust API
- Built-in async/streaming support
- Chat mode with message history
- Active maintenance

**Alternative Considered**: Direct `reqwest` HTTP calls
- Rejected: More boilerplate, we already have `ollama-rs`
- Keep as fallback for debugging

---

### 4. Architecture Integration

**Current Agent System**:
```rust
pub trait AnalysisAgent {
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError>;
    fn metadata(&self) -> AgentMetadata;
}
```

**Integration Approach**:
1. Create `OllamaProvider` module for LLM communication
2. Update each agent (Diagnostic, Fuel Coach, Safety Advisor) to use provider
3. Keep rule-based logic as fallback if Ollama unavailable
4. Add Tauri commands for model management

**Files to Create**:
- `src-tauri/src/agents/providers/mod.rs`
- `src-tauri/src/agents/providers/ollama.rs`
- Update `src-tauri/src/agents/implementations/*.rs`

---

### 5. Prompt Engineering Strategy

**Agent-Specific System Prompts**:

**Diagnostic Agent**:
```
You are an ASE-certified automotive diagnostic technician.
Analyze OBD-II telemetry for mechanical issues and maintenance needs.
Focus: temperature patterns, RPM stress, error codes, component wear.
Output: JSON with health_score, critical_issues, warnings, recommendations.
```

**Fuel Coach Agent**:
```
You are a fuel efficiency expert analyzing driving behavior.
Identify fuel-wasting patterns and provide actionable improvements.
Focus: acceleration, speed consistency, idle time, gear selection.
Output: JSON with mpg_potential, behaviors, techniques, savings.
```

**Safety Advisor Agent**:
```
You are a defensive driving instructor evaluating safety risks.
Analyze aggressive maneuvers and mechanical safety concerns.
Focus: hard braking, speed violations, cornering, collision risk.
Output: JSON with safety_score, high_risk_events, patterns, recommendations.
```

**JSON Mode**: Use structured output for reliable parsing

---

### 6. Performance Characteristics

**Expected Latency**:
- First token: ~100ms
- Total analysis (500 tokens): ~15 seconds (Llama 3.2 3B)
- Acceptable for post-drive analysis
- Too slow for real-time (>60fps) updates

**Memory Usage**:
- Llama 3.2 3B: 4-8 GB RAM
- Ollama service: ~100 MB overhead
- Total: Safe on 16GB machines

**Concurrent Requests**:
- Limited by RAM and CPU
- Recommend sequential processing for multi-session analysis
- Queue requests to avoid OOM

---

### 7. Error Handling & Fallbacks

**Failure Modes**:
1. Ollama not installed → Show user-friendly prompt with install link
2. Model not downloaded → Auto-download on first use (with permission)
3. Ollama service down → Fall back to rule-based analysis
4. Timeout (>60s) → Cancel and use fallback

**User Experience**:
```rust
match ollama_analysis().await {
    Ok(ai_result) => ai_result,
    Err(OllamaNotAvailable) => {
        warn!("Ollama unavailable, using rule-based analysis");
        rule_based_analysis()
    }
}
```

---

### 8. Development Workflow

**Local Development**:
1. Install Ollama: `brew install ollama`
2. Pull model: `ollama pull llama3.2:3b`
3. Start service: `ollama serve` (auto-starts on macOS)
4. Test integration: `cargo test --features ollama`

**CI/CD**:
- GitHub Actions: Use self-hosted runner with Ollama installed
- Or: Skip LLM tests in CI, test manually

**User Installation**:
- Detect Ollama presence on app startup
- Show setup wizard if not found
- Optionally bundle Ollama binary with Tauri app

---

### 9. Privacy & Security

**Data Flow**:
```
User's Car → OBD-II Adapter → Tauri App → Ollama (localhost) → Response
                                    ↓
                            Never leaves computer
```

**Advantages**:
- GDPR compliant (no third-party data processing)
- No telemetry sent to cloud
- Driving locations remain private
- Offline capability

**User Transparency**:
- Settings UI: "AI Analysis runs locally on your computer"
- Show model name and privacy level in agent metadata

---

### 10. Cost Analysis

**Setup Cost**: $0 (Ollama is free, open-source)

**Operational Cost**:
- Electricity: ~$0.01-0.05 per hour (GPU usage)
- Storage: 2-40 GB (model size)
- No API fees
- No rate limits

**Comparison to Cloud**:
- Google Gemini: $0 for 1,500 requests/day (then paid)
- OpenAI: $5 one-time, then $0.006 per analysis
- Claude: $0.006-0.03 per analysis
- **Ollama: $0 always**

**Break-even**: Immediate (no upfront cost)

---

## Implementation Plan Summary

**Phase 1: Core Integration**
1. Add `ollama-rs` dependency
2. Create `OllamaProvider` module
3. Update one agent (Diagnostic) to use Ollama
4. Test with `ollama pull llama3.2:3b`

**Phase 2: Full Agent Coverage**
1. Update Fuel Coach agent
2. Update Safety Advisor agent
3. Add model management Tauri commands

**Phase 3: UX Polish**
1. Detect Ollama availability
2. Show setup instructions if missing
3. Add settings for model selection
4. Implement streaming responses (live updates)

**Phase 4: Optimization**
1. Cache model in RAM (warmup on startup)
2. Optimize prompts for speed
3. Add request queuing
4. Benchmark performance

---

## Dependencies Required

**Rust**:
```toml
[dependencies]
ollama-rs = "0.3.2"
tokio = { version = "1.48.0", features = ["full"] }  # Already have
serde = { version = "1", features = ["derive"] }     # Already have
serde_json = "1"                                      # Already have
```

**System**:
- Ollama installed (`brew install ollama`)
- Llama model downloaded (`ollama pull llama3.2:3b`)

**Optional**:
```toml
# For structured JSON parsing
schemars = { version = "0.8", features = ["derive"] }
```

---

## Success Metrics

**Functional**:
- ✅ Ollama integration working end-to-end
- ✅ All three agents generating AI-powered insights
- ✅ Graceful fallback to rules if Ollama unavailable
- ✅ Model management from UI

**Performance**:
- ✅ Analysis completes in <30 seconds (Llama 3.2 3B)
- ✅ First token latency <200ms
- ✅ Memory usage <8GB (total app + model)

**Quality**:
- ✅ AI insights more detailed than rule-based placeholders
- ✅ JSON parsing reliability >95%
- ✅ Recommendations actionable and specific

**User Experience**:
- ✅ Clear setup instructions if Ollama missing
- ✅ Privacy messaging prominent
- ✅ Model selection in settings
- ✅ Offline capability confirmed

---

## Risks & Mitigations

**Risk 1: User doesn't have Ollama installed**
- **Mitigation**: Detect on startup, show friendly setup guide
- **Fallback**: Use rule-based analysis

**Risk 2: Model too large for user's machine**
- **Mitigation**: Default to Llama 3.2 3B (small), allow opt-in to larger models
- **Fallback**: Warn if RAM <8GB, suggest smaller model

**Risk 3: Analysis too slow**
- **Mitigation**: Show progress indicator, allow cancellation
- **Fallback**: Offer "quick analysis" mode with shorter prompts

**Risk 4: JSON parsing fails (malformed LLM output)**
- **Mitigation**: Retry with explicit JSON formatting instruction
- **Fallback**: Parse freeform text, extract key points

---

## References

- Ollama Documentation: https://github.com/ollama/ollama
- ollama-rs Crate: https://docs.rs/ollama-rs/
- Llama 3.2 Model Card: https://ollama.com/library/llama3.2
- Llama 3.3 Model Card: https://ollama.com/library/llama3.3
- Rust Async Patterns: https://rust-lang.github.io/async-book/
- OBD-II Parameter Reference: https://en.wikipedia.org/wiki/OBD-II_PIDs
