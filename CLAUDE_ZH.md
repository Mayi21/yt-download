# CLAUDE_ZH.md

本文件为 Claude Code (claude.ai/code) 在此代码库中工作时提供指导。

## 项目概述

YouTube Downloader Desktop - 一个跨平台的 YouTube 视频下载器，使用 Tauri 2.0、React 19、TypeScript 和 Rust 构建。支持最高 4K/HDR 下载、DASH 格式合并、实时进度跟踪和国际化（英文/中文）。

## 开发命令

### 前端 (React + Vite)
```bash
cd yt-dlp-desktop
npm install              # 安装依赖
npm run dev             # 仅启动 Vite 开发服务器（端口 5173）
npm run build           # 构建生产环境前端
```

### Tauri 应用
```bash
cd yt-dlp-desktop
npm run tauri dev       # 运行完整应用（构建 Rust + 启动 Vite）
npm run tauri build     # 构建生产环境应用及安装包
```

### Rust 后端
```bash
cd yt-dlp-desktop/src-tauri
cargo check             # 检查 Rust 代码
cargo test              # 运行 Rust 测试
cargo clean             # 清理构建产物
```

## 架构

### 混合应用结构
- **前端**: `yt-dlp-desktop/src/` - React 19 + TypeScript + Tailwind CSS 4
- **后端**: `yt-dlp-desktop/src-tauri/src/` - Rust with Tauri 2.0
- **二进制依赖**: `yt-dlp-desktop/src-tauri/bin/` - yt-dlp、ffmpeg、ffprobe 二进制文件

### 核心后端模块 (Rust)

**ytdlp.rs** - yt-dlp 核心集成 (806 行)
- `get_ytdlp_path()`: 开发/生产环境动态路径解析（macOS、Windows、Linux）
- `get_ffmpeg_path()`: 视频/音频合并所需的 ffmpeg 二进制位置
- `get_video_info(url)`: 通过 yt-dlp JSON 输出获取视频元数据和所有可用格式
- `download_video()`: 处理下载，包括实时进度解析、DASH 格式合并、重试逻辑
- `parse_download_progress()`: 基于正则表达式从 yt-dlp stdout 解析进度
- `diagnose_network_issue()`: SSL/超时错误检测和建议
- `get_fallback_formats()`: 下载失败时自动建议备用格式

**commands.rs** - Tauri 命令处理器 (164 行)
- 将前端 JavaScript 桥接到 Rust 后端函数
- 所有异步函数通过 `#[tauri::command]` 暴露
- 处理文件对话框、配置持久化、下载历史（占位符）

**types.rs** - 共享数据结构 (182 行)
- `VideoInfo`、`VideoFormat`: 来自 yt-dlp 的视频元数据
- `DownloadConfig`、`DownloadProgress`: 下载状态管理
- `YtDlpOutput` 从 yt-dlp JSON 反序列化，带格式过滤（移除故事板、图片）

**logger.rs** - 应用日志系统 (9.3KB)
- 单例日志记录器，支持文件轮转（10MB 限制，5 个备份）
- 日志级别：debug、info、warn、error
- 日志路径：`~/Library/Application Support/yt-dlp-desktop/logs/app.log` (macOS)

**config.rs** - 用户偏好存储 (2.2KB)
- 保存默认下载路径、语言偏好
- 配置路径：`~/Library/Application Support/yt-dlp-desktop/config.json` (macOS)

### 核心前端组件

**App.tsx** - 主应用逻辑 (6.3KB)
- 视频信息获取和状态管理
- 下载编排与进度事件监听
- 格式选择和保存路径处理

**FormatSelector.tsx** - 格式/质量选择器 (14.6KB)
- 按质量分组格式（4K、1440p、1080p、720p 等）
- 显示 HDR、FPS、编解码器信息
- 处理 DASH 格式合并逻辑显示

**DownloadProgress.tsx** - 进度可视化 (4.8KB)
- 实时速度、ETA、百分比显示
- 状态指示器：下载中、处理中（合并）、已完成、错误
- 文件大小格式化（字节转 MB/GB）

**i18n/** - 国际化
- 英文：`src/i18n/locales/en.json`
- 中文：`src/i18n/locales/zh.json`
- 使用 `i18next-browser-languagedetector` 自动检测

## 二进制路径解析策略

**开发模式** (`#[cfg(debug_assertions)]`):
- 硬编码绝对路径（例如：`/Users/liuge/project/yt-download/yt-dlp-desktop/src-tauri/bin/yt-dlp`）
- 开发时确保二进制文件存在于这些路径

**生产模式**:
- macOS: `.app/Contents/Resources/bin/yt-dlp`（相对于 `Contents/MacOS/` 中的可执行文件）
- Windows: `bin/yt-dlp.exe`（相对于可执行文件）
- Linux: `bin/yt-dlp`（相对于可执行文件）

**tauri.conf.json 资源配置**:
```json
"resources": ["bin/yt-dlp", "bin/ffmpeg", "bin/ffprobe"]
```

## DASH 格式合并

YouTube 对高质量视频（720p+）使用 DASH（基于 HTTP 的动态自适应流）。视频和音频流是分离的，需要合并：

1. **格式 ID 模式**: `视频格式ID+音频格式ID`（例如：`315+258` 表示 4K VP9 + AAC）
2. **合并触发**: 通过 format_id 中的 `+` 检测，或当 vcodec/acodec = "none" 时
3. **合并器**: yt-dlp 使用 ffmpeg 合并流（通过 `--ffmpeg-location` 配置）
4. **进度状态**: 下载中 → 处理中（合并）→ 已完成

## 常见问题与解决方案

### SSL/证书错误（4K 下载）
- ytdlp.rs 包含 `--no-check-certificates` 标志
- 重试逻辑：10 次重试，线性退避（1-5-10 秒）
- 通过 `get_fallback_formats()` 自动建议备用格式

### 找不到二进制文件
- 检查应用日志中的 `get_ytdlp_path()` 日志
- 验证开发环境下二进制文件存在于 `src-tauri/bin/`
- 生产环境确保资源正确打包

### 进度不更新
- 进度解析使用 `parse_download_progress()` 中的正则表达式
- 需要 `--newline --no-colors --progress` 标志
- 检查 Rust 异步任务中的 stdout 解析

## 测试

### 手动测试脚本
- `test_download.sh` - 测试各种格式的下载
- `test_ssl_fix.sh` - 验证 SSL 重试逻辑
- `test_logger_performance.sh` - 日志性能基准测试
- `debug_formats.sh` - 检查视频的可用格式

### Rust 测试
- `ytdlp.rs::tests` - 依赖网络的测试（标记为 `#[ignore]`）
- 在 `src-tauri/` 中使用 `cargo test` 运行

## 构建与分发

### macOS 构建
```bash
cd yt-dlp-desktop
npm run tauri build
```
输出：
- DMG: `src-tauri/target/release/bundle/dmg/YouTube Downloader_1.0.0_aarch64.dmg`
- .app: `src-tauri/target/release/bundle/macos/YouTube Downloader.app`

### 签名（可选）
添加到 tauri.conf.json：
```json
"macOS": {
  "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
}
```

## 配置文件

- **tauri.conf.json** - 应用元数据、窗口配置、打包设置
- **Cargo.toml** - Rust 依赖（tokio、serde、regex、chrono、env_logger）
- **package.json** - 前端依赖（React 19、i18next、Tauri API）
- **tsconfig.json** - TypeScript 严格模式、React JSX 转换

## 重要开发注意事项

1. **始终使用 ffmpeg-location**: 所有 yt-dlp 命令必须包含 `--ffmpeg-location`，以确保 DASH 合并在生产环境中正常工作
2. **日志详细程度**: 使用 `AppLogger::get()` 记录 Rust 日志，避免过多的调试打印（性能影响）
3. **错误处理**: 使用 `status: "error"` 发出进度事件，用于前端错误显示
4. **格式过滤**: types.rs 在反序列化期间过滤无用格式（mhtml、故事板、图片）
5. **异步操作**: 所有下载操作使用 tokio 异步运行时，为 stdout/stderr 解析使用单独的任务
