# YouTube Downloader Desktop

<div align="center">

![YouTube Downloader](https://img.shields.io/badge/YouTube-Downloader-red?style=for-the-badge&logo=youtube)
![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri)
![React](https://img.shields.io/badge/React-19-blue?style=for-the-badge&logo=react)
![TypeScript](https://img.shields.io/badge/TypeScript-5-blue?style=for-the-badge&logo=typescript)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)

A modern YouTube video downloader built with Tauri, React, and TypeScript.

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

## 📦 Download

### Prebuilt Releases

Download the latest version from [Releases](https://github.com/Mayi21/yt-download/releases):

**Current Version: v1.0.0**
- **macOS (Apple Silicon)**: [YouTube Downloader_1.0.0_aarch64.dmg](https://github.com/Mayi21/yt-download/releases/download/v1.0.0/YouTube.Downloader_1.0.0_aarch64.dmg)

### Platform Support

**v1.0.0 Status:**
- ✅ macOS (Apple Silicon - M1/M2/M3/M4)
- ⏳ macOS (Intel) - Planned
- ⏳ Windows - Planned
- ⏳ Linux - Planned

## 🚀 Build from Source

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Mayi21/yt-download.git
   cd yt-download/yt-dlp-desktop
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

4. **Build for production**
   ```bash
   npm run tauri build
   ```

> **Note**: The required binaries (yt-dlp, ffmpeg, ffprobe) are included in the repository under `src-tauri/bin/`.

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
- **macOS**: `~/Library/Application Support/com.youtube-downloader.desktop/config.json`

## 🏗️ Architecture

```
yt-download/                # Repository root
└── yt-dlp-desktop/        # Main project directory
    ├── src/               # React frontend
    │   ├── components/    # UI components
    │   ├── services/      # API services
    │   ├── types/         # TypeScript types
    │   └── i18n/          # Internationalization
    ├── src-tauri/         # Rust backend
    │   ├── src/
    │   │   ├── commands.rs    # Tauri commands
    │   │   ├── config.rs      # Configuration management
    │   │   ├── types.rs       # Rust types
    │   │   ├── ytdlp.rs       # yt-dlp integration
    │   │   └── logger.rs      # Logging system
    │   └── bin/               # Binaries (yt-dlp, ffmpeg, ffprobe)
    └── public/            # Static assets
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