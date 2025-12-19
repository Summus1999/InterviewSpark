//! Industry comparison generator
//!
//! Compares user performance against simulated industry benchmarks

use crate::db::{Repository, IndustryBenchmark, IndustryComparisonResult, now};
use crate::analysis::ProfileGenerator;
use anyhow::Result;

/// Generator for industry comparison data
pub struct IndustryComparisonGenerator;

impl IndustryComparisonGenerator {
    /// Generate industry comparison for user
    ///
    /// # Arguments
    /// * `repository` - Database repository
    /// * `user_id` - User identifier
    ///
    /// # Returns
    /// * `Ok(IndustryComparisonResult)` - Comparison with industry benchmarks
    /// * `Err` - Error if generation fails
    pub fn generate_comparison(
        repository: &Repository,
        user_id: &str,
    ) -> Result<IndustryComparisonResult> {
        // Get user profile
        let profile = ProfileGenerator::generate_profile(repository, user_id, None)?;
        
        if profile.total_sessions == 0 {
            return Self::create_empty_comparison();
        }
        
        // Simulated industry benchmarks (in a real app, these would come from aggregated data)
        let industry_data = vec![
            ("technical_depth", 55.0, 85.0),      // (dimension, avg, top)
            ("communication", 60.0, 90.0),
            ("problem_solving", 52.0, 88.0),
            ("domain_knowledge", 58.0, 82.0),
            ("adaptability", 50.0, 80.0),
        ];
        
        let mut benchmarks = Vec::new();
        let mut total_percentile = 0.0;
        
        for (dimension, industry_avg, industry_top) in industry_data {
            let user_score = match dimension {
                "technical_depth" => profile.dimensions.technical_depth,
                "communication" => profile.dimensions.communication,
                "problem_solving" => profile.dimensions.problem_solving,
                "domain_knowledge" => profile.dimensions.domain_knowledge,
                "adaptability" => profile.dimensions.adaptability,
                _ => 0.0,
            };
            
            // Calculate percentile (simplified calculation)
            let percentile = Self::calculate_percentile(user_score, industry_avg, industry_top);
            total_percentile += percentile;
            
            benchmarks.push(IndustryBenchmark {
                dimension: dimension.to_string(),
                user_score,
                industry_avg,
                industry_top,
                percentile,
            });
        }
        
        let overall_percentile = total_percentile / benchmarks.len() as f32;
        let user_level = Self::determine_level(overall_percentile);
        
        Ok(IndustryComparisonResult {
            benchmarks,
            overall_percentile,
            user_level,
            comparison_count: profile.total_sessions,
            generated_at: now(),
        })
    }
    
    /// Create empty comparison for new users
    fn create_empty_comparison() -> Result<IndustryComparisonResult> {
        Ok(IndustryComparisonResult {
            benchmarks: vec![],
            overall_percentile: 0.0,
            user_level: "新手".to_string(),
            comparison_count: 0,
            generated_at: now(),
        })
    }
    
    /// Calculate percentile rank based on user score and industry data
    fn calculate_percentile(user_score: f32, avg: f32, top: f32) -> f32 {
        if user_score >= top {
            return 95.0 + (user_score - top) / (100.0 - top) * 5.0;
        }
        
        if user_score >= avg {
            // Between average and top: 50-95 percentile
            return 50.0 + (user_score - avg) / (top - avg) * 45.0;
        }
        
        // Below average: 0-50 percentile
        (user_score / avg * 50.0).max(0.0)
    }
    
    /// Determine user level based on overall percentile
    fn determine_level(percentile: f32) -> String {
        match percentile as i32 {
            0..=25 => "入门级".to_string(),
            26..=50 => "初级".to_string(),
            51..=75 => "中级".to_string(),
            76..=90 => "高级".to_string(),
            _ => "专家级".to_string(),
        }
    }
}
