//! API module for external service integrations
//!
//! This module contains clients for interacting with AI services:
//! - SiliconFlow: AI-powered question generation and answer analysis

pub mod siliconflow;

pub use siliconflow::SiliconFlowClient;
