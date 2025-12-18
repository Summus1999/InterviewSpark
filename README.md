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

3. **编译测试**
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
src/                 # Vue 3 前端代码
  App.vue           # 主应用组件
  main.ts           # 前端入口
  vite-env.d.ts     # TypeScript 类型声明
src-tauri/          # Rust 后端代码
  src/
    lib.rs          # Tauri 应用入口和命令定义
    main.rs         # Rust 程序入口
  Cargo.toml        # Rust 依赖配置
  tauri.conf.json   # Tauri 应用配置
DevPlan.md          # 开发阶段计划
ProductionDoc.md    # 产品文档
```

## 技术栈

- **应用框架**: Tauri 2.9.5
- **后端语言**: Rust (edition 2021)
- **前端框架**: Vue 3.5 + TypeScript
- **构建工具**: Vite 7.2
- **代码规范**: ESLint + Prettier
