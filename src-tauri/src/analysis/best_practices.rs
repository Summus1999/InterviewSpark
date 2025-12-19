//! Best practices extractor
//!
//! Automatically extracts best answers from high-scoring interview sessions

use crate::db::{Repository, BestPractice, BestPracticesResult, now};
use anyhow::Result;

/// Extractor for best practices from interview history
pub struct BestPracticesExtractor;

impl BestPracticesExtractor {
    /// Extract best practices from high-scoring answers
    ///
    /// # Arguments
    /// * `repository` - Database repository
    /// * `score_threshold` - Minimum score to consider (default 7.5)
    /// * `limit` - Maximum practices to return
    ///
    /// # Returns
    /// * `Ok(BestPracticesResult)` - Extracted best practices
    /// * `Err` - Error if extraction fails
    pub fn extract_best_practices(
        repository: &Repository,
        score_threshold: f32,
        limit: usize,
    ) -> Result<BestPracticesResult> {
        let sessions = repository.get_interview_sessions()?;
        let mut all_practices: Vec<BestPractice> = Vec::new();
        let mut total_analyzed = 0;
        
        for session in sessions.iter() {
            if let Some(session_id) = session.id {
                let answers = repository.get_answers_by_session(session_id)?;
                
                for answer in answers.iter() {
                    total_analyzed += 1;
                    
                    if let Some(answer_id) = answer.id {
                        if let Ok(Some(analysis)) = repository.get_answer_analysis(answer_id) {
                            if analysis.overall_score >= score_threshold {
                                // Extract key points from the answer
                                let key_points = Self::extract_key_points(&answer.answer);
                                
                                all_practices.push(BestPractice {
                                    question: answer.question.clone(),
                                    answer: answer.answer.clone(),
                                    score: analysis.overall_score,
                                    session_id,
                                    extracted_at: now(),
                                    key_points,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by score descending
        all_practices.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        // Take top practices
        let practices: Vec<BestPractice> = all_practices
            .into_iter()
            .take(limit)
            .collect();
        
        Ok(BestPracticesResult {
            practices,
            total_analyzed,
            threshold_score: score_threshold,
            generated_at: now(),
        })
    }
    
    /// Extract key points from answer text
    fn extract_key_points(answer: &str) -> Vec<String> {
        let mut key_points = Vec::new();
        
        // Split by common separators
        let sentences: Vec<&str> = answer
            .split(|c| c == '.' || c == '。' || c == '；' || c == ';' || c == '\n')
            .filter(|s| !s.trim().is_empty())
            .collect();
        
        // Extract sentences with key indicators
        let key_indicators = vec![
            "首先", "其次", "第一", "第二", "关键", "重点", "核心",
            "主要", "另外", "此外", "最后", "总结", "因此", "所以"
        ];
        
        for sentence in sentences.iter() {
            let trimmed = sentence.trim();
            if trimmed.len() < 10 {
                continue;
            }
            
            // Check for key indicators
            for indicator in key_indicators.iter() {
                if trimmed.contains(indicator) {
                    key_points.push(trimmed.to_string());
                    break;
                }
            }
            
            // Limit to 5 key points
            if key_points.len() >= 5 {
                break;
            }
        }
        
        // If no key indicators found, take first 3 sentences
        if key_points.is_empty() {
            for sentence in sentences.iter().take(3) {
                let trimmed = sentence.trim();
                if trimmed.len() >= 10 {
                    key_points.push(trimmed.to_string());
                }
            }
        }
        
        key_points
    }
}
