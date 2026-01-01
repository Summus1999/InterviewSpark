// Agent scheduler for multi-agent interview rotation

use super::agents::{InterviewerAgent, InterviewerRole, InterviewContext, ConversationTurn, AnalysisResult};
use super::state_machine::InterviewPhase;
use anyhow::{Result, anyhow};

/// Agent scheduler
pub struct AgentScheduler {
    agents: Vec<Box<dyn InterviewerAgent>>,
    current_index: usize,
    rotation_strategy: RotationStrategy,
}

/// Rotation strategy enum
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RotationStrategy {
    /// Fixed order rotation: Tech → HR → Business → Tech...
    FixedOrder,
    /// Phase-based: all technical in tech phase, all HR in HR phase
    PhaseBased,
    /// Random rotation
    Random,
}

impl AgentScheduler {
    /// Create new scheduler with agents
    pub fn new(agents: Vec<Box<dyn InterviewerAgent>>) -> Self {
        Self {
            agents,
            current_index: 0,
            rotation_strategy: RotationStrategy::FixedOrder,
        }
    }
    
    /// Set rotation strategy
    #[allow(dead_code)]
    pub fn with_strategy(mut self, strategy: RotationStrategy) -> Self {
        self.rotation_strategy = strategy;
        self
    }
    
    /// Get current agent
    pub fn current_agent(&self) -> &dyn InterviewerAgent {
        self.agents[self.current_index].as_ref()
    }
    
    /// Switch to next agent
    #[allow(dead_code)]
    pub fn next_agent(&mut self) -> &dyn InterviewerAgent {
        match self.rotation_strategy {
            RotationStrategy::FixedOrder => {
                self.current_index = (self.current_index + 1) % self.agents.len();
            }
            RotationStrategy::Random => {
                use rand::Rng;
                self.current_index = rand::thread_rng().gen_range(0..self.agents.len());
            }
            RotationStrategy::PhaseBased => {
                // Controlled by state machine via select_by_phase
            }
        }
        self.current_agent()
    }
    
    /// Select agent by interview phase
    pub fn select_by_phase(&mut self, phase: InterviewPhase) -> &dyn InterviewerAgent {
        let target_role = match phase {
            InterviewPhase::WarmUp => InterviewerRole::HR,
            InterviewPhase::Technical => InterviewerRole::Technical,
            InterviewPhase::Behavioral => InterviewerRole::HR,
            InterviewPhase::Business => InterviewerRole::Business,
            InterviewPhase::Questions => InterviewerRole::HR,
            InterviewPhase::Completed => InterviewerRole::HR,
        };
        
        self.current_index = self.agents
            .iter()
            .position(|a| a.role() == target_role)
            .unwrap_or(0);
        
        self.current_agent()
    }
    
    /// Execute one interview turn (generate question)
    pub async fn execute_turn(
        &mut self,
        context: &mut InterviewContext,
    ) -> Result<ConversationTurn> {
        let agent = self.current_agent();
        
        // Generate question
        let question = agent.generate_question(context).await?;
        
        // Create conversation turn (waiting for user answer)
        let turn = ConversationTurn {
            role: agent.role(),
            role_name: agent.role_name().to_string(),
            question,
            answer: None,
            analysis: None,
        };
        
        context.conversation_history.push(turn.clone());
        Ok(turn)
    }
    
    /// Process user answer
    pub async fn process_answer(
        &self,
        context: &mut InterviewContext,
        answer: String,
    ) -> Result<AnalysisResult> {
        let agent = self.current_agent();
        
        // Extract question and record answer
        let question = {
            let last_turn = context.conversation_history.last_mut()
                .ok_or_else(|| anyhow!("No conversation turn"))?;
            last_turn.answer = Some(answer.clone());
            last_turn.question.clone()
        };
        
        // Analyze answer (no mutable borrow here)
        let analysis = agent.analyze_answer(
            &question,
            &answer,
            context,
        ).await?;
        
        // Update analysis
        if let Some(last_turn) = context.conversation_history.last_mut() {
            last_turn.analysis = Some(analysis.clone());
        }
        
        Ok(analysis)
    }
    
    /// Check if should follow up
    #[allow(dead_code)]
    pub async fn should_follow_up(
        &self,
        answer: &str,
        analysis: &AnalysisResult,
    ) -> bool {
        let agent = self.current_agent();
        agent.should_follow_up(answer, analysis).await
    }
}
