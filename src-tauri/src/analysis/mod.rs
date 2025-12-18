//! Analysis module for interview answer evaluation
//! 
//! This module provides multi-dimensional analysis of interview answers:
//! - Content analysis (logic, job match, keyword coverage)
//! - Expression analysis (clarity, presentation)
//! - Overall scoring and feedback generation

pub mod content;
pub mod scoring;
pub mod report;
pub mod export;
pub mod trends;
pub mod backup;
pub mod cache;

pub use content::ContentAnalyzer;
pub use scoring::ScoringEngine;
