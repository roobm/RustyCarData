# Feature Plan: Real AI Integration (OpenAI)

## 1. Goal
Implement the "Cloud Provider" portion of the [AI Strategy](../ai-strategy.md). Enable users to input an OpenAI API Key and receive personalized driving advice based on their session telemetry.

## 2. Scope
-   **Backend**:
    -   Add `reqwest` dependency.
    -   Update `analyze_current_session` to accept an optional API Key.
    -   Implement the HTTP POST request to OpenAI API (`gpt-3.5-turbo`).
    -   Construct the prompt using session stats.
-   **Frontend**:
    -   Update `AiAssistant` component to include a "Settings" toggle.
    -   Add an input field for the API Key.
    -   Pass the key to the backend when "Analyze" is clicked.
    -   Handle loading and error states.

## 3. Tasks
1.  [x] **Dependencies**: Add `reqwest` with `json` feature to `src-tauri/Cargo.toml`.
2.  [x] **Backend Logic**:
    -   Modify `analyze_current_session` signature.
    -   Implement the API call logic.
    -   Add error handling for network failures or invalid keys.
3.  [x] **Frontend UI**:
    -   Add `Settings` icon/button.
    -   Create the API Key input form (password field).
    -   Wire up the state to the API call.
4.  [ ] **Testing**:
    -   Verify "Mock" fallback still works when no key is provided.
    -   Verify "Real AI" works with a valid key.
    -   Verify error handling with an invalid key.

## 4. Constitution Check
-   **Rust Idioms**: Use `Result` for the command return type to handle API errors gracefully.
-   **Separation**: The API key is passed from Frontend to Backend; Backend does not store it persistently (stateless for now).
-   **Error Handling**: No `unwrap()` on the network request; map errors to string for the frontend.
