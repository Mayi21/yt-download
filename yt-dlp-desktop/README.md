# YouTube Downloader Desktop

基于 Tauri + React + TypeScript 构建的跨平台 YouTube 视频下载器。

## 🎯 特性

- ✅ 跨平台支持（macOS, Windows）
- ✅ 现代化 UI 界面
- ✅ 支持多种清晰度（4K, 1080p, 720p, 480p, 360p）
- ✅ 支持多种格式（MP4, WebM, MKV）
- ✅ 实时下载进度显示
- ✅ 仅音频下载选项
- ✅ 字幕下载支持

## 📋 前置要求

### 前端开发（只需要 Node.js）
- Node.js 18+
- npm 或 pnpm

### 后端开发（需要 Rust）
- Rust 1.70+
- yt-dlp（将被打包进应用）

## 🚀 快速开始

### 安装依赖

```bash
npm install
```

### 前端开发（使用 Mock 数据，不需要 Rust）

```bash
# 确保 .env.development 中 VITE_USE_MOCK=true
npm run dev
```

这将启动前端开发服务器，使用 Mock 数据，前端开发者可以独立开发 UI。

访问：http://localhost:5173

### 全栈开发（需要 Rust）

```bash
# 确保 .env.development 中 VITE_USE_MOCK=false
npm run tauri dev
```

这将启动完整的 Tauri 应用，前后端都会运行。

## 🔄 并行开发指南

本项目支持前后端并行开发！

### 前端开发者

**环境配置：**
- 设置 `.env.development` 中 `VITE_USE_MOCK=true`
- 运行 `npm run dev`
- 使用 Mock 数据开发 UI
- 无需安装 Rust 和 yt-dlp

**开发任务：**
- UI 组件优化
- 交互逻辑完善
- 响应式布局
- 深色模式支持

### 后端开发者

**环境配置：**
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 进入后端目录
cd src-tauri

# 运行测试
cargo test
```

**开发任务：**
- 实现 Tauri Commands
- 封装 yt-dlp 调用
- 解析下载进度
- 错误处理

## 📁 项目结构

```
yt-dlp-desktop/
├── src/                      # 前端代码
│   ├── components/           # React 组件
│   ├── services/             # API 和服务
│   ├── types/                # TypeScript 类型定义
│   └── App.tsx               # 主应用组件
│
├── src-tauri/                # Rust 后端
│   ├── src/
│   │   ├── main.rs           # 主入口
│   │   └── commands.rs       # Tauri 命令实现
│   └── Cargo.toml            # Rust 依赖
│
├── docs/                     # 技术文档
│   └── YouTube下载器技术方案.md
│
├── .env.development          # 开发环境变量
└── README.md                 # 本文档
```

## 📦 构建

### 开发构建
```bash
npm run tauri build
```

### 为特定平台构建

**macOS:**
```bash
npm run tauri build -- --target universal-apple-darwin
```

**Windows:**
```bash
npm run tauri build -- --target x86_64-pc-windows-msvc
```

## 🛠️ 技术栈

- React 18 + TypeScript
- TailwindCSS
- Tauri (Rust)
- yt-dlp

## 📚 相关资源

- [完整技术文档](../docs/YouTube下载器技术方案.md)
- [Tauri 文档](https://tauri.app/)
- [React 文档](https://react.dev/)
- [yt-dlp GitHub](https://github.com/yt-dlp/yt-dlp)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
