use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
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
    /// Create a new SiliconFlow client from environment variables
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("SILICONFLOW_API_KEY")
            .context("SILICONFLOW_API_KEY not found in environment")?;
        
        let base_url = env::var("SILICONFLOW_BASE_URL")
            .unwrap_or_else(|_| "https://api.siliconflow.cn/v1".to_string());
        
        let model = env::var("SILICONFLOW_MODEL")
            .unwrap_or_else(|_| "Qwen/Qwen3-8B".to_string());

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        Ok(Self {
            api_key,
            base_url,
            model,
            client,
            retry_policy: RetryPolicy::default(),
        })
    }

    /// Call chat completion API with retry logic
    pub async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        let url = format!("{}/chat/completions", self.base_url);
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
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
    ) -> Result<Vec<String>> {
        let system_prompt = "You are an experienced interviewer. You MUST respond with ONLY a valid JSON array, no additional text or explanations.";
        
        let user_prompt = format!(
            "Based on the following resume and job description, generate exactly {} relevant interview questions.\n\nResume:\n{}\n\nJob Description:\n{}\n\nIMPORTANT: Return ONLY a JSON array of strings. No explanations, no markdown, just the array. Format: [\"question1\", \"question2\", ...]\n\n重要提示：只返回JSON数组，不要任何解释说明。",
            count, resume, job_description
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
    ) -> Result<String> {
        let system_prompt = "You are an experienced interviewer providing constructive feedback on interview answers.";
        
        let user_prompt = format!(
            "Question: {}\n\nCandidate's Answer: {}\n\nJob Description: {}\n\nPlease analyze this answer and provide:\n1. Strengths\n2. Areas for improvement\n3. Suggestions for better response\n4. Relevance to job requirements",
            question, answer, job_description
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

        self.chat_completion(messages, Some(0.7), Some(1500)).await
    }

    /// Generate comprehensive interview report
    pub async fn generate_session_report(
        &self,
        questions: &[String],
        answers: &[String],
        job_description: &str,
    ) -> Result<String> {
        let system_prompt = "You are an expert interview evaluator. Generate a comprehensive interview report in JSON format with the following structure: {\"summary\": \"...\", \"overall_score\": 8.5, \"improvements\": [...], \"key_takeaways\": [...]}";
        
        let qa_pairs = questions
            .iter()
            .zip(answers.iter())
            .enumerate()
            .map(|(idx, (q, a))| format!("Q{}: {}\nA{}: {}", idx + 1, q, idx + 1, a))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        let user_prompt = format!(
            "Job Description:\n{}\n\nInterview Q&A:\n{}\n\nPlease generate a comprehensive report with:\n1. Overall performance summary (150-200 words)\n2. Overall score (1-10 scale)\n3. 3-5 specific improvement suggestions\n4. 2-3 key takeaways\n\nRespond ONLY with valid JSON, no other text.",
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

        self.chat_completion(messages, Some(0.7), Some(2500)).await
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
    ) -> Result<String> {
        let system_prompt = "You are an experienced interviewer analyzing candidate answers to determine if follow-up questions are needed. You MUST respond with ONLY valid JSON, no additional text.";
        
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
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        self.chat_completion(messages, Some(0.7), Some(2000)).await
    }
}
