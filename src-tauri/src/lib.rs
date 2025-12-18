//! InterviewSpark - AI-powered interview simulation application
//!
//! This module contains the Tauri backend commands for:
//! - IPC communication testing (greet)
//! - AI question generation (generate_questions)
//! - Answer analysis and feedback (analyze_answer)

mod api;

use api::SiliconFlowClient;
use std::sync::Mutex;
use tauri::State;

/// Application state holding the SiliconFlow API client
/// 
/// The client is wrapped in Mutex<Option<>> to handle:
/// - Thread-safe access across async commands
/// - Graceful handling when API key is not configured
struct AppState {
    api_client: Mutex<Option<SiliconFlowClient>>,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // Load environment variables
  dotenvy::dotenv().ok();

  // Initialize API client
  let api_client = SiliconFlowClient::from_env().ok();
  
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
    })
    .invoke_handler(tauri::generate_handler![greet, generate_questions, analyze_answer])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
