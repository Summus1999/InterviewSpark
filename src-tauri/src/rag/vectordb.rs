//! Vector database for storing and retrieving embeddings

use anyhow::{Context, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use hnsw_rs::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Search result from vector store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: i64,
    pub content: String,
    pub content_type: String,
    pub metadata: Option<String>,
    pub similarity: f32,
}

/// Vector store for managing embeddings
pub struct VectorStore {
    conn: Arc<Mutex<Connection>>,
    index: Arc<Mutex<Option<Hnsw<'static, f32, DistCosine>>>>,
}

impl VectorStore {
    /// Create new vector store
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
            index: Arc::new(Mutex::new(None)),
        }
    }

    /// Insert new vector into store
    pub async fn insert(
        &self,
        content_type: &str,
        content: &str,
        embedding: &[f32],
        metadata: Option<&str>,
    ) -> Result<i64> {
        let conn = self.conn.lock().await;
        let now = chrono::Utc::now().to_rfc3339();
        
        // Serialize embedding to bytes
        let embedding_bytes = bincode::serialize(embedding)
            .context("Failed to serialize embedding")?;
        
        conn.execute(
            "INSERT INTO knowledge_vectors (content_type, content, embedding, metadata, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                content_type,
                content,
                embedding_bytes,
                metadata,
                now
            ],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Build HNSW index from database
    pub async fn build_index(&self) -> Result<()> {
        let conn = self.conn.lock().await;
        
        let embeddings: Vec<(usize, Vec<f32>)> = {
            let mut stmt = conn.prepare(
                "SELECT id, embedding FROM knowledge_vectors"
            )?;
            
            let result = stmt
                .query_map([], |row| {
                    let id: i64 = row.get(0)?;
                    let embedding_bytes: Vec<u8> = row.get(1)?;
                    let embedding: Vec<f32> = bincode::deserialize(&embedding_bytes)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                    Ok((id as usize, embedding))
                })?
                .collect::<Result<Vec<_>, _>>()?;
            result
        }; // stmt is dropped here
        
        if embeddings.is_empty() {
            return Ok(());
        }
        
        // Get dimension from first embedding
        let dim = embeddings[0].1.len();
        
        // Build HNSW index
        let nb_layer = 16.min((embeddings.len() as f32).ln() as usize);
        let max_nb_connection = 48;
        let ef_construction = 200;
        
        let hnsw = Hnsw::<f32, DistCosine>::new(
            max_nb_connection,
            embeddings.len(),
            nb_layer,
            ef_construction,
            DistCosine {},
        );
        
        for (id, embedding) in &embeddings {
            hnsw.insert((embedding, *id));
        }
        
        let mut index = self.index.lock().await;
        *index = Some(hnsw);
        
        log::info!("Built HNSW index with {} vectors, dim={}", embeddings.len(), dim);
        Ok(())
    }

    /// Search similar vectors using HNSW
    pub async fn search(
        &self,
        embedding: &[f32],
        top_k: usize,
        content_type: Option<&str>,
    ) -> Result<Vec<SearchResult>> {
        // Get neighbors from HNSW
        let index_guard = self.index.lock().await;
        let hnsw = index_guard.as_ref().context("Index not built")?;
        
        let ef_search = (top_k * 2).max(50);
        let neighbors = hnsw.search(embedding, top_k, ef_search);
        
        // Retrieve content from database
        let conn = self.conn.lock().await;
        let mut results = Vec::new();
        
        for neighbor in neighbors {
            let id = neighbor.d_id as i64;
            let similarity = 1.0 - neighbor.distance; // Convert distance to similarity
            
            let mut query = "SELECT id, content, content_type, metadata FROM knowledge_vectors WHERE id = ?1".to_string();
            if let Some(ct) = content_type {
                query = format!("{} AND content_type = '{}'", query, ct);
            }
            
            let result: Result<SearchResult, _> = conn.query_row(
                &query,
                rusqlite::params![id],
                |row| {
                    Ok(SearchResult {
                        id: row.get(0)?,
                        content: row.get(1)?,
                        content_type: row.get(2)?,
                        metadata: row.get(3)?,
                        similarity,
                    })
                },
            );
            
            if let Ok(res) = result {
                results.push(res);
            }
        }
        
        Ok(results)
    }

    /// Delete vector by id
    #[allow(dead_code)]
    pub async fn delete(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM knowledge_vectors WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Count total vectors
    #[allow(dead_code)]
    pub async fn count(&self) -> Result<i64> {
        let conn = self.conn.lock().await;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM knowledge_vectors",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    /// Count vectors by type
    #[allow(dead_code)]
    pub async fn count_by_type(&self, content_type: &str) -> Result<i64> {
        let conn = self.conn.lock().await;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM knowledge_vectors WHERE content_type = ?1",
            [content_type],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}
