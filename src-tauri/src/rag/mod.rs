//! RAG (Retrieval-Augmented Generation) module
//! 
//! Provides semantic search and knowledge base management for interview questions

pub mod embedding;
pub mod vectordb;
pub mod retriever;
pub mod bootstrap;
pub mod service;

pub use embedding::EmbeddingService;
#[allow(unused_imports)]
pub use vectordb::{VectorStore, SearchResult};
#[allow(unused_imports)]
pub use bootstrap::{KnowledgeBootstrap, BootstrapProgress, BootstrapResult, KnowledgeStatus, KnowledgeStats};
pub use service::RagService;
