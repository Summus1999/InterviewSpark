//! Analytics engine for trend analysis and performance tracking

use crate::db::Repository;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Data point for trend visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendDataPoint {
    pub timestamp: i64,
    pub overall_score: f32,
    pub communication_score: f32,
    pub problem_solving_score: f32,
    pub technical_depth_score: f32,
    pub presentation_score: f32,
}

/// Performance statistics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStatistics {
    pub total_sessions: i32,
    pub average_overall: f32,
    pub highest_overall: f32,
    pub improvement_rate: f32,
    pub recent_trend: String,
}

/// Complete analytics data including trends and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalytics {
    pub data_points: Vec<TrendDataPoint>,
    pub statistics: PerformanceStatistics,
}

/// Analytics engine for performance analysis
pub struct AnalyticsEngine {
    db: Arc<Repository>,
}

impl AnalyticsEngine {
    /// Create new analytics engine instance
    pub fn new(db: Arc<Repository>) -> Self {
        Self { db }
    }

    /// Get trend analytics with optional time range filtering
    pub fn get_trend_analytics(&self, time_range: Option<i64>) -> Result<TrendAnalytics> {
        // Get historical data points
        let reports = self.db.get_historical_reports(time_range)?;
        
        let data_points: Vec<TrendDataPoint> = reports
            .into_iter()
            .map(|(timestamp, overall, communication, problem_solving, technical_depth, presentation)| {
                TrendDataPoint {
                    timestamp,
                    overall_score: overall,
                    communication_score: communication,
                    problem_solving_score: problem_solving,
                    technical_depth_score: technical_depth,
                    presentation_score: presentation,
                }
            })
            .collect();

        // Get statistics
        let (total_sessions, average_overall, highest_overall, improvement_rate, recent_trend) =
            self.db.get_statistics(time_range)?;

        let statistics = PerformanceStatistics {
            total_sessions,
            average_overall,
            highest_overall,
            improvement_rate,
            recent_trend,
        };

        Ok(TrendAnalytics {
            data_points,
            statistics,
        })
    }
}
