// HR interviewer agent

use super::{InterviewerAgent, InterviewerRole, InterviewContext, AnalysisResult};
use crate::rig_adapter::SiliconFlowProvider;
use async_trait::async_trait;
use anyhow::Result;

/// HR interviewer
pub struct HRInterviewer {
    provider: SiliconFlowProvider,
}

impl HRInterviewer {
    const SYSTEM_PROMPT: &'static str = r#"你是一位经验丰富的HR面试官，专注于评估候选人的软技能和文化匹配度。

评估重点：
- 沟通能力：表达清晰度、逻辑性
- 团队协作：过往协作经验和冲突处理
- 职业规划：发展目标与岗位匹配度
- 价值观：工作态度和职业素养

提问风格：
- 使用行为面试法（STAR）
- 关注过往经历中的具体案例
- 挖掘候选人的真实想法

语气：亲和、专业、善于引导"#;
    
    const ANALYSIS_PROMPT: &'static str = r#"请分析候选人的回答质量。

评估维度：
1. STAR结构：是否包含情境、任务、行动、结果
2. 真实性：案例的真实性和具体性
3. 沟通表达：表达是否清晰、有条理
4. 文化匹配：价值观是否与公司文化匹配

输出格式（JSON）：
{
  "score": 8.0,
  "strengths": ["案例真实具体", "沟通表达清晰"],
  "improvements": ["可以更多展示XXX"],
  "summary": "候选人具备良好的团队协作能力..."
}"#;

    pub fn new(provider: SiliconFlowProvider) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl InterviewerAgent for HRInterviewer {
    fn role(&self) -> InterviewerRole {
        InterviewerRole::HR
    }
    
    fn role_name(&self) -> &'static str {
        "HR面试官"
    }
    
    fn avatar(&self) -> &'static str {
        "hr"
    }
    
    async fn generate_question(&self, context: &InterviewContext) -> Result<String> {
        let agent = self.provider
            .completion_model("Pro/Qwen/Qwen2.5-7B-Instruct")
            .agent(Self::SYSTEM_PROMPT)
            .build();
        
        let prompt = format!(
            r#"基于以下JD和候选人简历，生成一个行为面试问题。

JD: {}

简历: {}

要求：
1. 只输出问题本身，不要包含任何引导说明、评估标准、STAR结构提示或内部注释
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
                    strengths: vec!["案例真实".to_string()],
                    improvements: vec!["可以更结构化表达".to_string()],
                    summary: "候选人具备基本的软技能。".to_string(),
                }
            });
        
        Ok(result)
    }
    
    async fn should_follow_up(&self, answer: &str, analysis: &AnalysisResult) -> bool {
        // Follow up if missing STAR structure or score is low
        answer.len() < 150 || analysis.score < 7.5
    }
}
