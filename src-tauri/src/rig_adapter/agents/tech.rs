// Technical interviewer agent

use super::{InterviewerAgent, InterviewerRole, InterviewContext, AnalysisResult};
use crate::rig_adapter::{SiliconFlowProvider, VectorStoreAdapter};
use async_trait::async_trait;
use anyhow::Result;

/// Technical interviewer
pub struct TechInterviewer {
    provider: SiliconFlowProvider,
    vector_store: VectorStoreAdapter,
}

impl TechInterviewer {
    const SYSTEM_PROMPT: &'static str = r#"你是一位资深技术面试官，拥有10年以上技术管理经验。

评估重点：
- 技术深度：对核心技术原理的理解程度
- 问题解决：分析问题和设计解决方案的能力
- 系统设计：架构思维和技术选型判断力
- 代码质量：编码规范和最佳实践意识

提问风格：
- 从基础概念切入，逐步深入到底层原理
- 追问实现细节和边界情况
- 结合实际场景考察应用能力

语气：专业、严谨、有深度"#;
    
    const ANALYSIS_PROMPT: &'static str = r#"请分析候选人的回答质量。

评估维度：
1. 技术准确性：回答是否准确无误
2. 深度广度：对问题的理解深度和覆盖广度
3. 表达逻辑：回答是否条理清晰、有逻辑
4. 实践经验：是否有实际项目经验支撑

输出格式（JSON）：
{
  "score": 8.5,
  "strengths": ["技术理解深入", "有实践经验"],
  "improvements": ["可以更详细说明XXX"],
  "summary": "候选人对该技术有扎实理解..."
}"#;

    pub fn new(provider: SiliconFlowProvider, vector_store: VectorStoreAdapter) -> Self {
        Self {
            provider,
            vector_store,
        }
    }
}

#[async_trait]
impl InterviewerAgent for TechInterviewer {
    fn role(&self) -> InterviewerRole {
        InterviewerRole::Technical
    }
    
    fn role_name(&self) -> &'static str {
        "技术面试官"
    }
    
    fn avatar(&self) -> &'static str {
        "tech"
    }
    
    async fn generate_question(&self, context: &InterviewContext) -> Result<String> {
        // Retrieve related technical questions from RAG
        let rag_context = self.vector_store
            .search_by_type(&context.job_description, 3, "question")
            .await
            .unwrap_or_default();
        
        let rag_questions: Vec<String> = rag_context
            .iter()
            .map(|(_, _, content)| content.clone())
            .collect();
        
        // Build prompt
        let agent = self.provider
            .completion_model("Pro/Qwen/Qwen2.5-7B-Instruct")
            .agent(Self::SYSTEM_PROMPT)
            .build();
        
        let prompt = format!(
            r#"基于以下JD和候选人简历，生成一个技术面试问题。

JD: {}

简历: {}

参考题库: {:?}

要求：
1. 只输出问题本身，不要包含任何引导说明、评估标准或内部提示
2. 使用纯文本格式，禁止使用Markdown（如**加粗**、#标题等）
3. 直接以面试官口吴提问，简洁自然"#,
            context.job_description,
            context.resume,
            rag_questions
        );
        
        agent.prompt(&prompt).await
    }
    
    async fn analyze_answer(
        &self,
        question: &str,
        answer: &str,
        _context: &InterviewContext,
    ) -> Result<AnalysisResult> {
        let agent = self.provider
            .completion_model("Pro/Qwen/Qwen2.5-7B-Instruct")
            .agent(Self::ANALYSIS_PROMPT)
            .build();
        
        let prompt = format!(
            "问题：{}\n\n候选人回答：{}\n\n请分析回答质量并输出JSON格式结果。",
            question,
            answer
        );
        
        let response = agent.prompt(&prompt).await?;
        
        // Parse JSON response
        let result: AnalysisResult = serde_json::from_str(&response)
            .unwrap_or_else(|_| {
                // Fallback if JSON parsing fails
                AnalysisResult {
                    score: 7.0,
                    strengths: vec!["回答完整".to_string()],
                    improvements: vec!["可以更详细展开".to_string()],
                    summary: "回答基本到位，有改进空间。".to_string(),
                }
            });
        
        Ok(result)
    }
    
    async fn should_follow_up(&self, answer: &str, analysis: &AnalysisResult) -> bool {
        // Follow up if answer is too short or score is low
        answer.len() < 100 || analysis.score < 7.0
    }
}
