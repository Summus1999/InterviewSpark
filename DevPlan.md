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

## Phase 8: rig 框架集成与 Agent 能力增强（规划）

**目标**: 引入 rig 框架实现 Multi-Agent 面试模拟，提升面试真实感和交互智能度

**周期**: Week 17-20

**状态**: 📋 规划中

### 核心价值

| 能力 | 描述 | 业务价值 |
|------|------|----------|
| Multi-Agent 面试 | 技术官+HR+业务官轮流提问 | 更真实的多轮面试体验 |
| 对话状态机 | 暖场→技术→行为→反问流程 | 面试节奏更自然 |
| 答案对比 Agent | 逐点分析用户答案与最佳答案差距 | 改进指导更精准 |

### 模块 1: 基础适配层（P0）

- [ ] 实现 rig VectorStoreIndex trait 适配现有 hnsw_rs
- [ ] 封装 SiliconFlowClient 为 rig LLM Provider
- [ ] 保留现有 fastembed 做本地 Embedding（rig 不支持离线 Embedding）

### 模块 2: Multi-Agent 面试系统（P0）

- [ ] 定义 InterviewerAgent trait
- [ ] 实现 TechInterviewer（技术面试官）
- [ ] 实现 HRInterviewer（HR 面试官）
- [ ] 实现 BusinessInterviewer（业务面试官）
- [ ] 实现 Agent 轮转调度逻辑
- [ ] 前端多角色头像和对话气泡

### 模块 3: 对话流程编排（P1）

- [ ] 定义面试阶段枚举（WarmUp/Technical/Behavioral/Questions）
- [ ] 实现状态机转换逻辑
- [ ] 根据用户表现动态调整流程
- [ ] 前端阶段进度指示器

### 模块 4: 答案对比 Agent（P1）

- [ ] 实现 ComparisonAgent
- [ ] 调用现有 generate_best_answer 获取最佳答案
- [ ] 逐点结构化对比输出
- [ ] 前端对比结果可视化

### 实施检查清单

```
Week 17: 基础适配层
1. [ ] 添加 rig-core 依赖
2. [ ] 实现 VectorStoreIndex trait
3. [ ] 实现 SiliconFlowProvider
4. [ ] 编译验证

Week 18: Multi-Agent 核心
5. [ ] 定义 Agent trait 和结构体
6. [ ] 实现三种面试官 Agent
7. [ ] 实现轮转调度器
8. [ ] 单元测试

Week 19: 前端集成
9. [ ] 创建 MultiAgentInterview.vue 组件
10. [ ] 实现多角色对话 UI
11. [ ] 集成到 App.vue
12. [ ] 前端编译验证

Week 20: 对话编排 + 对比 Agent
13. [ ] 实现面试阶段状态机
14. [ ] 实现 ComparisonAgent
15. [ ] 创建对比结果组件
16. [ ] 集成测试
```

### 交付标准

- Multi-Agent 面试流程完整可用
- 三种面试官风格差异明显
- 对话状态切换自然流畅
- 答案对比输出结构清晰

### 技术依赖

```toml
# Cargo.toml 新增
rig-core = "0.24"
async-trait = "0.1"
```

### 风险与备选

| 风险 | 影响 | 备选方案 |
|------|------|----------|
| rig API 变动 | 适配层需调整 | 锁定版本，跟进 changelog |
| 多 Agent 延迟累积 | 响应变慢 | 并行调用 + 缓存策略 |
| 离线 Embedding 不支持 | 功能受限 | 保留现有 fastembed 方案 |

---

## 阶段依赖关系

```
Phase 1-5.5 ✅ 基础功能完成
    ↓
Phase 6 ✅ 产品打磨（大部分完成）
    ↓
Phase 7 🔧 AI 反馈质量体系（进行中）
    ├─ Phase 7.1 RAG 知识库管理 ✅
    └─ Phase 7.2-7.5 反馈优化 ⭕ 待实施
    ↓
Phase 8 📋 rig 框架集成（规划）
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
