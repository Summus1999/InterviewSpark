//! Dashboard service for aggregating statistics and analytics data

use crate::db::{Repository, InterviewSession};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Dashboard statistics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_sessions: i32,
    pub average_score: f32,
    pub highest_score: f32,
    pub recent_improvement: f32,
}

/// Top question entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopQuestion {
    pub question: String,
    pub count: i32,
}

/// Weak area entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeakArea {
    pub area: String,
    pub average_score: f32,
    pub suggestion: String,
}

/// Recent session entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentSessionInfo {
    pub id: i64,
    pub created_at: String,
    pub question_count: i32,
    pub overall_score: Option<f32>,
}

/// Complete dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub stats: DashboardStats,
    pub top_questions: Vec<TopQuestion>,
    pub weak_areas: Vec<WeakArea>,
    pub recent_sessions: Vec<RecentSessionInfo>,
}

/// Dashboard service for aggregating data
pub struct DashboardService {
    db: Arc<Repository>,
}

impl DashboardService {
    /// Create new dashboard service instance
    pub fn new(db: Arc<Repository>) -> Self {
        Self { db }
    }

    /// Get complete dashboard data
    pub fn get_dashboard_data(&self) -> Result<DashboardData> {
        // Get basic statistics
        let total_sessions = self.db.get_total_sessions_count()?;
        let average_score = self.db.get_average_score()?;
        let highest_score = self.db.get_highest_score()?;
        
        // Calculate recent improvement (compare last 5 to first 5)
        let recent_improvement = if total_sessions >= 5 {
            let (_, avg_overall, _, improvement_rate, _) = 
                self.db.get_statistics(None)?;
            improvement_rate
        } else {
            0.0
        };

        let stats = DashboardStats {
            total_sessions,
            average_score,
            highest_score,
            recent_improvement,
        };

        // Get top questions (top 10)
        let top_question_tuples = self.db.get_top_questions(10)?;
        let top_questions: Vec<TopQuestion> = top_question_tuples
            .into_iter()
            .map(|(question, count)| TopQuestion { question, count })
            .collect();

        // Get weak areas
        let weak_area_tuples = self.db.get_weak_areas()?;
        let weak_areas: Vec<WeakArea> = weak_area_tuples
            .into_iter()
            .map(|(area, score)| {
                let suggestion = get_improvement_suggestion(&area, score);
                WeakArea {
                    area,
                    average_score: score,
                    suggestion,
                }
            })
            .collect();

        // Get recent sessions (last 5)
        let sessions = self.db.get_recent_sessions(5)?;
        let mut recent_sessions = Vec::new();
        for session in sessions {
            let question_count = session.questions.len() as i32;
            let overall_score = self
                .db
                .get_session_report(session.id.unwrap_or(0))?
                .map(|r| r.overall_score);
            
            recent_sessions.push(RecentSessionInfo {
                id: session.id.unwrap_or(0),
                created_at: session.created_at,
                question_count,
                overall_score,
            });
        }

        Ok(DashboardData {
            stats,
            top_questions,
            weak_areas,
            recent_sessions,
        })
    }
}

/// Generate improvement suggestion for a weak area
fn get_improvement_suggestion(area: &str, score: f32) -> String {
    match area {
        "Communication" => {
            if score < 50.0 {
                "Practice clear articulation and concise explanations".to_string()
            } else {
                "Focus on delivering answers more structured and organized".to_string()
            }
        }
        "Problem Solving" => {
            if score < 50.0 {
                "Work on breaking down complex problems into smaller steps".to_string()
            } else {
                "Enhance your analytical thinking and approach".to_string()
            }
        }
        "Technical Depth" => {
            if score < 50.0 {
                "Deep dive into technical concepts and best practices".to_string()
            } else {
                "Explore advanced topics and implementations".to_string()
            }
        }
        "Presentation" => {
            if score < 50.0 {
                "Improve delivery pace, tone, and body language".to_string()
            } else {
                "Refine presentation flow and visual communication".to_string()
            }
        }
        _ => "Continue improving in this area".to_string(),
    }
}
