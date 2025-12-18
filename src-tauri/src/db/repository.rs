//! Repository layer for database CRUD operations

use super::models::*;
use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use std::sync::Mutex;

/// Repository for database operations
pub struct Repository {
    conn: Mutex<Connection>,
}

impl Repository {
    /// Create new repository instance
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }

    // ===== Resume Operations =====

    /// Save a new resume
    pub fn save_resume(&self, title: String, content: String) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT INTO resumes (title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![title, content, timestamp, timestamp],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get all resumes
    pub fn get_resumes(&self) -> Result<Vec<Resume>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM resumes ORDER BY updated_at DESC"
        )?;
        
        let resumes = stmt
            .query_map([], |row| {
                Ok(Resume {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(resumes)
    }

    /// Delete a resume by ID
    pub fn delete_resume(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM resumes WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ===== Job Description Operations =====

    /// Save a new job description
    pub fn save_job_description(&self, title: String, content: String) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT INTO job_descriptions (title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![title, content, timestamp, timestamp],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get all job descriptions
    pub fn get_job_descriptions(&self) -> Result<Vec<JobDescription>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM job_descriptions ORDER BY updated_at DESC"
        )?;
        
        let jds = stmt
            .query_map([], |row| {
                Ok(JobDescription {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(jds)
    }

    /// Delete a job description by ID
    pub fn delete_job_description(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM job_descriptions WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ===== Interview Session Operations =====

    /// Create a new interview session
    pub fn create_interview_session(
        &self,
        resume_id: Option<i64>,
        job_description_id: Option<i64>,
        questions: Vec<String>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        let questions_json = serde_json::to_string(&questions)?;
        
        conn.execute(
            "INSERT INTO interview_sessions (resume_id, job_description_id, questions, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![resume_id, job_description_id, questions_json, timestamp],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get all interview sessions
    pub fn get_interview_sessions(&self) -> Result<Vec<InterviewSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, resume_id, job_description_id, questions, created_at FROM interview_sessions ORDER BY created_at DESC"
        )?;
        
        let sessions = stmt
            .query_map([], |row| {
                let questions_json: String = row.get(3)?;
                let questions: Vec<String> = serde_json::from_str(&questions_json)
                    .unwrap_or_default();
                
                Ok(InterviewSession {
                    id: Some(row.get(0)?),
                    resume_id: row.get(1)?,
                    job_description_id: row.get(2)?,
                    questions,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(sessions)
    }

    /// Get interview session by ID
    pub fn get_session_by_id(&self, session_id: i64) -> Result<Option<InterviewSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, resume_id, job_description_id, questions, created_at FROM interview_sessions WHERE id = ?1"
        )?;
        
        let session = stmt
            .query_row(params![session_id], |row| {
                let questions_json: String = row.get(3)?;
                let questions: Vec<String> = serde_json::from_str(&questions_json)
                    .unwrap_or_default();
                
                Ok(InterviewSession {
                    id: Some(row.get(0)?),
                    resume_id: row.get(1)?,
                    job_description_id: row.get(2)?,
                    questions,
                    created_at: row.get(4)?,
                })
            })
            .optional()?;
        
        Ok(session)
    }

    // ===== Interview Answer Operations =====

    /// Save an interview answer
    pub fn save_answer(
        &self,
        session_id: i64,
        question_index: i32,
        question: String,
        answer: String,
        feedback: String,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT INTO interview_answers (session_id, question_index, question, answer, feedback, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![session_id, question_index, question, answer, feedback, timestamp],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get all answers for a session
    pub fn get_answers_by_session(&self, session_id: i64) -> Result<Vec<InterviewAnswer>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_id, question_index, question, answer, feedback, created_at FROM interview_answers WHERE session_id = ?1 ORDER BY question_index"
        )?;
        
        let answers = stmt
            .query_map(params![session_id], |row| {
                Ok(InterviewAnswer {
                    id: Some(row.get(0)?),
                    session_id: row.get(1)?,
                    question_index: row.get(2)?,
                    question: row.get(3)?,
                    answer: row.get(4)?,
                    feedback: row.get(5)?,
                    created_at: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(answers)
    }

    // ===== Question Bank Operations =====

    /// Add question to bank
    pub fn add_to_question_bank(
        &self,
        question: String,
        best_answer: Option<String>,
        notes: Option<String>,
        job_category: Option<String>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT INTO question_bank (question, best_answer, notes, job_category, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![question, best_answer, notes, job_category, timestamp, timestamp],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get all questions from bank
    pub fn get_question_bank(&self) -> Result<Vec<QuestionBankItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, question, best_answer, notes, job_category, created_at, updated_at FROM question_bank ORDER BY updated_at DESC"
        )?;
        
        let items = stmt
            .query_map([], |row| {
                Ok(QuestionBankItem {
                    id: Some(row.get(0)?),
                    question: row.get(1)?,
                    best_answer: row.get(2)?,
                    notes: row.get(3)?,
                    job_category: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(items)
    }

    /// Update question bank item
    pub fn update_question_bank_item(
        &self,
        id: i64,
        best_answer: Option<String>,
        notes: Option<String>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "UPDATE question_bank SET best_answer = ?1, notes = ?2, updated_at = ?3 WHERE id = ?4",
            params![best_answer, notes, timestamp, id],
        )?;
        
        Ok(())
    }

    /// Delete question from bank
    pub fn delete_from_question_bank(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM question_bank WHERE id = ?1", params![id])?;
        Ok(())
    }
}
