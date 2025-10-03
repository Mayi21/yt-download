# YouTube 下载器桌面版

<div align="center">

![YouTube Downloader](https://img.shields.io/badge/YouTube-Downloader-red?style=for-the-badge&logo=youtube)
![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri)
![React](https://img.shields.io/badge/React-19-blue?style=for-the-badge&logo=react)
![TypeScript](https://img.shields.io/badge/TypeScript-5-blue?style=for-the-badge&logo=typescript)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)

基于 Tauri、React 和 TypeScript 构建的现代化 YouTube 视频下载器。

[English](README.md) | [中文](README_zh.md)

</div>

## ✨ 功能特性

- 🎥 **高质量下载**: 支持最高 4K (2160p) 视频下载
- 🌈 **HDR 支持**: 下载具有增强色彩深度的 HDR 视频
- 🎵 **纯音频模式**: 提取高质量音频轨道
- 📱 **多种格式**: 支持 MP4、WebM、MKV 格式
- 🔄 **DASH 格式合并**: 自动合并视频和音频流
- 📝 **字幕支持**: 下载带有嵌入字幕的视频
- 🌍 **国际化**: 支持英文和中文界面
- 📊 **实时进度**: 实时显示下载进度、速度和预计时间
- 💾 **智能默认**: 可配置的默认保存位置

## 📦 下载安装

### 预编译版本

从 [Releases](https://github.com/Mayi21/yt-download/releases) 下载最新版本：

**当前版本：v1.0.0**
- **macOS (Apple Silicon)**: [YouTube Downloader_1.0.0_aarch64.dmg](https://github.com/Mayi21/yt-download/releases/download/v1.0.0/YouTube.Downloader_1.0.0_aarch64.dmg)

### 平台支持

**v1.0.0 状态：**
- ✅ macOS (Apple Silicon - M1/M2/M3/M4)
- ⏳ macOS (Intel) - 计划中
- ⏳ Windows - 计划中
- ⏳ Linux - 计划中

## 🚀 从源码构建

### 环境要求

- [Node.js](https://nodejs.org/) (v16 或更高版本)
- [Rust](https://rustup.rs/) (最新稳定版)

### 安装步骤

1. **克隆仓库**
   ```bash
   git clone https://github.com/Mayi21/yt-download.git
   cd yt-download/yt-dlp-desktop
   ```

2. **安装依赖**
   ```bash
   npm install
   ```

3. **开发模式运行**
   ```bash
   npm run tauri dev
   ```

4. **生产环境构建**
   ```bash
   npm run tauri build
   ```

> **注意**：所需的二进制文件（yt-dlp、ffmpeg、ffprobe）已包含在仓库的 `src-tauri/bin/` 目录中。

## 📖 使用方法

1. **输入 YouTube 链接**: 粘贴任意 YouTube 视频链接
2. **选择清晰度**: 从可用分辨率中选择 (144p 到 4K)
3. **选择格式**: 选择 MP4、WebM 或 MKV
4. **配置选项**: 
   - 仅音频下载
   - 包含字幕
   - HDR 偏好
5. **设置保存位置**: 选择下载文件的保存位置
6. **开始下载**: 监控实时下载进度

## 🛠️ 配置

应用会自动保存您的偏好设置：
- 默认保存位置
- 语言偏好
- 下载设置

配置文件存储位置：
- **macOS**: `~/Library/Application Support/com.youtube-downloader.desktop/config.json`

## 🏗️ 项目架构

```
yt-download/                # 仓库根目录
└── yt-dlp-desktop/        # 主项目目录
    ├── src/               # React 前端
    │   ├── components/    # UI 组件
    │   ├── services/      # API 服务
    │   ├── types/         # TypeScript 类型
    │   └── i18n/          # 国际化
    ├── src-tauri/         # Rust 后端
    │   ├── src/
    │   │   ├── commands.rs    # Tauri 命令
    │   │   ├── config.rs      # 配置管理
    │   │   ├── types.rs       # Rust 类型
    │   │   ├── ytdlp.rs       # yt-dlp 集成
    │   │   └── logger.rs      # 日志系统
    │   └── bin/               # 二进制文件 (yt-dlp, ffmpeg, ffprobe)
    └── public/            # 静态资源
```

## 🤝 贡献

1. Fork 本仓库
2. 创建功能分支: `git checkout -b feature/amazing-feature`
3. 提交更改: `git commit -m 'Add amazing feature'`
4. 推送到分支: `git push origin feature/amazing-feature`
5. 创建 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - 强大的 YouTube 下载工具
- [Tauri](https://tauri.app/) - 构建更小、更快、更安全的桌面应用
- [React](https://reactjs.org/) - 用于构建用户界面的 JavaScript 库

---

<div align="center">

**用 ❤️ 由社区制作**

</div>