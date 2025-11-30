use std::time::Duration;
use ollama_rs::{
    generation::completion::request::GenerationRequest,
    Ollama,
};
use serde::{Deserialize, Serialize};

use crate::agents::{
    error::AgentError,
    types::{AnalysisRequest, SessionSummary},
};

/// Ollama LLM provider for local AI analysis
pub struct OllamaProvider {
    client: Ollama,
    model: String,
    timeout: Duration,
}

impl OllamaProvider {
    /// Create a new Ollama provider with default settings
    pub fn new() -> Self {
        let model = std::env::var("OLLAMA_MODEL")
            .unwrap_or_else(|_| "llama3.2:3b".to_string());

        let timeout_seconds = std::env::var("OLLAMA_TIMEOUT_SECONDS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(60);

        Self {
            client: Ollama::default(),
            model,
            timeout: Duration::from_secs(timeout_seconds),
        }
    }

    /// Create a new Ollama provider with custom model
    pub fn with_model(model: impl Into<String>) -> Self {
        let timeout_seconds = std::env::var("OLLAMA_TIMEOUT_SECONDS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(60);

        Self {
            client: Ollama::default(),
            model: model.into(),
            timeout: Duration::from_secs(timeout_seconds),
        }
    }

    /// Check if Ollama service is available
    pub async fn health_check(&self) -> bool {
        // Try to list models as a health check
        match tokio::time::timeout(
            Duration::from_secs(5),
            self.client.list_local_models()
        ).await {
            Ok(Ok(_)) => true,
            _ => false,
        }
    }

    /// Get the model name being used
    pub fn model_name(&self) -> String {
        self.model.clone()
    }

    /// Generate analysis using the LLM
    pub async fn generate(&self, prompt: String) -> Result<String, AgentError> {
        let request = GenerationRequest::new(self.model.clone(), prompt);

        let response = tokio::time::timeout(
            self.timeout,
            self.client.generate(request)
        )
        .await
        .map_err(|_| AgentError::Timeout(self.timeout.as_secs()))?
        .map_err(|e| AgentError::ProviderError(format!("Ollama generation failed: {}", e)))?;

        Ok(response.response)
    }

    /// Analyze a session with a custom system prompt
    pub async fn analyze_session(
        &self,
        request: &AnalysisRequest,
        system_prompt: &str,
    ) -> Result<String, AgentError> {
        let telemetry_prompt = Self::format_telemetry(&request.session_data);

        let full_prompt = format!(
            "{}\n\n# Session Data\n\n{}\n\n# Instructions\n\nAnalyze the above telemetry data and provide your response in JSON format.",
            system_prompt,
            telemetry_prompt
        );

        self.generate(full_prompt).await
    }

    /// Format telemetry data into a structured prompt
    fn format_telemetry(session: &SessionSummary) -> String {
        format!(
            r#"Session ID: {}
Duration: {} seconds ({} minutes)
Distance: {:.1} miles

Fuel Economy:
- Fuel Consumed: {:.2} gallons
- Average MPG: {:.1}

Speed Statistics:
- Average Speed: {:.1} mph
- Maximum Speed: {:.1} mph
- Speed Violations: {}

Engine Performance:
- Average RPM: {:.0}
- Maximum RPM: {:.0}
- High RPM Duration: {} seconds

Temperature Readings:
- Average Coolant Temp: {:.1}°F
- Maximum Coolant Temp: {:.1}°F
- Average Oil Temp: {:.1}°F

Driving Events:
- Hard Braking Events: {}
- Rapid Acceleration Events: {}
- Error Codes: {}
"#,
            session.session_id,
            session.duration_seconds,
            session.duration_seconds / 60,
            session.distance_miles,
            session.fuel_consumed_gallons,
            session.avg_fuel_efficiency_mpg,
            session.avg_speed_mph,
            session.max_speed_mph,
            session.speed_violations_count,
            session.avg_rpm,
            session.max_rpm,
            session.high_rpm_duration_seconds,
            session.avg_coolant_temp_f,
            session.max_coolant_temp_f,
            session.avg_oil_temp_f,
            session.hard_braking_events.len(),
            session.rapid_acceleration_events.len(),
            session.error_codes.len(),
        )
    }

    /// Parse JSON response from LLM, with fallback to freeform text
    pub fn parse_json_or_text<T>(&self, response: &str) -> Result<ParsedResponse<T>, AgentError>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Try to extract JSON from markdown code blocks first
        let json_str = if let Some(json) = Self::extract_json_from_markdown(response) {
            json
        } else {
            response
        };

        // Try to parse as JSON
        match serde_json::from_str::<T>(json_str) {
            Ok(parsed) => Ok(ParsedResponse::Structured(parsed)),
            Err(_) => {
                // Fallback: return freeform text
                Ok(ParsedResponse::Freeform(response.to_string()))
            }
        }
    }

    /// Extract JSON from markdown code blocks
    fn extract_json_from_markdown(text: &str) -> Option<&str> {
        // Look for ```json ... ``` or ``` ... ```
        if let Some(start) = text.find("```json") {
            let after_start = &text[start + 7..];
            if let Some(end) = after_start.find("```") {
                return Some(after_start[..end].trim());
            }
        } else if let Some(start) = text.find("```") {
            let after_start = &text[start + 3..];
            if let Some(end) = after_start.find("```") {
                let content = after_start[..end].trim();
                // Check if it looks like JSON
                if content.starts_with('{') || content.starts_with('[') {
                    return Some(content);
                }
            }
        }
        None
    }
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from LLM parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParsedResponse<T> {
    /// Successfully parsed structured JSON
    Structured(T),
    /// Fallback to freeform text
    Freeform(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_markdown() {
        let text = r#"Here's the analysis:

```json
{
  "score": 85,
  "insights": ["Good fuel efficiency"]
}
```

That's it!"#;

        let extracted = OllamaProvider::extract_json_from_markdown(text);
        assert!(extracted.is_some());
        let json_str = extracted.unwrap();
        assert!(json_str.contains("\"score\""));
        assert!(json_str.contains("85"));
    }

    #[test]
    fn test_extract_json_from_plain_code_block() {
        let text = r#"```
{
  "result": "test"
}
```"#;

        let extracted = OllamaProvider::extract_json_from_markdown(text);
        assert!(extracted.is_some());
        let json_str = extracted.unwrap();
        assert!(json_str.contains("\"result\""));
    }

    #[test]
    fn test_no_json_in_text() {
        let text = "Just plain text without any JSON";
        let extracted = OllamaProvider::extract_json_from_markdown(text);
        assert!(extracted.is_none());
    }
}
