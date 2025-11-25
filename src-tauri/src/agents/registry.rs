use std::collections::HashMap;
use std::sync::Arc;

use super::error::AgentError;
use super::traits::AnalysisAgent;
use super::types::{AgentDomain, AgentMetadata};

/// Registry of available agents
pub struct AgentRegistry {
    agents: HashMap<String, Arc<dyn AnalysisAgent>>,
}

impl AgentRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Register an agent
    pub fn register(&mut self, agent: Arc<dyn AnalysisAgent>) {
        let metadata = agent.metadata();
        self.agents.insert(metadata.id.clone(), agent);
    }

    /// Get an agent by ID
    pub fn get(&self, agent_id: &str) -> Result<Arc<dyn AnalysisAgent>, AgentError> {
        self.agents
            .get(agent_id)
            .cloned()
            .ok_or_else(|| AgentError::NotFound(agent_id.to_string()))
    }

    /// List all registered agents
    pub fn list(&self) -> Vec<AgentMetadata> {
        self.agents
            .values()
            .map(|agent| agent.metadata())
            .collect()
    }

    /// Get agents by domain
    pub fn get_by_domain(&self, domain: &AgentDomain) -> Vec<Arc<dyn AnalysisAgent>> {
        self.agents
            .values()
            .filter(|agent| &agent.metadata().domain == domain)
            .cloned()
            .collect()
    }

    /// Check if an agent exists
    pub fn contains(&self, agent_id: &str) -> bool {
        self.agents.contains_key(agent_id)
    }

    /// Get count of registered agents
    pub fn count(&self) -> usize {
        self.agents.len()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
