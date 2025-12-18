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

/// Helper function to get current timestamp as ISO 8601 string
pub fn now() -> String {
    Utc::now().to_rfc3339()
}
