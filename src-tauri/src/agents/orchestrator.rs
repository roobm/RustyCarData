use std::sync::Arc;
use tokio::sync::Mutex;

use super::cache::{generate_cache_key, AgentCache};
use super::error::AgentError;
use super::registry::AgentRegistry;
use super::traits::AnalysisAgent;
use super::types::{AnalysisRequest, AnalysisResult, PrivacyLevel};

/// Orchestrator coordinates multiple agents
pub struct AgentOrchestrator {
    registry: Arc<AgentRegistry>,
    cache: Arc<Mutex<AgentCache>>,
}

impl AgentOrchestrator {
    /// Create a new orchestrator
    pub fn new(registry: AgentRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
            cache: Arc::new(Mutex::new(AgentCache::new())),
        }
    }

    /// Analyze a session with multiple agents in parallel
    pub async fn analyze_session(
        &self,
        request: &AnalysisRequest,
        agent_ids: Vec<String>,
    ) -> Vec<Result<AnalysisResult, AgentError>> {
        let mut handles = vec![];

        for agent_id in agent_ids {
            let registry = self.registry.clone();
            let cache = self.cache.clone();
            let request = request.clone();
            let agent_id = agent_id.clone();

            // Spawn parallel task for each agent
            let handle = tokio::spawn(async move {
                // Get agent from registry
                let agent = registry.get(&agent_id)?;

                // Check cache
                let cache_key = generate_cache_key(
                    &request.session_data.session_id,
                    &agent_id,
                    agent.config(),
                );

                {
                    let mut cache_lock = cache.lock().await;
                    if let Some(cached) = cache_lock.get(&cache_key) {
                        return Ok(cached.clone());
                    }
                }

                // Check if agent is enabled
                if !agent.config().enabled {
                    return Err(AgentError::AgentDisabled);
                }

                // Privacy enforcement
                let metadata = agent.metadata();
                if metadata.privacy_level == PrivacyLevel::LocalOnly && !request.force_local {
                    // For now, just log - will be enforced when AI provider is added
                    eprintln!("Warning: {} requires LocalOnly execution", metadata.name);
                }

                // Execute agent analysis
                let start = std::time::Instant::now();
                let result = agent.analyze(&request).await?;
                let elapsed = start.elapsed().as_millis() as u64;

                // Update execution time
                let mut final_result = result;
                final_result.execution_time_ms = elapsed;

                // Cache the result
                {
                    let mut cache_lock = cache.lock().await;
                    cache_lock.insert(cache_key, final_result.clone());
                }

                Ok(final_result)
            });

            handles.push(handle);
        }

        // Wait for all agents to complete
        let results = futures::future::join_all(handles).await;

        // Convert JoinHandle results to agent results
        results
            .into_iter()
            .map(|r| match r {
                Ok(agent_result) => agent_result,
                Err(e) => Err(AgentError::ExecutionFailed(e.to_string())),
            })
            .collect()
    }

    /// Get the agent registry
    pub fn registry(&self) -> &AgentRegistry {
        &self.registry
    }

    /// Invalidate cache for a session
    pub async fn invalidate_cache(&self, session_id: &str) {
        let mut cache = self.cache.lock().await;
        cache.invalidate_session(session_id);
    }

    /// Clear all cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.lock().await;
        cache.clear();
    }
}
