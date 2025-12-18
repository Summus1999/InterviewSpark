# InterviewSpark 开发阶段计划

## 项目概述

InterviewSpark 是一款 AI 驱动的 Windows 桌面应用，帮助求职者通过模拟面试和结构化复盘分析提升面试技巧。本文档详细规划项目实现的五个开发阶段。

---

## Phase 1: 基础框架搭建

**目标**: 完成项目基础架构搭建，确保开发环境配置完整，Tauri + Rust + Vue 框架可正常运行。

**周期**: Week 1-2

**状态**: ✅ 已完成

### 功能清单

- [x] Tauri 项目初始化和配置
  - 创建 Tauri 项目结构
  - 配置 tauri.conf.json（窗口大小、应用名称、图标等）
  - 配置能力文件 (capabilities)

- [x] Rust 后端基础模块
  - 设置 Cargo.toml 依赖管理
  - 创建基础的 lib.rs 和 main.rs 结构
  - 实现简单的 Rust 命令系统（IPC 通信基础）

- [x] Vue 3 + TypeScript 前端框架
  - 初始化 Vue 3 项目结构
  - 配置 TypeScript 类型检查
  - 搭建基础页面布局（主窗口、菜单导航）
  - 配置 Vite 构建工具

- [x] 项目配置与工程化
  - 配置 .gitignore（排除 node_modules、target、dist 等）
  - 设置 ESLint 和 Prettier（代码风格统一）
  - 配置 package.json 脚本（dev、build、tauri 命令）

### 交付标准

- ✅ 应用能在开发环境成功编译运行
- ✅ 前端页面能正常显示并响应基础交互
- ✅ Rust 后端能接收并处理简单的前端请求
- ✅ 完整的开发环境文档 (在 README.md 中说明编译和运行步骤)

### 测试验证记录

**测试日期**: 2025-12-18

**测试项目**:
1. ✅ 前端构建成功 (vite build - 566ms)
2. ✅ Rust 编译成功 (cargo build - 29.97s)
3. ✅ 应用启动正常 (npm run tauri:dev)
4. ✅ IPC 通信测试通过 (greet 命令响应正常)
5. ✅ 代码规范检查通过 (ESLint + Prettier)

**验证结论**: Phase 1 基础框架搭建完成，所有功能正常运行。

---

## Phase 2: 核心文本交互

**目标**: 实现文本驱动的面试模拟闭环，集成硅基流动 API 进行问题生成和分析。

**周期**: Week 3-4

**状态**: ✅ 已完成

### 功能清单

- [x] 硅基流动 API 集成
  - 注册和配置硅基流动 API 凭证
  - 实现 API 调用封装（Rust 端）
  - 错误处理和重试机制

- [x] 用户输入模块
  - 简历文本输入界面（Vue 组件）
  - 岗位描述 (JD) 输入界面
  - 表单验证和数据收集

- [x] AI 问题生成功能
  - 调用硅基流动 API，基于简历和 JD 生成面试问题
  - 支持多个问题一次性生成
  - 前端展示生成的问题列表

- [x] 用户答题交互
  - 实时显示当前问题
  - 用户输入答案界面
  - 答题进度管理（显示第 X / 总数问题）

- [x] AI 文本分析与反馈
  - 调用硅基流动 API 分析用户答案
  - 评估答案的逻辑性、岗位匹配度、关键词覆盖
  - 生成具体改进建议

### 交付标准

- ✅ 用户能完整完成一次模拟面试（输入简历 → 生成问题 → 回答 → 获得反馈）
- ✅ API 调用稳定，错误处理完善
- ✅ 前端界面清晰易用，交互流畅

### 实现记录

**实现日期**: 2025-12-19

**完成内容**:
1. ✅ Rust 后端 API 封装 (SiliconFlowClient)
2. ✅ 环境变量配置 (.env 支持)
3. ✅ Tauri 命令暴露 (generate_questions, analyze_answer)
4. ✅ Vue 前端组件 (ResumeInput, JobDescription, QuestionList)
5. ✅ 完整面试流程界面
6. ✅ 前后端编译通过
7. ✅ 智能 JSON 解析（支持多种 API 响应格式）
8. ✅ 调试日志系统（便于问题排查）

**测试验证**:
- ✅ 问题生成功能正常（基于简历和 JD 生成 5 个问题）
- ✅ 答案分析功能正常（AI 反馈准确且详细）
- ✅ 完整面试流程测试通过（输入 → 生成 → 回答 → 反馈）
- ✅ API 调用稳定，错误处理完善
- ✅ 前端界面交互流畅，用户体验良好

**验证结论**: ✅ Phase 2 核心文本交互功能完整实现并测试通过，所有交付标准达成。

---

## Phase 3: 数据持久化

**目标**: 集成 SQLite 数据库，持久化用户数据和面试记录。

**周期**: Week 5

**状态**: ✅ 已完成

### 功能清单

- [x] SQLite 数据库设计
  - 设计数据库 Schema（用户表、面试记录表、题库表等）
  - 创建初始化脚本
  - 6 张表结构：user_config, resumes, job_descriptions, interview_sessions, interview_answers, question_bank

- [x] Rust 数据访问层 (DAL)
  - 集成 rusqlite 库
  - 实现 Repository 模式封装
  - 实现用户数据 CRUD 操作
  - 实现面试记录存储和查询

- [x] 用户信息存储
  - 存储简历内容
  - 存储岗位描述
  - 14 个 Tauri 命令封装

- [x] 面试记录存储
  - 保存每次模拟面试的完整记录
  - 包括：生成的问题、用户答案、AI 反馈、时间戳等
  - 支持按日期和岗位查询

- [x] 题库功能
  - 用户可收藏高频问题
  - 保存优化后的答案版本
  - 支持题库的增删改查

- [x] 前端界面功能
  - 历史记录页面（InterviewHistory.vue）
  - 题库管理页面（QuestionBank.vue）
  - 模式切换导航（面试/历史/题库）

### 交付标准

- ✅ 数据能正确存储和检索
- ✅ 应用重启后数据不丢失
- ✅ 支持基础查询功能（按日期、岗位筛选）

### 实现记录

**实现日期**: 2025-12-19

**完成内容**:
1. ✅ Rust 后端数据库层 (db/mod.rs, models.rs, schema.rs, repository.rs)
2. ✅ SQLite 数据库初始化（存储于 data/interview_spark.db）
3. ✅ 6 张表结构定义和创建
4. ✅ Repository 模式 CRUD 操作
5. ✅ 14 个 Tauri 数据库命令暴露
6. ✅ 前端数据服务层 (services/database.ts)
7. ✅ App.vue 集成数据库（自动保存面试记录）
8. ✅ 历史记录页面组件
9. ✅ 题库管理页面组件
10. ✅ 三种模式导航切换
11. ✅ 解决数据库文件变化触发重编译问题

**测试验证**:
- ✅ 面试会话创建和保存功能正常
- ✅ 答题记录自动保存到数据库
- ✅ 历史记录页面查看功能正常
- ✅ 题库添加、编辑、删除功能正常
- ✅ 数据库持久化验证通过（重启应用数据不丢失）
- ✅ 前后端编译通过，应用运行稳定

**验证结论**: ✅ Phase 3 数据持久化功能完整实现并测试通过，所有交付标准达成。

---

## Phase 4: 语音能力集成

**目标**: 集成语音识别和合成功能，实现真正的对话式模拟面试。

**周期**: Week 6-7

**状态**: ✅ 已完成

### 核心愿景

**AI 面试官语音发问**: 本阶段的核心目标是将应用打造成一个真正的 AI 面试官，能够通过语音向用户发问。用户启动应用后，将体验与真实面试类似的场景：
- AI 面试官语音播报面试问题
- 用户语音回答，系统实时识别
- AI 面试官语音给出反馈和追问
- 实现自然、流畅的人机语音对话体验

### 功能清单

- [x] 语音合成 (TTS)
  - 使用 Web Speech API 实现浏览器原生语音合成
  - 支持中文语音播报
  - 支持语速、音量调节
  - 自动播报面试问题和反馈

- [x] 语音识别前端界面
  - 实现"开始录音"按钮
  - 实时显示录音状态
  - 显示识别结果
  - 支持重新录制

- [x] 语音识别 (ASR)
  - 使用 Web Speech API 实现浏览器原生语音识别
  - 支持中文识别
  - 识别结果自动填入答题框

- [x] 语音播放功能
  - AI 面试官语音播报问题
  - 自动播放或用户点击播放
  - 音量和播放速度控制

- [x] 语音交互流程优化
  - 开始面试自动播报第一个问题
  - 提交答案后自动播报反馈
  - 下一题自动播报问题
  - 支持手动重复播放

### 交付标准

- ✅ 语音识别能够正常工作（中文）
- ✅ 语音合成音质自然，用户体验良好
- ✅ 整个语音交互流程连贯

### 实现记录

**实现日期**: 2025-12-19

**技术方案调整**:
- 原计划使用 SenseVoice + Piper TTS 本地模型
- 实际采用 Web Speech API 方案（更轻量、无需模型下载）

**完成内容**:
1. ✅ 创建语音服务层 (services/voice.ts)
   - TextToSpeech 类：语音合成
   - SpeechToText 类：语音识别
2. ✅ 创建语音控制组件 (VoiceControls.vue)
   - 录音按钮和状态指示
   - 播放控制
   - 语音设置（语速、音量）
3. ✅ 集成到面试界面 (App.vue)
   - 自动语音发问
   - 语音回答识别
   - 语音反馈播报
4. ✅ 前端编译通过
5. ✅ 应用运行测试通过

**测试验证**:
- ✅ 语音合成功能正常（自动播报问题和反馈）
- ✅ 语音识别功能正常（识别结果填入答题框）
- ✅ 语音设置功能正常（语速、音量可调）
- ✅ 完整语音面试流程测试通过

**注意事项**:
- 需要浏览器支持 Web Speech API（Chrome、Edge 推荐）
- 首次使用需授予麦克风权限
- 语音识别需要网络连接

**验证结论**: ✅ Phase 4 语音能力集成功能完整实现并测试通过，所有交付标准达成。

---

## Phase 5: 复盘分析系统

**目标**: 完善复盘报告的多维度分析和可视化展示。

**周期**: Week 8-9

**状态**: 待开发

### 功能清单

#### 模块 1: 多维度分析引擎

- [ ] 数据模型扩展
  - 在 `models.rs` 新增 `AnswerAnalysis` 结构体（评分、维度指标）
  - 在 `models.rs` 新增 `SessionReport` 结构体（综合报告）
  - 在 `schema.rs` 新增 `answer_analysis` 表结构
  - 在 `schema.rs` 新增 `session_reports` 表结构

- [ ] 分析算法实现
  - 在 `src-tauri/src/analysis/` 新建模块目录
  - 创建 `analysis/mod.rs` 模块入口
  - 创建 `analysis/content.rs` 实现内容分析（逻辑性、匹配度、关键词）
  - 创建 `analysis/scoring.rs` 实现评分算法（1-10分制）
  - 扩展 `siliconflow.rs` 新增 `generate_detailed_analysis()` 方法

- [ ] Repository 扩展
  - 在 `repository.rs` 新增 `save_answer_analysis()` 方法
  - 在 `repository.rs` 新增 `get_answer_analysis()` 方法
  - 在 `repository.rs` 新增 `save_session_report()` 方法
  - 在 `repository.rs` 新增 `get_session_report()` 方法

#### 模块 2: 复盘报告生成

- [ ] 报告生成后端
  - 创建 `analysis/report.rs` 报告生成模块
  - 实现 `generate_session_report()` 函数（调用硅基流动 API）
  - 实现报告结构化解析（题目回顾、评价、建议、参考答案）
  - 在 `lib.rs` 暴露 Tauri 命令 `generate_report`

- [ ] 报告导出功能
  - 创建 `analysis/export.rs` 导出模块
  - 实现 `export_to_text()` 纯文本导出
  - 实现 `export_to_html()` HTML 格式导出
  - 在 `lib.rs` 暴露 Tauri 命令 `export_report`

- [ ] 前端报告组件
  - 创建 `src/components/ReportView.vue` 报告展示组件
  - 实现报告内容渲染（题目、评分、建议）
  - 实现导出按钮和下载功能
  - 在 `services/database.ts` 新增报告相关 API 调用

#### 模块 3: 成长曲线追踪

- [ ] 统计数据存储
  - 在 `schema.rs` 新增 `performance_stats` 表（日期、平均分、维度分数）
  - 在 `models.rs` 新增 `PerformanceStats` 结构体
  - 在 `repository.rs` 新增 `save_performance_stats()` 方法
  - 在 `repository.rs` 新增 `get_performance_history()` 方法

- [ ] 趋势计算后端
  - 创建 `analysis/trends.rs` 趋势分析模块
  - 实现 `calculate_score_trend()` 评分趋势计算
  - 实现 `identify_strengths_weaknesses()` 强弱项识别
  - 在 `lib.rs` 暴露 Tauri 命令 `get_performance_trend`

- [ ] 前端图表组件
  - 安装 Chart.js 图表库（`npm install chart.js vue-chartjs`）
  - 创建 `src/components/GrowthChart.vue` 成长曲线组件
  - 实现折线图展示评分趋势
  - 实现强弱项变化对比图

#### 模块 4: 可视化仪表板

- [ ] 统计查询后端
  - 在 `repository.rs` 新增 `get_total_sessions_count()` 方法
  - 在 `repository.rs` 新增 `get_average_score()` 方法
  - 在 `repository.rs` 新增 `get_top_questions()` 热门问题查询
  - 在 `repository.rs` 新增 `get_weak_areas()` 薄弱领域查询
  - 在 `lib.rs` 暴露 Tauri 命令 `get_dashboard_stats`

- [ ] 仪表板前端组件
  - 创建 `src/components/Dashboard.vue` 仪表板主组件
  - 实现用户概览卡片（总次数、平均分、改进项）
  - 实现题目热度排行列表
  - 实现薄弱领域提示区域
  - 实现最近面试记录列表

- [ ] 导航集成
  - 在 `App.vue` 新增 "仪表板" 模式入口
  - 更新模式切换逻辑（面试/历史/题库/仪表板）

#### 模块 5: 历史记录管理增强

- [ ] 详情对比功能
  - 在 `repository.rs` 新增 `get_answers_comparison()` 对比查询
  - 创建 `src/components/AnswerComparison.vue` 答案对比组件
  - 实现同一问题不同时期答案并排对比

- [ ] 删除与备份
  - 在 `repository.rs` 新增 `delete_session()` 删除会话
  - 在 `repository.rs` 新增 `delete_all_sessions()` 清空记录
  - 创建 `analysis/backup.rs` 备份模块
  - 实现 `export_all_data()` JSON 全量导出
  - 实现 `import_data()` JSON 数据导入
  - 在 `lib.rs` 暴露 Tauri 命令 `backup_data`, `restore_data`

- [ ] 前端增强
  - 更新 `InterviewHistory.vue` 新增删除按钮
  - 新增备份/恢复按钮
  - 新增对比入口

#### 模块 6: 数据分析后端优化

- [ ] 查询性能优化
  - 在 `schema.rs` 为关键字段添加索引（session_id, created_at）
  - 实现分页查询（limit/offset）
  - 实现日期范围过滤查询

- [ ] 缓存机制
  - 创建 `src-tauri/src/cache/mod.rs` 缓存模块
  - 实现统计数据内存缓存
  - 实现缓存失效策略（数据变更时清除）

- [ ] 前端服务层扩展
  - 在 `services/database.ts` 新增统计相关 API
  - 在 `services/database.ts` 新增报告相关 API
  - 在 `services/database.ts` 新增备份相关 API

### 实施检查清单

```
1.  [ ] 创建 answer_analysis 表结构
2.  [ ] 创建 session_reports 表结构
3.  [ ] 创建 performance_stats 表结构
4.  [ ] 新增数据模型结构体
5.  [ ] 创建 analysis/ 模块目录和入口
6.  [ ] 实现内容分析算法
7.  [ ] 实现评分算法
8.  [ ] 扩展硅基流动 API 调用
9.  [ ] 实现报告生成功能
10. [ ] 实现报告导出功能
11. [ ] 实现趋势计算算法
12. [ ] 安装图表库依赖
13. [ ] 创建 Dashboard.vue 组件
14. [ ] 创建 GrowthChart.vue 组件
15. [ ] 创建 ReportView.vue 组件
16. [ ] 创建 AnswerComparison.vue 组件
17. [ ] 实现备份导出功能
18. [ ] 实现数据恢复功能
19. [ ] 添加数据库索引优化
20. [ ] 实现查询分页
21. [ ] 暴露所有 Tauri 命令
22. [ ] 更新 App.vue 导航
23. [ ] 前后端编译验证
24. [ ] 功能集成测试
```

### 交付标准

- 报告内容完整且有价值，能指导用户改进
- 可视化图表清晰易读
- 用户能清晰看到自己的成长轨迹
- 系统整体性能流畅（即使数据量较大）

---

## 阶段依赖关系

```
Phase 1 (基础框架)
    ↓
Phase 2 (核心文本交互)
    ↓
Phase 3 (数据持久化)
    ↓
Phase 4 (语音能力集成)
    ↓
Phase 5 (复盘分析系统)
```

**说明**: 每个阶段依赖前一个阶段完成，建议严格按序进行，避免并行开发导致集成困难。

---

## 质量标准

所有阶段交付均需满足：

- 代码规范：遵循项目规范文档 (sparkRules.md)
- 测试覆盖：关键功能有单元测试或集成测试
- 文档完整：代码有注释，功能有使用说明
- 性能要求：页面加载时间 < 2s，API 调用响应时间 < 5s

---

## 风险与备选方案

| 风险 | 影响 | 备选方案 |
|------|------|--------|
| 硅基流动 API 调用费用过高 | 项目成本增加 | 评估本地开源模型集成（如 LLaMA） |
| SenseVoice 模型加载过慢 | 用户体验下降 | 改用讯飞语音识别 API（云端） |
| SQLite 性能不足（大数据量） | 查询缓慢 | 迁移至 PostgreSQL |
| 语音识别准确率不够 | 体验不佳 | 调整模型参数或增加训练数据 |

