//! Cache management module for performance optimization

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::analysis::{DashboardData, TrendAnalytics};

/// Cache entry with TTL support
#[derive(Clone)]
pub struct CacheEntry<T: Clone> {
    pub data: T,
    pub created_at: Instant,
    pub ttl_seconds: u64,
}

impl<T: Clone> CacheEntry<T> {
    pub fn new(data: T, ttl_seconds: u64) -> Self {
        Self {
            data,
            created_at: Instant::now(),
            ttl_seconds,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > Duration::from_secs(self.ttl_seconds)
    }
}

/// Cache manager for frequently accessed data
#[derive(Clone)]
pub struct CacheManager {
    dashboard_cache: Arc<Mutex<Option<CacheEntry<DashboardData>>>>,
    analytics_cache: Arc<Mutex<Option<CacheEntry<TrendAnalytics>>>>,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new() -> Self {
        Self {
            dashboard_cache: Arc::new(Mutex::new(None)),
            analytics_cache: Arc::new(Mutex::new(None)),
        }
    }

    /// Get dashboard data from cache
    pub fn get_dashboard(&self) -> Option<DashboardData> {
        let cache = self.dashboard_cache.lock().unwrap();
        if let Some(entry) = cache.as_ref() {
            if !entry.is_expired() {
                return Some(entry.data.clone());
            }
        }
        None
    }

    /// Set dashboard data in cache (TTL: 60 seconds)
    pub fn set_dashboard(&self, data: DashboardData) {
        let mut cache = self.dashboard_cache.lock().unwrap();
        *cache = Some(CacheEntry::new(data, 60));
    }

    /// Get analytics data from cache
    pub fn get_analytics(&self) -> Option<TrendAnalytics> {
        let cache = self.analytics_cache.lock().unwrap();
        if let Some(entry) = cache.as_ref() {
            if !entry.is_expired() {
                return Some(entry.data.clone());
            }
        }
        None
    }

    /// Set analytics data in cache (TTL: 120 seconds)
    pub fn set_analytics(&self, data: TrendAnalytics) {
        let mut cache = self.analytics_cache.lock().unwrap();
        *cache = Some(CacheEntry::new(data, 120));
    }

    /// Invalidate dashboard cache
    pub fn invalidate_dashboard(&self) {
        let mut cache = self.dashboard_cache.lock().unwrap();
        *cache = None;
    }

    /// Invalidate analytics cache
    pub fn invalidate_analytics(&self) {
        let mut cache = self.analytics_cache.lock().unwrap();
        *cache = None;
    }

    /// Invalidate all caches
    pub fn invalidate_all(&self) {
        self.invalidate_dashboard();
        self.invalidate_analytics();
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}
