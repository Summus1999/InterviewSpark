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
    #[allow(dead_code)]
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

/// STAR method score breakdown
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct STARScoreBreakdown {
    pub situation: f32,    // 1-10: Situation description score
    pub task: f32,         // 1-10: Task explanation score
    pub action: f32,       // 1-10: Action description score
    pub result: f32,       // 1-10: Result quantification score
}

/// STAR scoring result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct STARScoringResult {
    pub overall_score: f32,        // 1-10: Overall STAR score
    pub breakdown: STARScoreBreakdown,
    pub completeness: f32,         // 0-100: Percentage of STAR elements present
    pub suggestions: Vec<String>,  // Improvement suggestions for each dimension
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
    #[allow(dead_code)]
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

/// STAR Scoring Engine
pub struct STARScoringEngine;

impl STARScoringEngine {
    /// Calculate STAR score from answer text
    pub fn calculate_star_score(answer: &str) -> STARScoringResult {
        let situation_score = Self::evaluate_situation(answer);
        let task_score = Self::evaluate_task(answer);
        let action_score = Self::evaluate_action(answer);
        let result_score = Self::evaluate_result(answer);

        let overall_score = (situation_score + task_score + action_score + result_score) / 4.0;
        let completeness = Self::calculate_completeness(situation_score, task_score, action_score, result_score);

        let breakdown = STARScoreBreakdown {
            situation: situation_score,
            task: task_score,
            action: action_score,
            result: result_score,
        };

        let suggestions = Self::generate_star_suggestions(&breakdown);

        STARScoringResult {
            overall_score,
            breakdown,
            completeness,
            suggestions,
        }
    }

    /// Evaluate Situation dimension
    fn evaluate_situation(answer: &str) -> f32 {
        let answer_lower = answer.to_lowercase();
        let situation_keywords = ["背景", "情况", "当时", "项目中", "团队里", "环境", "场景", "之前"];
        
        let mut score: f32 = 3.0; // Base score
        
        for keyword in situation_keywords.iter() {
            if answer_lower.contains(keyword) {
                score += 1.0;
            }
        }
        
        // Bonus for context richness
        if answer.len() > 100 && answer.contains("在") {
            score += 1.5;
        }
        
        score.min(10.0)
    }

    /// Evaluate Task dimension
    fn evaluate_task(answer: &str) -> f32 {
        let answer_lower = answer.to_lowercase();
        let task_keywords = ["任务", "目标", "需要", "负责", "职责", "要求", "期望", "挑战"];
        
        let mut score: f32 = 3.0;
        
        for keyword in task_keywords.iter() {
            if answer_lower.contains(keyword) {
                score += 1.0;
            }
        }
        
        // Bonus for clear task definition
        if answer_lower.contains("目标是") || answer_lower.contains("任务是") {
            score += 1.5;
        }
        
        score.min(10.0)
    }

    /// Evaluate Action dimension
    fn evaluate_action(answer: &str) -> f32 {
        let answer_lower = answer.to_lowercase();
        let action_keywords = ["采取", "实施", "执行", "通过", "使用", "做了", "进行", "设计", "开发"];
        
        let mut score: f32 = 3.0;
        
        for keyword in action_keywords.iter() {
            if answer_lower.contains(keyword) {
                score += 0.8;
            }
        }
        
        // Bonus for detailed action steps
        if answer.contains("首先") || answer.contains("然后") || answer.contains("最后") {
            score += 2.0;
        }
        
        score.min(10.0)
    }

    /// Evaluate Result dimension
    fn evaluate_result(answer: &str) -> f32 {
        let answer_lower = answer.to_lowercase();
        let result_keywords = ["结果", "效果", "提升", "降低", "完成", "达成", "成功", "实现"];
        
        let mut score: f32 = 3.0;
        
        for keyword in result_keywords.iter() {
            if answer_lower.contains(keyword) {
                score += 1.0;
            }
        }
        
        // Bonus for quantified results
        let has_numbers = answer.chars().any(|c| c.is_numeric());
        if has_numbers && (answer.contains("%") || answer.contains("倍") || answer.contains("次")) {
            score += 2.5;
        }
        
        score.min(10.0)
    }

    /// Calculate completeness percentage
    fn calculate_completeness(s: f32, t: f32, a: f32, r: f32) -> f32 {
        let threshold = 5.0; // Minimum score to be considered "present"
        let mut present_count = 0;
        
        if s >= threshold { present_count += 1; }
        if t >= threshold { present_count += 1; }
        if a >= threshold { present_count += 1; }
        if r >= threshold { present_count += 1; }
        
        (present_count as f32 / 4.0) * 100.0
    }

    /// Generate improvement suggestions
    fn generate_star_suggestions(breakdown: &STARScoreBreakdown) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if breakdown.situation < 6.0 {
            suggestions.push("建议补充情境背景，说明当时的环境、团队情况或项目背景".to_string());
        }
        
        if breakdown.task < 6.0 {
            suggestions.push("建议明确描述任务目标，说明你需要完成什么、面临什么挑战".to_string());
        }
        
        if breakdown.action < 6.0 {
            suggestions.push("建议详细说明行动步骤，使用'首先、然后、最后'等连词展示执行过程".to_string());
        }
        
        if breakdown.result < 6.0 {
            suggestions.push("建议量化结果，用数据（百分比、倍数等）说明效果和成果".to_string());
        }
        
        if suggestions.is_empty() {
            suggestions.push("STAR结构完整，继续保持这种回答方式".to_string());
        }
        
        suggestions
    }
}
