use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::collections::HashMap;
use std::sync::OnceLock;
use super::retry::RetryPolicy;
use futures::StreamExt;
use eventsource_stream::Eventsource;

/// SiliconFlow API client configuration
#[derive(Clone)]
pub struct SiliconFlowClient {
    api_key: String,
    base_url: String,
    model: String,
    client: Client,
    retry_policy: RetryPolicy,
}

/// Chat message structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat completion request
#[derive(Debug, Serialize, Clone)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

/// Chat completion response
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

/// Audio transcription response
#[derive(Debug, Deserialize)]
pub struct TranscriptionResponse {
    pub text: String,
}

/// Persona configuration
#[derive(Debug, Clone, Deserialize)]
pub struct PersonaConfig {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub description: String,
    pub prompt: String,
}

/// Global persona configurations cache
static PERSONAS: OnceLock<HashMap<String, PersonaConfig>> = OnceLock::new();

/// Load persona configurations from embedded resource
fn load_personas() -> HashMap<String, PersonaConfig> {
    let personas_json = include_str!("../../resources/personas.json");
    serde_json::from_str(personas_json)
        .unwrap_or_else(|e| {
            log::error!("Failed to load personas.json: {}", e);
            HashMap::new()
        })
}

/// Extract JSON array from response text
/// Supports both pure JSON and text with embedded JSON
fn extract_json_array(text: &str) -> Result<Vec<String>> {
    // First, try to parse the entire response as JSON
    if let Ok(questions) = serde_json::from_str::<Vec<String>>(text) {
        return Ok(questions);
    }

    // Try to find JSON array pattern in the text
    if let Some(start) = text.find('[') {
        if let Some(end) = text.rfind(']') {
            if start < end {
                let json_part = &text[start..=end];
                if let Ok(questions) = serde_json::from_str::<Vec<String>>(json_part) {
                    return Ok(questions);
                }
            }
        }
    }

    // Fallback: split by newlines and filter non-empty lines
    let questions: Vec<String> = text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('[') && !line.starts_with(']'))
        .filter(|line| line.len() > 5) // Filter out very short lines
        .map(|line| {
            // Remove common prefixes like "1.", "- ", etc.
            line.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == '-' || c == ' ')
                .trim()
                .trim_matches('"')
                .to_string()
        })
        .filter(|q| !q.is_empty())
        .collect();

    if questions.is_empty() {
        anyhow::bail!("Failed to extract questions from response")
    }

    Ok(questions)
}

impl SiliconFlowClient {
    /// Get system prompt based on interviewer persona
    pub fn get_persona_prompt(persona: &str) -> String {
        let personas = PERSONAS.get_or_init(load_personas);
        
        personas
            .get(persona)
            .map(|config| config.prompt.clone())
            .unwrap_or_else(|| {
                // Fallback to balanced persona
                personas
                    .get("balanced")
                    .map(|config| config.prompt.clone())
                    .unwrap_or_else(|| "You are an experienced interviewer providing balanced, constructive feedback on interview answers.".to_string())
            })
    }
    /// Create a new SiliconFlow client from environment variables
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("SILICONFLOW_API_KEY")
            .context("SILICONFLOW_API_KEY not found in environment")?;
        
        let base_url = env::var("SILICONFLOW_BASE_URL")
            .unwrap_or_else(|_| "https://api.siliconflow.cn/v1".to_string());
        
        let model = env::var("SILICONFLOW_MODEL")
            .unwrap_or_else(|_| "Pro/zai-org/GLM-4.7".to_string());

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        Ok(Self {
            api_key,
            base_url,
            model,
            client,
            retry_policy: RetryPolicy::default(),
        })
    }

    /// Create a new SiliconFlow client with custom configuration
    pub fn new(api_key: String, model: String) -> Result<Self> {
        let base_url = "https://api.siliconflow.cn/v1".to_string();
        
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        Ok(Self {
            api_key,
            base_url,
            model,
            client,
            retry_policy: RetryPolicy::default(),
        })
    }

    /// Update client configuration
    #[allow(dead_code)]
    pub fn update_config(&mut self, api_key: String, model: String) {
        self.api_key = api_key;
        self.model = model;
    }

    /// Call chat completion API with retry logic
    pub async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        self.chat_completion_with_model(messages, &self.model, temperature, max_tokens).await
    }

    /// Call chat completion API with model override
    pub async fn chat_completion_with_model(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        let url = format!("{}/chat/completions", self.base_url);
        
        let request = ChatCompletionRequest {
            model: model.to_string(),
            messages: messages.clone(),
            temperature: temperature.unwrap_or(0.7),
            max_tokens,
            stream: None,
        };

        // Execute with retry
        self.retry_policy.execute(|| async {
            let response = self
                .client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await
                .context("Failed to send request to SiliconFlow API")?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                anyhow::bail!("API request failed with status {}: {}", status, error_text);
            }

            let completion: ChatCompletionResponse = response
                .json()
                .await
                .context("Failed to parse API response")?;

            completion
                .choices
                .first()
                .map(|c| c.message.content.clone())
                .context("No choices in API response")
        }).await
    }

    /// Call chat completion API with streaming support
    /// Returns a stream of content chunks
    pub async fn chat_completion_stream(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<impl futures::Stream<Item = Result<String>>> {
        let url = format!("{}/chat/completions", self.base_url);
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages,
            temperature: temperature.unwrap_or(0.7),
            max_tokens,
            stream: Some(true),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send streaming request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Streaming request failed with status {}: {}", status, error_text);
        }

        // Create event source stream
        let stream = response
            .bytes_stream()
            .eventsource()
            .map(|event| {
                match event {
                    Ok(event) => {
                        // Parse SSE data
                        let data = event.data;
                        
                        // Check for completion marker
                        if data.trim() == "[DONE]" {
                            return Ok(String::new());
                        }
                        
                        // Parse JSON response
                        match serde_json::from_str::<serde_json::Value>(&data) {
                            Ok(json) => {
                                // Extract content from delta
                                if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
                                    if let Some(first_choice) = choices.first() {
                                        if let Some(delta) = first_choice.get("delta") {
                                            if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                                                return Ok(content.to_string());
                                            }
                                        }
                                    }
                                }
                                Ok(String::new())
                            }
                            Err(e) => {
                                log::warn!("Failed to parse SSE data: {}", e);
                                Ok(String::new())
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Stream error: {}", e);
                        Err(anyhow::anyhow!("Stream error: {}", e))
                    }
                }
            });

        Ok(stream)
    }

    /// Generate interview questions based on resume and job description
    pub async fn generate_questions(
        &self,
        resume: &str,
        job_description: &str,
        count: u32,
        persona: &str,
    ) -> Result<Vec<String>> {
        self.generate_questions_with_context(resume, job_description, count, persona, None).await
    }

    /// Generate questions with optional RAG context
    pub async fn generate_questions_with_context(
        &self,
        resume: &str,
        job_description: &str,
        count: u32,
        persona: &str,
        context: Option<&str>,
    ) -> Result<Vec<String>> {
        let base_prompt = Self::get_persona_prompt(persona);
        let system_prompt = format!("{} 你必须只返回有效的JSON数组，不要任何额外的文字或解释。", base_prompt);
        
        let context_section = if let Some(ctx) = context {
            format!("\n\n知识库参考问题：\n{}\n\n你可以参考以上示例，但需要根据提供的简历和岗位描述生成新的相关问题。", ctx)
        } else {
            String::new()
        };
        
        let user_prompt = format!(
            "根据以下简历和岗位描述，生成 {} 个相关的面试问题。所有问题必须使用中文。{}\n\n简历：\n{}\n\n岗位描述：\n{}\n\n重要提示：只返回JSON数组，不要任何解释说明。格式：[\"问题1\", \"问题2\", ...]",
            count, context_section, resume, job_description
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        let response = self.chat_completion(messages, Some(0.8), Some(2000)).await?;
        
        // Debug: print raw API response
        eprintln!("[DEBUG] API Response: {}", response);
        
        // Extract questions using intelligent parser
        let questions = extract_json_array(&response)
            .with_context(|| format!("Failed to parse questions. Raw response (first 200 chars): {}", 
                response.chars().take(200).collect::<String>()))?;

        if questions.len() != count as usize {
            eprintln!("[WARNING] Expected {} questions, got {}", count, questions.len());
        }

        Ok(questions)
    }

    /// Analyze user's answer and provide feedback
    pub async fn analyze_answer(
        &self,
        question: &str,
        answer: &str,
        job_description: &str,
        persona: &str,
    ) -> Result<String> {
        let system_prompt = Self::get_persona_prompt(persona);
        
        let user_prompt = format!(
            "Question: {}\n\nCandidate's Answer: {}\n\nJob Description: {}\n\nPlease analyze this answer and provide:\n1. Strengths\n2. Areas for improvement\n3. Suggestions for better response\n4. Relevance to job requirements",
            question, answer, job_description
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        self.chat_completion(messages, Some(0.7), Some(1500)).await
    }

    /// Generate comprehensive interview report
    #[allow(dead_code)]
    pub async fn generate_session_report(
        &self,
        questions: &[String],
        answers: &[String],
        job_description: &str,
    ) -> Result<String> {
        self.generate_session_report_with_model(questions, answers, job_description, None).await
    }

    /// Generate comprehensive interview report with optional model override
    pub async fn generate_session_report_with_model(
        &self,
        questions: &[String],
        answers: &[String],
        job_description: &str,
        model: Option<&str>,
    ) -> Result<String> {
        let model_to_use = model.unwrap_or(&self.model);
        let system_prompt = "你是一位资深面试评估专家。请用中文生成一份全面的面试复盘报告，输出JSON格式：{\"summary\": \"总结...\", \"overall_score\": 8.5, \"improvements\": [...], \"key_takeaways\": [...]}";
        
        let qa_pairs = questions
            .iter()
            .zip(answers.iter())
            .enumerate()
            .map(|(idx, (q, a))| format!("问题{}: {}\n回答{}: {}", idx + 1, q, idx + 1, a))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        let user_prompt = format!(
            "岗位描述：\n{}\n\n面试问答：\n{}\n\n请用中文生成一份全面的面试复盘报告，包含：\n1. 整体表现总结（150-200字）\n2. 综合评分（1-10分）\n3. 3-5条具体改进建议\n4. 2-3条核心要点\n\n请只返回有效的JSON格式，不要其他任何文字。",
            job_description, qa_pairs
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        self.chat_completion_with_model(messages, model_to_use, Some(0.7), Some(2500)).await
    }

    /// Analyze answer and determine if follow-up is needed
    pub async fn analyze_for_followup(
        &self,
        original_question: &str,
        answer: &str,
        conversation_history: &str,
        job_description: &str,
        max_followups: u32,
        preferred_types: &[String],
        persona: &str,
    ) -> Result<String> {
        let base_prompt = Self::get_persona_prompt(persona);
        let system_prompt = format!("{} You are analyzing candidate answers to determine if follow-up questions are needed. You MUST respond with ONLY valid JSON, no additional text.", base_prompt);
        
        let user_prompt = format!(
            r#"Original Question: {}

Candidate's Answer: {}

Conversation History:
{}

Job Description: {}

Max Follow-ups: {}
Preferred Types: {:?}

Analyze this answer and respond with ONLY a JSON object in this exact format:
{{
  "shouldFollowUp": true/false,
  "answerQuality": "excellent"|"good"|"acceptable"|"poor",
  "reasoning": "brief explanation",
  "followUpQuestions": [
    {{
      "question": "the follow-up question",
      "type": "clarification"|"deepening"|"scenario"|"challenge"|"extension",
      "reason": "why ask this",
      "context": "brief context"
    }}
  ]
}}

IMPORTANT: Return ONLY the JSON object, no markdown, no explanations.
只返回JSON对象，不要任何其他文字。"#,
            original_question,
            answer,
            conversation_history,
            job_description,
            max_followups,
            preferred_types
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        self.chat_completion(messages, Some(0.7), Some(2000)).await
    }

    /// Generate best answer for a question
    /// If historical_answers is empty, generates initial version based on question + JD
    /// If historical_answers has content, synthesizes from user's best attempts
    pub async fn generate_best_answer(
        &self,
        question: &str,
        job_description: &str,
        historical_answers: &[(String, f32)],  // (answer, score)
    ) -> Result<String> {
        let system_prompt = "你是一位资深面试辅导专家。请生成一份针对该面试问题的优秀答案，语言简洁专业，结构清晰，突出关键要点。";
        
        let user_prompt = if historical_answers.is_empty() {
            // First-time generation: based on question + JD only
            format!(
                "面试问题：{}\n\n岗位描述：{}\n\n请生成一份高质量的答案示例，包含：\n1. 核心要点\n2. 具体举例或经验\n3. 与岗位的关联\n\n直接输出答案内容，不需要额外格式或标题。",
                question, job_description
            )
        } else {
            // Iterative generation: synthesize from user's historical answers
            let answers_summary: String = historical_answers
                .iter()
                .enumerate()
                .map(|(i, (ans, score))| format!("\n第{}次回答(评分:{:.1}):\n{}", i + 1, score, ans))
                .collect();
            
            format!(
                "面试问题：{}\n\n岗位描述：{}\n\n用户历史回答：{}\n\n请基于用户的历史回答，提取其中的亮点和有效信息，综合生成一份更完善的优秀答案。\n要求：\n1. 保留用户回答中的有效经验和案例\n2. 优化表达结构和逻辑\n3. 补充缺失的关键要点\n\n直接输出答案内容，不需要额外格式或标题。",
                question, job_description, answers_summary
            )
        };

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        self.chat_completion(messages, Some(0.7), Some(2000)).await
    }

    /// Transcribe audio to text using SiliconFlow audio transcription API
    /// With 15 seconds timeout protection
    pub async fn transcribe_audio(
        &self,
        audio_data: &[u8],
        filename: &str,
    ) -> Result<String> {
        use tokio::time::{timeout, Duration};
        
        let url = format!("{}/audio/transcriptions", self.base_url);
        
        // Create multipart form with audio file
        let part = reqwest::multipart::Part::bytes(audio_data.to_vec())
            .file_name(filename.to_string())
            .mime_str("audio/webm")
            .context("Failed to create multipart file part")?;
        
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("model", "FunAudioLLM/SenseVoiceSmall");
        
        // Wrap request with 15 seconds timeout
        let request_future = async {
            let response = self
                .client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .multipart(form)
                .send()
                .await
                .context("Failed to send transcription request")?;
            
            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                anyhow::bail!("Transcription API failed with status {}: {}", status, error_text);
            }
            
            let result: TranscriptionResponse = response
                .json()
                .await
                .context("Failed to parse transcription response")?;
            
            Ok(result.text)
        };
        
        timeout(Duration::from_secs(15), request_future)
            .await
            .context("Transcription request timeout after 15 seconds")?
    }
}

// ===== Unit Tests =====

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // Available models for testing
    const MODEL_GLM4_7: &str = "Pro/zai-org/GLM-4.7";
    const MODEL_QWEN3_235B: &str = "Qwen/Qwen3-235B-A22B";
    const MODEL_QWEN3_VL_THINKING: &str = "Qwen/Qwen3-VL-235B-A22B-Thinking";

    // Simple test message
    fn create_test_messages() -> Vec<ChatMessage> {
        vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Say 'test passed' in exactly two words.".to_string(),
            },
        ]
    }

    // Helper to get API key from environment
    fn get_api_key() -> Option<String> {
        env::var("SILICONFLOW_API_KEY").ok()
    }

    #[tokio::test]
    async fn test_glm4_7_api_call() {
        let api_key = match get_api_key() {
            Some(key) => key,
            None => {
                println!("Skipping test: SILICONFLOW_API_KEY not set");
                return;
            }
        };

        let client = SiliconFlowClient::new(api_key, MODEL_GLM4_7.to_string())
            .expect("Failed to create client");

        let messages = create_test_messages();
        let result = client
            .chat_completion_with_model(messages, MODEL_GLM4_7, Some(0.5), Some(50))
            .await;

        assert!(result.is_ok(), "GLM-4.7 API call failed: {:?}", result.err());
        let response = result.unwrap();
        assert!(!response.is_empty(), "Response should not be empty");
        println!("GLM-4.7 response: {}", response);
    }

    #[tokio::test]
    async fn test_qwen3_235b_api_call() {
        let api_key = match get_api_key() {
            Some(key) => key,
            None => {
                println!("Skipping test: SILICONFLOW_API_KEY not set");
                return;
            }
        };

        let client = SiliconFlowClient::new(api_key, MODEL_QWEN3_235B.to_string())
            .expect("Failed to create client");

        let messages = create_test_messages();
        let result = client
            .chat_completion_with_model(messages, MODEL_QWEN3_235B, Some(0.5), Some(50))
            .await;

        assert!(result.is_ok(), "Qwen3-235B API call failed: {:?}", result.err());
        let response = result.unwrap();
        assert!(!response.is_empty(), "Response should not be empty");
        println!("Qwen3-235B response: {}", response);
    }

    #[tokio::test]
    async fn test_qwen3_vl_thinking_api_call() {
        let api_key = match get_api_key() {
            Some(key) => key,
            None => {
                println!("Skipping test: SILICONFLOW_API_KEY not set");
                return;
            }
        };

        let client = SiliconFlowClient::new(api_key, MODEL_QWEN3_VL_THINKING.to_string())
            .expect("Failed to create client");

        let messages = create_test_messages();
        let result = client
            .chat_completion_with_model(messages, MODEL_QWEN3_VL_THINKING, Some(0.5), Some(50))
            .await;

        assert!(result.is_ok(), "Qwen3-VL-Thinking API call failed: {:?}", result.err());
        let response = result.unwrap();
        assert!(!response.is_empty(), "Response should not be empty");
        println!("Qwen3-VL-Thinking (flagship) response: {}", response);
    }

    #[tokio::test]
    async fn test_all_models_available() {
        let api_key = match get_api_key() {
            Some(key) => key,
            None => {
                println!("Skipping test: SILICONFLOW_API_KEY not set");
                return;
            }
        };

        let models = vec![MODEL_GLM4_7, MODEL_QWEN3_235B, MODEL_QWEN3_VL_THINKING];
        let client = SiliconFlowClient::new(api_key, MODEL_GLM4_7.to_string())
            .expect("Failed to create client");

        let mut all_passed = true;
        for model in &models {
            let messages = create_test_messages();
            let result = client
                .chat_completion_with_model(messages, model, Some(0.5), Some(30))
                .await;

            match result {
                Ok(response) => {
                    println!("[PASS] {}: {}", model, &response[..response.len().min(50)]);
                }
                Err(e) => {
                    println!("[FAIL] {}: {:?}", model, e);
                    all_passed = false;
                }
            }
        }

        assert!(all_passed, "Not all models passed the API call test");
    }
}
