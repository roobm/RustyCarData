# Feature Spec: Migrate LLM Provider from Ollama to LM Studio

## Overview

Replace the existing Ollama LLM backend with LM Studio, providing better JSON output reliability, broader model support, and a more user-friendly UI for model management. The analysis capabilities remain identical; only the HTTP backend changes.

## User Request

> Replace Ollama with LM Studio using Qwen 3.5 9B for better structured output quality and newer model capabilities.

## Goals

- Migrate from `ollama-rs` crate to OpenAI-compatible HTTP API (LM Studio)
- Maintain identical analysis capabilities and agent interface
- Improve JSON output reliability for structured data parsing
- Use Qwen 3.5 9B (better reasoning than 3.2 3B)
- Simplify user setup with LM Studio's UI

## Current State

- Three agents (Diagnostic, Fuel Coach, Safety Advisor) use `OllamaProvider`
- Ollama backend running on `localhost:11434`
- Llama 3.2 3B as default model
- `ollama-rs` crate dependency

## Requirements

### Functional Requirements

1. **LM Studio Provider Implementation**
   - Create `LmStudioProvider` using OpenAI-compatible API
   - Support chat completions endpoint (`/v1/chat/completions`)
   - Implement health check against LM Studio server
   - Handle timeouts and connection errors gracefully

2. **Agent Integration**
   - Update Diagnostic Agent to use LmStudioProvider
   - Update Fuel Coach Agent to use LmStudioProvider
   - Update Safety Advisor Agent to use LmStudioProvider
   - Maintain identical AnalysisRequest/AnalysisResult types

3. **Configuration**
   - Support environment variables:
     - `LM_STUDIO_MODEL` (default: "qwen3.5-9b")
     - `LM_STUDIO_BASE_URL` (default: "http://localhost:1234/v1")
     - `LM_STUDIO_TIMEOUT_SECONDS` (default: 60)
   - Allow runtime model selection

4. **JSON Parsing**
   - Extract message content from OpenAI-format responses
   - Fallback to freeform text if JSON parsing fails
   - Maintain same error handling as Ollama provider

5. **Error Handling**
   - Health check detects LM Studio availability
   - Graceful fallback to rule-based analysis if server unavailable
   - Clear error messages for missing/invalid models

### Non-Functional Requirements

1. **Performance**
   - Response time equivalent to Ollama (Qwen 3.5 9B: 5-15 tokens/sec)
   - Timeout handling (default 60s, configurable)

2. **API Compatibility**
   - Full compatibility with OpenAI chat completions format
   - Support streaming (optional, for future enhancement)

3. **Code Quality**
   - Unit tests for JSON response parsing
   - Integration tests with LM Studio running
   - No breaking changes to agent interface

## Technical Constraints

- Rust backend with Tauri 2.x
- LM Studio server must be running on port 1234
- Requires `reqwest` HTTP client (already present)
- OpenAI API format compliance

## Success Criteria

- [ ] `LmStudioProvider` created and tested
- [ ] All three agents use new provider
- [ ] Health check works (server running/down detection)
- [ ] JSON parsing reliable with Qwen 3.5 9B output
- [ ] Identical analysis results vs. Ollama implementation
- [ ] All existing tests pass
- [ ] Error handling verified
- [ ] Documentation updated (.env.example, setup instructions)

## Out of Scope

- Support for other LLM providers simultaneously
- Streaming responses (can be added later)
- Advanced LM Studio UI features
- Model fine-tuning or customization

## Implementation Notes

### Architecture
```
OBD-II Data → Tauri Agent → reqwest HTTP → LM Studio (localhost:1234)
             (AnalysisRequest)    (OpenAI API)    (Qwen 3.5 9B)
                                                     ↓
                                              Structured JSON
                                                     ↓
                                           Agent-specific output
```

### OpenAI API Format
LM Studio supports standard OpenAI chat completions:
```json
POST /v1/chat/completions
{
  "model": "qwen3.5-9b",
  "messages": [
    {"role": "system", "content": "...system prompt..."},
    {"role": "user", "content": "...analysis request..."}
  ],
  "temperature": 0.3,
  "max_tokens": 2000
}
```

Response:
```json
{
  "choices": [{
    "message": {
      "content": "...json response or freeform text..."
    }
  }]
}
```

### Model Selection
- **Default**: Qwen 3.5 9B (better quality than Llama 3.2 3B)
- **Reason**: Excellent JSON output, better instruction following
- **Size**: ~5-6GB (requires 8-12GB RAM)
- **Speed**: 5-15 tokens/sec on M1/M2 Mac

### Fallback Strategy
If LM Studio unavailable:
1. Health check fails on startup
2. Agents fall back to rule-based analysis
3. User sees prompt: "LM Studio server not running. Using local analysis mode."
4. Analysis still works, just without LLM insights

## Testing Strategy

1. **Unit Tests**
   - JSON response parsing (OpenAI format)
   - Error handling (timeouts, invalid responses)
   - Message content extraction

2. **Integration Tests**
   - Health check with server running/down
   - Full agent analysis with actual LM Studio
   - JSON fallback to freeform text

3. **Manual Testing**
   - Load Qwen 3.5 9B in LM Studio
   - Run analysis on sample OBD-II data
   - Verify JSON output parsing
   - Test error scenarios (server down, invalid model)

## Migration Plan

See `plan.md` for detailed implementation phases.

**Summary**:
1. Create LmStudioProvider (Phase 1)
2. Add unit tests (Phase 1)
3. Update three agents (Phase 2)
4. Update dependencies (Phase 3)
5. Integration testing & cleanup (Phase 4)

## Rollback Plan

If issues arise:
1. Keep `ollama.rs` in codebase (mark as deprecated)
2. Revert agent provider imports to OllamaProvider
3. No dependency conflicts (can coexist)

## Dependencies

### Rust Crates
- `reqwest` (HTTP client) — already present
- `serde` + `serde_json` (JSON) — already present
- `tokio` (async) — already present

### System
- LM Studio installed from https://lmstudio.ai/
- Qwen 3.5 9B model downloaded (~5-6GB)
- Port 1234 available
