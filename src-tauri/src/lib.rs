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
mod rag;
mod rig_adapter;

use api::SiliconFlowClient;
use api::siliconflow::SiliconFlowClient as SFClient;
#[allow(unused_imports)]
use db::{init_database, Repository, Resume, JobDescription, InterviewSession, InterviewAnswer, QuestionBankItem, AnswerAnalysis, SessionReport, PerformanceStats, QuestionTag, InterviewProfile, RecommendationResult, BestPracticesResult, IndustryComparisonResult, User, QuestionBestAnswer};
use analysis::{ContentAnalyzer, ScoringEngine, STARScoringEngine, ReportGenerator, ReportExporter, AnalyticsEngine, TrendAnalytics, DashboardService, DashboardData, BackupManager, CacheManager, ProfileGenerator, RecommendationEngine, BestPracticesExtractor, IndustryComparisonGenerator};
#[allow(unused_imports)]
use rag::{KnowledgeStatus, KnowledgeStats, BootstrapResult, BootstrapProgress, RagService};
use rag::vectordb::SearchResult;
use rig_adapter::{
    SiliconFlowProvider, VectorStoreAdapter,
    InterviewContext, ConversationTurn, AnalysisResult,
    AgentScheduler, RotationStrategy,
    InterviewStateMachine, InterviewPhase, InterviewProgress,
};
use rig_adapter::agents::{TechInterviewer, HRInterviewer, BusinessInterviewer};
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::Mutex as TokioMutex;
use tauri::{State, Emitter};
use base64::Engine;

/// Multi-Agent interview session
struct MultiAgentSession {
    context: InterviewContext,
    scheduler: AgentScheduler,
    state_machine: InterviewStateMachine,
}

impl MultiAgentSession {
    fn new(
        resume: String,
        job_description: String,
        provider: SiliconFlowProvider,
        vector_store: VectorStoreAdapter,
    ) -> Self {
        // Create agents
        let tech = Box::new(TechInterviewer::new(provider.clone(), vector_store.clone())) as Box<dyn rig_adapter::agents::InterviewerAgent>;
        let hr = Box::new(HRInterviewer::new(provider.clone())) as Box<dyn rig_adapter::agents::InterviewerAgent>;
        let business = Box::new(BusinessInterviewer::new(provider)) as Box<dyn rig_adapter::agents::InterviewerAgent>;
        
        let agents = vec![tech, hr, business];
        let scheduler = AgentScheduler::new(agents).with_strategy(RotationStrategy::PhaseBased);
        let state_machine = InterviewStateMachine::new();
        
        let context = InterviewContext {
            resume,
            job_description,
            conversation_history: Vec::new(),
            current_phase: InterviewPhase::WarmUp,
        };
        
        Self {
            context,
            scheduler,
            state_machine,
        }
    }
}

/// Application state holding the SiliconFlow API client and database
/// 
/// The client is wrapped in Mutex<Option<>> to handle:
/// - Thread-safe access across async commands
/// - Graceful handling when API key is not configured
///
/// The repository is wrapped in Arc for shared ownership across threads
/// The cache manager provides high-performance caching for frequently accessed data
/// The rag service provides lazy-initialized knowledge retrieval capabilities
#[allow(dead_code)]
struct AppState {
    api_client: Mutex<Option<SiliconFlowClient>>,
    db: Arc<Repository>,
    cache: Arc<CacheManager>,
    rag: Arc<RagService>,
    // Generic caches for different data types
    session_cache: Arc<analysis::GenericCache<i64, db::models::InterviewSession>>,
    question_bank_cache: Arc<analysis::GenericCache<String, Vec<db::models::QuestionBankItem>>>,
    // Multi-Agent interview sessions (session_id -> session)
    multi_agent_sessions: Arc<TokioMutex<HashMap<String, MultiAgentSession>>>,
}

/// Helper function to safely retrieve API client from state
/// 
/// # Arguments
/// * `state` - Application state containing the API client
/// 
/// # Returns
/// * `Ok(SiliconFlowClient)` - Cloned API client if initialized
/// * `Err(String)` - Error message if lock fails or client not initialized
fn get_client(state: &State<AppState>) -> Result<SiliconFlowClient, String> {
    state.api_client.lock()
        .map_err(|e| format!("Failed to acquire API client lock: {}", e))?
        .clone()
        .ok_or_else(|| "API client not initialized. Please configure API key in settings.".to_string())
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
    let client = get_client(&state)?;
    
    // Use tokio::join! to parallelize RAG retrieval and API warm-up
    let (context, _) = tokio::join!(
        async {
            // Try to retrieve similar questions from knowledge base as context
            if !state.rag.is_empty() {
                match state.rag.retrieve_similar_questions(&job_description, 3).await {
                    Ok(results) => {
                        let context_text = RagService::build_context(&results, 500);
                        if !context_text.is_empty() {
                            log::info!("RAG context: {} chars", context_text.len());
                            Some(context_text)
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        // Silent failure - RAG is optional enhancement
                        log::warn!("RAG retrieval failed (degrading gracefully): {}", e);
                        None
                    }
                }
            } else {
                None
            }
        },
        async {
            // API client warm-up (placeholder for future optimizations)
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
    );
    
    client
        .generate_questions_with_context(
            &resume, 
            &job_description, 
            count, 
            &persona,
            context.as_deref(),
        )
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
    let client = get_client(&state)?;
    
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
    let client = get_client(&state)?;
    
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
    let client = get_client(&state)?;

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

/// Get sessions with answer count (optimized, no N+1 query)
#[tauri::command]
fn db_get_sessions_with_count(state: State<'_, AppState>) -> Result<Vec<(InterviewSession, i32)>, String> {
    state.db.get_sessions_with_answer_count()
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
async fn db_add_to_bank(
    question: String,
    best_answer: Option<String>,
    notes: Option<String>,
    job_category: Option<String>,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let question_id = state.db.add_to_question_bank(
        question.clone(), 
        best_answer.clone(), 
        notes.clone(), 
        job_category.clone()
    ).map_err(|e| e.to_string())?;
    
    // Sync to knowledge base (RAG)
    let metadata = serde_json::json!({
        "source": "question_bank",
        "source_id": question_id,
        "job_category": job_category
    }).to_string();
    
    match state.rag.embed_and_store(
        "user_question",
        &question,
        Some(&metadata)
    ).await {
        Ok(_) => log::info!("Question synced to knowledge base: {}", question_id),
        Err(e) => log::warn!("Failed to sync question to knowledge base: {}", e)
    }
    
    Ok(question_id)
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
async fn db_delete_from_bank(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    // Delete from knowledge base first (by metadata source_id)
    match state.db.delete_knowledge_by_source("question_bank", id) {
        Ok(count) => {
            if count > 0 {
                log::info!("Deleted {} knowledge entries for question_bank id {}", count, id);
            }
        }
        Err(e) => log::warn!("Failed to delete knowledge entries: {}", e)
    }
    
    // Delete from question bank
    state.db.delete_from_question_bank(id)
        .map_err(|e| e.to_string())?;
    
    // Rebuild index after deletion
    if let Err(e) = state.rag.rebuild_index().await {
        log::warn!("Failed to rebuild RAG index: {}", e);
    }
    
    Ok(())
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

// ===== User Management Commands =====

/// Create a new user
#[tauri::command]
fn create_user(username: String, avatar_color: String, avatar_path: Option<String>, state: State<'_, AppState>) -> Result<i64, String> {
    state.db.create_user(username, avatar_color, avatar_path)
        .map_err(|e| e.to_string())
}

/// Get all users
#[tauri::command]
fn get_all_users(state: State<'_, AppState>) -> Result<Vec<db::User>, String> {
    state.db.get_all_users()
        .map_err(|e| e.to_string())
}

/// Get current user
#[tauri::command]
fn get_current_user(state: State<'_, AppState>) -> Result<Option<db::User>, String> {
    let user_id = state.db.get_current_user_id()
        .map_err(|e| e.to_string())?;
    state.db.get_user_by_id(user_id)
        .map_err(|e| e.to_string())
}

/// Switch current user
#[tauri::command]
fn switch_user(user_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state.db.set_current_user_id(user_id)
        .map_err(|e| e.to_string())
}

/// Update user information
#[tauri::command]
fn update_user(id: i64, username: String, avatar_color: String, avatar_path: Option<String>, state: State<'_, AppState>) -> Result<(), String> {
    state.db.update_user(id, username, avatar_color, avatar_path)
        .map_err(|e| e.to_string())
}

/// Delete user
#[tauri::command]
fn delete_user(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    state.db.delete_user(id)
        .map_err(|e| e.to_string())
}

/// Upload user avatar image
#[tauri::command]
fn upload_avatar(
    user_id: i64,
    source_path: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    use std::fs;
    use std::path::Path;
    use tauri::Manager;
    
    // Get app data directory
    let app_data_dir = app_handle.path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    // Create avatars directory if not exists
    let avatars_dir = app_data_dir.join("avatars");
    fs::create_dir_all(&avatars_dir)
        .map_err(|e| format!("Failed to create avatars directory: {}", e))?;
    
    // Get file extension
    let source = Path::new(&source_path);
    let ext = source.extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| "Invalid file extension".to_string())?;
    
    // Validate extension
    if !["jpg", "jpeg", "png"].contains(&ext.to_lowercase().as_str()) {
        return Err("Only jpg, jpeg, png formats are supported".to_string());
    }
    
    // Target file path: avatars/{user_id}.{ext}
    let target_filename = format!("{}.{}", user_id, ext);
    let target_path = avatars_dir.join(&target_filename);
    
    // Copy file
    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("Failed to copy avatar file: {}", e))?;
    
    // Return relative path
    let relative_path = format!("avatars/{}", target_filename);
    Ok(relative_path)
}

/// Get avatar absolute path for frontend display
#[tauri::command]
fn get_avatar_path(
    avatar_path: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    use tauri::Manager;
    
    let app_data_dir = app_handle.path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let full_path = app_data_dir.join(&avatar_path);
    Ok(full_path.to_string_lossy().to_string())
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

/// Analyze all answers that don't have analysis records yet
/// Returns the number of answers analyzed
#[tauri::command]
fn analyze_missing_answers(state: State<'_, AppState>) -> Result<i32, String> {
    let sessions = state.db.get_interview_sessions()
        .map_err(|e| e.to_string())?;
    
    log::info!("analyze_missing_answers: Found {} sessions", sessions.len());
    
    let mut analyzed_count = 0;
    let mut total_answers = 0;
    
    for session in sessions {
        if let Some(session_id) = session.id {
            // Get job description for this session
            let job_desc = if let Some(jd_id) = session.job_description_id {
                state.db.get_job_descriptions()
                    .ok()
                    .and_then(|jds| jds.into_iter().find(|jd| jd.id == Some(jd_id)))
                    .map(|jd| jd.content)
                    .unwrap_or_default()
            } else {
                String::new()
            };
            
            let answers = state.db.get_answers_by_session(session_id)
                .map_err(|e| e.to_string())?;
            
            total_answers += answers.len();
            
            for answer in answers {
                if let Some(answer_id) = answer.id {
                    // Check if analysis already exists
                    let has_analysis = state.db.get_answer_analysis(answer_id)
                        .ok()
                        .flatten()
                        .is_some();
                    
                    if !has_analysis && !answer.answer.trim().is_empty() {
                        // Perform analysis
                        if let Ok(analysis) = ContentAnalyzer::analyze(
                            &answer.answer,
                            &answer.question,
                            &job_desc
                        ) {
                            let scoring_result = ScoringEngine::calculate_score(&analysis, None);
                            
                            let strengths_json = serde_json::to_string(&analysis.strengths).unwrap_or_default();
                            let weaknesses_json = serde_json::to_string(&analysis.weaknesses).unwrap_or_default();
                            let suggestions_json = serde_json::to_string(&ScoringEngine::get_improvement_suggestions(&scoring_result.score_breakdown)).unwrap_or_default();
                            
                            if state.db.save_answer_analysis(
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
                            ).is_ok() {
                                analyzed_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    log::info!("analyze_missing_answers: Total answers={}, Analyzed={}", total_answers, analyzed_count);
    
    Ok(analyzed_count)
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

// Flagship model for AI analysis - highest quality reasoning
const FLAGSHIP_MODEL: &str = "Qwen/Qwen3-235B-A22B-Thinking-2507";

/// Generate comprehensive interview report
#[tauri::command]
async fn generate_comprehensive_report(
    session_id: i64,
    _use_premium_model: Option<bool>,  // Deprecated: always uses flagship model
    state: State<'_, AppState>,
) -> Result<SessionReport, String> {
    let client = get_client(&state)?;
    
    // Always use flagship model for best quality analysis
    ReportGenerator::generate_report_with_model(session_id, &client, state.db.as_ref(), Some(FLAGSHIP_MODEL))
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

/// Get daily activity data for heatmap
#[tauri::command]
fn get_activity_data(state: State<'_, AppState>) -> Result<Vec<(String, i32)>, String> {
    state.db.get_daily_activity()
        .map_err(|e| e.to_string())
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

/// Compute SHA256 hash for question text
fn compute_question_hash(question: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    question.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Mark question's best answer as needing update
#[tauri::command]
fn mark_best_answer_needs_update(
    question: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let hash = compute_question_hash(&question);
    state.db.mark_answer_needs_update(&hash)
        .map_err(|e| e.to_string())
}

/// Get or generate best answer for a question
#[tauri::command]
async fn get_or_generate_best_answer(
    question: String,
    job_description: String,
    state: State<'_, AppState>,
) -> Result<QuestionBestAnswer, String> {
    let hash = compute_question_hash(&question);
    
    // Check if cached and not needing update
    if let Ok(Some(cached)) = state.db.get_best_answer_by_hash(&hash) {
        if !cached.needs_update {
            return Ok(cached);
        }
    }
    
    // Get API client
    let client = get_client(&state)?;
    
    // Get historical answers for this question
    let historical_answers = state.db.get_all_answers_for_question(&question)
        .map_err(|e| e.to_string())?;
    
    // Generate best answer via API
    let generated = client
        .generate_best_answer(&question, &job_description, &historical_answers)
        .await
        .map_err(|e| e.to_string())?;
    
    // Save to database
    let answer_count = historical_answers.len() as i32;
    let jd_context = if job_description.is_empty() { None } else { Some(job_description.as_str()) };
    
    state.db.upsert_best_answer(&hash, &question, &generated, answer_count, jd_context)
        .map_err(|e| e.to_string())?;
    
    // Return the newly saved data
    state.db.get_best_answer_by_hash(&hash)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Failed to retrieve saved best answer".to_string())
}

/// Get comparison data with best answer
#[tauri::command]
async fn get_comparison_with_best_answer(
    question: String,
    _job_description: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    // Get history comparison
    let history = state.db.get_answers_comparison(&question)
        .map_err(|e| e.to_string())?;
    
    // Get or generate best answer
    let hash = compute_question_hash(&question);
    let best_answer = if let Ok(Some(cached)) = state.db.get_best_answer_by_hash(&hash) {
        if !cached.needs_update {
            Some(cached)
        } else {
            // Need to regenerate - but don't block, return cached for now
            Some(cached)
        }
    } else {
        None
    };
    
    Ok(serde_json::json!({
        "history": history.iter().map(|(ts, ans, fb, score)| {
            serde_json::json!({
                "timestamp": ts,
                "answer": ans,
                "feedback": fb,
                "score": score
            })
        }).collect::<Vec<_>>(),
        "bestAnswer": best_answer,
        "needsGeneration": best_answer.is_none() || best_answer.as_ref().map(|b| b.needs_update).unwrap_or(false)
    }))
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
    let mut client_guard = state.api_client.lock()
        .map_err(|e| format!("Failed to acquire API client lock: {}", e))?;
    
    if api_key.is_empty() {
        return Err("API key cannot be empty".to_string());
    }
    
    // Create new client with updated config
    let new_client = SiliconFlowClient::new(api_key, model)
        .map_err(|e| e.to_string())?;
    
    *client_guard = Some(new_client);
    
    Ok(())
}

/// Transcribe audio to text using SiliconFlow API
#[tauri::command]
async fn transcribe_audio(
    audio_base64: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let client = get_client(&state)?;
    
    // Decode base64 audio data
    let audio_data = base64::engine::general_purpose::STANDARD
        .decode(&audio_base64)
        .map_err(|e| format!("Failed to decode audio data: {}", e))?;
    
    // Generate unique filename
    let timestamp = chrono::Utc::now().timestamp_millis();
    let filename = format!("recording_{}.webm", timestamp);
    
    // Call transcription API
    client
        .transcribe_audio(&audio_data, &filename)
        .await
        .map_err(|e| format!("Transcription failed: {}", e))
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

/// Read image file and return as base64 data URL
#[tauri::command]
fn read_image_base64(file_path: String) -> Result<String, String> {
    use std::fs;
    use std::path::Path;
    
    let path = Path::new(&file_path);
    
    // Determine MIME type from extension
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .ok_or_else(|| "Invalid file extension".to_string())?;
    
    let mime_type = match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => return Err(format!("Unsupported image format: {}", ext)),
    };
    
    // Read file content
    let content = fs::read(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Encode to base64
    use base64::Engine;
    let base64_data = base64::engine::general_purpose::STANDARD.encode(&content);
    
    // Return as data URL
    Ok(format!("data:{};base64,{}", mime_type, base64_data))
}

// ===== RAG Knowledge Base Commands =====

/// Get knowledge base status
#[tauri::command]
async fn get_knowledge_base_status(
    state: State<'_, AppState>,
) -> Result<KnowledgeStatus, String> {
    let total = state.db.get_knowledge_count();
    let question_count = state.db.get_knowledge_count_by_type("question");
    let answer_count = state.db.get_knowledge_count_by_type("answer");
    
    Ok(KnowledgeStatus {
        is_empty: total == 0,
        question_count,
        answer_count,
    })
}

/// Get knowledge base stats
#[tauri::command]
async fn get_knowledge_base_stats(
    state: State<'_, AppState>,
) -> Result<KnowledgeStats, String> {
    let total = state.db.get_knowledge_count();
    let question_count = state.db.get_knowledge_count_by_type("question");
    let answer_count = state.db.get_knowledge_count_by_type("answer");
    let jd_count = state.db.get_knowledge_count_by_type("jd");
    
    Ok(KnowledgeStats {
        total_vectors: total,
        question_count,
        answer_count,
        jd_count,
    })
}

/// Initialize knowledge base in background (non-blocking)
/// Returns immediately, initialization runs asynchronously
#[tauri::command]
async fn init_knowledge_base_background(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Check if already initialized
    let count = state.db.get_knowledge_count();
    if count > 0 {
        return Ok("Knowledge base already initialized".to_string());
    }
    
    // Get API client
    let api_client = get_client(&state)?;
    
    // Clone rag service for async task
    let rag = state.rag.clone();
    
    // Spawn background task
    tokio::spawn(async move {
        log::info!("Starting background knowledge base initialization...");
        
        // Emit start event
        let _ = app.emit("knowledge-init-started", ());
        
        // Build empty index first to avoid "Index not built" warnings during generation
        if let Err(e) = rag.rebuild_index().await {
            log::warn!("Failed to build initial index: {}", e);
        }
        
        // JD templates for question generation
        let templates = vec![
            ("frontend", "高级前端工程师", "岗位职责：负责核心产品前端开发。任职要求：3年以上前端经验，精通Vue.js和React。"),
            ("backend", "高级后端工程师", "岗位职责：负责后端系统架构设计。任职要求：5年以上后端经验，精通Java/Go/Python。"),
            ("pm", "产品经理", "岗位职责：负责产品模块的需求分析和设计。任职要求：2年以上产品经理经验。"),
            ("fullstack", "全栈工程师", "岗位职责：同时负责前后端开发。任职要求：4年以上开发经验，掌握React/Vue和Node.js。"),
            ("qa", "测试工程师", "岗位职责：负责产品功能和性能测试。任职要求：2年以上测试经验，掌握自动化测试。"),
            ("devops", "DevOps工程师", "岗位职责：负责基础设施和CI/CD流程设计。任职要求：3年以上运维经验，精通Docker和K8s。"),
        ];
        
        let questions_per_category = 5; // Reduced for faster init
        let mut total_generated = 0;
        
        for (category, name, jd_content) in templates {
            log::info!("Generating questions for: {}", name);
            
            // Generate questions
            match api_client.generate_questions("", jd_content, questions_per_category, "friendly").await {
                Ok(questions) => {
                    for question in &questions {
                        // Store question with embedding
                        let metadata = serde_json::json!({
                            "category": category,
                            "jd_name": name,
                        }).to_string();
                        
                        match rag.embed_and_store(
                            "question",
                            question,
                            Some(&metadata),
                        ).await {
                            Ok(_) => {
                                total_generated += 1;
                            }
                            Err(e) => {
                                log::error!("Failed to embed and store question: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to generate questions for {}: {}", category, e);
                }
            }
            
            // Small delay to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        // Rebuild HNSW index after all inserts
        if let Err(e) = rag.rebuild_index().await {
            log::error!("Failed to rebuild index: {}", e);
        }
        
        log::info!("Knowledge base initialization complete: {} items generated", total_generated);
        
        // Emit completion event
        let _ = app.emit("knowledge-init-completed", total_generated);
    });
    
    Ok("Knowledge base initialization started in background".to_string())
}

/// Search knowledge base
#[tauri::command]
async fn search_knowledge(
    query: String,
    content_type: Option<String>,
    top_k: usize,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    // Delegate to appropriate retrieval method based on content type
    let results = match content_type.as_deref() {
        Some("question") => {
            state.rag.retrieve_similar_questions(&query, top_k)
                .await
                .map_err(|e| e.to_string())?
        }
        Some("answer") => {
            state.rag.retrieve_best_answers(&query, top_k)
                .await
                .map_err(|e| e.to_string())?
        }
        Some("jd") => {
            state.rag.retrieve_similar_jd(&query, top_k)
                .await
                .map_err(|e| e.to_string())?
        }
        _ => {
            // Default to question search
            state.rag.retrieve_similar_questions(&query, top_k)
                .await
                .map_err(|e| e.to_string())?
        }
    };
    
    Ok(results)
}

/// Rebuild knowledge base HNSW index
#[tauri::command]
async fn rebuild_knowledge_index(
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.rag.rebuild_index()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok("Knowledge index rebuilt successfully".to_string())
}

/// List knowledge entries with pagination
#[tauri::command]
async fn list_knowledge_entries(
    page: i64,
    page_size: i64,
    content_type_filter: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<crate::db::models::KnowledgeEntry>, String> {
    state.db.list_knowledge_entries(
        page,
        page_size,
        content_type_filter.as_deref(),
    ).map_err(|e| e.to_string())
}

/// Delete knowledge entry by id
#[tauri::command]
async fn delete_knowledge_entry(
    id: i64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.db.delete_knowledge_entry(id)
        .map_err(|e| e.to_string())?;
    
    // Rebuild index after deletion
    state.rag.rebuild_index()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok("Knowledge entry deleted successfully".to_string())
}

/// Search knowledge by keyword
#[tauri::command]
async fn search_knowledge_by_keyword(
    keyword: String,
    limit: i64,
    state: State<'_, AppState>,
) -> Result<Vec<crate::db::models::KnowledgeEntry>, String> {
    state.db.search_knowledge_by_keyword(&keyword, limit)
        .map_err(|e| e.to_string())
}

/// Import knowledge from file (JSON or TXT)
#[tauri::command]
async fn import_knowledge_file(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<crate::db::models::ImportResult, String> {
    use std::path::Path;
    use crate::rag::{import_from_json, import_from_txt};
    
    let path = Path::new(&file_path);
    let extension = path.extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid file extension".to_string())?;
    
    let result = match extension.to_lowercase().as_str() {
        "json" => import_from_json(path, &state.rag)
            .await
            .map_err(|e| e.to_string())?,
        "txt" => import_from_txt(path, &state.rag)
            .await
            .map_err(|e| e.to_string())?,
        _ => return Err("Unsupported file format. Use .json or .txt".to_string()),
    };
    
    // Rebuild index after import
    if result.success_count > 0 {
        state.rag.rebuild_index()
            .await
            .map_err(|e| e.to_string())?;
    }
    
    Ok(result)
}

/// Sync existing question bank to knowledge base
#[tauri::command]
async fn sync_question_bank_to_knowledge(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let questions = state.db.get_question_bank()
        .map_err(|e| e.to_string())?;
    
    let mut success = 0;
    let mut skipped = 0;
    
    for item in questions {
        let question_id = item.id.unwrap_or(0);
        
        // Check if already synced
        let metadata_check = format!("\"source_id\":{}", question_id);
        if let Ok(entries) = state.db.search_knowledge_by_keyword(&metadata_check, 1) {
            if !entries.is_empty() {
                skipped += 1;
                continue;
            }
        }
        
        // Sync to knowledge base
        let metadata = serde_json::json!({
            "source": "question_bank",
            "source_id": question_id,
            "job_category": item.job_category
        }).to_string();
        
        match state.rag.embed_and_store(
            "user_question",
            &item.question,
            Some(&metadata)
        ).await {
            Ok(_) => success += 1,
            Err(e) => log::warn!("Failed to sync question {}: {}", question_id, e)
        }
    }
    
    // Rebuild index
    if success > 0 {
        state.rag.rebuild_index().await
            .map_err(|e| e.to_string())?;
    }
    
    Ok(format!("Synced: {}, Skipped: {}", success, skipped))
}

// ============ Multi-Agent Interview Commands ============

/// Start multi-agent interview session
#[tauri::command]
async fn start_multi_agent_interview(
    resume: String,
    job_description: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let session_id = format!("ma-{}", chrono::Utc::now().timestamp_millis());
    
    // Initialize provider
    let provider = SiliconFlowProvider::from_env()
        .map_err(|e| format!("Failed to initialize provider: {}", e))?;
    
    // Create vector store adapter (RAG is optional, use no-op if unavailable)
    let vector_store = VectorStoreAdapter::new_noop();
    log::info!("Multi-Agent session started without RAG (optional enhancement)");
    
    // Create session
    let session = MultiAgentSession::new(
        resume,
        job_description,
        provider,
        vector_store,
    );
    
    // Store session
    state.multi_agent_sessions.lock().await
        .insert(session_id.clone(), session);
    
    Ok(session_id)
}

/// Get next interview question
#[tauri::command]
async fn multi_agent_next_question(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<ConversationTurn, String> {
    let mut sessions = state.multi_agent_sessions.lock().await;
    
    let session = sessions.get_mut(&session_id)
        .ok_or_else(|| "Session not found".to_string())?;
    
    // Select agent by current phase
    session.scheduler.select_by_phase(session.state_machine.current_phase());
    
    // Execute turn
    session.scheduler.execute_turn(&mut session.context)
        .await
        .map_err(|e| e.to_string())
}

/// Submit user answer
#[tauri::command]
async fn multi_agent_submit_answer(
    session_id: String,
    answer: String,
    state: State<'_, AppState>,
) -> Result<AnalysisResult, String> {
    let mut sessions = state.multi_agent_sessions.lock().await;
    
    let session = sessions.get_mut(&session_id)
        .ok_or_else(|| "Session not found".to_string())?;
    
    // Process answer
    let analysis = session.scheduler.process_answer(&mut session.context, answer)
        .await
        .map_err(|e| e.to_string())?;
    
    // Update state machine
    session.state_machine.record_question();
    if let Some(_new_phase) = session.state_machine.maybe_advance(&analysis) {
        log::info!("Phase advanced to {:?}", session.state_machine.current_phase());
    }
    
    Ok(analysis)
}

/// Get interview progress
#[tauri::command]
async fn multi_agent_get_progress(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<InterviewProgress, String> {
    let sessions = state.multi_agent_sessions.lock().await;
    
    let session = sessions.get(&session_id)
        .ok_or_else(|| "Session not found".to_string())?;
    
    Ok(session.state_machine.progress())
}

/// End multi-agent interview session
#[tauri::command]
async fn multi_agent_end_session(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.multi_agent_sessions.lock().await
        .remove(&session_id);
    
    Ok(())
}

/// Recursively copy directory contents
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) {
    if let Err(e) = std::fs::create_dir_all(dst) {
        log::error!("Failed to create dir {:?}: {}", dst, e);
        return;
    }
    
    let entries = match std::fs::read_dir(src) {
        Ok(e) => e,
        Err(e) => {
            log::error!("Failed to read dir {:?}: {}", src, e);
            return;
        }
    };
    
    for entry in entries.flatten() {
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        
        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path);
        } else if let Err(e) = std::fs::copy(&path, &dest_path) {
            log::error!("Failed to copy {:?} to {:?}: {}", path, dest_path, e);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // Load environment variables
  dotenvy::dotenv().ok();

  // Initialize API client
  let api_client = SiliconFlowClient::from_env().ok();
  
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(
      tauri_plugin_log::Builder::default()
        .level(log::LevelFilter::Info)
        .build(),
    )
    .setup(move |app| {
      use tauri::Manager;
      
      // Get app data directory using Tauri standard path
      let app_data_dir = app.path().app_data_dir()
        .expect("Failed to get app data directory");
      
      // Database path: {app_data_dir}/data/interview_spark.db
      let db_path = app_data_dir.join("data").join("interview_spark.db");
      
      // Create data directory if it doesn't exist
      if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
      }
      
      log::info!("Database path: {:?}", db_path);
      
      // Model directory for embedding
      let model_dir = app_data_dir.join("models").join("bge-small-zh-v1.5");
      
      // Copy bundled model files if not exists
      if !model_dir.join("onnx").join("model.onnx").exists() {
        log::info!("Copying bundled embedding model to {:?}", model_dir);
        if let Ok(resource_dir) = app.path().resource_dir() {
          let bundled_model = resource_dir.join("resources").join("models").join("models--Xenova--bge-small-zh-v1.5");
          if bundled_model.exists() {
            copy_dir_recursive(&bundled_model, &model_dir);
            log::info!("Bundled model copied successfully");
          } else {
            log::warn!("Bundled model not found at {:?}", bundled_model);
          }
        }
      }
      
      log::info!("Embedding model dir: {:?}", model_dir);
      
      let conn = init_database(db_path.clone())
        .expect("Failed to initialize database");
      
      let repository = Arc::new(Repository::new(conn));
      let cache_manager = Arc::new(CacheManager::new());
      
      // Initialize RAG service (lazy-loaded on first use)
      let rag_service = Arc::new(RagService::new(repository.clone(), db_path, model_dir));
      
      // Manage AppState in setup
      app.manage(AppState {
        api_client: Mutex::new(api_client.clone()),
        db: repository,
        cache: cache_manager,
        rag: rag_service,
        // Initialize generic caches with appropriate TTLs
        session_cache: Arc::new(analysis::GenericCache::new(300)), // 5 min TTL
        question_bank_cache: Arc::new(analysis::GenericCache::new(600)), // 10 min TTL
        // Initialize multi-agent sessions storage
        multi_agent_sessions: Arc::new(TokioMutex::new(HashMap::new())),
      });
      
      Ok(())
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
      db_get_sessions_with_count,
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
      analyze_missing_answers,
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
      get_activity_data,
      get_answers_comparison,
      mark_best_answer_needs_update,
      get_or_generate_best_answer,
      get_comparison_with_best_answer,
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
      analyze_star_score,
      create_user,
      get_all_users,
      get_current_user,
      switch_user,
      update_user,
      delete_user,
      upload_avatar,
      get_avatar_path,
      read_image_base64,
      transcribe_audio,
      get_knowledge_base_status,
      get_knowledge_base_stats,
      init_knowledge_base_background,
      search_knowledge,
      rebuild_knowledge_index,
      list_knowledge_entries,
      delete_knowledge_entry,
      search_knowledge_by_keyword,
      import_knowledge_file,
      sync_question_bank_to_knowledge,
      // Multi-Agent interview commands
      start_multi_agent_interview,
      multi_agent_next_question,
      multi_agent_submit_answer,
      multi_agent_get_progress,
      multi_agent_end_session
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
