//! API module for external service integrations
//!
//! This module contains clients for interacting with AI services:
//! - SiliconFlow: AI-powered question generation and answer analysis
//! - Retry: Exponential backoff retry strategy for transient failures

pub mod siliconflow;
pub mod retry;

pub use siliconflow::SiliconFlowClient;
pub use retry::RetryPolicy;
