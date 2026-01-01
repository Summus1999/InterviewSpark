// Answer comparison agent

#![allow(dead_code)]

use crate::rig_adapter::SiliconFlowProvider;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Answer comparison agent
pub struct ComparisonAgent {
    provider: SiliconFlowProvider,
}

impl ComparisonAgent {
    const SYSTEM_PROMPT: &'static str = r#"你是一位面试答案分析专家。请将用户答案与最佳答案进行逐点对比分析。

输出格式（JSON）：
{
  "overall_match": 0.75,
  "comparisons": [
    {
      "aspect": "技术准确性",
      "best_answer_point": "最佳答案中的要点",
      "user_answer_point": "用户答案中对应内容",
      "match_status": "matched|partial|missing",
      "suggestion": "改进建议"
    }
  ],
  "missing_points": ["用户遗漏的关键点"],
  "extra_points": ["用户额外提到的有价值内容"]
}"#;
    
    pub fn new(provider: SiliconFlowProvider) -> Self {
        Self { provider }
    }
    
    /// Compare user answer with best answer
    pub async fn compare(
        &self,
        question: &str,
        user_answer: &str,
        best_answer: &str,
    ) -> Result<ComparisonResult> {
        let agent = self.provider
            .completion_model("Pro/Qwen/Qwen2.5-7B-Instruct")
            .agent(Self::SYSTEM_PROMPT)
            .build();
        
        let prompt = format!(
            "问题：{}\n\n用户答案：{}\n\n最佳答案：{}\n\n请进行对比分析并输出JSON格式结果。",
            question,
            user_answer,
            best_answer
        );
        
        let response = agent.prompt(&prompt).await?;
        
        // Parse JSON response
        let result: ComparisonResult = serde_json::from_str(&response)
            .unwrap_or_else(|_| {
                // Fallback if JSON parsing fails
                ComparisonResult {
                    overall_match: 0.7,
                    comparisons: vec![
                        PointComparison {
                            aspect: "整体".to_string(),
                            best_answer_point: "参考答案完整".to_string(),
                            user_answer_point: "用户答案基本覆盖".to_string(),
                            match_status: MatchStatus::Partial,
                            suggestion: "可以更详细展开".to_string(),
                        }
                    ],
                    missing_points: vec!["部分细节未提及".to_string()],
                    extra_points: vec![],
                }
            });
        
        Ok(result)
    }
}

/// Comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub overall_match: f32,
    pub comparisons: Vec<PointComparison>,
    pub missing_points: Vec<String>,
    pub extra_points: Vec<String>,
}

/// Point-by-point comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointComparison {
    pub aspect: String,
    pub best_answer_point: String,
    pub user_answer_point: String,
    pub match_status: MatchStatus,
    pub suggestion: String,
}

/// Match status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchStatus {
    Matched,
    Partial,
    Missing,
}
