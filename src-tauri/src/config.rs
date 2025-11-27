use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    pub supabase_url: Option<String>,
    pub supabase_anon_key: Option<String>,
}

impl Config {
    /// Load configuration from environment variables
    /// Tries to load .env file first, then falls back to system env
    pub fn load() -> Self {
        // Try to load .env file (will fail silently if not found)
        dotenv::dotenv().ok();

        Self {
            supabase_url: env::var("SUPABASE_URL").ok(),
            supabase_anon_key: env::var("SUPABASE_ANON_KEY").ok(),
        }
    }

    /// Check if Supabase is configured
    pub fn has_supabase(&self) -> bool {
        self.supabase_url.is_some() && self.supabase_anon_key.is_some()
    }

    /// Get Supabase URL (returns error if not configured)
    pub fn get_supabase_url(&self) -> Result<&str, ConfigError> {
        self.supabase_url
            .as_deref()
            .ok_or(ConfigError::MissingSupabaseUrl)
    }

    /// Get Supabase anon key (returns error if not configured)
    pub fn get_supabase_anon_key(&self) -> Result<&str, ConfigError> {
        self.supabase_anon_key
            .as_deref()
            .ok_or(ConfigError::MissingSupabaseKey)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("SUPABASE_URL not set in environment")]
    MissingSupabaseUrl,

    #[error("SUPABASE_ANON_KEY not set in environment")]
    MissingSupabaseKey,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load() {
        let config = Config::load();
        // Should not panic
        assert!(config.supabase_url.is_some() || config.supabase_url.is_none());
    }

    #[test]
    fn test_has_supabase() {
        let config = Config {
            supabase_url: Some("https://test.supabase.co".to_string()),
            supabase_anon_key: Some("test-key".to_string()),
        };
        assert!(config.has_supabase());

        let config_incomplete = Config {
            supabase_url: Some("https://test.supabase.co".to_string()),
            supabase_anon_key: None,
        };
        assert!(!config_incomplete.has_supabase());
    }
}
