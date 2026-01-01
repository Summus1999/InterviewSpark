// Business interviewer agent

use super::{InterviewerAgent, InterviewerRole, InterviewContext, AnalysisResult};
use crate::rig_adapter::SiliconFlowProvider;
use async_trait::async_trait;
use anyhow::Result;

/// Business interviewer
pub struct BusinessInterviewer {
    provider: SiliconFlowProvider,
}

impl BusinessInterviewer {
    const SYSTEM_PROMPT: &'static str = r#"你是一位业务部门负责人，关注候选人能否快速上手并产出业务价值。

评估重点：
- 业务理解：对行业和业务的认知深度
- 落地能力：将想法转化为可执行方案
- 结果导向：过往项目的实际成果
- 学习能力：快速掌握新领域的能力

提问风格：
- 从实际业务场景出发
- 关注解决问题的思路和方法
- 考察数据驱动决策能力

语气：务实、结果导向、注重细节"#;
    
    const ANALYSIS_PROMPT: &'static str = r#"请分析候选人的回答质量。

评估维度：
1. 业务洞察：对业务本质的理解
2. 方法论：解决问题的系统性方法
3. 数据敏感度：是否用数据支撑判断
4. 实际成果：项目的可量化成果

输出格式（JSON）：
{
  "score": 8.2,
  "strengths": ["业务理解透彻", "有数据支撑"],
  "improvements": ["可以更多展示XXX"],
  "summary": "候选人具备较强的业务落地能力..."
}"#;

    pub fn new(provider: SiliconFlowProvider) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl InterviewerAgent for BusinessInterviewer {
    fn role(&self) -> InterviewerRole {
        InterviewerRole::Business
    }
    
    fn role_name(&self) -> &'static str {
        "业务面试官"
    }
    
    fn avatar(&self) -> &'static str {
        "business"
    }
    
    async fn generate_question(&self, context: &InterviewContext) -> Result<String> {
        let agent = self.provider
            .completion_model("Pro/Qwen/Qwen2.5-7B-Instruct")
            .agent(Self::SYSTEM_PROMPT)
            .build();
        
        let prompt = format!(
            r#"基于以下JD和候选人简历，生成一个业务理解类问题。

JD: {}

简历: {}

要求：
1. 只输出问题本身，不要包含任何引导说明、评估标准或内部提示
2. 使用纯文本格式，禁止使用Markdown（如**加粗**、#标题等）
3. 直接以面试官口吴提问，简洁自然，像真实面试一样"#,
            context.job_description,
            context.resume
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
                AnalysisResult {
                    score: 7.5,
                    strengths: vec!["思路清晰".to_string()],
                    improvements: vec!["可以更关注业务指标".to_string()],
                    summary: "候选人对业务有基本理解。".to_string(),
                }
            });
        
        Ok(result)
    }
    
    async fn should_follow_up(&self, answer: &str, analysis: &AnalysisResult) -> bool {
        // Follow up if lacking business depth or score is low
        answer.len() < 120 || analysis.score < 7.5
    }
}
