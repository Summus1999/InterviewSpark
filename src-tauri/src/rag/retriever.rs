//! Retriever utilities for RAG

use super::vectordb::SearchResult;

/// Retriever utility methods
pub struct Retriever;

impl Retriever {
    /// Create placeholder (for compatibility)
    pub fn new_shared() -> Self {
        Self
    }

    /// Build context string from search results
    pub fn build_context(results: &[SearchResult], max_length: usize) -> String {
        let mut context = String::new();
        let mut current_length = 0;

        for (idx, result) in results.iter().enumerate() {
            let entry = format!("{}. {}\n", idx + 1, result.content);
            if current_length + entry.len() > max_length {
                break;
            }
            context.push_str(&entry);
            current_length += entry.len();
        }

        context
    }
}
