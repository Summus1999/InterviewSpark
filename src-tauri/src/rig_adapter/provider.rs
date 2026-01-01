// SiliconFlow Provider adapter for rig framework
// Adapts existing SiliconFlowClient to rig's CompletionClient trait

use anyhow::Result;
use crate::api::siliconflow::{SiliconFlowClient, ChatMessage};

/// SiliconFlow Provider - wraps existing SiliconFlowClient
#[derive(Clone)]
pub struct SiliconFlowProvider {
    inner: SiliconFlowClient,
}

impl SiliconFlowProvider {
    /// Create provider from API key and model
    #[allow(dead_code)]
    pub fn new(api_key: String, model: String) -> Result<Self> {
        Ok(Self {
            inner: SiliconFlowClient::new(api_key, model)?,
        })
    }
    
    /// Create provider from environment variables
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            inner: SiliconFlowClient::from_env()?,
        })
    }
    
    /// Get underlying client for direct access
    #[allow(dead_code)]
    pub fn client(&self) -> &SiliconFlowClient {
        &self.inner
    }
}

/// SiliconFlow Completion Model
#[derive(Clone)]
pub struct SiliconFlowCompletionModel {
    client: SiliconFlowClient,
    model_name: String,
    system_prompt: Option<String>,
}

impl SiliconFlowCompletionModel {
    /// Create a new completion model
    pub fn new(client: SiliconFlowClient, model_name: String) -> Self {
        Self {
            client,
            model_name,
            system_prompt: None,
        }
    }
    
    /// Set system prompt for the model
    pub fn with_system_prompt(mut self, prompt: String) -> Self {
        self.system_prompt = Some(prompt);
        self
    }
    
    /// Generate completion from prompt
    pub async fn prompt(&self, user_prompt: &str) -> Result<String> {
        let mut messages = Vec::new();
        
        // Add system prompt if exists
        if let Some(ref sys_prompt) = self.system_prompt {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: sys_prompt.clone(),
            });
        }
        
        // Add user prompt
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_prompt.to_string(),
        });
        
        // Call SiliconFlow API
        self.client
            .chat_completion_with_model(messages, &self.model_name, Some(0.7), None)
            .await
    }
    
    /// Generate completion from messages
    #[allow(dead_code)]
    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String> {
        let mut all_messages = Vec::new();
        
        // Add system prompt if exists
        if let Some(ref sys_prompt) = self.system_prompt {
            all_messages.push(ChatMessage {
                role: "system".to_string(),
                content: sys_prompt.clone(),
            });
        }
        
        // Add conversation messages
        all_messages.extend(messages);
        
        // Call SiliconFlow API
        self.client
            .chat_completion_with_model(all_messages, &self.model_name, Some(0.7), None)
            .await
    }
}

/// Agent builder pattern for SiliconFlow
pub struct AgentBuilder {
    provider: SiliconFlowProvider,
    model_name: String,
    system_prompt: Option<String>,
}

impl SiliconFlowProvider {
    /// Create a completion model with specified model name
    pub fn completion_model(&self, model: &str) -> AgentBuilder {
        AgentBuilder {
            provider: self.clone(),
            model_name: model.to_string(),
            system_prompt: None,
        }
    }
}

impl AgentBuilder {
    /// Set system prompt for the agent
    pub fn agent(mut self, prompt: &str) -> Self {
        self.system_prompt = Some(prompt.to_string());
        self
    }
    
    /// Build the completion model
    pub fn build(self) -> SiliconFlowCompletionModel {
        let model = SiliconFlowCompletionModel::new(
            self.provider.inner,
            self.model_name,
        );
        
        if let Some(prompt) = self.system_prompt {
            model.with_system_prompt(prompt)
        } else {
            model
        }
    }
}
