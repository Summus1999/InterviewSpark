# InterviewSpark

基于 Tauri + Rust + Vue 3 的 AI 模拟面试桌面应用。

## 环境要求

- Node.js >= 18
- Rust >= 1.77 (安装方法: https://www.rust-lang.org/tools/install)
- 系统依赖:
  - Windows: Microsoft Visual Studio C++ Build Tools
  - WebView2 (Windows 10/11 自带)

## 安装依赖

```bash
npm install
```

## 配置 API

**重要**：使用 AI 面试功能需要配置硅基流动 API。

1. 注册硅基流动账号：https://cloud.siliconflow.cn
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
# 仅运行前端开发服务器 (http://localhost:5173)
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

2. **测试 IPC 通信**
   - 应用窗口会自动打开（标题：InterviewSpark）
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

```bash
# 构建前端静态文件
npm run build

# 构建发布版应用 (生成安装包)
npm run tauri:build
```

构建产物位于 `src-tauri/target/release/bundle/`

## 项目结构

```bash
src/                      # Vue 3 前端代码
  App.vue                 # 主应用组件（面试流程控制）
  main.ts                 # 前端入口
  vite-env.d.ts           # TypeScript 类型声明
  components/             # Vue 组件
    ResumeInput.vue       # 简历输入组件
    JobDescription.vue    # 岗位描述输入组件
    QuestionList.vue      # 问题列表展示组件
    InterviewHistory.vue  # 历史记录查看组件
    QuestionBank.vue      # 题库管理组件
    VoiceControls.vue     # 语音控制组件
  services/               # 服务层
    database.ts           # 数据库服务接口
    voice.ts              # 语音服务（TTS/ASR）
src-tauri/                # Rust 后端代码
  src/
    lib.rs                # Tauri 应用入口和命令定义
    main.rs               # Rust 程序入口
    api/                  # API 模块
      mod.rs              # 模块入口
      siliconflow.rs      # 硅基流动 API 客户端
    db/                   # 数据库模块
      mod.rs              # 模块入口
      models.rs           # 数据模型定义
      schema.rs           # 表结构和初始化
      repository.rs       # 数据访问层
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

- **应用框架**: Tauri 2.9.5
- **后端语言**: Rust (edition 2021)
- **前端框架**: Vue 3.5 + TypeScript
- **构建工具**: Vite 7.2
- **代码规范**: ESLint + Prettier
- **AI 能力**: 硅基流动 API (Qwen/Qwen3-8B)
- **数据库**: SQLite + rusqlite
- **语音能力**: Web Speech API (TTS + ASR)
- **HTTP 客户端**: reqwest
- **异步运行时**: tokio

## 功能特性

### 已实现

- **基础框架** (Phase 1): Tauri + Rust + Vue 3 跨平台架构
- **模拟面试** (Phase 2): 基于简历和 JD 生成面试问题
- **AI 反馈** (Phase 2): 分析用户答案，提供改进建议
- **完整流程** (Phase 2): 输入 → 生成问题 → 回答 → 获得反馈
- **数据持久化** (Phase 3): SQLite 本地存储，自动保存面试记录
- **历史记录** (Phase 3): 查看过往面试会话和答题详情
- **题库管理** (Phase 3): 收藏问题、添加最佳答案、分类管理
- **语音交互** (Phase 4): AI 语音发问、语音回答、语音反馈

### 待开发

- **复盘分析** (Phase 5): 多维度评估和可视化报告
