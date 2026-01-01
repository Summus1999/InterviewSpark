// Multi-Agent interview system module

pub mod tech;
pub mod hr;
pub mod business;
pub mod comparison;

use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use super::state_machine::InterviewPhase;

/// Interviewer role enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterviewerRole {
    Technical,   // Technical interviewer
    HR,          // HR interviewer
    Business,    // Business interviewer
}

/// Interviewer Agent unified interface
#[allow(dead_code)]
#[async_trait]
pub trait InterviewerAgent: Send + Sync {
    /// Get agent role
    fn role(&self) -> InterviewerRole;
    
    /// Get role name (Chinese)
    fn role_name(&self) -> &'static str;
    
    /// Get role avatar identifier
    fn avatar(&self) -> &'static str;
    
    /// Generate interview question
    async fn generate_question(&self, context: &InterviewContext) -> Result<String>;
    
    /// Analyze user answer
    async fn analyze_answer(
        &self,
        question: &str,
        answer: &str,
        context: &InterviewContext,
    ) -> Result<AnalysisResult>;
    
    /// Decide whether to follow up
    async fn should_follow_up(&self, answer: &str, analysis: &AnalysisResult) -> bool;
}

/// Interview context
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterviewContext {
    pub resume: String,
    pub job_description: String,
    pub conversation_history: Vec<ConversationTurn>,
    pub current_phase: InterviewPhase,
}

/// Conversation turn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub role: InterviewerRole,
    pub role_name: String,
    pub question: String,
    pub answer: Option<String>,
    pub analysis: Option<AnalysisResult>,
}

/// Analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub score: f32,
    pub strengths: Vec<String>,
    pub improvements: Vec<String>,
    pub summary: String,
}

// Re-export agent implementations
pub use tech::TechInterviewer;
pub use hr::HRInterviewer;
pub use business::BusinessInterviewer;
