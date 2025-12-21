# InterviewSpark

基于 Tauri + Rust + Vue 3 的 AI 模拟面试桌面应用。

## 环境要求

- Node.js >= 18
- Rust >= 1.77 (安装方法: <https://www.rust-lang.org/tools/install>)
- 系统依赖:
  - Windows: Microsoft Visual Studio C++ Build Tools
  - WebView2 (Windows 10/11 自带)

## 安装依赖

```bash
npm install
```

## 配置 API

**重要**：使用 AI 面试功能需要配置硅基流动 API。

1. 注册硅基流动账号：<https://cloud.siliconflow.cn>
2. 获取 API Key
3. 复制 `.env.example` 为 `.env`：
   ```bash
   cp .env.example .env
   ```
4. 编辑 `.env` 文件，填入你的 API Key：
   ```
   SILICONFLOW_API_KEY=sk-xxxxxxxxxx
   ```

## 开发

```bash
# 仅运行前端开发服务器 (<http://localhost:5173>)
npm run dev

# 运行完整 Tauri 应用 (推荐)
npm run tauri:dev
```

## 代码规范

```bash
# 运行 ESLint 检查并自动修复
npm run lint

# 运行 Prettier 格式化
npm run format
```

## 测试

### 快速验证

1. **启动开发环境**
   ```bash
   npm run tauri:dev
   ```

2. **测试 IPC 通信**（仅开发环境）
   - 应用窗口会自动打开（标题：InterviewSpark）
   - 点击 "测试模式" 按钮进入测试界面（生产环境不显示此按钮）
   - 在输入框中输入你的名字
   - 点击 "Test IPC Connection" 按钮
   - 应该看到返回消息：`Hello, [你的名字]! Welcome to InterviewSpark.`

3. **测试模拟面试功能**
   - 点击 "Switch to Interview Mode" 进入面试模式
   - 输入简历和岗位描述（JD）
   - 点击 "生成面试问题" 生成 AI 问题
   - 点击 "开始面试" 开始答题
   - 输入答案并点击 "提交答案" 获取 AI 反馈

4. **测试语音功能**
   - 开始面试后，AI 会自动语音播报问题
   - 点击 "语音回答" 按钮，说出你的答案
   - 识别结果会自动填入答题框
   - 点击 "播放问题" 可重复播放当前问题
   - 语音设置可调节语速和音量
   - **注意**: 需要浏览器支持 (Chrome/Edge 推荐)

5. **编译测试**
   ```bash
   # 前端构建
   npm run build
   
   # Rust 编译检查
   cd src-tauri && cargo check && cd ..
   ```

### 故障排查

| 问题 | 解决方案 |
|------|----------|
| 窗口未打开 | 检查任务栏或使用 Alt+Tab 切换窗口 |
| IPC 调用失败 | 检查 Rust 后端是否正常编译，查看终端错误信息 |
| 编译错误 | 确保已安装 Rust 和 Visual Studio C++ Build Tools |
| 依赖安装失败 | 删除 node_modules 后重新运行 `npm install` |
| 语音播放无声 | 检查系统音量设置和浏览器音频权限 |
| 语音识别不工作 | 确认浏览器支持 Web Speech API（Chrome/Edge）|
| 麦克风权限 | 首次使用时需在浏览器中允许麦克风访问 |

## 构建

### 编译指令

本项目提供三种编译指令，分别适用于不同场景：

```bash
# 开发版 exe（debug 模式，不打包）
npm run build:dev

# 测试版 exe（release 模式，不打包）
npm run build:test

# 发布版安装包（NSIS 格式，含卸载清理功能）
npm run build:release
```

构建产物位于 `src-tauri/target/release/bundle/`：
- NSIS 安装包: `bundle/nsis/InterviewSpark_1.0.0_x64-setup.exe`
- 独立 exe（build:dev/test）: `target/release/app.exe` 或 `target/debug/app.exe`

### 传统构建方式

```bash
# 构建前端静态文件
npm run build

# Rust 编译检查
cd src-tauri && cargo check && cd ..

# 构建发布版应用 (同步版本号并生成安装包)
npm run release
```

### 离线构建 (国内网络)

首次构建时，Tauri 需要下载 NSIS 打包工具。国内网络可能下载失败，可手动配置：

**NSIS 离线安装**:
- 下载以下文件:
  - <https://github.com/tauri-apps/binary-releases/releases/download/nsis-3/nsis-3.zip>
  - <https://github.com/tauri-apps/nsis-tauri-utils/releases/download/nsis_tauri_utils-v0.1.1/nsis_tauri_utils.dll>
  - <https://github.com/tauri-apps/binary-releases/releases/download/nsis-plugins-v0/NSIS-ApplicationID.zip>
- 解压 nsis-3.zip 到: `C:\Users\{用户名}\AppData\Local\tauri\NSIS`
- 复制 nsis_tauri_utils.dll 和 ApplicationID.dll 到: `NSIS\Plugins\x86-unicode`

### 打包格式配置

当前项目配置为 NSIS 打包格式，在 `src-tauri/tauri.conf.json` 中：

```json
{
  "bundle": {
    "targets": ["nsis"],
    "windows": {
      "nsis": {
        "installerHooks": "nsis/hooks.nsi"
      }
    }
  }
}
```

## 数据存储位置

安装后应用数据存储在用户目录：
- Windows: `C:\Users\{用户名}\AppData\Roaming\com.interviewspark.app\`
  - `data/interview_spark.db` - SQLite 数据库
  - `avatars/` - 用户头像

### 卸载行为

**重要**: 卸载应用时，系统会自动删除以下目录及所有数据：
- `C:\Users\{用户名}\AppData\Roaming\com.interviewspark.app\`

如需保留数据，请在卸载前使用应用内的"数据备份"功能导出数据（历史记录页面 → 备份数据按钮）。导出的 JSON 文件可在重新安装后导入。

## 项目结构

```bash
src/                      # Vue 3 前端代码
  App.vue                 # 主应用组件（面试流程控制）
  main.ts                 # 前端入口
  vite-env.d.ts           # TypeScript 类型声明
  components/             # Vue 组件（39个）
    ResumeInput.vue       # 简历输入组件（支持模板选择）
    JobDescription.vue    # 岗位描述输入组件（支持模板选择）
    TemplateSelector.vue  # 简历/JD模板选择器
    QuestionList.vue      # 问题列表展示组件
    InterviewHistory.vue  # 历史记录查看组件
    QuestionBank.vue      # 题库管理组件
    VoiceControls.vue     # 语音控制组件
    Dashboard.vue         # 用户管理界面
    GrowthView.vue        # 成长曲线页面
    ReportView.vue        # 复盘报告展示
    AnswerComparison.vue  # 答案对比组件（支持时间线/并排模式）
    FeedbackDisplay.vue   # AI 流式反馈展示
    SettingsPanel.vue     # 设置面板（主题/模型/API Key）
    TimerDisplay.vue      # 计时器显示组件
    TimerSettings.vue     # 计时设置组件
    OnboardingGuide.vue   # 首次使用引导组件
    TooltipBubble.vue     # 通用提示气泡组件
    FollowUpPanel.vue     # AI 追问面板
    FollowUpSettings.vue  # 追问设置组件
    ConversationHistory.vue # 对话历史组件
    ProfileView.vue       # 个人面试画像（分析模式）
    IndustryComparison.vue  # 行业水平对比（分析模式）
    RecommendationList.vue  # 智能练习推荐（分析模式）
    BestPracticesList.vue   # 最佳实践（分析模式）
    STARScoreDisplay.vue    # STAR 法则评分展示
    ...及其他辅助组件
  composables/            # Vue Composition API 封装
    useAutoSave.ts        # 自动保存草稿
    useDataPreloader.ts   # 数据预加载
    useStreaming.ts       # 流式响应处理
  data/                   # 数据模块
    templates.ts          # 简历/JD预置模板（6个简历模板+6个JD模板）
  stores/                 # Pinia 状态管理
    questionBank.ts       # 题库缓存 Store（5分钟缓存）
    session.ts            # 会话历史缓存 Store（3分钟缓存）
  types/                  # TypeScript 类型定义
    follow-up.ts          # 追问类型定义
  services/               # 服务层
    database.ts           # 数据库服务接口
    voice.ts              # 语音服务（TTS/ASR）
    streaming.ts          # 流式响应服务
    errorHandler.ts       # 统一错误处理
    draftStorage.ts       # IndexedDB 草稿存储
    settings.ts           # 应用设置管理
src-tauri/                # Rust 后端代码
  src/
    lib.rs                # Tauri 应用入口和命令定义
    main.rs               # Rust 程序入口
    api/                  # API 模块
      mod.rs              # 模块入口
      siliconflow.rs      # 硅基流动 API 客户端（支持流式输出）
      retry.rs            # 重试策略（指数退避）
    db/                   # 数据库模块
      mod.rs              # 模块入口
      models.rs           # 数据模型定义
      schema.rs           # 表结构和初始化
      repository.rs       # 数据访问层
    analysis/             # 分析模块
      mod.rs              # 模块入口
      content.rs          # 内容分析
      scoring.rs          # 评分算法
      report.rs           # 报告生成
      export.rs           # 报告导出
      analytics.rs        # 趋势分析
      dashboard.rs        # 仪表板服务
      backup.rs           # 数据备份
      cache.rs            # 缓存管理
      best_practices.rs   # 最佳实践提取
      profile.rs          # 个人画像生成
      recommendation.rs   # 智能推荐
      industry.rs         # 行业对比分析
      trends.rs           # 趋势计算
  Cargo.toml              # Rust 依赖配置
  tauri.conf.json         # Tauri 应用配置
data/                     # 数据存储目录
  interview_spark.db      # SQLite 数据库文件
.env                      # 环境变量配置（API Key）
.env.example              # 环境变量模板
DevPlan.md                # 开发阶段计划
ProductionDoc.md          # 产品文档
```

## 技术栈

- **应用框架**: Tauri 2.x
- **后端语言**: Rust (edition 2021)
- **前端框架**: Vue 3.5 + TypeScript
- **状态管理**: Pinia
- **构建工具**: Vite 7.x
- **代码规范**: ESLint + Prettier
- **AI 能力**: 硅基流动 API，支持 6 个模型（Qwen3-8B/Qwen Plus/Qwen Max/Kimi Large/GLM-4-6v/MiniMax-M2）
- **数据模板**: 预置6个简历模板和6个JD模板（前端/后端/产品/全栈/QA/DevOps）
- **数据库**: SQLite + rusqlite
- **本地存储**: IndexedDB（草稿自动保存）
- **语音能力**: Web Speech API (TTS + ASR)
- **图表库**: ECharts + vue-echarts
- **HTTP 客户端**: reqwest（支持流式请求）
- **异步运行时**: tokio
- **SSE 解析**: eventsource-stream

## 功能特性

- **基础框架** (Phase 1): Tauri + Rust + Vue 3 跨平台架构
- **模拟面试** (Phase 2): 基于简历和 JD 生成面试问题
- **AI 反馈** (Phase 2): 分析用户答案，提供改进建议
- **完整流程** (Phase 2): 输入 → 生成问题 → 回答 → 获得反馈
- **数据持久化** (Phase 3): SQLite 本地存储，自动保存面试记录
- **历史记录** (Phase 3): 查看过往面试会话和答题详情
- **题库管理** (Phase 3): 收藏问题、添加最佳答案、分类管理
- **语音交互** (Phase 4): AI 语音发问、语音回答、语音反馈
- **复盘分析** (Phase 5): 多维度评估和可视化报告
- **成长曲线** (Phase 5): 评分趋势追踪和统计分析
- **数据仪表板** (Phase 5): 综合数据概览和热门问题统计
- **答案对比** (Phase 5): 同一问题不同时期答案对比
- **数据备份** (Phase 5): 支持 JSON 全量导出和导入
- **技术优化** (Phase 6):
  - 预置模板: 6个简历模板 + 6个JD模板一键填充
  - 计时模式: 每道题并发计时，浅色超时提示
  - AI 追问機制: 基于第一次回答的深度追问
  - 分析模式: 个人画像、智能练习推荐、行业对比、最佳实践
  - 首次引导: 5步交互式引导，降低上手门槛
  - 提示气泡: 关键步骤智能提示，支持"不再显示"
  - 设置面板: 支持主题切换、模型选择、API Key 配置
  - AI 流式输出: 打字机效果实时显示 AI 反馈
  - API 重试機制: 指数退避策略，提升请求稳定性
  - 数据预加载: Pinia 缓存常用数据，减少加载延迟
  - 草稿自动保存: IndexedDB 本地存储，防止数据丢失
  - 环境区分: 测试模式仅在开发环境显示
