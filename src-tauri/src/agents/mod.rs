// Agent module exports
// Note: Allow dead_code as this infrastructure will be integrated with backend in future phases
#![allow(dead_code)]

pub mod cache;
pub mod error;
pub mod implementations;
pub mod orchestrator;
pub mod registry;
pub mod traits;
pub mod types;

// Re-export commonly used types
pub use error::AgentError;
pub use orchestrator::AgentOrchestrator;
pub use registry::AgentRegistry;
pub use traits::AnalysisAgent;
pub use types::*;
