// VectorStore adapter for existing HNSW implementation
// Bridges existing VectorStore to simplified interface

use anyhow::Result;
use std::sync::Arc;
use crate::rag::{EmbeddingService, VectorStore};

/// VectorStore adapter - wraps existing VectorStore and EmbeddingService
/// Supports graceful degradation when RAG is unavailable
#[derive(Clone)]
pub struct VectorStoreAdapter {
    embedding_service: Option<Arc<EmbeddingService>>,
    vector_store: Option<Arc<VectorStore>>,
}

impl VectorStoreAdapter {
    /// Create new adapter with real RAG services
    pub fn new(
        embedding_service: Arc<EmbeddingService>,
        vector_store: Arc<VectorStore>,
    ) -> Self {
        Self {
            embedding_service: Some(embedding_service),
            vector_store: Some(vector_store),
        }
    }
    
    /// Create no-op adapter (for when RAG is unavailable)
    pub fn new_noop() -> Self {
        Self {
            embedding_service: None,
            vector_store: None,
        }
    }
    
    /// Check if RAG is available
    #[allow(dead_code)]
    pub fn is_available(&self) -> bool {
        self.embedding_service.is_some() && self.vector_store.is_some()
    }
    
    /// Search for top N similar items
    #[allow(dead_code)]
    pub async fn top_n(
        &self,
        query: &str,
        n: usize,
    ) -> Result<Vec<(f64, String, String)>> {
        // Return empty if RAG not available
        let (embedding_service, vector_store) = match (&self.embedding_service, &self.vector_store) {
            (Some(e), Some(v)) => (e, v),
            _ => return Ok(vec![]),
        };
        
        // Generate query embedding
        let embedding = embedding_service
            .embed_text(query)
            .await?;
        
        // Search using HNSW
        let results = vector_store
            .search(&embedding, n, None)
            .await?;
        
        // Convert to (score, id, content) format
        let converted: Vec<(f64, String, String)> = results
            .into_iter()
            .map(|r| {
                (
                    r.similarity as f64,
                    r.id.to_string(),
                    r.content,
                )
            })
            .collect();
        
        Ok(converted)
    }
    
    /// Search for top N IDs only
    #[allow(dead_code)]
    pub async fn top_n_ids(
        &self,
        query: &str,
        n: usize,
    ) -> Result<Vec<(f64, String)>> {
        // Return empty if RAG not available
        let (embedding_service, vector_store) = match (&self.embedding_service, &self.vector_store) {
            (Some(e), Some(v)) => (e, v),
            _ => return Ok(vec![]),
        };
        
        // Generate query embedding
        let embedding = embedding_service
            .embed_text(query)
            .await?;
        
        // Search using HNSW
        let results = vector_store
            .search(&embedding, n, None)
            .await?;
        
        // Convert to (score, id) format
        let converted: Vec<(f64, String)> = results
            .into_iter()
            .map(|r| (r.similarity as f64, r.id.to_string()))
            .collect();
        
        Ok(converted)
    }
    
    /// Search by content type
    pub async fn search_by_type(
        &self,
        query: &str,
        n: usize,
        content_type: &str,
    ) -> Result<Vec<(f64, String, String)>> {
        // Return empty if RAG not available
        let (embedding_service, vector_store) = match (&self.embedding_service, &self.vector_store) {
            (Some(e), Some(v)) => (e, v),
            _ => return Ok(vec![]),
        };
        
        // Generate query embedding
        let embedding = embedding_service
            .embed_text(query)
            .await?;
        
        // Search using HNSW with content type filter
        let results = vector_store
            .search(&embedding, n, Some(content_type))
            .await?;
        
        // Convert to (score, id, content) format
        let converted: Vec<(f64, String, String)> = results
            .into_iter()
            .map(|r| {
                (
                    r.similarity as f64,
                    r.id.to_string(),
                    r.content,
                )
            })
            .collect();
        
        Ok(converted)
    }
}
