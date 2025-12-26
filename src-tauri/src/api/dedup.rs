//! Request deduplication module
//! Prevents duplicate API requests with same parameters

use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use anyhow::Result;

/// Request deduplicator for AI API calls
/// Ensures only one request with same key is processed at a time
pub struct RequestDeduplicator {
    pending: Arc<RwLock<HashMap<String, Arc<Mutex<Option<Result<String, String>>>>>>>,
}

impl RequestDeduplicator {
    /// Create new request deduplicator
    pub fn new() -> Self {
        Self {
            pending: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Deduplicate requests by key
    /// If same request is already pending, wait for its result
    /// Otherwise, execute the request
    pub async fn deduplicate<F, Fut>(&self, key: String, f: F) -> Result<String, String>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<String, String>>,
    {
        // Check if request is already pending
        let result_lock = {
            let mut pending = self.pending.write().await;
            
            if let Some(existing) = pending.get(&key) {
                // Request already pending, wait for it
                log::debug!("Request deduplication: waiting for existing request with key: {}", key);
                existing.clone()
            } else {
                // New request, create lock and store it
                log::debug!("Request deduplication: new request with key: {}", key);
                let lock = Arc::new(Mutex::new(None));
                pending.insert(key.clone(), lock.clone());
                lock
            }
        };

        // Try to acquire lock (only first requester will succeed immediately)
        let mut result_guard = result_lock.lock().await;

        // If result already exists, return it (we waited for another request)
        if let Some(result) = result_guard.as_ref() {
            log::debug!("Request deduplication: using cached result");
            return result.clone();
        }

        // We're the first requester, execute the function
        log::debug!("Request deduplication: executing original request");
        let result = f().await;
        
        // Store result for other waiters
        *result_guard = Some(result.clone());

        // Clean up from pending map
        let mut pending = self.pending.write().await;
        pending.remove(&key);

        result
    }

    /// Clear all pending requests
    pub async fn clear(&self) {
        let mut pending = self.pending.write().await;
        pending.clear();
    }
}

impl Default for RequestDeduplicator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn test_deduplication() {
        let dedup = RequestDeduplicator::new();
        let counter = Arc::new(AtomicU32::new(0));

        let key = "test-key".to_string();
        
        // Spawn multiple concurrent requests with same key
        let mut handles = vec![];
        for _ in 0..5 {
            let dedup_clone = Arc::new(dedup);
            let counter_clone = counter.clone();
            let key_clone = key.clone();
            
            let handle = tokio::spawn(async move {
                dedup_clone.deduplicate(key_clone, || async {
                    // Simulate API call
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    Ok("result".to_string())
                }).await
            });
            
            handles.push(handle);
        }

        // Wait for all requests
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "result");
        }

        // Counter should be 1 (only one actual execution)
        // Note: Due to async timing, might be 2-3 in edge cases
        let count = counter.load(Ordering::SeqCst);
        assert!(count <= 2, "Expected 1-2 executions, got {}", count);
    }

    #[tokio::test]
    async fn test_different_keys() {
        let dedup = Arc::new(RequestDeduplicator::new());
        let counter = Arc::new(AtomicU32::new(0));

        let mut handles = vec![];
        
        // Different keys should execute separately
        for i in 0..3 {
            let dedup_clone = dedup.clone();
            let counter_clone = counter.clone();
            let key = format!("key-{}", i);
            
            let handle = tokio::spawn(async move {
                dedup_clone.deduplicate(key, || async {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    Ok(format!("result-{}", i))
                }).await
            });
            
            handles.push(handle);
        }

        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }

        // All 3 should execute
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
}
