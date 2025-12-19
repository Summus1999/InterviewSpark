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
pub mod analytics;
pub mod dashboard;
pub mod trends;
pub mod backup;
pub mod cache;

pub use content::ContentAnalyzer;
pub use scoring::ScoringEngine;
pub use report::ReportGenerator;
pub use export::ReportExporter;
pub use analytics::{AnalyticsEngine, TrendAnalytics, TrendDataPoint, PerformanceStatistics};
pub use dashboard::{DashboardService, DashboardData, DashboardStats, TopQuestion, WeakArea, RecentSessionInfo};
pub use backup::{BackupManager, BackupData};
pub use cache::{CacheManager, CacheEntry};
