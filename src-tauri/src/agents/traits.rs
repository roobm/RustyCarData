use async_trait::async_trait;

use super::error::AgentError;
use super::types::{AgentConfig, AgentMetadata, AnalysisRequest, AnalysisResult};

/// Core trait that all agents must implement
#[async_trait]
pub trait AnalysisAgent: Send + Sync {
    /// Perform analysis on the given request
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError>;

    /// Get agent metadata
    fn metadata(&self) -> AgentMetadata;

    /// Get current configuration
    fn config(&self) -> &AgentConfig;

    /// Update configuration
    fn set_config(&mut self, config: AgentConfig) -> Result<(), AgentError>;

    /// Validate that this agent can handle the request
    fn can_handle(&self, _request: &AnalysisRequest) -> bool {
        // Default implementation: all agents can handle all requests
        // Override for specialized agents
        true
    }

    /// Estimate token usage for this request (for cost prediction)
    fn estimate_tokens(&self, _request: &AnalysisRequest) -> u32 {
        // Default: rough estimate based on session data size
        1000
    }
}
