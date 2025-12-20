//! InterviewSpark - AI-powered interview simulation application
//!
//! This module contains the Tauri backend commands for:
//! - IPC communication testing (greet)
//! - AI question generation (generate_questions)
//! - Answer analysis and feedback (analyze_answer)
//! - Data persistence (resumes, job descriptions, interview sessions, question bank)

mod api;
mod db;
mod analysis;

use api::SiliconFlowClient;
use api::siliconflow::SiliconFlowClient as SFClient;
use db::{init_database, Repository, Resume, JobDescription, InterviewSession, InterviewAnswer, QuestionBankItem, AnswerAnalysis, SessionReport, PerformanceStats, QuestionTag, InterviewProfile, RecommendationResult, BestPracticesResult, IndustryComparisonResult};
use analysis::{ContentAnalyzer, ScoringEngine, STARScoringEngine, ReportGenerator, ReportExporter, AnalyticsEngine, TrendAnalytics, DashboardService, DashboardData, BackupManager, CacheManager, ProfileGenerator, RecommendationEngine, BestPracticesExtractor, IndustryComparisonGenerator};
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use tauri::{State, Emitter};

/// Application state holding the SiliconFlow API client and database
/// 
/// The client is wrapped in Mutex<Option<>> to handle:
/// - Thread-safe access across async commands
/// - Graceful handling when API key is not configured
///
/// The repository is wrapped in Arc for shared ownership across threads
/// The cache manager provides high-performance caching for frequently accessed data
struct AppState {
    api_client: Mutex<Option<SiliconFlowClient>>,
    db: Arc<Repository>,
    cache: Arc<CacheManager>,
}

/// Greet command for testing IPC communication between frontend and backend
/// 
/// # Arguments
/// * `name` - User's name to greet
/// 
/// # Returns
/// A welcome message string
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to InterviewSpark.", name)
}

/// Generate interview questions based on resume and job description
///
/// # Arguments
/// * `resume` - User's resume text
/// * `job_description` - Target job description
/// * `count` - Number of questions to generate
/// * `state` - Application state containing API client
///
/// # Returns
/// * `Ok(Vec<String>)` - List of generated interview questions
/// * `Err(String)` - Error message if generation fails
#[tauri::command]
async fn generate_questions(
    resume: String,
    job_description: String,
    count: u32,
    persona: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let client = {
        let client_guard = state.api_client.lock().unwrap();
        client_guard.clone().ok_or("API client not initialized")?
    };
    
    client
        .generate_questions(&resume, &job_description, count, &persona)
        .await
        .map_err(|e| e.to_string())
}

/// Analyze user's answer and provide constructive feedback
///
/// # Arguments
/// * `question` - The interview question being answered
/// * `answer` - User's answer to analyze
/// * `job_description` - Target job description for relevance check
/// * `state` - Application state containing API client
///
/// # Returns
/// * `Ok(String)` - AI feedback including strengths, improvements, suggestions
/// * `Err(String)` - Error message if analysis fails
#[tauri::command]
async fn analyze_answer(
    question: String,
    answer: String,
    job_description: String,
    persona: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let client = {
        let client_guard = state.api_client.lock().unwrap();
        client_guard.clone().ok_or("API client not initialized")?
    };
    
    client
        .analyze_answer(&question, &answer, &job_description, &persona)
        .await
        .map_err(|e| e.to_string())
}

/// Analyze answer for follow-up question generation
///
/// # Arguments
/// * `original_question` - Original interview question
/// * `answer` - Candidate's answer
/// * `conversation_history` - JSON string of conversation history
/// * `job_description` - Target job description
/// * `max_followups` - Maximum number of follow-up questions
/// * `preferred_types` - Preferred follow-up types
/// * `state` - Application state
///
/// # Returns
/// * `Ok(String)` - JSON analysis result with follow-up suggestions
/// * `Err(String)` - Error message
#[tauri::command]
async fn analyze_for_followup(
    original_question: String,
    answer: String,
    conversation_history: String,
    job_description: String,
    max_followups: u32,
    preferred_types: Vec<String>,
    persona: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let client = {
        let client_guard = state.api_client.lock().unwrap();
        client_guard.clone().ok_or("API client not initialized")?
    };
    
    client
        .analyze_for_followup(
            &original_question,
            &answer,
            &conversation_history,
            &job_description,
            max_followups,
            &preferred_types,
            &persona,
        )
        .await
        .map_err(|e| e.to_string())
}

/// Analyze answer with streaming feedback
/// Emits incremental feedback through Tauri events
#[tauri::command]
async fn analyze_answer_stream(
    question: String,
    answer: String,
    job_description: String,
    persona: String,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let client = {
        let client_guard = state.api_client.lock().unwrap();
        client_guard.clone().ok_or("API client not initialized")?
    };

    // Prepare streaming messages
    let system_prompt = SFClient::get_persona_prompt(&persona);
    let user_prompt = format!(
        "Question: {}\n\nCandidate's Answer: {}\n\nJob Description: {}\n\nPlease analyze this answer and provide:\n1. Strengths\n2. Areas for improvement\n3. Suggestions for better response\n4. Relevance to job requirements",
        question, answer, job_description
    );

    let messages = vec![
        api::siliconflow::ChatMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        api::siliconflow::ChatMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    // Get streaming response
    let mut stream = client
        .chat_completion_stream(messages, Some(0.7), Some(1500))
        .await
        .map_err(|e| e.to_string())?;

    // Emit chunks through Tauri events
    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => {
                if !chunk.is_empty() {
                    app.emit("answer-feedback-chunk", chunk)
                        .map_err(|e| e.to_string())?;
                }
            }
            Err(e) => {
                log::error!("Stream error: {}", e);
                return Err(e.to_string());
            }
        }
    }

    // Emit completion event
    app.emit("answer-feedback-complete", ())
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ===== Resume Commands =====

/// Save resume to database
#[tauri::command]
fn db_save_resume(
    title: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    state.db.save_resume(title, content)
        .map_err(|e| e.to_string())
}

/// Get all resumes from database
#[tauri::command]
fn db_get_resumes(state: State<'_, AppState>) -> Result<Vec<Resume>, String> {
    state.db.get_resumes()
        .map_err(|e| e.to_string())
}

/// Delete resume from database
#[tauri::command]
fn db_delete_resume(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state.db.delete_resume(id)
        .map_err(|e| e.to_string())
}

// ===== Job Description Commands =====

/// Save job description to database
#[tauri::command]
fn db_save_job_description(
    title: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    state.db.save_job_description(title, content)
        .map_err(|e| e.to_string())
}

/// Get all job descriptions from database
#[tauri::command]
fn db_get_job_descriptions(state: State<'_, AppState>) -> Result<Vec<JobDescription>, String> {
    state.db.get_job_descriptions()
        .map_err(|e| e.to_string())
}

/// Delete job description from database
#[tauri::command]
fn db_delete_job_description(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state.db.delete_job_description(id)
        .map_err(|e| e.to_string())
}

// ===== Interview Session Commands =====

/// Create new interview session
#[tauri::command]
fn db_create_session(
    resume_id: Option<i64>,
    job_description_id: Option<i64>,
    questions: Vec<String>,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    state.db.create_interview_session(resume_id, job_description_id, questions)
        .map_err(|e| e.to_string())
}

/// Get all interview sessions
#[tauri::command]
fn db_get_sessions(state: State<'_, AppState>) -> Result<Vec<InterviewSession>, String> {
    state.db.get_interview_sessions()
        .map_err(|e| e.to_string())
}

/// Get interview session by ID
#[tauri::command]
fn db_get_session(session_id: i64, state: State<'_, AppState>) -> Result<Option<InterviewSession>, String> {
    state.db.get_session_by_id(session_id)
        .map_err(|e| e.to_string())
}

/// Save interview answer
#[tauri::command]
fn db_save_answer(
    session_id: i64,
    question_index: i32,
    question: String,
    answer: String,
    feedback: String,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    state.db.save_answer(session_id, question_index, question, answer, feedback)
        .map_err(|e| e.to_string())
}

/// Get all answers for a session
#[tauri::command]
fn db_get_answers(session_id: i64, state: State<'_, AppState>) -> Result<Vec<InterviewAnswer>, String> {
    state.db.get_answers_by_session(session_id)
        .map_err(|e| e.to_string())
}

// ===== Question Bank Commands =====

/// Add question to bank
#[tauri::command]
fn db_add_to_bank(
    question: String,
    best_answer: Option<String>,
    notes: Option<String>,
    job_category: Option<String>,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    state.db.add_to_question_bank(question, best_answer, notes, job_category)
        .map_err(|e| e.to_string())
}

/// Get question bank
#[tauri::command]
fn db_get_bank(state: State<'_, AppState>) -> Result<Vec<QuestionBankItem>, String> {
    state.db.get_question_bank()
        .map_err(|e| e.to_string())
}

/// Update question bank item
#[tauri::command]
fn db_update_bank_item(
    id: i64,
    best_answer: Option<String>,
    notes: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.db.update_question_bank_item(id, best_answer, notes)
        .map_err(|e| e.to_string())
}

/// Delete from question bank
#[tauri::command]
fn db_delete_from_bank(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state.db.delete_from_question_bank(id)
        .map_err(|e| e.to_string())
}

// ===== Question Tag Commands =====

/// Create a new tag
#[tauri::command]
fn db_create_tag(name: String, color: String, state: State<'_, AppState>) -> Result<i64, String> {
    state.db.create_tag(name, color)
        .map_err(|e| e.to_string())
}

/// Get all tags
#[tauri::command]
fn db_get_all_tags(state: State<'_, AppState>) -> Result<Vec<QuestionTag>, String> {
    state.db.get_all_tags()
        .map_err(|e| e.to_string())
}

/// Update a tag
#[tauri::command]
fn db_update_tag(id: i64, name: String, color: String, state: State<'_, AppState>) -> Result<(), String> {
    state.db.update_tag(id, name, color)
        .map_err(|e| e.to_string())
}

/// Delete a tag
#[tauri::command]
fn db_delete_tag(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state.db.delete_tag(id)
        .map_err(|e| e.to_string())
}

/// Add a tag to a question
#[tauri::command]
fn db_add_tag_to_question(question_id: i64, tag_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state.db.add_tag_to_question(question_id, tag_id)
        .map_err(|e| e.to_string())
}

/// Remove a tag from a question
#[tauri::command]
fn db_remove_tag_from_question(question_id: i64, tag_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state.db.remove_tag_from_question(question_id, tag_id)
        .map_err(|e| e.to_string())
}

/// Get tags for a specific question
#[tauri::command]
fn db_get_tags_for_question(question_id: i64, state: State<'_, AppState>) -> Result<Vec<QuestionTag>, String> {
    state.db.get_tags_for_question(question_id)
        .map_err(|e| e.to_string())
}

/// Get questions by tag
#[tauri::command]
fn db_get_questions_by_tag(tag_id: i64, state: State<'_, AppState>) -> Result<Vec<QuestionBankItem>, String> {
    state.db.get_questions_by_tag(tag_id)
        .map_err(|e| e.to_string())
}

// ===== Answer Analysis Commands =====

/// Analyze answer and save analysis results
#[tauri::command]
async fn analyze_answer_with_scoring(
    answer_id: i64,
    answer: String,
    question: String,
    job_description: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    // Perform content analysis
    let analysis = ContentAnalyzer::analyze(&answer, &question, &job_description)
        .map_err(|e| e.to_string())?;
    
    // Calculate scores
    let scoring_result = ScoringEngine::calculate_score(&analysis, None);
    
    // Save analysis to database
    let strengths_json = serde_json::to_string(&analysis.strengths).unwrap_or_default();
    let weaknesses_json = serde_json::to_string(&analysis.weaknesses).unwrap_or_default();
    let suggestions_json = serde_json::to_string(&ScoringEngine::get_improvement_suggestions(&scoring_result.score_breakdown)).unwrap_or_default();
    
    state.db.save_answer_analysis(
        answer_id,
        scoring_result.content_score,
        scoring_result.score_breakdown.logic,
        scoring_result.score_breakdown.job_match,
        scoring_result.score_breakdown.keyword_coverage,
        scoring_result.expression_score,
        scoring_result.overall_score,
        strengths_json,
        weaknesses_json,
        suggestions_json,
    ).map_err(|e| e.to_string())?;
    
    Ok(serde_json::json!({
        "overall_score": scoring_result.overall_score,
        "content_score": scoring_result.content_score,
        "grade": scoring_result.score_grade,
        "breakdown": {
            "logic": scoring_result.score_breakdown.logic,
            "job_match": scoring_result.score_breakdown.job_match,
            "keyword_coverage": scoring_result.score_breakdown.keyword_coverage,
        },
        "strengths": analysis.strengths,
        "weaknesses": analysis.weaknesses,
        "suggestions": ScoringEngine::get_improvement_suggestions(&scoring_result.score_breakdown),
    }))
}

/// Get answer analysis
#[tauri::command]
fn db_get_answer_analysis(answer_id: i64, state: State<'_, AppState>) -> Result<Option<AnswerAnalysis>, String> {
    state.db.get_answer_analysis(answer_id)
        .map_err(|e| e.to_string())
}

// ===== Session Report Commands =====

/// Save session report
#[tauri::command]
fn db_save_session_report(
    session_id: i64,
    overall_score: f32,
    content_analysis: String,
    expression_analysis: Option<String>,
    summary: String,
    improvements: String,
    key_takeaways: String,
    reference_answers: Option<String>,
    api_response_time: Option<i32>,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    state.db.save_session_report(
        session_id,
        overall_score,
        content_analysis,
        expression_analysis,
        summary,
        improvements,
        key_takeaways,
        reference_answers,
        api_response_time,
    ).map_err(|e| e.to_string())
}

/// Get session report
#[tauri::command]
fn db_get_session_report(session_id: i64, state: State<'_, AppState>) -> Result<Option<SessionReport>, String> {
    state.db.get_session_report(session_id)
        .map_err(|e| e.to_string())
}

// ===== Performance Stats Commands =====

/// Save performance statistics
#[tauri::command]
fn db_save_performance_stats(
    session_date: String,
    total_sessions: i32,
    average_score: f32,
    content_avg: f32,
    expression_avg: Option<f32>,
    highest_score: f32,
    lowest_score: f32,
    improvement_trend: f32,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    state.db.save_performance_stats(
        session_date,
        total_sessions,
        average_score,
        content_avg,
        expression_avg,
        highest_score,
        lowest_score,
        improvement_trend,
    ).map_err(|e| e.to_string())
}

/// Get performance history
#[tauri::command]
fn db_get_performance_history(state: State<'_, AppState>) -> Result<Vec<PerformanceStats>, String> {
    state.db.get_performance_history()
        .map_err(|e| e.to_string())
}

// ===== Report Generation Commands =====

/// Generate comprehensive interview report
#[tauri::command]
async fn generate_comprehensive_report(
    session_id: i64,
    state: State<'_, AppState>,
) -> Result<SessionReport, String> {
    let client = {
        let client_guard = state.api_client.lock().unwrap();
        client_guard.clone().ok_or("API client not initialized")?
    };
    
    ReportGenerator::generate_report(session_id, &client, state.db.as_ref())
        .await
        .map_err(|e| e.to_string())
}

/// Get session report
#[tauri::command]
fn db_get_report(session_id: i64, state: State<'_, AppState>) -> Result<Option<SessionReport>, String> {
    state.db.get_session_report(session_id)
        .map_err(|e| e.to_string())
}

/// Export report to text format
#[tauri::command]
fn export_report_text(
    session_id: i64,
    file_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let report = state
        .db
        .get_session_report(session_id)
        .map_err(|e| e.to_string())?
        .ok_or("Report not found")?;

    let path = std::path::PathBuf::from(&file_path);
    ReportExporter::export_to_text(&report, &path)
        .map_err(|e| e.to_string())
}

/// Export report to HTML format
#[tauri::command]
fn export_report_html(
    session_id: i64,
    file_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let report = state
        .db
        .get_session_report(session_id)
        .map_err(|e| e.to_string())?
        .ok_or("Report not found")?;

    let path = std::path::PathBuf::from(&file_path);
    ReportExporter::export_to_html(&report, &path)
        .map_err(|e| e.to_string())
}

// ===== Analytics Commands =====

/// Get trend analytics for growth tracking (with caching)
#[tauri::command]
async fn get_trend_analytics(
    time_range_days: Option<i64>,
    state: State<'_, AppState>,
) -> Result<TrendAnalytics, String> {
    // Only use cache if no time range specified (default view)
    if time_range_days.is_none() {
        if let Some(cached_data) = state.cache.get_analytics() {
            return Ok(cached_data);
        }
    }
    
    let time_range = time_range_days.map(|days| {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        now - (days * 86400)
    });

    let engine = AnalyticsEngine::new(state.db.clone());
    let data = engine.get_trend_analytics(time_range)
        .map_err(|e| e.to_string())?;
    
    // Cache only default view
    if time_range_days.is_none() {
        state.cache.set_analytics(data.clone());
    }
    
    Ok(data)
}

// ===== Dashboard Commands =====

/// Get complete dashboard data (with caching)
#[tauri::command]
fn get_dashboard_data(state: State<'_, AppState>) -> Result<DashboardData, String> {
    // Try to get from cache first
    if let Some(cached_data) = state.cache.get_dashboard() {
        return Ok(cached_data);
    }
    
    // If cache miss, fetch from database
    let service = DashboardService::new(state.db.clone());
    let data = service.get_dashboard_data()
        .map_err(|e| e.to_string())?;
    
    // Store in cache
    state.cache.set_dashboard(data.clone());
    
    Ok(data)
}

// ===== History Management Commands =====

/// Get comparison data for same question across different sessions
#[tauri::command]
fn get_answers_comparison(
    question: String,
    state: State<'_, AppState>,
) -> Result<Vec<(String, String, String, String)>, String> {
    state.db.get_answers_comparison(&question)
        .map_err(|e| e.to_string())
}

/// Delete a specific interview session
#[tauri::command]
fn delete_session(session_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state.db.delete_session(session_id)
        .map_err(|e| e.to_string())?;
    
    // Invalidate caches
    state.cache.invalidate_all();
    
    Ok(())
}

/// Delete all interview sessions
#[tauri::command]
fn delete_all_sessions(state: State<'_, AppState>) -> Result<(), String> {
    state.db.delete_all_sessions()
        .map_err(|e| e.to_string())?;
    
    // Invalidate caches
    state.cache.invalidate_all();
    
    Ok(())
}

/// Backup all data to JSON file
#[tauri::command]
fn backup_data(file_path: String, state: State<'_, AppState>) -> Result<(), String> {
    // Export all data
    let backup_data = BackupManager::export_all_data(&state.db)
        .map_err(|e| e.to_string())?;
    
    // Save to file
    BackupManager::save_backup_to_file(&backup_data, &file_path)
        .map_err(|e| format!("Failed to save backup: {}", e))?;
    
    Ok(())
}

/// Restore data from JSON backup file
#[tauri::command]
fn restore_data(file_path: String, state: State<'_, AppState>) -> Result<(), String> {
    // Validate backup file
    BackupManager::validate_backup(&file_path)
        .map_err(|e| format!("Failed to validate backup: {}", e))?
        .then_some(())
        .ok_or_else(|| "Invalid backup file format".to_string())?;
    
    // Load backup data
    let backup_data = BackupManager::load_backup_from_file(&file_path)
        .map_err(|e| format!("Failed to load backup: {}", e))?;
    
    // Import data
    BackupManager::import_data(&state.db, &backup_data)
        .map_err(|e| format!("Failed to import data: {}", e))?;
    
    // Invalidate caches after data restore
    state.cache.invalidate_all();
    
    Ok(())
}

// ===== Pagination and Filtering Commands =====

/// Get paginated interview sessions
#[tauri::command]
fn get_sessions_paginated(
    page: i32,
    page_size: i32,
    state: State<'_, AppState>,
) -> Result<(Vec<InterviewSession>, i32), String> {
    state.db.get_sessions_paginated(page, page_size)
        .map_err(|e| e.to_string())
}

/// Get paginated answers for a session
#[tauri::command]
fn get_answers_paginated(
    session_id: i64,
    page: i32,
    page_size: i32,
    state: State<'_, AppState>,
) -> Result<(Vec<InterviewAnswer>, i32), String> {
    state.db.get_answers_paginated(session_id, page, page_size)
        .map_err(|e| e.to_string())
}

/// Get sessions filtered by date range
#[tauri::command]
fn get_sessions_by_date_range(
    start_date: String,
    end_date: String,
    state: State<'_, AppState>,
) -> Result<Vec<InterviewSession>, String> {
    state.db.get_sessions_by_date_range(&start_date, &end_date)
        .map_err(|e| e.to_string())
}

/// Get reports filtered by date range
#[tauri::command]
fn get_reports_by_date_range(
    start_date: String,
    end_date: String,
    state: State<'_, AppState>,
) -> Result<Vec<SessionReport>, String> {
    state.db.get_reports_by_date_range(&start_date, &end_date)
        .map_err(|e| e.to_string())
}

// ===== Profile Commands =====

/// Generate interview profile for user
#[tauri::command]
fn generate_interview_profile(
    user_id: String,
    session_limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<InterviewProfile, String> {
    ProfileGenerator::generate_profile(&state.db, &user_id, session_limit)
        .map_err(|e| e.to_string())
}

/// Generate practice recommendations for user
#[tauri::command]
fn generate_practice_recommendations(
    user_id: String,
    limit: usize,
    state: State<'_, AppState>,
) -> Result<RecommendationResult, String> {
    RecommendationEngine::generate_recommendations(&state.db, &user_id, limit)
        .map_err(|e| e.to_string())
}

/// Extract best practices from high-scoring answers
#[tauri::command]
fn extract_best_practices(
    score_threshold: f32,
    limit: usize,
    state: State<'_, AppState>,
) -> Result<BestPracticesResult, String> {
    BestPracticesExtractor::extract_best_practices(&state.db, score_threshold, limit)
        .map_err(|e| e.to_string())
}

/// Generate industry comparison for user
#[tauri::command]
fn generate_industry_comparison(
    user_id: String,
    state: State<'_, AppState>,
) -> Result<IndustryComparisonResult, String> {
    IndustryComparisonGenerator::generate_comparison(&state.db, &user_id)
        .map_err(|e| e.to_string())
}

/// Update API configuration at runtime
/// 
/// # Arguments
/// * `model` - New model name
/// * `api_key` - New API key
/// * `state` - Application state
/// 
/// # Returns
/// * `Ok(())` - Configuration updated successfully
/// * `Err(String)` - Error message
#[tauri::command]
async fn update_api_config(
    model: String,
    api_key: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut client_guard = state.api_client.lock().unwrap();
    
    if api_key.is_empty() {
        return Err("API key cannot be empty".to_string());
    }
    
    // Create new client with updated config
    let new_client = SiliconFlowClient::new(api_key, model)
        .map_err(|e| e.to_string())?;
    
    *client_guard = Some(new_client);
    
    Ok(())
}

/// Analyze answer using STAR method
/// Returns STAR score breakdown and suggestions
#[tauri::command]
fn analyze_star_score(
    answer: String,
) -> Result<String, String> {
    let star_result = STARScoringEngine::calculate_star_score(&answer);
    serde_json::to_string(&star_result)
        .map_err(|e| format!("Failed to serialize STAR result: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // Load environment variables
  dotenvy::dotenv().ok();

  // Initialize API client
  let api_client = SiliconFlowClient::from_env().ok();
  
  // Initialize database - store in parent directory to avoid Tauri rebuild triggers
  let db_path = std::env::current_dir()
    .unwrap_or_else(|_| std::path::PathBuf::from("."))
    .parent()  // Go to workspace root (parent of src-tauri)
    .unwrap_or_else(|| std::path::Path::new("."))
    .join("data")
    .join("interview_spark.db");
  
  // Create data directory if it doesn't exist
  if let Some(parent) = db_path.parent() {
    std::fs::create_dir_all(parent).ok();
  }
  
  let conn = init_database(db_path)
    .expect("Failed to initialize database");
  
  let repository = Arc::new(Repository::new(conn));
  let cache_manager = Arc::new(CacheManager::new());
  
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .manage(AppState {
      api_client: Mutex::new(api_client),
      db: repository,
      cache: cache_manager,
    })
    .invoke_handler(tauri::generate_handler![
      greet,
      generate_questions,
      analyze_answer,
      analyze_answer_stream,
      analyze_for_followup,
      db_save_resume,
      db_get_resumes,
      db_delete_resume,
      db_save_job_description,
      db_get_job_descriptions,
      db_delete_job_description,
      db_create_session,
      db_get_sessions,
      db_get_session,
      db_save_answer,
      db_get_answers,
      db_add_to_bank,
      db_get_bank,
      db_update_bank_item,
      db_delete_from_bank,
      db_create_tag,
      db_get_all_tags,
      db_update_tag,
      db_delete_tag,
      db_add_tag_to_question,
      db_remove_tag_from_question,
      db_get_tags_for_question,
      db_get_questions_by_tag,
      analyze_answer_with_scoring,
      db_get_answer_analysis,
      db_save_session_report,
      db_get_session_report,
      db_save_performance_stats,
      db_get_performance_history,
      generate_comprehensive_report,
      db_get_report,
      export_report_text,
      export_report_html,
      get_trend_analytics,
      get_dashboard_data,
      get_answers_comparison,
      delete_session,
      delete_all_sessions,
      backup_data,
      restore_data,
      get_sessions_paginated,
      get_answers_paginated,
      get_sessions_by_date_range,
      get_reports_by_date_range,
      generate_interview_profile,
      generate_practice_recommendations,
      extract_best_practices,
      generate_industry_comparison,
      update_api_config,
      analyze_star_score
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
