//! InterviewSpark - AI-powered interview simulation application
//!
//! This module contains the Tauri backend commands for:
//! - IPC communication testing (greet)
//! - AI question generation (generate_questions)
//! - Answer analysis and feedback (analyze_answer)
//! - Data persistence (resumes, job descriptions, interview sessions, question bank)

mod api;
mod db;

use api::SiliconFlowClient;
use db::{init_database, Repository, Resume, JobDescription, InterviewSession, InterviewAnswer, QuestionBankItem};
use std::sync::{Arc, Mutex};
use tauri::State;

/// Application state holding the SiliconFlow API client and database
/// 
/// The client is wrapped in Mutex<Option<>> to handle:
/// - Thread-safe access across async commands
/// - Graceful handling when API key is not configured
///
/// The repository is wrapped in Arc for shared ownership across threads
struct AppState {
    api_client: Mutex<Option<SiliconFlowClient>>,
    db: Arc<Repository>,
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
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let client = {
        let client_guard = state.api_client.lock().unwrap();
        client_guard.clone().ok_or("API client not initialized")?
    };
    
    client
        .generate_questions(&resume, &job_description, count)
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
    state: State<'_, AppState>,
) -> Result<String, String> {
    let client = {
        let client_guard = state.api_client.lock().unwrap();
        client_guard.clone().ok_or("API client not initialized")?
    };
    
    client
        .analyze_answer(&question, &answer, &job_description)
        .await
        .map_err(|e| e.to_string())
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
    })
    .invoke_handler(tauri::generate_handler![
      greet,
      generate_questions,
      analyze_answer,
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
      db_delete_from_bank
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
