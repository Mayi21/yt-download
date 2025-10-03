# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

YouTube Downloader Desktop - A cross-platform YouTube video downloader built with Tauri 2.0, React 19, TypeScript, and Rust. Supports downloads up to 4K/HDR with DASH format merging, real-time progress tracking, and internationalization (English/Chinese).

## Development Commands

### Frontend (React + Vite)
```bash
cd yt-dlp-desktop
npm install              # Install dependencies
npm run dev             # Start Vite dev server only (port 5173)
npm run build           # Build frontend for production
```

### Tauri Application
```bash
cd yt-dlp-desktop
npm run tauri dev       # Run full app (builds Rust + starts Vite)
npm run tauri build     # Build production app with bundles
```

### Rust Backend
```bash
cd yt-dlp-desktop/src-tauri
cargo check             # Check Rust code
cargo test              # Run Rust tests
cargo clean             # Clean build artifacts
```

## Architecture

### Hybrid Application Structure
- **Frontend**: `yt-dlp-desktop/src/` - React 19 + TypeScript + Tailwind CSS 4
- **Backend**: `yt-dlp-desktop/src-tauri/src/` - Rust with Tauri 2.0
- **Binary Dependencies**: `yt-dlp-desktop/src-tauri/bin/` - yt-dlp, ffmpeg, ffprobe binaries

### Key Backend Modules (Rust)

**ytdlp.rs** - Core yt-dlp integration (806 lines)
- `get_ytdlp_path()`: Dynamic path resolution for dev/prod environments (macOS, Windows, Linux)
- `get_ffmpeg_path()`: ffmpeg binary location for video/audio merging
- `get_video_info(url)`: Fetches video metadata and all available formats via yt-dlp JSON output
- `download_video()`: Handles downloads with real-time progress parsing, DASH format merging, retry logic
- `parse_download_progress()`: Regex-based progress parsing from yt-dlp stdout
- `diagnose_network_issue()`: SSL/timeout error detection and recommendations
- `get_fallback_formats()`: Automatic format downgrade suggestions for failed downloads

**commands.rs** - Tauri command handlers (164 lines)
- Bridges frontend JavaScript to Rust backend functions
- All async functions exposed via `#[tauri::command]`
- Handles file dialogs, config persistence, download history (placeholder)

**types.rs** - Shared data structures (182 lines)
- `VideoInfo`, `VideoFormat`: Video metadata from yt-dlp
- `DownloadConfig`, `DownloadProgress`: Download state management
- `YtDlpOutput` deserialization from yt-dlp JSON with format filtering (removes storyboards, images)

**logger.rs** - Application logging system (9.3KB)
- Singleton logger with file rotation (10MB limit, 5 backups)
- Log levels: debug, info, warn, error
- Log path: `~/Library/Application Support/yt-dlp-desktop/logs/app.log` (macOS)

**config.rs** - User preferences storage (2.2KB)
- Saves default download path, language preference
- Config path: `~/Library/Application Support/yt-dlp-desktop/config.json` (macOS)

### Key Frontend Components

**App.tsx** - Main application logic (6.3KB)
- Video info fetching and state management
- Download orchestration with progress event listeners
- Format selection and save path handling

**FormatSelector.tsx** - Format/quality picker (14.6KB)
- Groups formats by quality (4K, 1440p, 1080p, 720p, etc.)
- Displays HDR, FPS, codec information
- Handles DASH format merging logic display

**DownloadProgress.tsx** - Progress visualization (4.8KB)
- Real-time speed, ETA, percentage display
- Status indicators: downloading, processing (merging), finished, error
- File size formatting (bytes to MB/GB)

**i18n/** - Internationalization
- English: `src/i18n/locales/en.json`
- Chinese: `src/i18n/locales/zh.json`
- Auto-detection with `i18next-browser-languagedetector`

## Binary Path Resolution Strategy

**Development Mode** (`#[cfg(debug_assertions)]`):
- Hardcoded absolute paths (e.g., `/Users/liuge/project/yt-download/yt-dlp-desktop/src-tauri/bin/yt-dlp`)
- When working in dev, ensure binaries exist at these paths

**Production Mode**:
- macOS: `.app/Contents/Resources/bin/yt-dlp` (relative to exe in `Contents/MacOS/`)
- Windows: `bin/yt-dlp.exe` (relative to exe)
- Linux: `bin/yt-dlp` (relative to exe)

**tauri.conf.json resources**:
```json
"resources": ["bin/yt-dlp", "bin/ffmpeg", "bin/ffprobe"]
```

## DASH Format Merging

YouTube uses DASH (Dynamic Adaptive Streaming over HTTP) for high-quality videos (720p+). Video and audio streams are separate and require merging:

1. **Format ID Pattern**: `video_format_id+audio_format_id` (e.g., `315+258` for 4K VP9 + AAC)
2. **Merge Trigger**: Detected by `+` in format_id or when vcodec/acodec = "none"
3. **Merger**: yt-dlp uses ffmpeg to merge streams (configured via `--ffmpeg-location`)
4. **Progress States**: downloading → processing (merging) → finished

## Common Issues & Solutions

### SSL/Certificate Errors (4K downloads)
- ytdlp.rs includes `--no-check-certificates` flag
- Retry logic: 10 retries, linear backoff (1-5-10 seconds)
- Fallback formats auto-suggested via `get_fallback_formats()`

### Binary Not Found
- Check `get_ytdlp_path()` logs in app logs
- Verify binary exists in `src-tauri/bin/` for dev
- For production, ensure resources are bundled correctly

### Progress Not Updating
- Progress parsing uses regex in `parse_download_progress()`
- Requires `--newline --no-colors --progress` flags
- Check stdout parsing in Rust async task

## Testing

### Manual Testing Scripts
- `test_download.sh` - Test downloads with various formats
- `test_ssl_fix.sh` - Validate SSL retry logic
- `test_logger_performance.sh` - Log performance benchmarks
- `debug_formats.sh` - Inspect available formats for a video

### Rust Tests
- `ytdlp.rs::tests` - Network-dependent tests (marked `#[ignore]`)
- Run with `cargo test` in `src-tauri/`

## Build & Distribution

### macOS Build
```bash
cd yt-dlp-desktop
npm run tauri build
```
Output:
- DMG: `src-tauri/target/release/bundle/dmg/YouTube Downloader_1.0.0_aarch64.dmg`
- .app: `src-tauri/target/release/bundle/macos/YouTube Downloader.app`

### Signing (Optional)
Add to tauri.conf.json:
```json
"macOS": {
  "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
}
```

## Configuration Files

- **tauri.conf.json** - App metadata, window config, bundle settings
- **Cargo.toml** - Rust dependencies (tokio, serde, regex, chrono, env_logger)
- **package.json** - Frontend dependencies (React 19, i18next, Tauri API)
- **tsconfig.json** - TypeScript strict mode, React JSX transform

## Important Development Notes

1. **Always use ffmpeg-location**: All yt-dlp commands must include `--ffmpeg-location` to ensure DASH merging works in production
2. **Logging verbosity**: Use `AppLogger::get()` for Rust logs, avoid excessive debug prints (performance impact)
3. **Error handling**: Emit progress events with `status: "error"` for frontend error display
4. **Format filtering**: types.rs filters out useless formats (mhtml, storyboard, images) during deserialization
5. **Async operations**: All download operations use tokio async runtime with separate tasks for stdout/stderr parsing
