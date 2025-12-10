# InterviewSpark

基于 Tauri + Rust + Vue 3 的 AI 模拟面试桌面应用。

## 环境要求

- Node.js >= 18
- Rust >= 1.77
- Tauri CLI (`cargo install tauri-cli`)

## 安装依赖

```bash
npm install
```

## 开发

```bash
# 仅运行前端
npm run dev

# 运行完整 Tauri 应用
npm run tauri dev
```

## 构建

```bash
# 构建前端
npm run build

# 构建发布版应用
npm run tauri build
```

## 项目结构

```
src/           # Vue 3 前端
src-tauri/     # Rust 后端
  src/         # Rust 源码
  Cargo.toml   # Rust 依赖配置
```
