//! Interview profile generator
//! 
//! Analyzes user's interview history to generate comprehensive profile
//! with multi-dimensional scoring and improvement suggestions

use crate::db::{Repository, InterviewProfile, ProfileDimension, now};
use anyhow::Result;

/// Profile generator for creating user interview profiles
pub struct ProfileGenerator;

impl ProfileGenerator {
    /// Generate interview profile based on user's session history
    /// 
    /// # Arguments
    /// * `repository` - Database repository
    /// * `user_id` - User identifier (default to "default_user")
    /// * `session_limit` - Maximum number of recent sessions to analyze (None for all)
    /// 
    /// # Returns
    /// * `Ok(InterviewProfile)` - Generated profile with dimension scores
    /// * `Err` - Error if profile generation fails
    pub fn generate_profile(
        repository: &Repository,
        user_id: &str,
        session_limit: Option<usize>,
    ) -> Result<InterviewProfile> {
        // Get all sessions
        let sessions = repository.get_interview_sessions()?;
        
        if sessions.is_empty() {
            return Self::create_empty_profile(user_id);
        }
        
        // Limit to recent sessions if specified
        let sessions_to_analyze = if let Some(limit) = session_limit {
            sessions.into_iter().take(limit).collect::<Vec<_>>()
        } else {
            sessions
        };
        
        let total_sessions = sessions_to_analyze.len() as i32;
        
        // Collect all answers with analysis
        let mut all_scores = Vec::new();
        let mut dimension_scores = ProfileDimension {
            technical_depth: 0.0,
            communication: 0.0,
            problem_solving: 0.0,
            domain_knowledge: 0.0,
            adaptability: 0.0,
        };
        
        for session in sessions_to_analyze.iter() {
            if let Some(session_id) = session.id {
                let answers = repository.get_answers_by_session(session_id)?;
                
                for answer in answers {
                    if let Some(answer_id) = answer.id {
                        if let Ok(Some(analysis)) = repository.get_answer_analysis(answer_id) {
                            all_scores.push(analysis.overall_score);
                            
                            // Map scores to dimensions
                            dimension_scores.technical_depth += analysis.content_score;
                            dimension_scores.communication += analysis.expression_score.unwrap_or(7.0);
                            dimension_scores.problem_solving += analysis.logic_score;
                            dimension_scores.domain_knowledge += analysis.job_match_score;
                            dimension_scores.adaptability += analysis.keyword_coverage / 10.0;
                        }
                    }
                }
            }
        }
        
        // Calculate average scores
        let answer_count = all_scores.len() as f32;
        let average_score = if answer_count > 0.0 {
            all_scores.iter().sum::<f32>() / answer_count
        } else {
            0.0
        };
        
        // Normalize dimension scores to 0-100 scale
        if answer_count > 0.0 {
            dimension_scores.technical_depth = (dimension_scores.technical_depth / answer_count) * 10.0;
            dimension_scores.communication = (dimension_scores.communication / answer_count) * 10.0;
            dimension_scores.problem_solving = (dimension_scores.problem_solving / answer_count) * 10.0;
            dimension_scores.domain_knowledge = (dimension_scores.domain_knowledge / answer_count) * 10.0;
            dimension_scores.adaptability = (dimension_scores.adaptability / answer_count) * 10.0;
        }
        
        // Identify strongest and weakest dimensions
        let dimensions_map = vec![
            ("technical_depth", dimension_scores.technical_depth),
            ("communication", dimension_scores.communication),
            ("problem_solving", dimension_scores.problem_solving),
            ("domain_knowledge", dimension_scores.domain_knowledge),
            ("adaptability", dimension_scores.adaptability),
        ];
        
        let strongest = dimensions_map
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(name, _)| name.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        let weakest = dimensions_map
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(name, _)| name.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        // Generate improvement suggestions
        let suggestions = Self::generate_suggestions(&dimension_scores, &weakest);
        
        Ok(InterviewProfile {
            user_id: user_id.to_string(),
            dimensions: dimension_scores,
            total_sessions,
            average_score,
            strongest_dimension: strongest,
            weakest_dimension: weakest,
            improvement_suggestions: suggestions,
            generated_at: now(),
        })
    }
    
    /// Create empty profile for users with no session history
    fn create_empty_profile(user_id: &str) -> Result<InterviewProfile> {
        Ok(InterviewProfile {
            user_id: user_id.to_string(),
            dimensions: ProfileDimension {
                technical_depth: 0.0,
                communication: 0.0,
                problem_solving: 0.0,
                domain_knowledge: 0.0,
                adaptability: 0.0,
            },
            total_sessions: 0,
            average_score: 0.0,
            strongest_dimension: "none".to_string(),
            weakest_dimension: "none".to_string(),
            improvement_suggestions: vec![
                "Start your first interview practice session to build your profile".to_string(),
            ],
            generated_at: now(),
        })
    }
    
    /// Generate improvement suggestions based on dimension scores
    fn generate_suggestions(dimensions: &ProfileDimension, weakest: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Suggestions based on weakest dimension
        match weakest {
            "technical_depth" => {
                suggestions.push("深入学习目标领域的技术细节和底层原理".to_string());
                suggestions.push("多练习技术深度类问题，提升专业知识储备".to_string());
            }
            "communication" => {
                suggestions.push("练习结构化表达，使用总分总或STAR法则".to_string());
                suggestions.push("注意语速和停顿，提高表达清晰度".to_string());
            }
            "problem_solving" => {
                suggestions.push("多练习算法和系统设计问题".to_string());
                suggestions.push("培养逻辑思维，学会拆解复杂问题".to_string());
            }
            "domain_knowledge" => {
                suggestions.push("深入了解目标岗位的业务领域知识".to_string());
                suggestions.push("关注行业动态和最佳实践".to_string());
            }
            "adaptability" => {
                suggestions.push("练习应对突发问题，提升灵活应变能力".to_string());
                suggestions.push("扩展知识面，增强跨领域理解能力".to_string());
            }
            _ => {
                suggestions.push("继续保持练习，全面提升各维度能力".to_string());
            }
        }
        
        // General suggestions based on overall performance
        if dimensions.technical_depth < 50.0 {
            suggestions.push("加强技术基础学习".to_string());
        }
        if dimensions.communication < 50.0 {
            suggestions.push("提升沟通表达能力".to_string());
        }
        
        suggestions
    }
}
