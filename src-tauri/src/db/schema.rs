//! Database schema initialization and migrations

use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

/// SQL statements for creating tables
const CREATE_TABLES_SQL: &str = r#"
-- Users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    avatar_color TEXT DEFAULT '#3b82f6',
    created_at TEXT NOT NULL
);

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
    user_id INTEGER NOT NULL DEFAULT 1,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Job description table
CREATE TABLE IF NOT EXISTS job_descriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL DEFAULT 1,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Interview session table
CREATE TABLE IF NOT EXISTS interview_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL DEFAULT 1,
    resume_id INTEGER,
    job_description_id INTEGER,
    questions TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
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
    user_id INTEGER NOT NULL DEFAULT 1,
    question TEXT NOT NULL,
    best_answer TEXT,
    notes TEXT,
    job_category TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Question tags table
CREATE TABLE IF NOT EXISTS question_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL DEFAULT 1,
    name TEXT NOT NULL,
    color TEXT DEFAULT '#667eea',
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, name)
);

-- Question tag mappings table (many-to-many)
CREATE TABLE IF NOT EXISTS question_tag_mappings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question_bank_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (question_bank_id) REFERENCES question_bank(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES question_tags(id) ON DELETE CASCADE,
    UNIQUE(question_bank_id, tag_id)
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
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_resumes_user_id ON resumes(user_id);
CREATE INDEX IF NOT EXISTS idx_job_descriptions_user_id ON job_descriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_interview_sessions_user_id ON interview_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_question_bank_user_id ON question_bank(user_id);
CREATE INDEX IF NOT EXISTS idx_question_tags_user_id ON question_tags(user_id);
CREATE INDEX IF NOT EXISTS idx_answer_analysis_answer_id ON answer_analysis(answer_id);
CREATE INDEX IF NOT EXISTS idx_session_reports_session_id ON session_reports(session_id);
CREATE INDEX IF NOT EXISTS idx_performance_stats_date ON performance_stats(session_date);
CREATE INDEX IF NOT EXISTS idx_interview_answers_session_id ON interview_answers(session_id);
CREATE INDEX IF NOT EXISTS idx_interview_sessions_created_at ON interview_sessions(created_at);
CREATE INDEX IF NOT EXISTS idx_tag_mappings_question_id ON question_tag_mappings(question_bank_id);
CREATE INDEX IF NOT EXISTS idx_tag_mappings_tag_id ON question_tag_mappings(tag_id);
"#;

/// Check if a column exists in a table
fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool> {
    let query = format!("PRAGMA table_info({})", table);
    let mut stmt = conn.prepare(&query)?;
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get(1))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(columns.contains(&column.to_string()))
}

/// Migrate existing tables to add user_id column
fn migrate_tables(conn: &Connection) -> Result<()> {
    // Tables that need user_id column
    let tables = vec![
        "resumes",
        "job_descriptions",
        "interview_sessions",
        "question_bank",
        "question_tags"
    ];
    
    for table in tables {
        // Check if table exists
        let table_exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1)",
                [table],
                |row| row.get(0)
            )
            .unwrap_or(false);
        
        if table_exists && !column_exists(conn, table, "user_id")? {
            log::info!("Migrating table {} to add user_id column", table);
            conn.execute(
                &format!("ALTER TABLE {} ADD COLUMN user_id INTEGER NOT NULL DEFAULT 1", table),
                []
            )?;
        }
    }
    
    Ok(())
}

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
    
    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    // Migrate existing tables before creating new schema
    migrate_tables(&conn)?;
    
    // Execute schema creation
    conn.execute_batch(CREATE_TABLES_SQL)?;
    
    // Create default user if not exists
    let default_user_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE id = 1)",
            [],
            |row| row.get(0)
        )
        .unwrap_or(false);
    
    if !default_user_exists {
        use crate::db::models::now;
        conn.execute(
            "INSERT INTO users (id, username, avatar_color, created_at) VALUES (1, '默认用户', '#3b82f6', ?1)",
            [now()]
        )?;
        log::info!("Created default user");
    }
    
    Ok(conn)
}
