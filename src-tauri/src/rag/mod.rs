//! RAG (Retrieval-Augmented Generation) module
//! 
//! Provides semantic search and knowledge base management for interview questions

pub mod embedding;
pub mod vectordb;
pub mod retriever;
pub mod bootstrap;
pub mod service;
pub mod import;

pub use embedding::EmbeddingService;
#[allow(unused_imports)]
pub use vectordb::{VectorStore, SearchResult};
#[allow(unused_imports)]
pub use bootstrap::{KnowledgeBootstrap, BootstrapProgress, BootstrapResult, KnowledgeStatus, KnowledgeStats};
pub use service::RagService;
pub use import::{import_from_json, import_from_txt};
