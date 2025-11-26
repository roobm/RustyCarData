use thiserror::Error;

/// Errors that can occur during agent operations
#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Agent not found: {0}")]
    NotFound(String),

    #[error("Agent execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Failed to parse agent response: {0}")]
    ParseError(String),

    #[error("AI provider error: {0}")]
    ProviderError(String),

    #[error("Request timeout after {0} seconds")]
    Timeout(u64),

    #[error("No local provider available (required for LocalOnly agent)")]
    NoLocalProviderAvailable,

    #[error("API key not found for provider: {0}")]
    ApiKeyNotFound(String),

    #[error("Circuit breaker open (too many failures)")]
    CircuitOpen,

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Agent is disabled")]
    AgentDisabled,

    #[error("Internal error: {0}")]
    Internal(String),
}
