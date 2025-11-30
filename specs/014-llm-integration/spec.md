# Feature Spec: LLM Integration for AI Agents

## Overview

Integrate a free LLM provider (Google Gemini) to power the AI analysis agents with actual natural language generation capabilities.

## User Request

> Next we could implement the actual AI and llm logic behind our agents. What provider would you prefer. This is just for me atm for learning so i would like to use free one. Could it be google for example? I think they had some free plan?

## Goals

- Integrate Google Gemini API (free tier) for LLM capabilities
- Connect existing agent infrastructure to LLM backend
- Generate actual insights and recommendations using AI
- Maintain agent abstraction layer for potential provider switching

## Current State

- Agent infrastructure exists with mock implementations
- Three agent types: Fuel Coach, Safety Advisor, Diagnostic Agent
- Agent trait defines interface for analysis
- No actual LLM integration - agents return placeholder data

## Requirements

### Functional Requirements

1. **LLM Provider Integration**
   - Use Google Gemini API (free tier available)
   - Support API key configuration via environment variables
   - Handle rate limiting and errors gracefully

2. **Agent LLM Integration**
   - Each agent sends telemetry data to LLM with domain-specific prompts
   - Parse LLM responses into structured insights and recommendations
   - Maintain agent personality (fuel efficiency, safety, diagnostics)

3. **Prompt Engineering**
   - Create agent-specific system prompts
   - Format telemetry data for LLM consumption
   - Guide response structure (insights + recommendations)

4. **Error Handling**
   - Graceful degradation when LLM unavailable
   - Fallback to cached/placeholder responses
   - User-friendly error messages

### Non-Functional Requirements

1. **Performance**
   - Cache LLM responses to minimize API calls
   - Async LLM calls to avoid blocking UI
   - Timeout handling for slow responses

2. **Cost Management**
   - Use free tier (Gemini API offers 60 requests/minute free)
   - Implement request throttling
   - Monitor usage

3. **Security**
   - API keys stored in .env file
   - No telemetry data sent to third parties beyond LLM provider
   - User consent for cloud analysis

## Technical Constraints

- Rust backend (existing agent infrastructure)
- Must work with Tauri 2.x
- Free tier limitations (rate limits, token limits)

## Success Criteria

- [ ] Gemini API successfully integrated
- [ ] All three agents generate real insights using LLM
- [ ] Responses are relevant to telemetry data
- [ ] Error handling works (tested by disconnecting API)
- [ ] Free tier usage stays within limits
- [ ] Frontend displays LLM-generated insights correctly

## Out of Scope

- Multiple LLM provider support (can be added later)
- Fine-tuning or custom models
- Local LLM integration
- Advanced prompt optimization
