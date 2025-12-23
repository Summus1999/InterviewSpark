//! Knowledge import module for batch importing knowledge entries

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use crate::db::models::ImportResult;

/// Knowledge item for import (JSON format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub content_type: String,  // "question" | "answer" | "jd"
    pub content: String,
    pub metadata: Option<String>,
}

/// Import knowledge from JSON file
/// 
/// Expected JSON format: array of KnowledgeItem
/// ```json
/// [
///   {
///     "content_type": "question",
///     "content": "What is your experience with Rust?",
///     "metadata": null
///   }
/// ]
/// ```
pub async fn import_from_json(
    file_path: &Path,
    rag_service: &crate::rag::RagService,
) -> Result<ImportResult> {
    let content = fs::read_to_string(file_path)
        .context("Failed to read JSON file")?;
    
    let items: Vec<KnowledgeItem> = serde_json::from_str(&content)
        .context("Failed to parse JSON")?;
    
    let mut success_count = 0;
    let mut fail_count = 0;
    let mut errors = Vec::new();
    
    for (idx, item) in items.iter().enumerate() {
        match rag_service.embed_and_store(
            &item.content_type,
            &item.content,
            item.metadata.as_deref(),
        ).await {
            Ok(_) => success_count += 1,
            Err(e) => {
                fail_count += 1;
                errors.push(format!("Line {}: {}", idx + 1, e));
            }
        }
    }
    
    Ok(ImportResult {
        success_count,
        fail_count,
        errors,
    })
}

/// Import knowledge from TXT file
/// 
/// Expected format: one knowledge entry per line
/// Format: content_type|content|metadata (metadata is optional)
/// 
/// Example:
/// ```
/// question|What is your experience with Rust?|
/// answer|I have 3 years of Rust development experience|backend
/// ```
pub async fn import_from_txt(
    file_path: &Path,
    rag_service: &crate::rag::RagService,
) -> Result<ImportResult> {
    let content = fs::read_to_string(file_path)
        .context("Failed to read TXT file")?;
    
    let mut success_count = 0;
    let mut fail_count = 0;
    let mut errors = Vec::new();
    
    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }
        
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 2 {
            fail_count += 1;
            errors.push(format!("Line {}: Invalid format (need at least 2 fields)", line_num + 1));
            continue;
        }
        
        let content_type = parts[0].trim();
        let content_text = parts[1].trim();
        let metadata = if parts.len() > 2 && !parts[2].trim().is_empty() {
            Some(parts[2].trim())
        } else {
            None
        };
        
        match rag_service.embed_and_store(content_type, content_text, metadata).await {
            Ok(_) => success_count += 1,
            Err(e) => {
                fail_count += 1;
                errors.push(format!("Line {}: {}", line_num + 1, e));
            }
        }
    }
    
    Ok(ImportResult {
        success_count,
        fail_count,
        errors,
    })
}
