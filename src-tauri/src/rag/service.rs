//! RAG service with lazy initialization
//! Provides high-level interface for knowledge retrieval with delayed model loading

use anyhow::{Result, anyhow};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::OnceCell;
use super::{EmbeddingService, VectorStore, vectordb::SearchResult};
use super::retriever::Retriever;
use crate::db::Repository;

/// Timeout for RAG initialization (10 seconds)
const INIT_TIMEOUT: Duration = Duration::from_secs(10);

/// Internal state for RAG service
struct RagInternals {
    embedding_service: EmbeddingService,
    vector_store: VectorStore,
    #[allow(dead_code)]
    retriever: Retriever,
}

/// RAG service with lazy initialization
pub struct RagService {
    db: Arc<Repository>,
    db_path: PathBuf,
    model_dir: PathBuf,
    internals: OnceCell<Arc<RagInternals>>,
    init_failed: AtomicBool,
}

impl RagService {
    /// Create new RAG service (uninitialized)
    /// 
    /// # Arguments
    /// * `db` - Database repository
    /// * `db_path` - Path to database file
    /// * `model_dir` - Directory containing local model files
    pub fn new(db: Arc<Repository>, db_path: PathBuf, model_dir: PathBuf) -> Self {
        Self {
            db,
            db_path,
            model_dir,
            internals: OnceCell::new(),
            init_failed: AtomicBool::new(false),
        }
    }

    /// Ensure internals are initialized (lazy loading with timeout)
    async fn ensure_initialized(&self) -> Result<Arc<RagInternals>> {
        // Fast path: if init already failed, don't retry
        if self.init_failed.load(Ordering::Relaxed) {
            return Err(anyhow!("RAG initialization previously failed"));
        }
        
        // Try to get or init with timeout
        match tokio::time::timeout(INIT_TIMEOUT, self.do_init()).await {
            Ok(result) => result,
            Err(_) => {
                self.init_failed.store(true, Ordering::Relaxed);
                log::error!("RAG initialization timed out after {:?}", INIT_TIMEOUT);
                Err(anyhow!("RAG initialization timed out"))
            }
        }
    }
    
    /// Internal initialization logic
    async fn do_init(&self) -> Result<Arc<RagInternals>> {
        self.internals
            .get_or_try_init(|| async {
                log::info!("Initializing RAG service (first-time use)...");
                log::info!("Model directory: {:?}", self.model_dir);
                
                // Clone paths for move into spawn_blocking
                let model_dir = self.model_dir.clone();
                let db_path = self.db_path.clone();
                
                // Run blocking initialization in dedicated thread pool
                let (embedding_service, vector_store) = tokio::task::spawn_blocking(move || {
                    log::info!("Loading embedding model in blocking thread...");
                    
                    // Initialize embedding service from local model files (blocking I/O)
                    let embedding_service = EmbeddingService::new_from_local(model_dir)?;
                    
                    // Open new connection for vector store (blocking I/O)
                    let conn = rusqlite::Connection::open(&db_path)?;
                    let vector_store = VectorStore::new(conn);
                    
                    log::info!("Blocking initialization complete");
                    Ok::<_, anyhow::Error>((embedding_service, vector_store))
                })
                .await
                .map_err(|e| anyhow::anyhow!("Spawn blocking task failed: {}", e))??;
                
                // Build HNSW index (async operation)
                vector_store.build_index().await?;
                
                // Create retriever
                let retriever = Retriever::new_shared();
                
                log::info!("RAG service initialized successfully");
                
                Ok(Arc::new(RagInternals {
                    embedding_service,
                    vector_store,
                    retriever,
                }))
            })
            .await
            .map(Arc::clone)
    }

    /// Embed text and store in knowledge base
    pub async fn embed_and_store(
        &self,
        content_type: &str,
        content: &str,
        metadata: Option<&str>,
    ) -> Result<i64> {
        let internals = self.ensure_initialized().await?;
        
        // Generate embedding
        let embedding = internals.embedding_service.embed_text(content).await?;
        
        // Store in database (VectorStore handles serialization)
        let id = internals.vector_store.insert(
            content_type,
            content,
            &embedding,
            metadata,
        ).await?;
        
        Ok(id)
    }

    /// Retrieve similar questions from knowledge base
    pub async fn retrieve_similar_questions(
        &self,
        jd: &str,
        top_k: usize,
    ) -> Result<Vec<SearchResult>> {
        let internals = self.ensure_initialized().await?;
        let embedding = internals.embedding_service.embed_text(jd).await?;
        internals.vector_store.search(&embedding, top_k, Some("question")).await
    }

    /// Retrieve best answers for a question
    pub async fn retrieve_best_answers(
        &self,
        question: &str,
        top_k: usize,
    ) -> Result<Vec<SearchResult>> {
        let internals = self.ensure_initialized().await?;
        let embedding = internals.embedding_service.embed_text(question).await?;
        internals.vector_store.search(&embedding, top_k, Some("answer")).await
    }

    /// Retrieve similar job descriptions
    pub async fn retrieve_similar_jd(
        &self,
        jd: &str,
        top_k: usize,
    ) -> Result<Vec<SearchResult>> {
        let internals = self.ensure_initialized().await?;
        let embedding = internals.embedding_service.embed_text(jd).await?;
        internals.vector_store.search(&embedding, top_k, Some("jd")).await
    }

    /// Rebuild HNSW index
    pub async fn rebuild_index(&self) -> Result<()> {
        let internals = self.ensure_initialized().await?;
        internals.vector_store.build_index().await
    }

    /// Build context string from search results
    pub fn build_context(results: &[SearchResult], max_length: usize) -> String {
        Retriever::build_context(results, max_length)
    }

    /// Check if knowledge base is empty
    pub fn is_empty(&self) -> bool {
        self.db.get_knowledge_count() == 0
    }
}
