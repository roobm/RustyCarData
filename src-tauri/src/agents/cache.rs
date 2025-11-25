use lru::LruCache;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;

use super::types::{AgentConfig, AnalysisResult};

/// Cache for agent analysis results
pub struct AgentCache {
    cache: LruCache<String, AnalysisResult>,
}

impl AgentCache {
    /// Create a new cache with 100-entry limit
    pub fn new() -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(100).unwrap()),
        }
    }

    /// Get a cached result
    pub fn get(&mut self, key: &str) -> Option<&AnalysisResult> {
        self.cache.get(key)
    }

    /// Insert a result into the cache
    pub fn insert(&mut self, key: String, mut result: AnalysisResult) {
        // Mark as from cache
        result.metadata.from_cache = true;
        result.metadata.cache_key = Some(key.clone());

        self.cache.put(key, result);
    }

    /// Invalidate all entries for a session
    pub fn invalidate_session(&mut self, session_id: &str) {
        let prefix = format!("{}:", session_id);
        self.cache.retain(|k, _| !k.starts_with(&prefix));
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl Default for AgentCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a cache key from session, agent, and config
pub fn generate_cache_key(
    session_id: &str,
    agent_id: &str,
    config: &AgentConfig,
) -> String {
    let config_hash = hash_config(config);
    format!("{}:{}:{}", session_id, agent_id, config_hash)
}

/// Hash agent config to detect changes
fn hash_config(config: &AgentConfig) -> String {
    let mut hasher = DefaultHasher::new();
    config.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
