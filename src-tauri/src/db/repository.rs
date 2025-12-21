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
            "SELECT id, user_id, title, content, created_at, updated_at FROM resumes ORDER BY updated_at DESC"
        )?;
        
        let resumes = stmt
            .query_map([], |row| {
                Ok(Resume {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
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
            "SELECT id, user_id, title, content, created_at, updated_at FROM job_descriptions ORDER BY updated_at DESC"
        )?;
        
        let jds = stmt
            .query_map([], |row| {
                Ok(JobDescription {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
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
            "SELECT id, user_id, resume_id, job_description_id, questions, created_at FROM interview_sessions ORDER BY created_at DESC"
        )?;
        
        let sessions = stmt
            .query_map([], |row| {
                let questions_json: String = row.get(4)?;
                let questions: Vec<String> = serde_json::from_str(&questions_json)
                    .unwrap_or_default();
                
                Ok(InterviewSession {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    resume_id: row.get(2)?,
                    job_description_id: row.get(3)?,
                    questions,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(sessions)
    }

    /// Get interview session by ID
    pub fn get_session_by_id(&self, session_id: i64) -> Result<Option<InterviewSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, resume_id, job_description_id, questions, created_at FROM interview_sessions WHERE id = ?1"
        )?;
        
        let session = stmt
            .query_row(params![session_id], |row| {
                let questions_json: String = row.get(4)?;
                let questions: Vec<String> = serde_json::from_str(&questions_json)
                    .unwrap_or_default();
                
                Ok(InterviewSession {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    resume_id: row.get(2)?,
                    job_description_id: row.get(3)?,
                    questions,
                    created_at: row.get(5)?,
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
            "SELECT id, user_id, question, best_answer, notes, job_category, created_at, updated_at FROM question_bank ORDER BY updated_at DESC"
        )?;
        
        let items = stmt
            .query_map([], |row| {
                Ok(QuestionBankItem {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    question: row.get(2)?,
                    best_answer: row.get(3)?,
                    notes: row.get(4)?,
                    job_category: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
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

    // ===== Answer Analysis Operations =====

    /// Save analysis for an answer
    pub fn save_answer_analysis(
        &self,
        answer_id: i64,
        content_score: f32,
        logic_score: f32,
        job_match_score: f32,
        keyword_coverage: f32,
        expression_score: Option<f32>,
        overall_score: f32,
        strengths: String,
        weaknesses: String,
        suggestions: String,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT INTO answer_analysis (answer_id, content_score, logic_score, job_match_score, keyword_coverage, expression_score, overall_score, strengths, weaknesses, suggestions, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                answer_id, content_score, logic_score, job_match_score, keyword_coverage,
                expression_score, overall_score, strengths, weaknesses, suggestions, timestamp
            ],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get analysis for an answer
    pub fn get_answer_analysis(&self, answer_id: i64) -> Result<Option<AnswerAnalysis>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, answer_id, content_score, logic_score, job_match_score, keyword_coverage, expression_score, overall_score, strengths, weaknesses, suggestions, created_at FROM answer_analysis WHERE answer_id = ?1"
        )?;
        
        let analysis = stmt
            .query_row(params![answer_id], |row| {
                Ok(AnswerAnalysis {
                    id: Some(row.get(0)?),
                    answer_id: row.get(1)?,
                    content_score: row.get(2)?,
                    logic_score: row.get(3)?,
                    job_match_score: row.get(4)?,
                    keyword_coverage: row.get(5)?,
                    expression_score: row.get(6)?,
                    overall_score: row.get(7)?,
                    strengths: row.get(8)?,
                    weaknesses: row.get(9)?,
                    suggestions: row.get(10)?,
                    created_at: row.get(11)?,
                })
            })
            .optional()?;
        
        Ok(analysis)
    }

    // ===== Session Report Operations =====

    /// Save report for a session
    pub fn save_session_report(
        &self,
        session_id: i64,
        overall_score: f32,
        content_analysis: String,
        expression_analysis: Option<String>,
        summary: String,
        improvements: String,
        key_takeaways: String,
        reference_answers: Option<String>,
        api_response_time: Option<i32>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT INTO session_reports (session_id, overall_score, content_analysis, expression_analysis, summary, improvements, key_takeaways, reference_answers, generated_at, api_response_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                session_id, overall_score, content_analysis, expression_analysis,
                summary, improvements, key_takeaways, reference_answers, timestamp, api_response_time
            ],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get report for a session
    pub fn get_session_report(&self, session_id: i64) -> Result<Option<SessionReport>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_id, overall_score, content_analysis, expression_analysis, summary, improvements, key_takeaways, reference_answers, generated_at, api_response_time FROM session_reports WHERE session_id = ?1"
        )?;
        
        let report = stmt
            .query_row(params![session_id], |row| {
                Ok(SessionReport {
                    id: Some(row.get(0)?),
                    session_id: row.get(1)?,
                    overall_score: row.get(2)?,
                    content_analysis: row.get(3)?,
                    expression_analysis: row.get(4)?,
                    summary: row.get(5)?,
                    improvements: row.get(6)?,
                    key_takeaways: row.get(7)?,
                    reference_answers: row.get(8)?,
                    generated_at: row.get(9)?,
                    api_response_time: row.get(10)?,
                })
            })
            .optional()?;
        
        Ok(report)
    }

    // ===== Performance Stats Operations =====

    /// Save performance statistics
    pub fn save_performance_stats(
        &self,
        session_date: String,
        total_sessions: i32,
        average_score: f32,
        content_avg: f32,
        expression_avg: Option<f32>,
        highest_score: f32,
        lowest_score: f32,
        improvement_trend: f32,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT OR REPLACE INTO performance_stats (session_date, total_sessions, average_score, content_avg, expression_avg, highest_score, lowest_score, improvement_trend, recorded_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                session_date, total_sessions, average_score, content_avg, expression_avg,
                highest_score, lowest_score, improvement_trend, timestamp
            ],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get performance history
    pub fn get_performance_history(&self) -> Result<Vec<PerformanceStats>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_date, total_sessions, average_score, content_avg, expression_avg, highest_score, lowest_score, improvement_trend, recorded_at FROM performance_stats ORDER BY session_date DESC LIMIT 30"
        )?;
        
        let stats = stmt
            .query_map([], |row| {
                Ok(PerformanceStats {
                    id: Some(row.get(0)?),
                    session_date: row.get(1)?,
                    total_sessions: row.get(2)?,
                    average_score: row.get(3)?,
                    content_avg: row.get(4)?,
                    expression_avg: row.get(5)?,
                    highest_score: row.get(6)?,
                    lowest_score: row.get(7)?,
                    improvement_trend: row.get(8)?,
                    recorded_at: row.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(stats)
    }

    // ===== Analytics Operations =====

    /// Get historical reports for trend analysis
    pub fn get_historical_reports(&self, time_range: Option<i64>) -> Result<Vec<(i64, f32, f32, f32, f32, f32)>> {
        let conn = self.conn.lock().unwrap();
        
        let query = match time_range {
            Some(timestamp) => {
                format!(
                    "SELECT s.created_at, r.overall_score, \
                     COALESCE(json_extract(r.content_analysis, '$.communication_score'), 0.0), \
                     COALESCE(json_extract(r.content_analysis, '$.problem_solving_score'), 0.0), \
                     COALESCE(json_extract(r.content_analysis, '$.technical_depth_score'), 0.0), \
                     COALESCE(json_extract(r.content_analysis, '$.presentation_score'), 0.0) \
                     FROM session_reports r \
                     JOIN interview_sessions s ON r.session_id = s.id \
                     WHERE s.created_at >= {} \
                     ORDER BY s.created_at ASC", timestamp
                )
            },
            None => {
                "SELECT s.created_at, r.overall_score, \
                 COALESCE(json_extract(r.content_analysis, '$.communication_score'), 0.0), \
                 COALESCE(json_extract(r.content_analysis, '$.problem_solving_score'), 0.0), \
                 COALESCE(json_extract(r.content_analysis, '$.technical_depth_score'), 0.0), \
                 COALESCE(json_extract(r.content_analysis, '$.presentation_score'), 0.0) \
                 FROM session_reports r \
                 JOIN interview_sessions s ON r.session_id = s.id \
                 ORDER BY s.created_at ASC".to_string()
            }
        };
        
        let mut stmt = conn.prepare(&query)?;
        
        let reports = stmt
            .query_map([], |row| {
                Ok((
                    row.get(0)?,  // timestamp
                    row.get(1)?,  // overall_score
                    row.get(2)?,  // communication_score
                    row.get(3)?,  // problem_solving_score
                    row.get(4)?,  // technical_depth_score
                    row.get(5)?,  // presentation_score
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(reports)
    }

    /// Get statistics for performance analytics
    pub fn get_statistics(&self, time_range: Option<i64>) -> Result<(i32, f32, f32, f32, String)> {
        let conn = self.conn.lock().unwrap();
        
        let query = match time_range {
            Some(timestamp) => {
                format!(
                    "SELECT COUNT(*), AVG(r.overall_score), MAX(r.overall_score) \
                     FROM session_reports r \
                     JOIN interview_sessions s ON r.session_id = s.id \
                     WHERE s.created_at >= {}", timestamp
                )
            },
            None => {
                "SELECT COUNT(*), AVG(r.overall_score), MAX(r.overall_score) \
                 FROM session_reports r \
                 JOIN interview_sessions s ON r.session_id = s.id".to_string()
            }
        };
        
        let (total_sessions, average_overall, highest_overall): (i32, f32, f32) = conn
            .query_row(&query, [], |row| {
                Ok((
                    row.get(0).unwrap_or(0),
                    row.get(1).unwrap_or(0.0),
                    row.get(2).unwrap_or(0.0),
                ))
            })?;
        
        // Calculate improvement rate: compare first 5 and last 5 sessions
        let improvement_rate = if total_sessions >= 5 {
            let first_avg: f32 = conn
                .query_row(
                    "SELECT AVG(r.overall_score) \
                     FROM session_reports r \
                     JOIN interview_sessions s ON r.session_id = s.id \
                     ORDER BY s.created_at ASC LIMIT 5",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0.0);
            
            let last_avg: f32 = conn
                .query_row(
                    "SELECT AVG(r.overall_score) \
                     FROM session_reports r \
                     JOIN interview_sessions s ON r.session_id = s.id \
                     ORDER BY s.created_at DESC LIMIT 5",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0.0);
            
            if first_avg > 0.0 {
                ((last_avg - first_avg) / first_avg) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // Determine trend: compare recent 3 vs previous 3
        let recent_trend = if total_sessions >= 6 {
            let recent_avg: f32 = conn
                .query_row(
                    "SELECT AVG(r.overall_score) \
                     FROM session_reports r \
                     JOIN interview_sessions s ON r.session_id = s.id \
                     ORDER BY s.created_at DESC LIMIT 3",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0.0);
            
            let previous_avg: f32 = conn
                .query_row(
                    "SELECT AVG(r.overall_score) \
                     FROM session_reports r \
                     JOIN interview_sessions s ON r.session_id = s.id \
                     ORDER BY s.created_at DESC LIMIT 3 OFFSET 3",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0.0);
            
            let diff = recent_avg - previous_avg;
            if diff > 2.0 {
                "improving".to_string()
            } else if diff < -2.0 {
                "declining".to_string()
            } else {
                "stable".to_string()
            }
        } else {
            "stable".to_string()
        };
        
        Ok((total_sessions, average_overall, highest_overall, improvement_rate, recent_trend))
    }

    // ===== Dashboard Operations =====

    /// Get total interview sessions count
    pub fn get_total_sessions_count(&self) -> Result<i32> {
        let conn = self.conn.lock().unwrap();
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM interview_sessions",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    /// Get average score across all sessions
    pub fn get_average_score(&self) -> Result<f32> {
        let conn = self.conn.lock().unwrap();
        let avg: f32 = conn.query_row(
            "SELECT AVG(overall_score) FROM session_reports",
            [],
            |row| row.get(0),
        ).unwrap_or(0.0);
        Ok(avg)
    }

    /// Get highest score achieved
    pub fn get_highest_score(&self) -> Result<f32> {
        let conn = self.conn.lock().unwrap();
        let highest: f32 = conn.query_row(
            "SELECT MAX(overall_score) FROM session_reports",
            [],
            |row| row.get(0),
        ).unwrap_or(0.0);
        Ok(highest)
    }

    /// Get top questions by frequency
    pub fn get_top_questions(&self, limit: i32) -> Result<Vec<(String, i32)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT question, COUNT(*) as count FROM interview_answers \
             GROUP BY question \
             ORDER BY count DESC \
             LIMIT ?1"
        )?;

        let questions = stmt
            .query_map([limit], |row| {
                Ok((
                    row.get(0)?,  // question
                    row.get(1)?,  // count
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(questions)
    }

    /// Get weak areas based on average scores per dimension
    pub fn get_weak_areas(&self) -> Result<Vec<(String, f32)>> {
        let conn = self.conn.lock().unwrap();
        
        // For simplicity, we'll calculate from the analysis data in content_analysis JSON
        // This is a basic implementation that returns dimension scores
        let mut stmt = conn.prepare(
            "SELECT \
             CASE \
               WHEN avg_communication < avg_overall THEN 'Communication' \
               WHEN avg_problem_solving < avg_overall THEN 'Problem Solving' \
               WHEN avg_technical_depth < avg_overall THEN 'Technical Depth' \
               WHEN avg_presentation < avg_overall THEN 'Presentation' \
             END as area, \
             CASE \
               WHEN avg_communication < avg_overall THEN avg_communication \
               WHEN avg_problem_solving < avg_overall THEN avg_problem_solving \
               WHEN avg_technical_depth < avg_overall THEN avg_technical_depth \
               WHEN avg_presentation < avg_overall THEN avg_presentation \
             END as score \
             FROM ( \
               SELECT \
                 AVG(overall_score) as avg_overall, \
                 AVG(COALESCE(json_extract(content_analysis, '$.communication_score'), 0)) as avg_communication, \
                 AVG(COALESCE(json_extract(content_analysis, '$.problem_solving_score'), 0)) as avg_problem_solving, \
                 AVG(COALESCE(json_extract(content_analysis, '$.technical_depth_score'), 0)) as avg_technical_depth, \
                 AVG(COALESCE(json_extract(content_analysis, '$.presentation_score'), 0)) as avg_presentation \
               FROM session_reports \
             ) \
             WHERE area IS NOT NULL \
             ORDER BY score ASC \
             LIMIT 4"
        )?;

        let weak_areas = stmt
            .query_map([], |row| {
                Ok((
                    row.get(0)?,  // area name
                    row.get(1)?,  // score
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(weak_areas)
    }

    /// Get recent sessions with limit
    pub fn get_recent_sessions(&self, limit: i32) -> Result<Vec<InterviewSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, resume_id, job_description_id, questions, created_at \
             FROM interview_sessions \
             ORDER BY created_at DESC \
             LIMIT ?1"
        )?;

        let sessions = stmt
            .query_map([limit], |row| {
                let questions_json: String = row.get(4)?;
                let questions: Vec<String> = serde_json::from_str(&questions_json)
                    .unwrap_or_default();
                
                Ok(InterviewSession {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    resume_id: row.get(2)?,
                    job_description_id: row.get(3)?,
                    questions,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    // ===== History Management Operations =====

    /// Get comparison data for a specific question across all sessions
    pub fn get_answers_comparison(&self, question: &str) -> Result<Vec<(String, String, String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT s.created_at, ia.answer, ia.feedback, \
             COALESCE(sr.overall_score, 0) \
             FROM interview_answers ia \
             JOIN interview_sessions s ON ia.session_id = s.id \
             LEFT JOIN session_reports sr ON s.id = sr.session_id \
             WHERE ia.question = ?1 \
             ORDER BY s.created_at ASC"
        )?;

        let results = stmt
            .query_map([question], |row| {
                Ok((
                    row.get::<_, String>(0)?,  // created_at
                    row.get::<_, String>(1)?,  // answer
                    row.get::<_, String>(2)?,  // feedback
                    row.get::<_, f32>(3)?.to_string(),  // overall_score as string
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(results)
    }

    /// Delete a specific interview session and related data
    pub fn delete_session(&self, session_id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Delete in reverse dependency order
        conn.execute(
            "DELETE FROM session_reports WHERE session_id = ?1",
            [session_id],
        )?;
        
        conn.execute(
            "DELETE FROM interview_answers WHERE session_id = ?1",
            [session_id],
        )?;
        
        conn.execute(
            "DELETE FROM interview_sessions WHERE id = ?1",
            [session_id],
        )?;
        
        Ok(())
    }

    /// Delete all interview sessions and related data
    pub fn delete_all_sessions(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute("DELETE FROM session_reports", [])?;
        conn.execute("DELETE FROM interview_answers", [])?;
        conn.execute("DELETE FROM interview_sessions", [])?;
        
        Ok(())
    }

    // ===== Pagination and Filtering Operations =====

    /// Get paginated interview sessions
    pub fn get_sessions_paginated(&self, page: i32, page_size: i32) -> Result<(Vec<InterviewSession>, i32)> {
        let conn = self.conn.lock().unwrap();
        
        // Get total count
        let total: i32 = conn.query_row(
            "SELECT COUNT(*) FROM interview_sessions",
            [],
            |row| row.get(0),
        )?;
        
        // Calculate offset
        let offset = (page - 1) * page_size;
        
        // Get paginated data
        let mut stmt = conn.prepare(
            "SELECT id, user_id, resume_id, job_description_id, questions, created_at \
             FROM interview_sessions \
             ORDER BY created_at DESC \
             LIMIT ?1 OFFSET ?2"
        )?;
        
        let sessions = stmt
            .query_map([page_size, offset], |row| {
                let questions_json: String = row.get(4)?;
                let questions: Vec<String> = serde_json::from_str(&questions_json)
                    .unwrap_or_default();
                
                Ok(InterviewSession {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    resume_id: row.get(2)?,
                    job_description_id: row.get(3)?,
                    questions,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok((sessions, total))
    }

    /// Get paginated answers for a session
    pub fn get_answers_paginated(&self, session_id: i64, page: i32, page_size: i32) -> Result<(Vec<InterviewAnswer>, i32)> {
        let conn = self.conn.lock().unwrap();
        
        // Get total count
        let total: i32 = conn.query_row(
            "SELECT COUNT(*) FROM interview_answers WHERE session_id = ?1",
            [session_id],
            |row| row.get(0),
        )?;
        
        // Calculate offset
        let offset = (page - 1) * page_size;
        
        // Get paginated data
        let mut stmt = conn.prepare(
            "SELECT id, session_id, question_index, question, answer, feedback, created_at \
             FROM interview_answers \
             WHERE session_id = ?1 \
             ORDER BY question_index ASC \
             LIMIT ?2 OFFSET ?3"
        )?;
        
        let answers = stmt
            .query_map(params![session_id, page_size, offset], |row| {
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
        
        Ok((answers, total))
    }

    /// Get sessions filtered by date range
    pub fn get_sessions_by_date_range(&self, start_date: &str, end_date: &str) -> Result<Vec<InterviewSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, resume_id, job_description_id, questions, created_at \
             FROM interview_sessions \
             WHERE created_at >= ?1 AND created_at <= ?2 \
             ORDER BY created_at DESC"
        )?;
        
        let sessions = stmt
            .query_map(params![start_date, end_date], |row| {
                let questions_json: String = row.get(4)?;
                let questions: Vec<String> = serde_json::from_str(&questions_json)
                    .unwrap_or_default();
                
                Ok(InterviewSession {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    resume_id: row.get(2)?,
                    job_description_id: row.get(3)?,
                    questions,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(sessions)
    }

    /// Get session reports filtered by date range
    pub fn get_reports_by_date_range(&self, start_date: &str, end_date: &str) -> Result<Vec<SessionReport>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT r.id, r.session_id, r.overall_score, r.content_analysis, \
             r.expression_analysis, r.summary, r.improvements, r.key_takeaways, \
             r.reference_answers, r.generated_at, r.api_response_time \
             FROM session_reports r \
             JOIN interview_sessions s ON r.session_id = s.id \
             WHERE s.created_at >= ?1 AND s.created_at <= ?2 \
             ORDER BY s.created_at DESC"
        )?;
        
        let reports = stmt
            .query_map(params![start_date, end_date], |row| {
                Ok(SessionReport {
                    id: Some(row.get(0)?),
                    session_id: row.get(1)?,
                    overall_score: row.get(2)?,
                    content_analysis: row.get(3)?,
                    expression_analysis: row.get(4)?,
                    summary: row.get(5)?,
                    improvements: row.get(6)?,
                    key_takeaways: row.get(7)?,
                    reference_answers: row.get(8)?,
                    generated_at: row.get(9)?,
                    api_response_time: row.get(10)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(reports)
    }

    // ===== Question Tag Operations =====

    /// Create a new tag
    pub fn create_tag(&self, name: String, color: String) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT INTO question_tags (name, color, created_at) VALUES (?1, ?2, ?3)",
            params![name, color, timestamp],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get all tags
    pub fn get_all_tags(&self) -> Result<Vec<QuestionTag>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, color, created_at FROM question_tags ORDER BY name"
        )?;
        
        let tags = stmt
            .query_map([], |row| {
                Ok(QuestionTag {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    name: row.get(2)?,
                    color: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(tags)
    }

    /// Update a tag
    pub fn update_tag(&self, id: i64, name: String, color: String) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE question_tags SET name = ?1, color = ?2 WHERE id = ?3",
            params![name, color, id],
        )?;
        Ok(())
    }

    /// Delete a tag
    pub fn delete_tag(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM question_tags WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Add a tag to a question
    pub fn add_tag_to_question(&self, question_id: i64, tag_id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT OR IGNORE INTO question_tag_mappings (question_bank_id, tag_id, created_at) VALUES (?1, ?2, ?3)",
            params![question_id, tag_id, timestamp],
        )?;
        
        Ok(())
    }

    /// Remove a tag from a question
    pub fn remove_tag_from_question(&self, question_id: i64, tag_id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM question_tag_mappings WHERE question_bank_id = ?1 AND tag_id = ?2",
            params![question_id, tag_id],
        )?;
        Ok(())
    }

    /// Get tags for a specific question
    pub fn get_tags_for_question(&self, question_id: i64) -> Result<Vec<QuestionTag>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT t.id, t.user_id, t.name, t.color, t.created_at \
             FROM question_tags t \
             JOIN question_tag_mappings m ON t.id = m.tag_id \
             WHERE m.question_bank_id = ?1 \
             ORDER BY t.name"
        )?;
        
        let tags = stmt
            .query_map(params![question_id], |row| {
                Ok(QuestionTag {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    name: row.get(2)?,
                    color: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(tags)
    }

    /// Get questions by tag
    pub fn get_questions_by_tag(&self, tag_id: i64) -> Result<Vec<QuestionBankItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT q.id, q.user_id, q.question, q.best_answer, q.notes, q.job_category, q.created_at, q.updated_at \
             FROM question_bank q \
             JOIN question_tag_mappings m ON q.id = m.question_bank_id \
             WHERE m.tag_id = ?1 \
             ORDER BY q.updated_at DESC"
        )?;
        
        let questions = stmt
            .query_map(params![tag_id], |row| {
                Ok(QuestionBankItem {
                    id: Some(row.get(0)?),
                    user_id: row.get(1)?,
                    question: row.get(2)?,
                    best_answer: row.get(3)?,
                    notes: row.get(4)?,
                    job_category: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(questions)
    }

    /// Get daily activity data for heatmap (past 365 days)
    pub fn get_daily_activity(&self) -> Result<Vec<(String, i32)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT DATE(created_at) as date, COUNT(*) as count \
             FROM interview_sessions \
             WHERE created_at >= DATE('now', '-365 days') \
             GROUP BY DATE(created_at) \
             ORDER BY date ASC"
        )?;
        
        let activity = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(activity)
    }

    // ===== User Management Operations =====

    /// Create a new user
    pub fn create_user(&self, username: String, avatar_color: String, avatar_path: Option<String>) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT INTO users (username, avatar_color, avatar_path, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![username, avatar_color, avatar_path, timestamp],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Get all users
    pub fn get_all_users(&self) -> Result<Vec<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, avatar_color, avatar_path, created_at FROM users ORDER BY created_at ASC"
        )?;
        
        let users = stmt
            .query_map([], |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    username: row.get(1)?,
                    avatar_color: row.get(2)?,
                    avatar_path: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(users)
    }

    /// Get user by ID
    pub fn get_user_by_id(&self, id: i64) -> Result<Option<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, avatar_color, avatar_path, created_at FROM users WHERE id = ?1"
        )?;
        
        let user = stmt
            .query_row(params![id], |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    username: row.get(1)?,
                    avatar_color: row.get(2)?,
                    avatar_path: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })
            .optional()?;
        
        Ok(user)
    }

    /// Update user information
    pub fn update_user(&self, id: i64, username: String, avatar_color: String, avatar_path: Option<String>) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE users SET username = ?1, avatar_color = ?2, avatar_path = ?3 WHERE id = ?4",
            params![username, avatar_color, avatar_path, id],
        )?;
        Ok(())
    }

    /// Delete user (cascade delete all user data)
    pub fn delete_user(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Foreign key ON DELETE CASCADE will handle related data
        conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
        
        Ok(())
    }

    /// Get current user ID from config
    pub fn get_current_user_id(&self) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let user_id: i64 = conn
            .query_row(
                "SELECT value FROM user_config WHERE key = 'current_user_id'",
                [],
                |row| {
                    let value_str: String = row.get(0)?;
                    Ok(value_str.parse::<i64>().unwrap_or(1))
                }
            )
            .unwrap_or(1);
        
        Ok(user_id)
    }

    /// Set current user ID in config
    pub fn set_current_user_id(&self, user_id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        conn.execute(
            "INSERT OR REPLACE INTO user_config (key, value, updated_at) VALUES ('current_user_id', ?1, ?2)",
            params![user_id.to_string(), timestamp],
        )?;
        
        Ok(())
    }

    // ===== Question Best Answer Operations =====

    /// Get best answer by question hash
    pub fn get_best_answer_by_hash(&self, question_hash: &str) -> Result<Option<QuestionBestAnswer>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, question_hash, question_text, generated_answer, source_answer_count, \
             version, needs_update, job_context, created_at, updated_at \
             FROM question_best_answers WHERE question_hash = ?1"
        )?;
        
        let answer = stmt
            .query_row(params![question_hash], |row| {
                Ok(QuestionBestAnswer {
                    id: Some(row.get(0)?),
                    question_hash: row.get(1)?,
                    question_text: row.get(2)?,
                    generated_answer: row.get(3)?,
                    source_answer_count: row.get(4)?,
                    version: row.get(5)?,
                    needs_update: row.get::<_, i32>(6)? != 0,
                    job_context: row.get(7)?,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                })
            })
            .optional()?;
        
        Ok(answer)
    }

    /// Insert or update best answer
    pub fn upsert_best_answer(
        &self,
        question_hash: &str,
        question_text: &str,
        generated_answer: &str,
        source_answer_count: i32,
        job_context: Option<&str>,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let timestamp = now();
        
        // Check if exists
        let existing: Option<(i64, i32)> = conn
            .query_row(
                "SELECT id, version FROM question_best_answers WHERE question_hash = ?1",
                params![question_hash],
                |row| Ok((row.get(0)?, row.get(1)?))
            )
            .optional()?;
        
        if let Some((id, version)) = existing {
            // Update existing
            conn.execute(
                "UPDATE question_best_answers SET \
                 generated_answer = ?1, source_answer_count = ?2, version = ?3, \
                 needs_update = 0, job_context = ?4, updated_at = ?5 \
                 WHERE id = ?6",
                params![generated_answer, source_answer_count, version + 1, job_context, timestamp, id],
            )?;
            Ok(id)
        } else {
            // Insert new
            conn.execute(
                "INSERT INTO question_best_answers \
                 (question_hash, question_text, generated_answer, source_answer_count, version, needs_update, job_context, created_at, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, 1, 0, ?5, ?6, ?7)",
                params![question_hash, question_text, generated_answer, source_answer_count, job_context, timestamp, timestamp],
            )?;
            Ok(conn.last_insert_rowid())
        }
    }

    /// Mark question's best answer as needing update
    pub fn mark_answer_needs_update(&self, question_hash: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let rows_affected = conn.execute(
            "UPDATE question_best_answers SET needs_update = 1 WHERE question_hash = ?1",
            params![question_hash],
        )?;
        Ok(rows_affected > 0)
    }

    /// Get count of answers for a specific question
    #[allow(dead_code)]
    pub fn get_answer_count_for_question(&self, question: &str) -> Result<i32> {
        let conn = self.conn.lock().unwrap();
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM interview_answers WHERE question = ?1",
                params![question],
                |row| row.get(0),
            )
            .unwrap_or(0);
        Ok(count)
    }

    /// Get all answers for a specific question (for generating best answer)
    pub fn get_all_answers_for_question(&self, question: &str) -> Result<Vec<(String, f32)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT ia.answer, COALESCE(aa.overall_score, 0) as score \
             FROM interview_answers ia \
             LEFT JOIN answer_analysis aa ON ia.id = aa.answer_id \
             WHERE ia.question = ?1 \
             ORDER BY score DESC"
        )?;
        
        let results = stmt
            .query_map(params![question], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(results)
    }
}
