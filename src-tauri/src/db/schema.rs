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
    overall_score REAL DEFAULT 0,
    logic_score REAL DEFAULT 0,
    match_score REAL DEFAULT 0,
    keyword_score REAL DEFAULT 0,
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

-- Answer analysis table
CREATE TABLE IF NOT EXISTS answer_analysis (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    answer_id INTEGER NOT NULL UNIQUE,
    content_score REAL NOT NULL,
    logic_score REAL NOT NULL,
    job_match_score REAL NOT NULL,
    keyword_coverage REAL NOT NULL,
    expression_score REAL,
    overall_score REAL NOT NULL,
    strengths TEXT NOT NULL,
    weaknesses TEXT NOT NULL,
    suggestions TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (answer_id) REFERENCES interview_answers(id)
);

-- Session report table
CREATE TABLE IF NOT EXISTS session_reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL UNIQUE,
    overall_score REAL NOT NULL,
    content_analysis TEXT NOT NULL,
    expression_analysis TEXT,
    summary TEXT NOT NULL,
    improvements TEXT NOT NULL,
    key_takeaways TEXT NOT NULL,
    reference_answers TEXT,
    generated_at TEXT NOT NULL,
    api_response_time INTEGER,
    FOREIGN KEY (session_id) REFERENCES interview_sessions(id)
);

-- Performance statistics table
CREATE TABLE IF NOT EXISTS performance_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_date TEXT NOT NULL UNIQUE,
    total_sessions INTEGER NOT NULL,
    average_score REAL NOT NULL,
    content_avg REAL NOT NULL,
    expression_avg REAL,
    highest_score REAL NOT NULL,
    lowest_score REAL NOT NULL,
    improvement_trend REAL NOT NULL,
    recorded_at TEXT NOT NULL
);

-- Create indices for performance optimization
CREATE INDEX IF NOT EXISTS idx_answer_analysis_answer_id ON answer_analysis(answer_id);
CREATE INDEX IF NOT EXISTS idx_session_reports_session_id ON session_reports(session_id);
CREATE INDEX IF NOT EXISTS idx_performance_stats_date ON performance_stats(session_date);
CREATE INDEX IF NOT EXISTS idx_interview_answers_session_id ON interview_answers(session_id);
CREATE INDEX IF NOT EXISTS idx_interview_sessions_created_at ON interview_sessions(created_at);
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
