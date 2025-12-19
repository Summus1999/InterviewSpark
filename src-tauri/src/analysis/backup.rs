//! Data backup and restore module for full data export/import

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::db::models::{
    Resume, JobDescription, InterviewSession, InterviewAnswer, 
    QuestionBankItem, SessionReport,
};
use crate::db::repository::Repository;

/// Complete backup data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupData {
    pub version: String,
    pub created_at: String,
    pub resumes: Vec<Resume>,
    pub job_descriptions: Vec<JobDescription>,
    pub sessions: Vec<InterviewSession>,
    pub answers: Vec<InterviewAnswer>,
    pub reports: Vec<SessionReport>,
    pub question_bank: Vec<QuestionBankItem>,
}

/// Backup manager for handling data export/import operations
pub struct BackupManager;

impl BackupManager {
    /// Export all data to a JSON backup file
    pub fn export_all_data(repo: &Repository) -> std::io::Result<BackupData> {
        let resumes = repo.get_resumes()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let job_descriptions = repo.get_job_descriptions()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let sessions = repo.get_interview_sessions()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let mut answers = vec![];
        // Collect all answers from all sessions
        for session in &sessions {
            if let Some(session_id) = session.id {
                let session_answers = repo.get_answers_by_session(session_id)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                answers.extend(session_answers);
            }
        }
        let _reports = repo.get_performance_history()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let question_bank = repo.get_question_bank()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        
        Ok(BackupData {
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            resumes,
            job_descriptions,
            sessions,
            answers,
            reports: vec![],  // Empty reports for now (would need session_reports table scan)
            question_bank,
        })
    }

    /// Save backup data to a JSON file
    pub fn save_backup_to_file(backup_data: &BackupData, file_path: &str) -> std::io::Result<()> {
        let json_content = serde_json::to_string_pretty(backup_data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(file_path, json_content)?;
        Ok(())
    }

    /// Load backup data from a JSON file
    pub fn load_backup_from_file(file_path: &str) -> std::io::Result<BackupData> {
        let json_content = fs::read_to_string(file_path)?;
        let backup_data: BackupData = serde_json::from_str(&json_content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(backup_data)
    }

    /// Import backup data into database (will append to existing data)
    pub fn import_data(repo: &Repository, backup_data: &BackupData) -> std::io::Result<()> {
        // Insert resumes
        for resume in &backup_data.resumes {
            let _ = repo.save_resume(
                resume.title.clone(),
                resume.content.clone(),
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }
        
        // Insert job descriptions
        for job_desc in &backup_data.job_descriptions {
            let _ = repo.save_job_description(
                job_desc.title.clone(),
                job_desc.content.clone(),
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }
        
        // Insert sessions
        for session in &backup_data.sessions {
            let _ = repo.create_interview_session(
                session.resume_id,
                session.job_description_id,
                session.questions.clone(),
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }
        
        // Insert answers
        for answer in &backup_data.answers {
            let _ = repo.save_answer(
                answer.session_id,
                answer.question_index,
                answer.question.clone(),
                answer.answer.clone(),
                answer.feedback.clone(),
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }
        
        // Insert reports (direct database insertion needed)
        for report in &backup_data.reports {
            let _ = repo.save_session_report(
                report.session_id,
                report.overall_score,
                report.content_analysis.clone(),
                report.expression_analysis.clone(),
                report.summary.clone(),
                report.improvements.clone(),
                report.key_takeaways.clone(),
                report.reference_answers.clone(),
                report.api_response_time,
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }
        
        // Insert question bank items
        for item in &backup_data.question_bank {
            let _ = repo.add_to_question_bank(
                item.question.clone(),
                item.best_answer.clone(),
                item.notes.clone(),
                item.job_category.clone(),
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        }
        
        Ok(())
    }

    /// Validate backup file format and structure
    pub fn validate_backup(file_path: &str) -> std::io::Result<bool> {
        if !Path::new(file_path).exists() {
            return Ok(false);
        }
        
        let json_content = fs::read_to_string(file_path)?;
        match serde_json::from_str::<BackupData>(&json_content) {
            Ok(backup) => Ok(!backup.version.is_empty()),
            Err(_) => Ok(false),
        }
    }
}
