//! Data models for database entities

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Resume entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Job description entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDescription {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Interview session entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewSession {
    pub id: Option<i64>,
    pub resume_id: Option<i64>,
    pub job_description_id: Option<i64>,
    pub questions: Vec<String>,
    pub created_at: String,
}

/// Interview answer entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewAnswer {
    pub id: Option<i64>,
    pub session_id: i64,
    pub question_index: i32,
    pub question: String,
    pub answer: String,
    pub feedback: String,
    pub created_at: String,
}

/// Question bank item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionBankItem {
    pub id: Option<i64>,
    pub question: String,
    pub best_answer: Option<String>,
    pub notes: Option<String>,
    pub job_category: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Answer analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerAnalysis {
    pub id: Option<i64>,
    pub answer_id: i64,
    pub content_score: f32,              // Content dimension score (1-10)
    pub logic_score: f32,                // Logic score (1-10)
    pub job_match_score: f32,            // Job match score (1-10)
    pub keyword_coverage: f32,           // Keyword coverage ratio (0-100%)
    pub expression_score: Option<f32>,   // Expression dimension score (1-10)
    pub overall_score: f32,              // Overall score (1-10)
    pub strengths: String,               // JSON array of strength points
    pub weaknesses: String,              // JSON array of weakness points
    pub suggestions: String,             // JSON array of suggestions
    pub created_at: String,
}

/// Session report with comprehensive analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReport {
    pub id: Option<i64>,
    pub session_id: i64,
    pub overall_score: f32,              // Overall session score (1-10)
    pub content_analysis: String,        // Content dimension analysis (JSON)
    pub expression_analysis: Option<String>, // Expression dimension analysis (JSON)
    pub summary: String,                 // Overall summary
    pub improvements: String,            // JSON array of improvement areas
    pub key_takeaways: String,          // JSON array of key points
    pub reference_answers: Option<String>, // JSON object with reference answers
    pub generated_at: String,
    pub api_response_time: Option<i32>,  // Response time in ms
}

/// Performance statistics for tracking user growth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub id: Option<i64>,
    pub session_date: String,           // Date in format YYYY-MM-DD
    pub total_sessions: i32,            // Total sessions up to this date
    pub average_score: f32,             // Average score
    pub content_avg: f32,               // Content dimension average
    pub expression_avg: Option<f32>,    // Expression dimension average
    pub highest_score: f32,             // Highest score in this period
    pub lowest_score: f32,              // Lowest score in this period
    pub improvement_trend: f32,         // Score trend (-10 to +10)
    pub recorded_at: String,
}

/// Question tag entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionTag {
    pub id: Option<i64>,
    pub name: String,
    pub color: String,
    pub created_at: String,
}

/// Question tag mapping entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagMapping {
    pub id: Option<i64>,
    pub question_bank_id: i64,
    pub tag_id: i64,
    pub created_at: String,
}

/// Interview profile dimension scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDimension {
    pub technical_depth: f32,        // Technical depth score (0-100)
    pub communication: f32,          // Communication score (0-100)
    pub problem_solving: f32,        // Problem solving score (0-100)
    pub domain_knowledge: f32,       // Domain knowledge score (0-100)
    pub adaptability: f32,           // Adaptability score (0-100)
    pub job_intention: f32,          // Job intention match score (0-100)
}

/// Complete interview profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewProfile {
    pub user_id: String,
    pub dimensions: ProfileDimension,
    pub total_sessions: i32,
    pub average_score: f32,
    pub strongest_dimension: String,
    pub weakest_dimension: String,
    pub improvement_suggestions: Vec<String>,
    pub generated_at: String,
}

/// Practice recommendation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeRecommendation {
    pub question_id: i64,
    pub question: String,
    pub reason: String,
    pub priority: u8,              // 1-5, higher is more important
    pub dimension: String,          // Which dimension this targets
    pub estimated_improvement: f32, // Expected score improvement
}

/// Recommendation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationResult {
    pub recommendations: Vec<PracticeRecommendation>,
    pub weak_dimensions: Vec<String>,
    pub total_available: i32,
    pub generated_at: String,
}

/// Best practice extracted from high-scoring answers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestPractice {
    pub question: String,
    pub answer: String,
    pub score: f32,
    pub session_id: i64,
    pub extracted_at: String,
    pub key_points: Vec<String>,
}

/// Best practices result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestPracticesResult {
    pub practices: Vec<BestPractice>,
    pub total_analyzed: i32,
    pub threshold_score: f32,
    pub generated_at: String,
}

/// Industry benchmark data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryBenchmark {
    pub dimension: String,
    pub user_score: f32,
    pub industry_avg: f32,
    pub industry_top: f32,
    pub percentile: f32,        // User's percentile rank (0-100)
}

/// Industry comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryComparisonResult {
    pub benchmarks: Vec<IndustryBenchmark>,
    pub overall_percentile: f32,
    pub user_level: String,      // "Beginner", "Intermediate", "Advanced", "Expert"
    pub comparison_count: i32,   // Number of sessions used for comparison
    pub generated_at: String,
}

/// Helper function to get current timestamp as ISO 8601 string
pub fn now() -> String {
    Utc::now().to_rfc3339()
}
