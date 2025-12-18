//! Database schema initialization and migrations

use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

/// SQL statements for creating tables
const CREATE_TABLES_SQL: &str = r#"
-- User configuration table
CREATE TABLE IF NOT EXISTS user_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT UNIQUE NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Resume storage table
CREATE TABLE IF NOT EXISTS resumes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Job description table
CREATE TABLE IF NOT EXISTS job_descriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Interview session table
CREATE TABLE IF NOT EXISTS interview_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    resume_id INTEGER,
    job_description_id INTEGER,
    questions TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (resume_id) REFERENCES resumes(id),
    FOREIGN KEY (job_description_id) REFERENCES job_descriptions(id)
);

-- Interview answer table
CREATE TABLE IF NOT EXISTS interview_answers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    question_index INTEGER NOT NULL,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    feedback TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES interview_sessions(id)
);

-- Question bank table
CREATE TABLE IF NOT EXISTS question_bank (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question TEXT NOT NULL,
    best_answer TEXT,
    notes TEXT,
    job_category TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
"#;

/// Initialize database and create tables
///
/// # Arguments
/// * `db_path` - Path to the database file
///
/// # Returns
/// * `Ok(Connection)` - Database connection if successful
/// * `Err` - Error if initialization fails
pub fn init_database(db_path: PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    // Execute schema creation
    conn.execute_batch(CREATE_TABLES_SQL)?;
    
    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    Ok(conn)
}
