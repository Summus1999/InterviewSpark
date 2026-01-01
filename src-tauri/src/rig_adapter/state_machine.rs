// Interview phase state machine

use serde::{Deserialize, Serialize};
use super::agents::{InterviewerRole, AnalysisResult};

/// Interview phase enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterviewPhase {
    WarmUp,      // Warm-up (1-2 questions)
    Technical,   // Technical assessment (3-5 questions)
    Behavioral,  // Behavioral interview (2-3 questions)
    Business,    // Business understanding (2-3 questions)
    Questions,   // Q&A session (1-2 questions)
    Completed,   // Interview completed
}

/// Phase configuration
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PhaseConfig {
    pub phase: InterviewPhase,
    pub min_questions: u32,
    pub max_questions: u32,
    pub primary_role: InterviewerRole,
}

/// Interview state machine
pub struct InterviewStateMachine {
    current_phase: InterviewPhase,
    phase_question_count: u32,
    total_question_count: u32,
    phase_configs: Vec<PhaseConfig>,
}

impl InterviewStateMachine {
    /// Create new state machine
    pub fn new() -> Self {
        Self {
            current_phase: InterviewPhase::WarmUp,
            phase_question_count: 0,
            total_question_count: 0,
            phase_configs: Self::default_configs(),
        }
    }
    
    /// Get default phase configurations
    fn default_configs() -> Vec<PhaseConfig> {
        vec![
            PhaseConfig {
                phase: InterviewPhase::WarmUp,
                min_questions: 1,
                max_questions: 2,
                primary_role: InterviewerRole::HR,
            },
            PhaseConfig {
                phase: InterviewPhase::Technical,
                min_questions: 3,
                max_questions: 5,
                primary_role: InterviewerRole::Technical,
            },
            PhaseConfig {
                phase: InterviewPhase::Behavioral,
                min_questions: 2,
                max_questions: 3,
                primary_role: InterviewerRole::HR,
            },
            PhaseConfig {
                phase: InterviewPhase::Business,
                min_questions: 2,
                max_questions: 3,
                primary_role: InterviewerRole::Business,
            },
            PhaseConfig {
                phase: InterviewPhase::Questions,
                min_questions: 1,
                max_questions: 2,
                primary_role: InterviewerRole::HR,
            },
        ]
    }
    
    /// Get current phase
    pub fn current_phase(&self) -> InterviewPhase {
        self.current_phase
    }
    
    /// Record a question and check if phase should advance
    pub fn record_question(&mut self) -> Option<InterviewPhase> {
        self.phase_question_count += 1;
        self.total_question_count += 1;
        
        let current_config = self.phase_configs
            .iter()
            .find(|c| c.phase == self.current_phase)?;
        
        // Force advance if reached max questions
        if self.phase_question_count >= current_config.max_questions {
            return self.advance_phase();
        }
        
        None
    }
    
    /// Maybe advance phase based on user performance
    pub fn maybe_advance(&mut self, analysis: &AnalysisResult) -> Option<InterviewPhase> {
        let current_config = self.phase_configs
            .iter()
            .find(|c| c.phase == self.current_phase)?;
        
        // Can advance if reached min questions and excellent performance
        if self.phase_question_count >= current_config.min_questions && analysis.score >= 8.0 {
            return self.advance_phase();
        }
        
        None
    }
    
    /// Advance to next phase
    fn advance_phase(&mut self) -> Option<InterviewPhase> {
        self.phase_question_count = 0;
        
        let next_phase = match self.current_phase {
            InterviewPhase::WarmUp => InterviewPhase::Technical,
            InterviewPhase::Technical => InterviewPhase::Behavioral,
            InterviewPhase::Behavioral => InterviewPhase::Business,
            InterviewPhase::Business => InterviewPhase::Questions,
            InterviewPhase::Questions => InterviewPhase::Completed,
            InterviewPhase::Completed => return None,
        };
        
        self.current_phase = next_phase;
        Some(next_phase)
    }
    
    /// Get progress information
    pub fn progress(&self) -> InterviewProgress {
        InterviewProgress {
            current_phase: self.current_phase,
            phase_question_count: self.phase_question_count,
            total_question_count: self.total_question_count,
            is_completed: self.current_phase == InterviewPhase::Completed,
        }
    }
    
    /// Get primary role for current phase
    #[allow(dead_code)]
    pub fn current_primary_role(&self) -> Option<InterviewerRole> {
        self.phase_configs
            .iter()
            .find(|c| c.phase == self.current_phase)
            .map(|c| c.primary_role)
    }
}

/// Interview progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewProgress {
    pub current_phase: InterviewPhase,
    pub phase_question_count: u32,
    pub total_question_count: u32,
    pub is_completed: bool,
}

impl Default for InterviewStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
