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
pub mod profile;
pub mod recommendation;
pub mod best_practices;
pub mod industry;

pub use content::ContentAnalyzer;
pub use scoring::{ScoringEngine, STARScoringEngine, STARScoringResult, STARScoreBreakdown};
pub use report::ReportGenerator;
pub use export::ReportExporter;
pub use analytics::{AnalyticsEngine, TrendAnalytics, TrendDataPoint, PerformanceStatistics};
pub use dashboard::{DashboardService, DashboardData};
pub use backup::{BackupManager, BackupData};
pub use cache::{CacheManager, CacheEntry};
pub use profile::ProfileGenerator;
pub use recommendation::RecommendationEngine;
pub use best_practices::BestPracticesExtractor;
pub use industry::IndustryComparisonGenerator;
