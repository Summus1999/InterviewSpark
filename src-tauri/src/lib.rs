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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
