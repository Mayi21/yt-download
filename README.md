# YouTube Downloader Desktop

<div align="center">

![YouTube Downloader](https://img.shields.io/badge/YouTube-Downloader-red?style=for-the-badge&logo=youtube)
![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri)
![React](https://img.shields.io/badge/React-18-blue?style=for-the-badge&logo=react)
![TypeScript](https://img.shields.io/badge/TypeScript-5-blue?style=for-the-badge&logo=typescript)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)

A modern, cross-platform YouTube video downloader built with Tauri, React, and TypeScript.

[English](README.md) | [中文](README_zh.md)

</div>

## ✨ Features

- 🎥 **High-Quality Downloads**: Support for up to 4K (2160p) video downloads
- 🌈 **HDR Support**: Download HDR videos with enhanced color depth
- 🎵 **Audio-Only Mode**: Extract high-quality audio tracks
- 📱 **Multiple Formats**: MP4, WebM, MKV support
- 🔄 **DASH Format Merging**: Automatic video and audio stream combination
- 📝 **Subtitle Support**: Download videos with embedded subtitles
- 🌍 **Internationalization**: English and Chinese language support
- 📊 **Real-time Progress**: Live download progress with speed and ETA
- 💾 **Smart Defaults**: Configurable default save locations
- 🖥️ **Cross-Platform**: Windows, macOS, and Linux support

## 🚀 Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) binary

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/yt-dlp-desktop.git
   cd yt-dlp-desktop
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Setup yt-dlp binary**
   - Download the latest yt-dlp binary from [releases](https://github.com/yt-dlp/yt-dlp/releases)
   - Place it in `src-tauri/bin/` directory
   - Make it executable (Linux/macOS): `chmod +x src-tauri/bin/yt-dlp`

4. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

5. **Build for production**
   ```bash
   npm run tauri build
   ```

## 📖 Usage

1. **Enter YouTube URL**: Paste any YouTube video URL
2. **Select Quality**: Choose from available resolutions (144p to 4K)
3. **Choose Format**: Select MP4, WebM, or MKV
4. **Configure Options**: 
   - Audio-only downloads
   - Subtitle inclusion
   - HDR preference
5. **Set Save Location**: Choose where to save your downloads
6. **Start Download**: Monitor real-time progress

## 🛠️ Configuration

The app automatically saves your preferences:
- Default save location
- Language preference
- Download settings

Configuration is stored in:
- **Windows**: `%APPDATA%/yt-dlp-desktop/config.json`
- **macOS**: `~/Library/Application Support/yt-dlp-desktop/config.json`
- **Linux**: `~/.config/yt-dlp-desktop/config.json`

## 🏗️ Architecture

```
yt-dlp-desktop/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── services/          # API services
│   ├── types/             # TypeScript types
│   └── i18n/              # Internationalization
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── commands.rs    # Tauri commands
│   │   ├── config.rs      # Configuration management
│   │   ├── types.rs       # Rust types
│   │   └── ytdlp.rs       # yt-dlp integration
│   └── bin/               # yt-dlp binary
└── public/                # Static assets
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - The powerful YouTube downloader
- [Tauri](https://tauri.app/) - Build smaller, faster, and more secure desktop applications
- [React](https://reactjs.org/) - A JavaScript library for building user interfaces

---

<div align="center">

**Made with ❤️ by the community**

</div>