// rig framework adapter module for InterviewSpark
// This module bridges existing SiliconFlow API and vector store with rig framework

pub mod provider;
pub mod vector_store;
pub mod agents;
pub mod scheduler;
pub mod state_machine;

pub use provider::SiliconFlowProvider;
pub use vector_store::VectorStoreAdapter;
pub use agents::{InterviewContext, ConversationTurn, AnalysisResult};
pub use scheduler::{AgentScheduler, RotationStrategy};
pub use state_machine::{InterviewStateMachine, InterviewPhase, InterviewProgress};
