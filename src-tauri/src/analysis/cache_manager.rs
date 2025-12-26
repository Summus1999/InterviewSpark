//! Generic cache manager for high-frequency queries
//! Provides type-safe caching with TTL support and hit rate monitoring

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// Cache entry with value and expiration time
#[derive(Clone)]
struct CacheEntry<V: Clone> {
    value: V,
    expires_at: SystemTime,
}

impl<V: Clone> CacheEntry<V> {
    fn new(value: V, ttl: Duration) -> Self {
        Self {
            value,
            expires_at: SystemTime::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
}

/// Generic cache with TTL support
pub struct GenericCache<K: Hash + Eq, V: Clone> {
    cache: Arc<Mutex<HashMap<K, CacheEntry<V>>>>,
    default_ttl: Duration,
    hits: Arc<Mutex<u64>>,
    misses: Arc<Mutex<u64>>,
}

impl<K: Hash + Eq, V: Clone> GenericCache<K, V> {
    /// Create new cache with default TTL
    pub fn new(default_ttl_secs: u64) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            default_ttl: Duration::from_secs(default_ttl_secs),
            hits: Arc::new(Mutex::new(0)),
            misses: Arc::new(Mutex::new(0)),
        }
    }

    /// Get value from cache
    /// Returns None if not found or expired
    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock().unwrap();
        
        if let Some(entry) = cache.get(key) {
            if entry.is_expired() {
                cache.remove(key);
                *self.misses.lock().unwrap() += 1;
                None
            } else {
                *self.hits.lock().unwrap() += 1;
                Some(entry.value.clone())
            }
        } else {
            *self.misses.lock().unwrap() += 1;
            None
        }
    }

    /// Set value with custom TTL
    pub fn set(&self, key: K, value: V, ttl: Option<Duration>) {
        let mut cache = self.cache.lock().unwrap();
        let ttl = ttl.unwrap_or(self.default_ttl);
        cache.insert(key, CacheEntry::new(value, ttl));
    }

    /// Invalidate specific key
    pub fn invalidate(&self, key: &K) {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(key);
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get cache hit rate (0.0 to 1.0)
    pub fn hit_rate(&self) -> f64 {
        let hits = *self.hits.lock().unwrap();
        let misses = *self.misses.lock().unwrap();
        let total = hits + misses;
        
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }

    /// Clean expired entries
    pub fn cleanup(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.retain(|_, entry| !entry.is_expired());
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let cache = self.cache.lock().unwrap();
        let hits = *self.hits.lock().unwrap();
        let misses = *self.misses.lock().unwrap();
        
        CacheStats {
            entries: cache.len(),
            hits,
            misses,
            hit_rate: self.hit_rate(),
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_cache_set_get() {
        let cache = GenericCache::<String, i32>::new(60);
        cache.set("key1".to_string(), 42, None);
        
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert_eq!(cache.get(&"key2".to_string()), None);
    }

    #[test]
    fn test_cache_expiration() {
        let cache = GenericCache::<String, i32>::new(1);
        cache.set("key1".to_string(), 42, Some(Duration::from_millis(100)));
        
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        
        sleep(Duration::from_millis(150));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_invalidate() {
        let cache = GenericCache::<String, i32>::new(60);
        cache.set("key1".to_string(), 42, None);
        
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        
        cache.invalidate(&"key1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_hit_rate() {
        let cache = GenericCache::<String, i32>::new(60);
        cache.set("key1".to_string(), 42, None);
        
        // 2 hits
        cache.get(&"key1".to_string());
        cache.get(&"key1".to_string());
        
        // 1 miss
        cache.get(&"key2".to_string());
        
        assert_eq!(cache.hit_rate(), 2.0 / 3.0);
    }
}
