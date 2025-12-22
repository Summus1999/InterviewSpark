//! RAG (Retrieval-Augmented Generation) module
//! 
//! Provides semantic search and knowledge base management for interview questions

pub mod embedding;
pub mod vectordb;
pub mod retriever;
pub mod bootstrap;
pub mod service;

pub use embedding::EmbeddingService;
pub use vectordb::{VectorStore, SearchResult};
pub use bootstrap::{KnowledgeBootstrap, BootstrapProgress, BootstrapResult, KnowledgeStatus, KnowledgeStats};
pub use service::RagService;
