# AI Integration Strategy

## 1. Objective
Implement a flexible, privacy-aware AI analysis layer for the automotive telemetry platform. The goal is to provide dynamic, context-aware driving advice based on actual telemetry data, supporting both cloud-based (high accuracy) and local (privacy-focused) inference models.

## 2. User Stories
- **As a driver**, I want personalized feedback on my fuel efficiency and safety habits.
- **As a mechanic**, I want the AI to spot anomalies in sensor data (e.g., "Coolant temp rising too fast relative to RPM").
- **As a privacy-conscious user**, I want the option to run analysis locally without sending data to the cloud.

## 3. Architecture

### A. Data Collection Layer (Rust)
The backend aggregates high-frequency telemetry (CAN frames) into session statistics:
- **Session Metrics**: Duration, Distance, Fuel Consumed.
- **Extremes**: Max Speed, Max RPM, Max Temp.
- **Events**: Hard braking, rapid acceleration, high RPM events.

### B. Inference Provider Layer
The system supports a pluggable provider model:

1.  **Cloud Provider (OpenAI / Anthropic)**:
    -   *Use Case*: Detailed, high-quality coaching and complex diagnostics.
    -   *Configuration*: User provides API Key via UI.
    -   *Privacy*: Only summary statistics are sent (Data Minimization).

2.  **Local Provider (Ollama / LocalAI)**:
    -   *Use Case*: Offline usage, strict privacy, zero cost.
    -   *Configuration*: Connects to local endpoint (e.g., `http://localhost:11434`).
    -   *Model*: Llama 3, Mistral, or specialized automotive fine-tunes.

3.  **Rule-Based Fallback (Embedded)**:
    -   *Use Case*: Instant feedback, no setup required.
    -   *Logic*: Hardcoded thresholds (e.g., "If RPM > 6000, warn about efficiency").

### C. Workflow
1.  **Session End**: User requests analysis.
2.  **Context Assembly**: Backend gathers session stats and vehicle metadata (e.g., "2015 Honda Civic").
3.  **Provider Selection**:
    -   Check for Local LLM availability.
    -   Check for Cloud API Key.
    -   Default to Rule-Based.
4.  **Prompt Engineering**: Construct a structured prompt with role definition ("You are a senior master technician...").
5.  **Response Rendering**: Display Markdown-formatted advice in the Dashboard.

## 4. Technical Implementation Plan

### Backend (`src-tauri/src/commands.rs`)
-   [x] `analyze_current_session` command.
-   [ ] `reqwest` integration for HTTP calls.
-   [ ] Provider abstraction trait (allow swapping OpenAI/Ollama).

### Frontend (`src/components/AiAssistant.tsx`)
-   [x] Analysis display component.
-   [ ] Settings panel for API Keys / Local Endpoint URL.
-   [ ] Persistence for user preferences (localStorage or Tauri Store).

## 5. Privacy & Security
-   **Data Minimization**: Never send raw CAN logs to cloud providers unless explicitly opted-in for deep diagnostics.
-   **Key Management**: API keys stored in memory or OS Keychain (not plain text config).
-   **Transparency**: UI clearly indicates when data is leaving the device.

## 6. Future Roadmap
-   **RAG (Retrieval Augmented Generation)**: Index vehicle service manuals (PDFs) to allow the AI to quote specific torque specs or maintenance schedules.
-   **Real-time Voice Assistant**: TTS integration for "Co-pilot" mode while driving.
-   **Fine-tuning**: Collect user-opted data to fine-tune a small model specifically for OBD-II data interpretation.
