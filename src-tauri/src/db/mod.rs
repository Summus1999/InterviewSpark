//! Database module for SQLite data persistence
//!
//! This module provides:
//! - Schema initialization and migrations
//! - Data models for all entities
//! - Repository layer for CRUD operations

pub mod models;
pub mod repository;
pub mod schema;

pub use models::*;
pub use repository::Repository;
pub use schema::init_database;
