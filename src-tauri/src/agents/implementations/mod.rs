// Agent implementations

pub mod mock_agent;
pub mod fuel_coach;
pub mod safety_advisor;
pub mod diagnostic;

pub use mock_agent::MockAgent;
pub use fuel_coach::FuelCoachAgent;
pub use safety_advisor::SafetyAdvisorAgent;
pub use diagnostic::DiagnosticAgent;
