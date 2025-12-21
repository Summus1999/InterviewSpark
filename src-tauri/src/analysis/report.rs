//! Comprehensive report generation for interview sessions

use crate::api::SiliconFlowClient;
use crate::db::{Repository, SessionReport};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Instant;
use tokio::time::{timeout, Duration};

/// Comprehensive report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveReport {
    pub summary: String,
    pub overall_score: f32,
    pub improvements: Vec<String>,
    pub key_takeaways: Vec<String>,
}

/// Report generator for interview sessions
pub struct ReportGenerator;

impl ReportGenerator {
    /// Generate comprehensive report for a session
    #[allow(dead_code)]
    pub async fn generate_report(
        session_id: i64,
        api_client: &SiliconFlowClient,
        db: &Repository,
    ) -> Result<SessionReport> {
        Self::generate_report_with_model(session_id, api_client, db, None).await
    }

    /// Generate comprehensive report with optional model override
    pub async fn generate_report_with_model(
        session_id: i64,
        api_client: &SiliconFlowClient,
        db: &Repository,
        model: Option<&str>,
    ) -> Result<SessionReport> {
        let start_time = Instant::now();

        // Get session info
        let session = db
            .get_session_by_id(session_id)
            .context("Failed to get session")?
            .context("Session not found")?;

        // Get all answers
        let answers = db
            .get_answers_by_session(session_id)
            .context("Failed to get answers")?;

        if answers.is_empty() {
            anyhow::bail!("No answers found for session");
        }

        // Extract question and answer texts
        let questions: Vec<String> = answers.iter().map(|a| a.question.clone()).collect();
        let answer_texts: Vec<String> = answers.iter().map(|a| a.answer.clone()).collect();

        // Get job description if available
        let job_description = if let Some(jd_id) = session.job_description_id {
            match db.get_job_descriptions() {
                Ok(jds) => jds
                    .iter()
                    .find(|jd| jd.id == Some(jd_id))
                    .map(|jd| jd.content.clone())
                    .unwrap_or_default(),
                _ => String::new(),
            }
        } else {
            String::new()
        };

        // Call API to generate report with timeout protection
        let api_response = match timeout(
            Duration::from_secs(90),
            api_client.generate_session_report_with_model(&questions, &answer_texts, &job_description, model)
        )
        .await
        {
            Ok(result) => result.context("Failed to generate report from API")?,
            Err(_) => anyhow::bail!("Report generation timeout after 90 seconds"),
        };

        // Parse response
        let report = Self::parse_report_response(&api_response)?;

        // Calculate response time
        let api_response_time = start_time.elapsed().as_millis() as i32;

        // Save to database
        let content_analysis = serde_json::to_string(&json!({
            "questions": questions,
            "answer_count": answers.len(),
            "api_response_time": api_response_time,
        }))?;

        let improvements_json = serde_json::to_string(&report.improvements)?;
        let key_takeaways_json = serde_json::to_string(&report.key_takeaways)?;

        let timestamp = chrono::Utc::now().to_rfc3339();

        let report_id = db
            .save_session_report(
                session_id,
                report.overall_score,
                content_analysis,
                None,
                report.summary.clone(),
                improvements_json.clone(),
                key_takeaways_json.clone(),
                None,
                Some(api_response_time),
            )
            .context("Failed to save report")?;

        Ok(SessionReport {
            id: Some(report_id),
            session_id,
            overall_score: report.overall_score,
            content_analysis: "{}".to_string(),
            expression_analysis: None,
            summary: report.summary.clone(),
            improvements: improvements_json.clone(),
            key_takeaways: key_takeaways_json.clone(),
            reference_answers: None,
            generated_at: timestamp,
            api_response_time: Some(api_response_time),
        })
    }

    /// Parse API response to structured report
    fn parse_report_response(response: &str) -> Result<ComprehensiveReport> {
        // Try to parse as JSON
        if let Ok(json_value) = serde_json::from_str::<Value>(response) {
            return Self::extract_from_json(&json_value);
        }

        // Try to find JSON in text
        if let Some(start) = response.find('{') {
            if let Some(end) = response.rfind('}') {
                if start < end {
                    let json_part = &response[start..=end];
                    if let Ok(json_value) = serde_json::from_str::<Value>(json_part) {
                        return Self::extract_from_json(&json_value);
                    }
                }
            }
        }

        // Fallback: create basic report from raw text
        Ok(ComprehensiveReport {
            summary: response.chars().take(500).collect(),
            overall_score: 7.0,
            improvements: vec![
                "提高回答的结构性和清晰度".to_string(),
                "多使用具体例子支撑观点".to_string(),
                "加强对岗位要求的理解和匹配".to_string(),
            ],
            key_takeaways: vec![
                "继续保持良好的逻辑表达".to_string(),
                "注意补充行业相关的专业术语".to_string(),
            ],
        })
    }

    /// Extract structured data from JSON value
    fn extract_from_json(json: &Value) -> Result<ComprehensiveReport> {
        let summary = json
            .get("summary")
            .and_then(|v| v.as_str())
            .unwrap_or("Interview performance report")
            .to_string();

        let overall_score = json
            .get("overall_score")
            .and_then(|v| v.as_f64())
            .unwrap_or(7.0) as f32;

        let improvements = json
            .get("improvements")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_else(|| {
                vec![
                    "提高回答的结构性".to_string(),
                    "补充具体案例".to_string(),
                    "加强岗位匹配度".to_string(),
                ]
            });

        let key_takeaways = json
            .get("key_takeaways")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_else(|| {
                vec![
                    "保持良好的表达能力".to_string(),
                    "继续完善专业知识".to_string(),
                ]
            });

        Ok(ComprehensiveReport {
            summary,
            overall_score: overall_score.max(1.0).min(10.0),
            improvements,
            key_takeaways,
        })
    }
}
