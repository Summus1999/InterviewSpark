# InterviewSpark 开发阶段计划

## 项目概述

InterviewSpark 是一款 AI 驱动的 Windows 桌面应用，帮助求职者通过模拟面试和结构化复盘分析提升面试技巧。本文档详细规划项目实现的开发阶段。

---

## Phase 1-5: 基础功能完成 ✅

| 阶段 | 内容 | 完成日期 |
|------|------|----------|
| Phase 1 | 基础框架搭建（Tauri + Rust + Vue） | 2025-12-18 |
| Phase 2 | 核心文本交互（硅基流动 API 集成） | 2025-12-19 |
| Phase 3 | 数据持久化（SQLite + Repository） | 2025-12-19 |
| Phase 4 | 语音能力集成（Web Speech API） | 2025-12-19 |
| Phase 5 | 复盘分析系统（报告/趋势/仪表板） | 2025-12-19 |
| Phase 5.5 | 多用户与活跃度系统 | 2025-12-19 |

---

## Phase 6: 产品打磨与优化 ✅ 大部分完成

**状态**: 大部分已完成

### 已完成功能

- ✅ 设置面板（主题/模型/API Key）
- ✅ 首次使用引导（5步引导流程）
- ✅ 关键步骤提示气泡
- ✅ 计时模式（倒计时 + 进度条）
- ✅ AI 追问机制（5种追问类型）
- ✅ 预置简历/JD 模板（各6个）
- ✅ STAR 法则评分（雷达图展示）
- ✅ AI 流式输出（打字机效果）
- ✅ 自动保存草稿（IndexedDB）
- ✅ API 重试机制（指数退避）
- ✅ 数据预加载（Pinia 缓存）
- ✅ 环境区分（测试模式仅开发环境）
- ✅ 个人面试画像
- ✅ 智能练习推荐
- ✅ 行业水平对比
- ✅ 最佳实践提取

### 待完成功能

- [ ] 响应式布局优化
- [ ] 面试官人设选择（多风格）
- [ ] 多轮对话式面试
- [ ] 离线模式支持
- [ ] 数据库复合索引优化
- [ ] 多 AI 模型 trait 抽象
- [ ] 插件化架构

---

## Phase 7: AI 反馈质量体系与 RAG 能力增强 🔧 进行中

**目标**: 提升 AI 反馈的可操作性和一致性，完善 RAG 知识库管理

**周期**: Week 13-16

**状态**: Phase 7.1 已完成

### 模块 1: RAG 知识库管理（P0）✅ 已完成

- ✅ 知识库 CRUD（查询/搜索/删除）
- ✅ 知识导入（JSON/TXT 格式）
- ✅ KnowledgeBaseView.vue 管理界面
- ✅ 题库自动同步 RAG
- ✅ 历史数据迁移工具
- [ ] RAG 检索可视化（规划中）

### 模块 2: Prompt 工程优化（P0）待实施

- [ ] 重构 analyze_answer Prompt（结构化 JSON 输出）
- [ ] 强化面试官人设 Prompt（4种人设详细描述）
- [ ] 行业特化 Prompt（技术/产品/运营差异化）

### 模块 3: 结构化反馈展示（P0）待实施

- [ ] FeedbackScore.vue（圆环图评分）
- [ ] FeedbackStrengths.vue（亮点卡片）
- [ ] FeedbackImprovements.vue（改进建议三段式）
- [ ] JobMatchIndicator.vue（岗位匹配度）
- [ ] 重构 FeedbackDisplay.vue（集成新组件）

### 模块 4: 反馈一致性保障（P1）待实施

- [ ] 评分校准机制（AI 70% + 本地 30%）
- [ ] 格式验证机制（JSON 结构校验）
- [ ] 失败重试策略（格式异常降级处理）

### 模块 5: 反馈进化机制（P1）待实施

- [ ] 反馈评价 UI（有帮助/没帮助）
- [ ] 评价数据存储（feedback_ratings 表）
- [ ] 评价统计查询

### 交付标准

- 反馈结构完整率 > 95%
- JSON 解析成功率 > 98%
- 用户"有帮助"评价占比 > 70%

---

## Phase 8: rig 框架集成与 Multi-Agent 面试系统

**目标**: 引入 rig 框架实现 Multi-Agent 面试模拟，提升面试真实感和交互智能度

**周期**: Week 17-20

**状态**: 🔧 基础设施已完成，前端集成待实施

---

### 8.1 架构设计

#### 8.1.1 整体架构

```text
┌─────────────────────────────────────────────────────────────┐
│                        前端层 (Vue 3)                        │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │ 标准面试模式    │  │ 多角色面试模式  │  │ 答案对比模式 │ │
│  │ (现有,不变)     │  │ (新增)          │  │ (新增)       │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Tauri 命令层                            │
│  ┌─────────────────┐  ┌─────────────────────────────────┐   │
│  │ 现有命令        │  │ 新增命令                        │   │
│  │ (generate_xxx)  │  │ (multi_agent_interview, etc.)   │   │
│  └─────────────────┘  └─────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Rust 后端层                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                  rig_adapter/ (新增)                    ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌───────────────┐ ││
│  │  │ provider.rs  │  │ vector_store │  │ agents.rs     │ ││
│  │  │ (LLM适配)    │  │ (检索适配)   │  │ (Agent定义)   │ ││
│  │  └──────────────┘  └──────────────┘  └───────────────┘ ││
│  └─────────────────────────────────────────────────────────┘│
│                              │                               │
│                              ▼                               │
│  ┌─────────────────────────────────────────────────────────┐│
│  │              现有模块 (零修改)                          ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌───────────────┐ ││
│  │  │ siliconflow  │  │ vectordb.rs  │  │ embedding.rs  │ ││
│  │  │ (API调用)    │  │ (HNSW检索)   │  │ (fastembed)   │ ││
│  │  └──────────────┘  └──────────────┘  └───────────────┘ ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

#### 8.1.2 核心价值

| 能力 | 描述 | 业务价值 |
|------|------|----------|
| Multi-Agent 面试 | 技术官+HR+业务官轮流提问 | 更真实的多轮面试体验 |
| 对话状态机 | 暖场→技术→行为→反问流程 | 面试节奏更自然 |
| 答案对比 Agent | 逐点分析用户答案与最佳答案差距 | 改进指导更精准 |
| 适配器模式 | 现有代码零修改 | 低风险、可回退 |

#### 8.1.3 目录结构

```text
src-tauri/src/
├── rig_adapter/                 # 新增：rig 适配层
│   ├── mod.rs                   # 模块入口
│   ├── provider.rs              # SiliconFlow Provider 实现
│   ├── vector_store.rs          # VectorStoreIndex 适配
│   ├── agents/                  # Agent 定义
│   │   ├── mod.rs
│   │   ├── tech.rs              # 技术面试官
│   │   ├── hr.rs                # HR 面试官
│   │   ├── business.rs          # 业务面试官
│   │   └── comparison.rs        # 答案对比 Agent
│   ├── scheduler.rs             # Agent 轮转调度器
│   └── state_machine.rs         # 面试阶段状态机
├── api/
│   └── siliconflow.rs           # 保留不变
├── rag/
│   ├── vectordb.rs              # 保留不变
│   └── embedding.rs             # 保留不变
└── lib.rs                       # 新增 rig 相关命令
```

---

### 8.2 模块详细设计

#### 8.2.1 模块 1: SiliconFlow Provider 适配 (P0)

**目标**: 将现有 `SiliconFlowClient` 封装为 rig 的 `CompletionClient`

**文件**: `src-tauri/src/rig_adapter/provider.rs`

```rust
// 核心结构定义
use rig::client::CompletionClient;
use rig::completion::{CompletionModel, CompletionRequest, CompletionResponse};
use crate::api::SiliconFlowClient;

/// SiliconFlow Provider - 适配 rig CompletionClient trait
pub struct SiliconFlowProvider {
    inner: SiliconFlowClient,  // 复用现有客户端
}

impl SiliconFlowProvider {
    pub fn new(api_key: String, model: String) -> Result<Self> {
        Ok(Self {
            inner: SiliconFlowClient::new(api_key, model)?,
        })
    }
    
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            inner: SiliconFlowClient::from_env()?,
        })
    }
}

/// SiliconFlow Completion Model
pub struct SiliconFlowCompletionModel {
    client: SiliconFlowClient,
    model_name: String,
}

impl CompletionClient for SiliconFlowProvider {
    type CompletionModel = SiliconFlowCompletionModel;
    
    fn completion_model(&self, model: &str) -> Self::CompletionModel {
        SiliconFlowCompletionModel {
            client: self.inner.clone(),
            model_name: model.to_string(),
        }
    }
}

impl CompletionModel for SiliconFlowCompletionModel {
    type Response = SiliconFlowResponse;
    type StreamingResponse = SiliconFlowStreamChunk;
    
    async fn completion(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse<Self::Response>, CompletionError> {
        // 1. 将 rig CompletionRequest 转换为 ChatMessage 列表
        let messages = convert_to_chat_messages(&request);
        
        // 2. 调用现有 SiliconFlowClient
        let response = self.client
            .chat_completion_with_model(messages, &self.model_name, None, None)
            .await?;
        
        // 3. 封装为 rig CompletionResponse
        Ok(CompletionResponse::new(SiliconFlowResponse { content: response }))
    }
    
    async fn stream(
        &self,
        request: CompletionRequest,
    ) -> Result<StreamingCompletionResponse<Self::StreamingResponse>, CompletionError> {
        // 复用现有 chat_completion_stream 实现
        // ...
    }
}
```

实施清单:

- [✓] 创建 `rig_adapter/mod.rs` 模块入口
- [✓] 创建 `rig_adapter/provider.rs`
- [✓] 实现 `SiliconFlowProvider` 结构体
- [ ] 实现 `CompletionClient` trait（简化版已完成，rig trait 未引入）
- [✓] 实现 `SiliconFlowCompletionModel` 结构体
- [ ] 实现 `CompletionModel` trait（简化版已完成）
- [✓] 实现请求/响应转换函数
- [ ] 单元测试

---

#### 8.2.2 模块 2: VectorStoreIndex 适配 (P0)

**目标**: 将现有 `VectorStore` 封装为 rig 的 `VectorStoreIndex`

**文件**: `src-tauri/src/rig_adapter/vector_store.rs`

```rust
use rig::vector_store::{VectorStoreIndex, VectorStoreError};
use crate::rag::{EmbeddingService, VectorStore, SearchResult};

/// VectorStore 适配器 - 桥接到现有 HNSW 实现
pub struct VectorStoreAdapter {
    embedding_service: Arc<EmbeddingService>,
    vector_store: Arc<VectorStore>,
}

impl VectorStoreAdapter {
    pub fn new(
        embedding_service: Arc<EmbeddingService>,
        vector_store: Arc<VectorStore>,
    ) -> Self {
        Self { embedding_service, vector_store }
    }
}

impl VectorStoreIndex for VectorStoreAdapter {
    /// 返回 Top N 结果，包含文档内容
    async fn top_n<T: for<'de> serde::Deserialize<'de> + Send>(
        &self,
        query: &str,
        n: usize,
    ) -> Result<Vec<(f64, String, T)>, VectorStoreError> {
        // 1. 生成查询向量
        let embedding = self.embedding_service
            .embed_text(query)
            .await
            .map_err(|e| VectorStoreError::Other(e.to_string()))?;
        
        // 2. 调用现有 HNSW 检索
        let results = self.vector_store
            .search(&embedding, n, None)
            .await
            .map_err(|e| VectorStoreError::Other(e.to_string()))?;
        
        // 3. 转换为 rig 格式 (score, id, doc)
        results.into_iter()
            .map(|r| {
                let doc: T = serde_json::from_str(&r.content)
                    .map_err(|e| VectorStoreError::Other(e.to_string()))?;
                Ok((r.similarity as f64, r.id.to_string(), doc))
            })
            .collect()
    }
    
    /// 返回 Top N ID
    async fn top_n_ids(
        &self,
        query: &str,
        n: usize,
    ) -> Result<Vec<(f64, String)>, VectorStoreError> {
        let embedding = self.embedding_service
            .embed_text(query)
            .await
            .map_err(|e| VectorStoreError::Other(e.to_string()))?;
        
        let results = self.vector_store
            .search(&embedding, n, None)
            .await
            .map_err(|e| VectorStoreError::Other(e.to_string()))?;
        
        Ok(results.into_iter()
            .map(|r| (r.similarity as f64, r.id.to_string()))
            .collect())
    }
}
```

**实施清单**:

- [✓] 创建 `rig_adapter/vector_store.rs`
- [✓] 实现 `VectorStoreAdapter` 结构体
- [ ] 实现 `VectorStoreIndex` trait（简化版已完成）
- [✓] 实现 `top_n` 方法
- [✓] 实现 `top_n_ids` 方法
- [ ] 单元测试

---

#### 8.2.3 模块 3: Multi-Agent 定义 (P0)

**目标**: 定义三种面试官 Agent，实现差异化提问风格

**文件**: `src-tauri/src/rig_adapter/agents/`

```rust
// agents/mod.rs
pub mod tech;
pub mod hr;
pub mod business;
pub mod comparison;

use rig::agent::Agent;
use async_trait::async_trait;

/// 面试官角色枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterviewerRole {
    Technical,   // 技术面试官
    HR,          // HR 面试官
    Business,    // 业务面试官
}

/// 面试官 Agent 统一接口
#[async_trait]
pub trait InterviewerAgent: Send + Sync {
    /// 获取角色
    fn role(&self) -> InterviewerRole;
    
    /// 获取角色名称（中文）
    fn role_name(&self) -> &'static str;
    
    /// 获取角色头像标识
    fn avatar(&self) -> &'static str;
    
    /// 生成面试问题
    async fn generate_question(
        &self,
        context: &InterviewContext,
    ) -> Result<String>;
    
    /// 分析用户回答
    async fn analyze_answer(
        &self,
        question: &str,
        answer: &str,
        context: &InterviewContext,
    ) -> Result<AnalysisResult>;
    
    /// 决定是否追问
    async fn should_follow_up(
        &self,
        answer: &str,
        analysis: &AnalysisResult,
    ) -> bool;
}

/// 面试上下文
#[derive(Debug, Clone)]
pub struct InterviewContext {
    pub resume: String,
    pub job_description: String,
    pub conversation_history: Vec<ConversationTurn>,
    pub current_phase: InterviewPhase,
}

/// 对话轮次
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub role: InterviewerRole,
    pub role_name: String,
    pub question: String,
    pub answer: Option<String>,
    pub analysis: Option<AnalysisResult>,
}

/// 分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub score: f32,
    pub strengths: Vec<String>,
    pub improvements: Vec<String>,
    pub summary: String,
}
```

```rust
// agents/tech.rs - 技术面试官
pub struct TechInterviewer {
    provider: SiliconFlowProvider,
    vector_store: VectorStoreAdapter,
}

impl TechInterviewer {
    const SYSTEM_PROMPT: &'static str = r#"
你是一位资深技术面试官，拥有10年以上技术管理经验。

评估重点：
- 技术深度：对核心技术原理的理解程度
- 问题解决：分析问题和设计解决方案的能力
- 系统设计：架构思维和技术选型判断力
- 代码质量：编码规范和最佳实践意识

提问风格：
- 从基础概念切入，逐步深入到底层原理
- 追问实现细节和边界情况
- 结合实际场景考察应用能力

语气：专业、严谨、有深度
"#;
}

#[async_trait]
impl InterviewerAgent for TechInterviewer {
    fn role(&self) -> InterviewerRole { InterviewerRole::Technical }
    fn role_name(&self) -> &'static str { "技术面试官" }
    fn avatar(&self) -> &'static str { "tech" }
    
    async fn generate_question(&self, context: &InterviewContext) -> Result<String> {
        // 1. 从 RAG 检索相关技术问题
        let rag_context = self.vector_store
            .top_n::<String>(&context.job_description, 3)
            .await?;
        
        // 2. 构建 prompt 生成问题
        let agent = self.provider
            .completion_model("Pro/zai-org/GLM-4.7")
            .agent(Self::SYSTEM_PROMPT)
            .build();
        
        agent.prompt(&format!(
            "基于以下JD和候选人简历，生成一个技术面试问题。\n\nJD: {}\n\n简历: {}\n\n参考问题: {:?}",
            context.job_description, context.resume, rag_context
        )).await
    }
    // ... 其他方法实现
}
```

```rust
// agents/hr.rs - HR 面试官
pub struct HRInterviewer { /* ... */ }

impl HRInterviewer {
    const SYSTEM_PROMPT: &'static str = r#"
你是一位经验丰富的HR面试官，专注于评估候选人的软技能和文化匹配度。

评估重点：
- 沟通能力：表达清晰度、逻辑性
- 团队协作：过往协作经验和冲突处理
- 职业规划：发展目标与岗位匹配度
- 价值观：工作态度和职业素养

提问风格：
- 使用行为面试法（STAR）
- 关注过往经历中的具体案例
- 挖掘候选人的真实想法

语气：亲和、专业、善于引导
"#;
}
```

```rust
// agents/business.rs - 业务面试官
pub struct BusinessInterviewer { /* ... */ }

impl BusinessInterviewer {
    const SYSTEM_PROMPT: &'static str = r#"
你是一位业务部门负责人，关注候选人能否快速上手并产出业务价值。

评估重点：
- 业务理解：对行业和业务的认知深度
- 落地能力：将想法转化为可执行方案
- 结果导向：过往项目的实际成果
- 学习能力：快速掌握新领域的能力

提问风格：
- 从实际业务场景出发
- 关注解决问题的思路和方法
- 考察数据驱动决策能力

语气：务实、结果导向、注重细节
"#;
}
```

**实施清单**:

- [✓] 创建 `rig_adapter/agents/mod.rs`
- [✓] 定义 `InterviewerAgent` trait
- [✓] 定义 `InterviewContext` 和相关结构体
- [✓] 实现 `TechInterviewer`
- [✓] 实现 `HRInterviewer`
- [✓] 实现 `BusinessInterviewer`
- [ ] 单元测试

---

#### 8.2.4 模块 4: Agent 轮转调度器 (P0)

**目标**: 实现多 Agent 轮转提问逻辑

**文件**: `src-tauri/src/rig_adapter/scheduler.rs`

```rust
use super::agents::{InterviewerAgent, InterviewerRole, InterviewContext, ConversationTurn};

/// Agent 调度器
pub struct AgentScheduler {
    agents: Vec<Box<dyn InterviewerAgent>>,
    current_index: usize,
    rotation_strategy: RotationStrategy,
}

/// 轮转策略
#[derive(Debug, Clone)]
pub enum RotationStrategy {
    /// 固定顺序轮转：技术→HR→业务→技术...
    FixedOrder,
    /// 按阶段切换：技术阶段全部技术官，HR阶段全部HR
    PhaseBased,
    /// 随机轮转
    Random,
}

impl AgentScheduler {
    pub fn new(agents: Vec<Box<dyn InterviewerAgent>>) -> Self {
        Self {
            agents,
            current_index: 0,
            rotation_strategy: RotationStrategy::FixedOrder,
        }
    }
    
    /// 获取当前 Agent
    pub fn current_agent(&self) -> &dyn InterviewerAgent {
        self.agents[self.current_index].as_ref()
    }
    
    /// 切换到下一个 Agent
    pub fn next_agent(&mut self) -> &dyn InterviewerAgent {
        match self.rotation_strategy {
            RotationStrategy::FixedOrder => {
                self.current_index = (self.current_index + 1) % self.agents.len();
            }
            RotationStrategy::Random => {
                use rand::Rng;
                self.current_index = rand::thread_rng().gen_range(0..self.agents.len());
            }
            RotationStrategy::PhaseBased => {
                // 由 state_machine 控制
            }
        }
        self.current_agent()
    }
    
    /// 根据阶段选择 Agent
    pub fn select_by_phase(&mut self, phase: InterviewPhase) -> &dyn InterviewerAgent {
        let target_role = match phase {
            InterviewPhase::WarmUp => InterviewerRole::HR,
            InterviewPhase::Technical => InterviewerRole::Technical,
            InterviewPhase::Behavioral => InterviewerRole::HR,
            InterviewPhase::Business => InterviewerRole::Business,
            InterviewPhase::Questions => InterviewerRole::HR,
        };
        
        self.current_index = self.agents
            .iter()
            .position(|a| a.role() == target_role)
            .unwrap_or(0);
        
        self.current_agent()
    }
    
    /// 执行一轮面试问答
    pub async fn execute_turn(
        &mut self,
        context: &mut InterviewContext,
    ) -> Result<ConversationTurn> {
        let agent = self.current_agent();
        
        // 1. 生成问题
        let question = agent.generate_question(context).await?;
        
        // 2. 创建对话轮次（等待用户回答）
        let turn = ConversationTurn {
            role: agent.role(),
            role_name: agent.role_name().to_string(),
            question,
            answer: None,
            analysis: None,
        };
        
        context.conversation_history.push(turn.clone());
        Ok(turn)
    }
    
    /// 处理用户回答
    pub async fn process_answer(
        &self,
        context: &mut InterviewContext,
        answer: String,
    ) -> Result<AnalysisResult> {
        let agent = self.current_agent();
        let last_turn = context.conversation_history.last_mut()
            .ok_or(anyhow!("No conversation turn"))?;
        
        // 1. 记录回答
        last_turn.answer = Some(answer.clone());
        
        // 2. 分析回答
        let analysis = agent.analyze_answer(
            &last_turn.question,
            &answer,
            context,
        ).await?;
        
        last_turn.analysis = Some(analysis.clone());
        Ok(analysis)
    }
}
```

**实施清单**:

- [✓] 创建 `rig_adapter/scheduler.rs`
- [✓] 实现 `AgentScheduler` 结构体
- [✓] 实现 `RotationStrategy` 枚举
- [✓] 实现 `execute_turn` 方法
- [✓] 实现 `process_answer` 方法
- [ ] 单元测试

---

#### 8.2.5 模块 5: 面试阶段状态机 (P1)

**目标**: 管理面试流程阶段转换

**文件**: `src-tauri/src/rig_adapter/state_machine.rs`

```rust
/// 面试阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterviewPhase {
    WarmUp,      // 暖场（1-2题）
    Technical,   // 技术考察（3-5题）
    Behavioral,  // 行为面试（2-3题）
    Business,    // 业务理解（2-3题）
    Questions,   // 反问环节（1-2题）
    Completed,   // 面试结束
}

/// 阶段配置
#[derive(Debug, Clone)]
pub struct PhaseConfig {
    pub phase: InterviewPhase,
    pub min_questions: u32,
    pub max_questions: u32,
    pub primary_role: InterviewerRole,
}

/// 面试状态机
pub struct InterviewStateMachine {
    current_phase: InterviewPhase,
    phase_question_count: u32,
    total_question_count: u32,
    phase_configs: Vec<PhaseConfig>,
}

impl InterviewStateMachine {
    pub fn new() -> Self {
        Self {
            current_phase: InterviewPhase::WarmUp,
            phase_question_count: 0,
            total_question_count: 0,
            phase_configs: Self::default_configs(),
        }
    }
    
    fn default_configs() -> Vec<PhaseConfig> {
        vec![
            PhaseConfig { phase: InterviewPhase::WarmUp, min_questions: 1, max_questions: 2, primary_role: InterviewerRole::HR },
            PhaseConfig { phase: InterviewPhase::Technical, min_questions: 3, max_questions: 5, primary_role: InterviewerRole::Technical },
            PhaseConfig { phase: InterviewPhase::Behavioral, min_questions: 2, max_questions: 3, primary_role: InterviewerRole::HR },
            PhaseConfig { phase: InterviewPhase::Business, min_questions: 2, max_questions: 3, primary_role: InterviewerRole::Business },
            PhaseConfig { phase: InterviewPhase::Questions, min_questions: 1, max_questions: 2, primary_role: InterviewerRole::HR },
        ]
    }
    
    pub fn current_phase(&self) -> InterviewPhase {
        self.current_phase
    }
    
    /// 记录一个问题，检查是否需要切换阶段
    pub fn record_question(&mut self) -> Option<InterviewPhase> {
        self.phase_question_count += 1;
        self.total_question_count += 1;
        
        let current_config = self.phase_configs.iter()
            .find(|c| c.phase == self.current_phase)?;
        
        // 达到最大问题数，强制切换
        if self.phase_question_count >= current_config.max_questions {
            return self.advance_phase();
        }
        
        None
    }
    
    /// 基于用户表现决定是否提前切换阶段
    pub fn maybe_advance(&mut self, analysis: &AnalysisResult) -> Option<InterviewPhase> {
        let current_config = self.phase_configs.iter()
            .find(|c| c.phase == self.current_phase)?;
        
        // 已达最小问题数 + 回答优秀，可以切换
        if self.phase_question_count >= current_config.min_questions && analysis.score >= 8.0 {
            return self.advance_phase();
        }
        
        None
    }
    
    fn advance_phase(&mut self) -> Option<InterviewPhase> {
        self.phase_question_count = 0;
        
        let next_phase = match self.current_phase {
            InterviewPhase::WarmUp => InterviewPhase::Technical,
            InterviewPhase::Technical => InterviewPhase::Behavioral,
            InterviewPhase::Behavioral => InterviewPhase::Business,
            InterviewPhase::Business => InterviewPhase::Questions,
            InterviewPhase::Questions => InterviewPhase::Completed,
            InterviewPhase::Completed => return None,
        };
        
        self.current_phase = next_phase;
        Some(next_phase)
    }
    
    /// 获取进度信息
    pub fn progress(&self) -> InterviewProgress {
        InterviewProgress {
            current_phase: self.current_phase,
            phase_question_count: self.phase_question_count,
            total_question_count: self.total_question_count,
            is_completed: self.current_phase == InterviewPhase::Completed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewProgress {
    pub current_phase: InterviewPhase,
    pub phase_question_count: u32,
    pub total_question_count: u32,
    pub is_completed: bool,
}
```

**实施清单**:

- [✓] 创建 `rig_adapter/state_machine.rs`
- [✓] 实现 `InterviewPhase` 枚举
- [✓] 实现 `InterviewStateMachine` 结构体
- [✓] 实现阶段转换逻辑
- [✓] 实现进度追踪
- [ ] 单元测试

---

#### 8.2.6 模块 6: 答案对比 Agent (P1)

**目标**: 将用户答案与最佳答案逐点对比

**文件**: `src-tauri/src/rig_adapter/agents/comparison.rs`

```rust
/// 答案对比 Agent
pub struct ComparisonAgent {
    provider: SiliconFlowProvider,
}

impl ComparisonAgent {
    const SYSTEM_PROMPT: &'static str = r#"
你是一位面试答案分析专家。请将用户答案与最佳答案进行逐点对比分析。

输出格式（JSON）：
{
  "overall_match": 0.75,  // 整体匹配度 0-1
  "comparisons": [
    {
      "aspect": "技术准确性",
      "best_answer_point": "最佳答案中的要点",
      "user_answer_point": "用户答案中对应内容",
      "match": "matched|partial|missing",
      "suggestion": "改进建议"
    }
  ],
  "missing_points": ["用户遗漏的关键点"],
  "extra_points": ["用户额外提到的有价值内容"]
}
"#;
    
    pub async fn compare(
        &self,
        question: &str,
        user_answer: &str,
        best_answer: &str,
    ) -> Result<ComparisonResult> {
        let agent = self.provider
            .completion_model("Pro/zai-org/GLM-4.7")
            .agent(Self::SYSTEM_PROMPT)
            .build();
        
        let response = agent.prompt(&format!(
            "问题：{}\n\n用户答案：{}\n\n最佳答案：{}\n\n请进行对比分析。",
            question, user_answer, best_answer
        )).await?;
        
        // 解析 JSON 响应
        let result: ComparisonResult = serde_json::from_str(&response)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub overall_match: f32,
    pub comparisons: Vec<PointComparison>,
    pub missing_points: Vec<String>,
    pub extra_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointComparison {
    pub aspect: String,
    pub best_answer_point: String,
    pub user_answer_point: String,
    pub match_status: MatchStatus,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchStatus {
    Matched,
    Partial,
    Missing,
}
```

**实施清单**:

- [✓] 创建 `rig_adapter/agents/comparison.rs`
- [✓] 实现 `ComparisonAgent` 结构体
- [✓] 实现 `compare` 方法
- [✓] 定义 `ComparisonResult` 结构体
- [ ] 单元测试

---

### 8.3 前端组件设计

#### 8.3.1 MultiAgentInterview.vue

**位置**: `src/components/MultiAgentInterview.vue`

**功能**:

- 多角色对话界面
- 不同面试官头像和颜色区分
- 阶段进度指示器
- 实时阶段切换提示

**UI 布局**:

```text
┌──────────────────────────────────────────────┐
│  阶段进度条: [暖场] → [技术] → [行为] → ...   │
├──────────────────────────────────────────────┤
│  ┌────┐                                      │
│  │ 👨‍💼 │ 技术面试官: 请介绍一下你对微服务的理解 │
│  └────┘                                      │
│                                              │
│                        ┌────┐                │
│    用户回答: ...       │ 👤 │                │
│                        └────┘                │
│  ┌────┐                                      │
│  │ 👩‍💼 │ HR: 能分享一个团队协作的经历吗？      │
│  └────┘                                      │
├──────────────────────────────────────────────┤
│  [输入回答...]                    [提交]     │
└──────────────────────────────────────────────┘
```

**实施清单**:

- [✓] 创建 `MultiAgentInterview.vue` 组件
- [ ] 实现多角色消息气泡
- [ ] 实现阶段进度指示器
- [ ] 实现角色头像和颜色区分
- [ ] 集成语音输入（复用现有 VoiceControls）
- [ ] 集成到 App.vue 导航

#### 8.3.2 AnswerComparison.vue 对比功能

**位置**: `src/components/AnswerComparison.vue`

**功能**:

- 用户答案与最佳答案并排对比
- 匹配点高亮显示
- 缺失点红色标记
- 额外亮点绿色标记

**实施清单**:

- [✓] 组件已存在（当前实现为时间线/并排模式）
- [ ] 集成 ComparisonAgent 逐点对比功能
- [ ] 实现匹配状态高亮
- [ ] 实现改进建议展示

---

### 8.4 Tauri 命令暴露

**文件**: `src-tauri/src/lib.rs`

```rust
// 新增命令

/// 启动多角色面试会话
#[tauri::command]
async fn start_multi_agent_interview(
    state: State<'_, AppState>,
    resume: String,
    job_description: String,
) -> Result<MultiAgentSession, String> {
    // 创建会话，初始化 Agent 和状态机
}

/// 获取下一个面试问题
#[tauri::command]
async fn get_next_question(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<InterviewQuestion, String> {
    // 调用 scheduler.execute_turn()
}

/// 提交用户回答
#[tauri::command]
async fn submit_multi_agent_answer(
    state: State<'_, AppState>,
    session_id: String,
    answer: String,
) -> Result<AnswerAnalysis, String> {
    // 调用 scheduler.process_answer()
}

/// 获取面试进度
#[tauri::command]
async fn get_interview_progress(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<InterviewProgress, String> {
    // 调用 state_machine.progress()
}

/// 对比用户答案与最佳答案
#[tauri::command]
async fn compare_with_best_answer(
    state: State<'_, AppState>,
    question: String,
    user_answer: String,
) -> Result<ComparisonResult, String> {
    // 调用 ComparisonAgent.compare()
}
```

**实施清单**:

- [ ] 实现 `rig_adapter/mod.rs` 向 lib.rs 暴露（已完成）
- [ ] 实现 `start_multi_agent_interview` 命令
- [ ] 实现 `get_next_question` 命令
- [ ] 实现 `submit_multi_agent_answer` 命令
- [ ] 实现 `get_interview_progress` 命令
- [ ] 实现 `compare_with_best_answer` 命令
- [ ] 注册到 `invoke_handler`

---

### 8.5 依赖配置

**文件**: `src-tauri/Cargo.toml`

```toml
[dependencies]
# 现有依赖保持不变
# ...

# 新增 rig 相关依赖
rig-core = { version = "0.24", default-features = false }
async-trait = "0.1"
rand = "0.8"  # 用于随机轮转策略
```

---

### 8.6 实施时间表

| 周次 | 阶段 | 任务 | 产出 |
|------|------|------|------|
| Week 17 | 基础适配层 | Provider + VectorStore 适配 | ✓ 编译通过，简化版已完成 |
| Week 18 | Agent 核心 | 3种面试官 + 调度器 | ✓ Agent 基础结构已完成 |
| Week 19 | 前端集成 | MultiAgentInterview.vue | 组件已创建，功能待完善 |
| Week 20 | 完善优化 | 状态机 + 对比 Agent + 测试 | ✓ 后端逻辑已完成，前端集成待实施 |

---

### 8.7 交付标准

- [✓] rig_adapter 基础设施完成（Provider/VectorStore/Agents/Scheduler/StateMachine）
- [✓] 三种面试官风格差异明显（通过 Prompt 验证）
- [ ] Multi-Agent 面试流程完整可用
- [ ] 对话状态切换自然流畅（用户无感知）
- [ ] 答案对比输出结构清晰（JSON 解析成功率 > 95%）
- [✓] 现有功能完全不受影响（回归测试通过）
- [✓] 后端编译通过，无 warning
- [ ] 前端编译通过，无 error

---

### 8.8 风险与备选方案

| 风险 | 概率 | 影响 | 备选方案 |
|------|------|------|----------|
| rig API 变动 | 低 | 适配层需调整 | 锁定版本 `0.24`，跟进 changelog |
| 多 Agent 延迟累积 | 中 | 响应变慢 | 并行调用 + 缓存策略 |
| 离线 Embedding 不支持 | 低 | 功能受限 | 保留现有 fastembed 方案 |
| 前端复杂度增加 | 中 | 开发周期延长 | 复用现有组件（VoiceControls, FeedbackDisplay） |

---

## 阶段依赖关系

```text
Phase 1-5.5 ✅ 基础功能完成
    ↓
Phase 6 ✅ 产品打磨（大部分完成）
    ↓
Phase 7 🔧 AI 反馈质量体系（进行中）
    ├─ Phase 7.1 RAG 知识库管理 ✅
    └─ Phase 7.2-7.5 反馈优化 ⭕ 待实施
    ↓
Phase 8 🔧 rig 框架集成（基础设施已完成）
    ├─ 8.1-8.2.6 后端基础设施 ✅
    └─ 8.3 前端集成 ⭕ 待实施
```

---

## 构建与发布配置 ✅

### 编译指令

```bash
npm run build:dev      # 开发版 exe
npm run build:test     # 测试版 exe
npm run build:release  # 发布版安装包
```

### 卸载数据清理

配置文件: `src-tauri/nsis/hooks.nsi`
清理路径: `$APPDATA\com.interviewspark.app`

---

## 风险与备选方案

| 风险 | 影响 | 备选方案 |
|------|------|--------|
| 硅基流动 API 调用费用过高 | 项目成本增加 | 评估本地开源模型集成 |
| SQLite 性能不足（大数据量） | 查询缓慢 | 迁移至 PostgreSQL |
| 语音识别准确率不够 | 体验不佳 | 调整模型参数或增加训练数据 |
