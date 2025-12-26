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
pub mod cache_manager;
pub mod profile;
pub mod recommendation;
pub mod best_practices;
pub mod industry;

pub use content::ContentAnalyzer;
// STAR types are part of the public API and used by frontend via Tauri commands
#[allow(unused_imports)]
pub use scoring::{ScoringEngine, STARScoringEngine, STARScoringResult, STARScoreBreakdown};
// Report types are part of the public API
#[allow(unused_imports)]
pub use report::ReportGenerator;
// Report types are part of the public API
#[allow(unused_imports)]
pub use export::ReportExporter;
// Analytics types are part of the public API
#[allow(unused_imports)]
pub use analytics::{AnalyticsEngine, TrendAnalytics, TrendDataPoint, PerformanceStatistics};
pub use dashboard::{DashboardService, DashboardData};
// Backup types are part of the public API
#[allow(unused_imports)]
pub use backup::{BackupManager, BackupData};
// Cache types are part of the public API
#[allow(unused_imports)]
pub use cache::{CacheManager, CacheEntry};
#[allow(unused_imports)]
pub use cache_manager::{GenericCache, CacheStats};
pub use profile::ProfileGenerator;
pub use recommendation::RecommendationEngine;
pub use best_practices::BestPracticesExtractor;
pub use industry::IndustryComparisonGenerator;
