//! Scoring engine for interview answers
//! Implements 1-10 scoring algorithm with weighted dimensions

use super::content::ContentAnalysisResult;

/// Scoring engine for interview evaluation
pub struct ScoringEngine;

/// Scoring weights for different dimensions
#[derive(Debug, Clone)]
pub struct ScoringWeights {
    pub logic_weight: f32,           // 30%
    pub job_match_weight: f32,       // 35%
    pub keyword_weight: f32,         // 20%
    pub expression_weight: f32,      // 15%
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            logic_weight: 0.30,
            job_match_weight: 0.35,
            keyword_weight: 0.20,
            expression_weight: 0.15,
        }
    }
}

/// Comprehensive scoring result
#[derive(Debug, Clone)]
pub struct ScoringResult {
    pub overall_score: f32,          // 1-10: Overall score
    pub content_score: f32,          // 1-10: Content quality score
    pub expression_score: Option<f32>, // 1-10: Optional expression score
    pub score_breakdown: ScoreBreakdown,
    pub score_grade: String,         // A+, A, B+, B, C+, C, D, F
}

/// Detailed score breakdown
#[derive(Debug, Clone)]
pub struct ScoreBreakdown {
    pub logic: f32,
    pub job_match: f32,
    pub keyword_coverage: f32,
    pub expression: Option<f32>,
}

impl ScoringEngine {
    /// Calculate overall score from content analysis
    pub fn calculate_score(
        content_analysis: &ContentAnalysisResult,
        weights: Option<ScoringWeights>,
    ) -> ScoringResult {
        let weights = weights.unwrap_or_default();

        // Normalize keyword coverage to 1-10 scale
        let keyword_score = (content_analysis.keyword_coverage / 10.0).min(10.0);

        // Calculate content score (weighted average of logic, match, keywords)
        let content_score = (content_analysis.logic_score * weights.logic_weight
            + content_analysis.job_match_score * weights.job_match_weight
            + keyword_score * weights.keyword_weight)
            / (weights.logic_weight + weights.job_match_weight + weights.keyword_weight);

        // Overall score (assuming no expression score for now)
        let overall_score = content_score;

        let score_breakdown = ScoreBreakdown {
            logic: content_analysis.logic_score,
            job_match: content_analysis.job_match_score,
            keyword_coverage: keyword_score,
            expression: None,
        };

        let score_grade = Self::calculate_grade(overall_score);

        ScoringResult {
            overall_score: overall_score.max(1.0).min(10.0),
            content_score: content_score.max(1.0).min(10.0),
            expression_score: None,
            score_breakdown,
            score_grade,
        }
    }

    /// Calculate with expression score
    pub fn calculate_score_with_expression(
        content_analysis: &ContentAnalysisResult,
        expression_score: f32,
        weights: Option<ScoringWeights>,
    ) -> ScoringResult {
        let weights = weights.unwrap_or_default();

        let keyword_score = (content_analysis.keyword_coverage / 10.0).min(10.0);

        // Content score
        let content_score = (content_analysis.logic_score * weights.logic_weight
            + content_analysis.job_match_score * weights.job_match_weight
            + keyword_score * weights.keyword_weight)
            / (weights.logic_weight + weights.job_match_weight + weights.keyword_weight);

        // Overall score with expression
        let overall_score = content_score * (1.0 - weights.expression_weight)
            + expression_score * weights.expression_weight;

        let score_breakdown = ScoreBreakdown {
            logic: content_analysis.logic_score,
            job_match: content_analysis.job_match_score,
            keyword_coverage: keyword_score,
            expression: Some(expression_score),
        };

        let score_grade = Self::calculate_grade(overall_score);

        ScoringResult {
            overall_score: overall_score.max(1.0).min(10.0),
            content_score: content_score.max(1.0).min(10.0),
            expression_score: Some(expression_score),
            score_breakdown,
            score_grade,
        }
    }

    /// Calculate letter grade from numerical score
    fn calculate_grade(score: f32) -> String {
        match score {
            s if s >= 9.0 => "A+".to_string(),
            s if s >= 8.5 => "A".to_string(),
            s if s >= 8.0 => "A-".to_string(),
            s if s >= 7.5 => "B+".to_string(),
            s if s >= 7.0 => "B".to_string(),
            s if s >= 6.5 => "B-".to_string(),
            s if s >= 6.0 => "C+".to_string(),
            s if s >= 5.0 => "C".to_string(),
            s if s >= 4.0 => "D".to_string(),
            _ => "F".to_string(),
        }
    }

    /// Get improvement suggestions based on scores
    pub fn get_improvement_suggestions(
        score_breakdown: &ScoreBreakdown,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        if score_breakdown.logic < 5.0 {
            suggestions.push(
                "建议多使用逻辑连词，如'首先、其次、最后'等，使答案结构更清晰".to_string()
            );
        }

        if score_breakdown.job_match < 5.0 {
            suggestions.push(
                "建议更多地引用职位描述中的关键词和要求，提高岗位匹配度".to_string()
            );
        }

        if score_breakdown.keyword_coverage < 50.0 {
            suggestions.push(
                "建议补充岗位相关的专业术语和技能关键词".to_string()
            );
        }

        if let Some(expr) = score_breakdown.expression {
            if expr < 5.0 {
                suggestions.push(
                    "建议改进表达方式，使用更清晰、更专业的语言".to_string()
                );
            }
        }

        if suggestions.is_empty() {
            suggestions.push("保持目前的水平，继续完善细节".to_string());
        }

        suggestions
    }
}
