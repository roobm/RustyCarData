# Research: LM Studio vs Ollama

## Decision: Use LM Studio with Qwen 3.5 9B

**Selected Approach**: Migrate from Ollama to LM Studio's OpenAI-compatible API

### Rationale

1. **Better JSON Output**: Qwen 3.5 9B has superior structured output reliability vs Llama 3.2 3B
2. **Newer Model**: Qwen 3.5 releases more recent and refined than Llama 3.2
3. **Better Instruction Following**: More predictable responses to system prompts
4. **User-Friendly UI**: LM Studio provides better model management UI
5. **Same Privacy**: Data still stays local, no cloud integration

### Comparison: Ollama vs LM Studio

| Factor | Ollama | LM Studio |
|--------|--------|-----------|
| **UI** | CLI-only | Full GUI, easier model switching |
| **Models Available** | Large library | Large library (same origins) |
| **API** | Proprietary `ollama-rs` crate | OpenAI-compatible REST API |
| **Latency** | ~10-50 tokens/sec | ~5-15 tokens/sec (Qwen 3.5 9B) |
| **Setup** | `brew install ollama` | Download from lmstudio.ai |
| **Memory** | ~4-8GB (3B models) | ~8-12GB (9B models) |
| **JSON Quality** | Good | Excellent |
| **Learning Curve** | Moderate (CLI, env vars) | Low (GUI, intuitive) |

---

## Model Selection: Qwen 3.5 9B

### Why Qwen 3.5 9B?

**Strengths**:
- ✅ Larger (9B vs 3.2B) = better reasoning for complex telemetry analysis
- ✅ Newer model with better instruction tuning
- ✅ Excellent at structured JSON output (critical for our use case)
- ✅ Better adherence to system prompts
- ✅ Superior numeric reasoning (perfect for fuel economy, RPM data)

**Trade-offs**:
- Requires ~8-12GB RAM (vs 4-8GB for 3.2B)
- Slightly slower (~5-15 tokens/sec vs 10-50 for 3B)
- But still fast enough for post-drive analysis (<30s)

### Alternatives Considered

**Llama 3.2 3B**:
- Current model, proven to work
- Smaller, faster
- JSON output less reliable
- ❌ Decision: Too small, inconsistent JSON for structured analysis

**Llama 3.3 70B**:
- Highest quality
- Requires 48GB+ RAM
- ❌ Decision: Too large for typical user machines

**Mistral 7B**:
- Good middle ground
- Smaller than Qwen 3.5 9B
- ❌ Decision: Qwen more optimized for numeric/structured data

**Phi 3.5 Small**:
- Very small (~3.8B)
- Fast but lower quality
- ❌ Decision: Quality not sufficient for complex car analysis

---

## Technical Approach: OpenAI-Compatible API

### Why OpenAI API Format?

1. **Standardization**: Industry-standard format, widely documented
2. **Future-Proof**: Can easily swap providers (Claude, GPT, etc.) later
3. **Simplicity**: Well-known request/response format
4. **Library Support**: Mature ecosystem of HTTP clients

### Implementation Strategy

**Current (Ollama)**:
```rust
use ollama_rs::{Ollama, GenerationRequest};
let client = Ollama::default();
let response = client.generate(request).await;
```

**New (LM Studio)**:
```rust
use reqwest::Client;
use serde_json::json;
let client = Client::new();
let response = client.post("http://localhost:1234/v1/chat/completions")
    .json(&payload)
    .send()
    .await;
```

### LM Studio API Endpoint

**Base URL**: `http://localhost:1234/v1`

**Chat Completions**:
```
POST /v1/chat/completions

Request:
{
  "model": "qwen3.5-9b",
  "messages": [
    {"role": "system", "content": "..."},
    {"role": "user", "content": "..."}
  ],
  "temperature": 0.3,
  "max_tokens": 2000
}

Response:
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

---

## JSON Parsing Strategy

### Challenge
Both Ollama and LM Studio can return:
1. Proper JSON objects `{...}`
2. Markdown code blocks ` ```json {...} ``` `
3. Freeform text (fallback)

### Solution
Keep existing parsing logic from `OllamaProvider`:
1. Try to extract from markdown code blocks
2. Try to extract raw JSON object
3. Fallback to freeform text

**No changes needed** — LM Studio output format is compatible.

---

## Performance Characteristics

### Expected Latency (Qwen 3.5 9B on M1/M2 Mac)

| Metric | Value |
|--------|-------|
| First Token | ~100-200ms |
| Tokens/sec | 5-15 |
| Sample Analysis (500 tokens) | ~30-100s |
| Timeout (recommended) | 60s |

**Acceptable** for post-drive analysis (not real-time).

### Memory Usage

- LM Studio service: ~100MB
- Qwen 3.5 9B model (loaded): ~6-8GB
- Total system: ~8-10GB
- Safe on 16GB machines, tight on 8GB machines

---

## Error Handling

### Scenarios

1. **LM Studio not running**
   - HTTP connection fails → timeout
   - Health check returns false
   - Fallback: Rule-based analysis

2. **Invalid model name**
   - HTTP 404 error
   - Clear message: "Model not found in LM Studio"

3. **Timeout (>60s)**
   - Request cancelled
   - User sees: "Analysis taking too long, using local mode"

4. **Malformed JSON response**
   - JSON parsing fails
   - Fallback: Extract insights from freeform text
   - Log error for debugging

### Implementation
```rust
pub async fn health_check(&self) -> bool {
    match tokio::time::timeout(
        Duration::from_secs(5),
        self.client.post(&self.list_models_url())
            .send()
    ).await {
        Ok(Ok(response)) if response.status().is_success() => true,
        _ => false,
    }
}
```

---

## Prompt Engineering (No Changes)

System prompts remain identical:

**Diagnostic Agent**:
```
You are an ASE-certified automotive diagnostic technician.
Analyze OBD-II telemetry for mechanical issues.
Output: JSON with health_score, critical_issues, warnings, recommendations.
```

**Fuel Coach Agent**:
```
You are a fuel efficiency expert.
Identify fuel-wasting patterns and improvements.
Output: JSON with mpg_potential, behaviors, techniques, savings.
```

**Safety Advisor Agent**:
```
You are a defensive driving instructor.
Evaluate safety risks from driving behavior.
Output: JSON with safety_score, high_risk_events, patterns, recommendations.
```

**Improvement**: Qwen 3.5 9B follows these prompts more reliably than Llama 3.2.

---

## Cost Analysis

### Setup
- **LM Studio**: Free download
- **Qwen 3.5 9B**: Free, open-source
- **Storage**: ~5-6GB
- **Total**: $0

### Operation
- **Electricity**: ~$0.01-0.05/hour (GPU usage)
- **API Fees**: $0
- **Rate Limits**: None (local processing)
- **Total**: $0/month

---

## Migration Complexity

### Low Risk Because:
- ✅ Same analysis interface (AnalysisAgent trait)
- ✅ Same telemetry data format
- ✅ Same prompt structure
- ✅ OpenAI API is well-documented
- ✅ Existing JSON parsing logic compatible

### What Changes:
- HTTP client: `ollama_rs` → `reqwest`
- API endpoint: `localhost:11434` → `localhost:1234/v1`
- Response format: Ollama → OpenAI standard
- Provider implementation: ~150 lines new code

### What Stays the Same:
- Agent trait and implementations
- AnalysisRequest/AnalysisResult types
- Prompt engineering
- Error handling patterns
- Testing approach

---

## References

- LM Studio: https://lmstudio.ai/
- Qwen 3.5 Model: https://huggingface.co/Qwen/Qwen3.5-9B-Instruct
- OpenAI Chat Completions: https://platform.openai.com/docs/api-reference/chat/create
- LM Studio API Docs: https://github.com/lm-studio/docs (community docs)
