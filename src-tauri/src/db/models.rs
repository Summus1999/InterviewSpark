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

/// Helper function to get current timestamp as ISO 8601 string
pub fn now() -> String {
    Utc::now().to_rfc3339()
}
