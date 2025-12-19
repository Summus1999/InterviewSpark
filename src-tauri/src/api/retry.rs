//! Retry strategy for API requests
//!
//! Implements exponential backoff retry policy for handling transient failures

use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

/// Retry policy configuration
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial delay before first retry (milliseconds)
    pub initial_delay_ms: u64,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Maximum delay cap (milliseconds)
    pub max_delay_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
            max_delay_ms: 10000,
        }
    }
}

impl RetryPolicy {
    /// Execute a function with retry logic
    ///
    /// # Arguments
    /// * `operation` - Async function to retry
    ///
    /// # Returns
    /// * `Ok(T)` - Result from successful operation
    /// * `Err` - Error after all retries exhausted
    pub async fn execute<F, Fut, T>(&self, mut operation: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut attempts = 0;
        let mut delay = self.initial_delay_ms;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    attempts += 1;
                    
                    if attempts >= self.max_retries {
                        log::error!("Request failed after {} attempts: {}", attempts, err);
                        return Err(err);
                    }

                    log::warn!("Request failed (attempt {}/{}): {}. Retrying in {}ms...", 
                              attempts, self.max_retries, err, delay);

                    // Wait before retry
                    sleep(Duration::from_millis(delay)).await;

                    // Calculate next delay with exponential backoff
                    delay = ((delay as f64) * self.backoff_multiplier) as u64;
                    delay = delay.min(self.max_delay_ms);
                }
            }
        }
    }

    /// Check if an error is retryable
    pub fn is_retryable(error: &anyhow::Error) -> bool {
        let error_str = error.to_string().to_lowercase();
        
        // Retry on network errors and server errors (5xx)
        error_str.contains("timeout")
            || error_str.contains("connection")
            || error_str.contains("network")
            || error_str.contains("500")
            || error_str.contains("502")
            || error_str.contains("503")
            || error_str.contains("504")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_success_on_second_attempt() {
        let policy = RetryPolicy::default();
        let mut attempt = 0;

        let result = policy
            .execute(|| async {
                attempt += 1;
                if attempt == 1 {
                    anyhow::bail!("First attempt fails")
                } else {
                    Ok("Success")
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
        assert_eq!(attempt, 2);
    }

    #[tokio::test]
    async fn test_retry_exhausted() {
        let policy = RetryPolicy {
            max_retries: 2,
            initial_delay_ms: 10,
            ..Default::default()
        };
        let mut attempt = 0;

        let result = policy
            .execute(|| async {
                attempt += 1;
                anyhow::bail!("Always fails")
            })
            .await;

        assert!(result.is_err());
        assert_eq!(attempt, 2);
    }
}
