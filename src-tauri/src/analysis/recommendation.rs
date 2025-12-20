//! Smart practice recommendation engine
//!
//! Analyzes user's weak areas and recommends questions for practice

use crate::db::{Repository, PracticeRecommendation, RecommendationResult, ProfileDimension, now};
use crate::analysis::ProfileGenerator;
use anyhow::Result;

/// Recommendation engine for suggesting practice questions
pub struct RecommendationEngine;

impl RecommendationEngine {
    /// Generate practice recommendations based on user's profile
    ///
    /// # Arguments
    /// * `repository` - Database repository
    /// * `user_id` - User identifier
    /// * `limit` - Maximum recommendations to return
    ///
    /// # Returns
    /// * `Ok(RecommendationResult)` - List of recommended questions
    /// * `Err` - Error if generation fails
    pub fn generate_recommendations(
        repository: &Repository,
        user_id: &str,
        limit: usize,
    ) -> Result<RecommendationResult> {
        // Get user profile to identify weak areas
        let profile = ProfileGenerator::generate_profile(repository, user_id, None)?;
        
        // Identify weak dimensions (score < 60)
        let weak_dimensions = Self::identify_weak_dimensions(&profile.dimensions);
        
        // Get all questions from bank
        let all_questions = repository.get_question_bank()?;
        
        if all_questions.is_empty() {
            return Ok(RecommendationResult {
                recommendations: vec![],
                weak_dimensions,
                total_available: 0,
                generated_at: now(),
            });
        }
        
        // Score and rank questions
        let mut scored_questions: Vec<(i64, String, f32, String, String)> = Vec::new();
        
        for question in all_questions.iter() {
            if let Some(id) = question.id {
                let (score, dimension, reason) = Self::score_question(
                    &question.question,
                    question.job_category.as_deref(),
                    &weak_dimensions,
                    &profile.dimensions,
                );
                scored_questions.push((id, question.question.clone(), score, dimension, reason));
            }
        }
        
        // Sort by score (higher is better)
        scored_questions.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        
        // Take top recommendations
        let recommendations: Vec<PracticeRecommendation> = scored_questions
            .into_iter()
            .take(limit)
            .enumerate()
            .map(|(_idx, (id, question, score, dimension, reason))| {
                PracticeRecommendation {
                    question_id: id,
                    question,
                    reason,
                    priority: Self::score_to_priority(score),
                    dimension,
                    estimated_improvement: score * 0.1,
                }
            })
            .collect();
        
        Ok(RecommendationResult {
            recommendations,
            weak_dimensions,
            total_available: all_questions.len() as i32,
            generated_at: now(),
        })
    }
    
    /// Identify dimensions with scores below threshold
    fn identify_weak_dimensions(dimensions: &ProfileDimension) -> Vec<String> {
        let threshold = 60.0;
        let mut weak = Vec::new();
        
        if dimensions.technical_depth < threshold {
            weak.push("technical_depth".to_string());
        }
        if dimensions.communication < threshold {
            weak.push("communication".to_string());
        }
        if dimensions.problem_solving < threshold {
            weak.push("problem_solving".to_string());
        }
        if dimensions.domain_knowledge < threshold {
            weak.push("domain_knowledge".to_string());
        }
        if dimensions.adaptability < threshold {
            weak.push("adaptability".to_string());
        }
        
        weak
    }
    
    /// Score a question based on relevance to weak areas
    fn score_question(
        question: &str,
        category: Option<&str>,
        weak_dimensions: &[String],
        _dimensions: &ProfileDimension,
    ) -> (f32, String, String) {
        let q_lower = question.to_lowercase();
        let mut score: f32 = 50.0; // Base score
        let mut primary_dimension = "general".to_string();
        let mut reason = "基础练习题目".to_string();
        
        // Keyword-based dimension detection
        let dimension_keywords = vec![
            ("technical_depth", vec!["原理", "底层", "实现", "架构", "性能", "算法", "数据结构"]),
            ("communication", vec!["介绍", "描述", "解释", "沟通", "表达", "团队"]),
            ("problem_solving", vec!["解决", "问题", "调试", "优化", "方案", "设计"]),
            ("domain_knowledge", vec!["业务", "行业", "领域", "经验", "项目"]),
            ("adaptability", vec!["变化", "学习", "新技术", "挑战", "压力"]),
        ];
        
        for (dim, keywords) in dimension_keywords.iter() {
            for kw in keywords {
                if q_lower.contains(kw) {
                    if weak_dimensions.contains(&dim.to_string()) {
                        score += 20.0;
                        primary_dimension = dim.to_string();
                        reason = format!("针对薄弱维度「{}」的练习", Self::dimension_name(dim));
                    } else {
                        score += 5.0;
                    }
                    break;
                }
            }
        }
        
        // Boost score for questions in weak dimensions
        if let Some(cat) = category {
            let cat_lower = cat.to_lowercase();
            for weak in weak_dimensions {
                if cat_lower.contains(weak) {
                    score += 15.0;
                    reason = format!("分类匹配薄弱维度: {}", weak);
                }
            }
        }
        
        (score.min(100.0), primary_dimension, reason)
    }
    
    /// Convert score to priority level (1-5)
    fn score_to_priority(score: f32) -> u8 {
        match score as u32 {
            0..=20 => 1,
            21..=40 => 2,
            41..=60 => 3,
            61..=80 => 4,
            _ => 5,
        }
    }
    
    /// Get dimension display name
    fn dimension_name(dim: &str) -> &str {
        match dim {
            "technical_depth" => "技术深度",
            "communication" => "沟通表达",
            "problem_solving" => "问题解决",
            "domain_knowledge" => "领域知识",
            "adaptability" => "应变能力",
            _ => "综合能力",
        }
    }
}
