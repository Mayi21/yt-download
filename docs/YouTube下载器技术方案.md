# YouTube 下载器桌面应用技术方案

## 目录
- [项目概述](#项目概述)
- [技术栈选型](#技术栈选型)
- [项目架构设计](#项目架构设计)
- [核心功能设计](#核心功能设计)
- [接口定义](#接口定义)
- [并行开发指南](#并行开发指南)
- [实施计划](#实施计划)

---

## 项目概述

### 目标
基于现有的 yt-dlp 项目，开发一个跨平台的桌面应用，支持 macOS 和 Windows 系统。

### 核心需求
- ✅ 支持输入 YouTube 链接
- ✅ 展示视频信息（标题、时长、缩略图等）
- ✅ 可选择视频格式（mp4、webm、mkv 等）
- ✅ 可选择清晰度（4K、1080p、720p、480p 等）
- ✅ 实时显示下载进度
- ✅ 可视化界面，用户体验友好
- ✅ 跨平台打包（.app for macOS, .exe for Windows）

---

## 技术栈选型

### 方案对比

#### 1. Python + PyQt6
**优势：**
- 原生集成 yt-dlp Python 库
- 跨平台支持
- 打包简单（PyInstaller）

**劣势：**
- ❌ UI 不够现代化
- ❌ 打包体积较大（50MB+）
- ❌ Python 依赖管理复杂

#### 2. Electron + React
**优势：**
- 最成熟的跨平台方案
- Web 技术栈，开发快速
- 生态完善

**劣势：**
- ❌ 体积巨大（100MB+）
- ❌ 内存占用高
- ❌ 启动速度慢

#### 3. Tauri + React ⭐ **最终选择**
**优势：**
- ✅ 体积小（5-8MB）
- ✅ 内存占用低
- ✅ 启动速度快
- ✅ 前端使用 React，开发体验好
- ✅ Rust 后端安全性高
- ✅ 完美的跨平台支持

**劣势：**
- 需要基础的 Rust 知识（但已有模板）

#### 4. Flutter Desktop
**优势：**
- 单一代码库
- 性能优秀
- UI 美观

**劣势：**
- ❌ 需要学习 Dart 语言
- ❌ 桌面端生态相对不成熟

### 最终选型：Tauri + React + TypeScript

**理由：**
1. **体积优势**：仅 5-8MB（Electron 的 1/15）
2. **性能优越**：内存占用约 Electron 的 1/3
3. **开发体验**：前端使用熟悉的 React 技术栈
4. **易于维护**：TypeScript 提供类型安全
5. **完美跨平台**：macOS 和 Windows 原生支持
6. **易于集成**：通过命令行调用 yt-dlp

---

## 项目架构设计

### 目录结构

```
yt-dlp-desktop/
├── src/                          # 前端代码（React + TypeScript）
│   ├── App.tsx                   # 主组件
│   ├── components/
│   │   ├── UrlInput.tsx          # URL 输入框
│   │   ├── VideoInfo.tsx         # 视频信息展示
│   │   ├── FormatSelector.tsx    # 格式/清晰度选择
│   │   ├── DownloadProgress.tsx  # 下载进度条
│   │   └── DownloadHistory.tsx   # 下载历史
│   ├── services/
│   │   ├── api.ts                # Tauri API 调用
│   │   └── mock.ts               # Mock 数据（开发用）
│   ├── types/
│   │   ├── index.ts              # TypeScript 类型定义
│   │   └── commands.ts           # Tauri Commands 类型
│   ├── styles/
│   │   └── App.css               # 全局样式
│   ├── utils/
│   │   └── helpers.ts            # 工具函数
│   └── main.tsx                  # 入口文件
│
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs               # 主入口
│   │   ├── commands.rs           # Tauri Commands
│   │   ├── ytdlp.rs              # yt-dlp 封装模块
│   │   └── utils.rs              # 工具函数
│   ├── Cargo.toml                # Rust 依赖配置
│   ├── tauri.conf.json           # Tauri 应用配置
│   └── icons/                    # 应用图标
│       ├── icon.icns             # macOS 图标
│       ├── icon.ico              # Windows 图标
│       └── icon.png              # 通用图标
│
├── public/                       # 静态资源
│   └── vite.svg
│
├── package.json                  # Node.js 依赖
├── tsconfig.json                 # TypeScript 配置
├── vite.config.ts                # Vite 构建配置
├── tailwind.config.js            # TailwindCSS 配置
├── .env.development              # 开发环境变量
├── .env.production               # 生产环境变量
└── README.md                     # 项目说明
```

### 技术栈详细说明

#### 前端
- **React 18** - UI 框架
- **TypeScript** - 类型安全
- **Vite** - 构建工具（快速热重载）
- **TailwindCSS** - 样式框架
- **@tauri-apps/api** - Tauri 前端 API
- **React Hooks** - 状态管理

#### 后端
- **Rust** - Tauri 后端语言
- **Tauri** - 跨平台框架
- **serde** - JSON 序列化
- **tokio** - 异步运行时
- **yt-dlp** - 通过命令行调用

#### 开发工具
- **ESLint** - 代码检查
- **Prettier** - 代码格式化
- **Cargo** - Rust 包管理器
- **npm/pnpm** - Node.js 包管理器

---

## 核心功能设计

### 1. URL 输入区域

**组件：** `UrlInput.tsx`

**功能：**
- 输入框支持粘贴 YouTube 链接
- URL 格式验证（支持多种 YouTube URL 格式）
- "获取信息" 按钮
- 加载状态显示（Spinner）
- 错误提示

**UI 设计：**
```
┌─────────────────────────────────────────────┐
│ 🔗 粘贴 YouTube 链接                        │
│ ┌─────────────────────────────────────────┐ │
│ │ https://www.youtube.com/watch?v=...    │ │
│ └─────────────────────────────────────────┘ │
│                              [获取信息 →]    │
└─────────────────────────────────────────────┘
```

### 2. 视频信息展示

**组件：** `VideoInfo.tsx`

**展示内容：**
- 视频缩略图（可点击预览）
- 视频标题
- 视频时长
- 上传者名称
- 上传日期
- 视频描述（可折叠）
- 观看次数

**UI 设计：**
```
┌─────────────────────────────────────────────┐
│ ┌────────┐  标题: Amazing Video             │
│ │        │  时长: 10:25                      │
│ │ 缩略图 │  上传者: Channel Name             │
│ │        │  日期: 2024-01-15                 │
│ └────────┘  观看: 1.2M views                 │
└─────────────────────────────────────────────┘
```

### 3. 格式选择器

**组件：** `FormatSelector.tsx`

**选项：**

**清晰度选择：**
- ⚡ 4K (2160p)
- 🎬 1080p（推荐）
- 📺 720p
- 📱 480p
- 🔽 360p

**格式选择：**
- MP4（推荐）
- WebM
- MKV

**特殊选项：**
- [ ] 仅音频（最佳音质）
- [ ] 包含字幕

**UI 设计：**
```
┌─────────────────────────────────────────────┐
│ 清晰度                                       │
│ (•) 1080p  ( ) 720p  ( ) 480p  ( ) 360p    │
│                                              │
│ 格式                                         │
│ ▼ MP4       ▼                                │
│                                              │
│ ☐ 仅音频    ☐ 包含字幕                       │
└─────────────────────────────────────────────┘
```

### 4. 保存路径选择

**功能：**
- 显示当前保存路径
- "浏览" 按钮打开文件选择器
- 默认路径：用户下载文件夹

**UI 设计：**
```
┌─────────────────────────────────────────────┐
│ 保存位置                                     │
│ ┌─────────────────────────────────────┐     │
│ │ /Users/username/Downloads           │[📁] │
│ └─────────────────────────────────────┘     │
└─────────────────────────────────────────────┘
```

### 5. 下载进度

**组件：** `DownloadProgress.tsx`

**显示内容：**
- 进度条（0-100%）
- 下载速度（MB/s）
- 已下载/总大小（125MB / 500MB）
- 预计剩余时间（ETA）
- 当前状态（准备中/下载中/合并中/完成）

**控制按钮：**
- ⏸️ 暂停
- ▶️ 继续
- ❌ 取消

**UI 设计：**
```
┌─────────────────────────────────────────────┐
│ 下载中...                                    │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 65%         │
│                                              │
│ 速度: 2.5 MB/s                               │
│ 已下载: 325MB / 500MB                        │
│ 剩余时间: 1 分 15 秒                         │
│                                              │
│      [⏸️ 暂停]  [❌ 取消]                    │
└─────────────────────────────────────────────┘
```

### 6. 下载历史

**组件：** `DownloadHistory.tsx`

**功能：**
- 显示最近下载的视频列表
- 支持重新下载
- 支持打开文件位置
- 支持清空历史

---

## 接口定义

### TypeScript 类型定义

```typescript
// src/types/index.ts

/**
 * 视频信息
 */
export interface VideoInfo {
  id: string;                    // 视频 ID
  title: string;                 // 标题
  duration: number;              // 时长（秒）
  thumbnail: string;             // 缩略图 URL
  uploader: string;              // 上传者
  upload_date: string;           // 上传日期 (YYYYMMDD)
  description: string;           // 描述
  view_count: number;            // 观看次数
  formats: VideoFormat[];        // 可用格式列表
}

/**
 * 视频格式
 */
export interface VideoFormat {
  format_id: string;             // 格式 ID
  ext: string;                   // 扩展名 (mp4, webm, mkv)
  resolution: string;            // 分辨率 (1920x1080)
  quality_label: string;         // 清晰度标签 (1080p, 720p)
  filesize: number | null;       // 文件大小（字节）
  fps: number;                   // 帧率
  vcodec: string;                // 视频编码
  acodec: string;                // 音频编码
  tbr: number;                   // 比特率
  format_note: string;           // 格式说明
}

/**
 * 下载进度
 */
export interface DownloadProgress {
  status: 'downloading' | 'processing' | 'finished' | 'error';
  percent: number;               // 进度百分比 (0-100)
  speed: number;                 // 下载速度（bytes/s）
  eta: number;                   // 预计剩余时间（秒）
  downloaded: number;            // 已下载字节数
  total: number;                 // 总字节数
  filename: string;              // 文件名
}

/**
 * 下载配置
 */
export interface DownloadConfig {
  url: string;                   // 视频 URL
  format_id: string;             // 选择的格式 ID
  output_path: string;           // 输出路径
  audio_only: boolean;           // 是否仅下载音频
  include_subtitles: boolean;    // 是否包含字幕
}

/**
 * 下载结果
 */
export interface DownloadResult {
  success: boolean;
  file_path: string;
  error?: string;
}

/**
 * 下载历史记录
 */
export interface DownloadHistoryItem {
  id: string;
  title: string;
  url: string;
  file_path: string;
  download_date: string;
  thumbnail: string;
}
```

### Tauri Commands 接口

```typescript
// src/types/commands.ts

/**
 * Tauri 后端命令接口
 */
export interface TauriCommands {
  /**
   * 获取视频信息
   * @param url YouTube 视频 URL
   * @returns 视频信息对象
   */
  get_video_info(url: string): Promise<VideoInfo>;

  /**
   * 列出可用格式
   * @param url YouTube 视频 URL
   * @returns 格式列表
   */
  list_formats(url: string): Promise<VideoFormat[]>;

  /**
   * 开始下载
   * @param config 下载配置
   * @returns 下载任务 ID
   */
  start_download(config: DownloadConfig): Promise<string>;

  /**
   * 取消下载
   * @param download_id 下载任务 ID
   */
  cancel_download(download_id: string): Promise<void>;

  /**
   * 选择保存路径
   * @returns 选择的路径，如果取消则返回 null
   */
  select_save_path(): Promise<string | null>;

  /**
   * 获取下载历史
   * @returns 历史记录列表
   */
  get_download_history(): Promise<DownloadHistoryItem[]>;

  /**
   * 清空下载历史
   */
  clear_download_history(): Promise<void>;

  /**
   * 打开文件位置
   * @param file_path 文件路径
   */
  open_file_location(file_path: string): Promise<void>;
}
```

### Rust Command 实现示例

```rust
// src-tauri/src/commands.rs

use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub duration: u32,
    pub thumbnail: String,
    pub uploader: String,
    pub upload_date: String,
    pub description: String,
    pub view_count: u64,
    pub formats: Vec<VideoFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoFormat {
    pub format_id: String,
    pub ext: String,
    pub resolution: String,
    pub quality_label: String,
    pub filesize: Option<u64>,
    pub fps: u32,
    pub vcodec: String,
    pub acodec: String,
    pub tbr: f64,
    pub format_note: String,
}

/// 获取视频信息
#[command]
pub async fn get_video_info(url: String) -> Result<VideoInfo, String> {
    crate::ytdlp::get_video_info(&url).await
}

/// 开始下载
#[command]
pub async fn start_download(
    config: DownloadConfig,
    window: tauri::Window,
) -> Result<String, String> {
    crate::ytdlp::download_video(config, window).await
}

/// 选择保存路径
#[command]
pub async fn select_save_path() -> Result<Option<String>, String> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    Ok(FileDialogBuilder::new()
        .set_title("选择保存位置")
        .pick_folder()
        .map(|p| p.to_string_lossy().to_string()))
}
```

### Mock 数据示例

```typescript
// src/services/mock.ts

export const mockVideoInfo: VideoInfo = {
  id: 'dQw4w9WgXcQ',
  title: 'Rick Astley - Never Gonna Give You Up (Official Video)',
  duration: 212,
  thumbnail: 'https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg',
  uploader: 'Rick Astley',
  upload_date: '20091025',
  description: 'The official video for "Never Gonna Give You Up"...',
  view_count: 1400000000,
  formats: [
    {
      format_id: '137',
      ext: 'mp4',
      resolution: '1920x1080',
      quality_label: '1080p',
      filesize: 52428800,
      fps: 30,
      vcodec: 'avc1.640028',
      acodec: 'mp4a.40.2',
      tbr: 2500,
      format_note: 'DASH video'
    },
    {
      format_id: '136',
      ext: 'mp4',
      resolution: '1280x720',
      quality_label: '720p',
      filesize: 32428800,
      fps: 30,
      vcodec: 'avc1.4d401f',
      acodec: 'mp4a.40.2',
      tbr: 1500,
      format_note: 'DASH video'
    },
    // ...更多格式
  ]
};

/**
 * 模拟下载进度
 */
export function mockDownloadProgress(
  callback: (progress: DownloadProgress) => void
): () => void {
  let percent = 0;
  const interval = setInterval(() => {
    percent += Math.random() * 5 + 2; // 每次增加 2-7%

    if (percent >= 100) {
      percent = 100;
      callback({
        status: 'finished',
        percent: 100,
        speed: 0,
        eta: 0,
        downloaded: 52428800,
        total: 52428800,
        filename: 'video.mp4'
      });
      clearInterval(interval);
      return;
    }

    callback({
      status: 'downloading',
      percent,
      speed: 1024 * 1024 * (2 + Math.random()), // 2-3 MB/s
      eta: Math.floor((100 - percent) * 0.8),
      downloaded: Math.floor(52428800 * percent / 100),
      total: 52428800,
      filename: 'video.mp4'
    });
  }, 500);

  // 返回清理函数
  return () => clearInterval(interval);
}
```

---

## 并行开发指南

### 开发流程

```
Step 1: 定义接口契约（1 小时）
    ↓
  ┌─────────────┬─────────────┬─────────────┐
  ↓             ↓             ↓             ↓
前端开发      后端开发      UI/UX设计    文档编写
(使用Mock)   (独立测试)   (设计规范)   (API文档)
  ↓             ↓             ↓             ↓
  └─────────────┴─────────────┴─────────────┘
              ↓
          集成测试
              ↓
          优化打包
```

### 前端开发指南

#### 环境配置

```bash
# 克隆项目
git clone <repository>
cd yt-dlp-desktop

# 安装依赖
npm install

# 启动开发服务器（使用 Mock 数据）
npm run dev
```

#### 使用 Mock 数据开发

创建 `.env.development` 文件：
```env
VITE_USE_MOCK=true
```

API 调用层自动切换：
```typescript
// src/services/api.ts
import { invoke } from '@tauri-apps/api/tauri';
import { mockVideoInfo, mockDownloadProgress } from './mock';

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true';

export async function getVideoInfo(url: string): Promise<VideoInfo> {
  if (USE_MOCK) {
    console.log('[Mock] Getting video info for:', url);
    await delay(1000); // 模拟网络延迟
    return mockVideoInfo;
  }
  return invoke('get_video_info', { url });
}

export function startDownload(
  config: DownloadConfig,
  onProgress: (progress: DownloadProgress) => void
): () => void {
  if (USE_MOCK) {
    console.log('[Mock] Starting download:', config);
    return mockDownloadProgress(onProgress);
  }

  // 真实实现：使用 Tauri Event 监听进度
  const unlisten = listen('download-progress', (event) => {
    onProgress(event.payload as DownloadProgress);
  });

  invoke('start_download', { config });

  return unlisten;
}

const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
```

#### 前端开发任务清单

- [ ] URL 输入组件（包含验证）
- [ ] 视频信息展示组件
- [ ] 格式选择器组件
- [ ] 下载进度组件
- [ ] 保存路径选择
- [ ] 下载历史列表
- [ ] 错误提示 Toast
- [ ] 响应式布局
- [ ] 深色模式支持
- [ ] 状态管理（React Context/Zustand）

### 后端开发指南

#### 环境配置

```bash
# 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Tauri CLI
cargo install tauri-cli

# 构建后端
cd src-tauri
cargo build

# 运行测试
cargo test
```

#### 单元测试开发

```rust
// src-tauri/src/ytdlp.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_video_info() {
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = get_video_info(url).await;

        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(!info.title.is_empty());
        assert!(info.duration > 0);
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let url = "https://invalid-url.com";
        let result = get_video_info(url).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_flow() {
        // 测试完整的下载流程
        let config = DownloadConfig {
            url: "https://www.youtube.com/watch?v=test".to_string(),
            format_id: "137".to_string(),
            output_path: "/tmp/test.mp4".to_string(),
            audio_only: false,
            include_subtitles: false,
        };

        // 这里可以使用测试用的短视频
        // let result = download_video(config).await;
        // assert!(result.is_ok());
    }
}
```

#### 后端开发任务清单

- [ ] yt-dlp 命令执行封装
- [ ] 视频信息解析（JSON 输出）
- [ ] 格式列表提取和过滤
- [ ] 下载进度实时解析
- [ ] 文件系统操作（保存、移动）
- [ ] 下载任务管理（暂停、取消）
- [ ] 错误处理和日志记录
- [ ] 跨平台兼容性（macOS/Windows）
- [ ] 单元测试覆盖

### UI/UX 设计指南

#### 设计规范

**色彩方案：**
```css
/* 主色调 */
--primary: #3B82F6;      /* 蓝色 */
--primary-hover: #2563EB;
--success: #10B981;       /* 绿色 */
--error: #EF4444;         /* 红色 */
--warning: #F59E0B;       /* 橙色 */

/* 中性色 */
--bg-primary: #FFFFFF;
--bg-secondary: #F9FAFB;
--text-primary: #111827;
--text-secondary: #6B7280;
--border: #E5E7EB;

/* 深色模式 */
--dark-bg-primary: #1F2937;
--dark-bg-secondary: #111827;
--dark-text-primary: #F9FAFB;
--dark-text-secondary: #9CA3AF;
--dark-border: #374151;
```

**字体：**
- 标题：Inter, -apple-system, system-ui
- 正文：-apple-system, BlinkMacSystemFont, "Segoe UI"
- 代码：Menlo, Monaco, "Courier New"

**间距：**
- 小：8px
- 中：16px
- 大：24px
- 超大：32px

#### 设计交付物

- [ ] 主界面设计稿（Figma/Sketch）
- [ ] 深色模式设计稿
- [ ] 应用图标（1024x1024 PNG）
- [ ] 状态图标（进度、错误、成功等）
- [ ] 交互流程图
- [ ] 组件库文档

### 集成测试清单

当前后端开发完成后，进行集成测试：

#### 功能测试
- [ ] URL 输入和验证
- [ ] 视频信息获取（多种 URL 格式）
- [ ] 格式列表显示正确
- [ ] 清晰度选择生效
- [ ] 下载进度实时更新
- [ ] 暂停/恢复功能
- [ ] 取消下载功能
- [ ] 文件保存到指定位置
- [ ] 下载完成通知
- [ ] 下载历史记录

#### 异常处理测试
- [ ] 无效 URL 错误提示
- [ ] 网络断开重试机制
- [ ] 磁盘空间不足提示
- [ ] 权限不足错误处理
- [ ] 并发下载限制

#### 性能测试
- [ ] 大文件下载（4K 视频）
- [ ] 多任务并发下载
- [ ] 内存占用监控
- [ ] CPU 使用率监控

#### 跨平台测试
- [ ] macOS 10.15+ 兼容性
- [ ] Windows 10+ 兼容性
- [ ] 高 DPI 屏幕显示
- [ ] 深色模式适配

---

## 实施计划

### 时间线（并行开发）

#### Week 1: 项目初始化和接口定义

**Day 1-2: 项目搭建**
- [x] 创建 Tauri + React 项目
- [x] 配置 TypeScript
- [x] 配置 TailwindCSS
- [x] 设置项目目录结构
- [x] 配置 Git 仓库

**Day 3: 接口定义（全员参与）**
- [x] 定义 TypeScript 类型
- [x] 定义 Tauri Commands
- [x] 编写 Mock 数据
- [x] API 文档编写

**Day 4-5: 环境配置**
- [ ] 下载 yt-dlp 二进制文件
- [ ] 配置 Rust 依赖
- [ ] 配置打包脚本
- [ ] 设置 CI/CD（可选）

---

#### Week 2: 并行开发阶段

**前端团队：**
- [x] URL 输入组件（Mock 数据）
- [ ] 视频信息展示组件
- [ ] 格式选择器组件
- [ ] 保存路径选择
- [ ] 基础样式和布局

**后端团队：**
- [ ] yt-dlp 命令封装
- [ ] 视频信息获取 Command
- [ ] 格式列表提取
- [ ] 单元测试编写

**UI/UX 团队：**
- [ ] 主界面设计稿
- [ ] 图标设计
- [ ] 交互流程设计

---

#### Week 3: 核心功能开发

**前端团队：**
- [ ] 下载进度组件
- [ ] 下载历史列表
- [ ] 错误提示系统
- [ ] 状态管理集成
- [ ] 响应式布局优化

**后端团队：**
- [ ] 下载功能 Command
- [ ] 进度解析和事件发送
- [ ] 暂停/取消功能
- [ ] 文件系统操作
- [ ] 错误处理

---

#### Week 4: 集成和优化

**Day 1-2: 集成测试**
- [ ] 前后端 API 对接
- [ ] 功能测试
- [ ] Bug 修复

**Day 3-4: 优化**
- [ ] 性能优化
- [ ] UI 细节调整
- [ ] 错误处理完善
- [ ] 用户体验优化

**Day 5: 打包测试**
- [ ] macOS 打包测试
- [ ] Windows 打包测试
- [ ] 安装包测试

---

#### Week 5: 测试和发布

**Day 1-3: 全面测试**
- [ ] 功能测试（全流程）
- [ ] 异常测试
- [ ] 跨平台测试
- [ ] 性能测试

**Day 4: 文档编写**
- [ ] 用户手册
- [ ] 安装指南
- [ ] FAQ 文档

**Day 5: 发布**
- [ ] 创建 Release 版本
- [ ] 上传安装包
- [ ] 发布公告

---

### 里程碑

| 里程碑 | 时间 | 交付物 |
|--------|------|--------|
| M1: 项目初始化 | Week 1 | 项目框架、接口文档 |
| M2: 基础功能 | Week 2 | 前端 UI、后端 API |
| M3: 核心功能 | Week 3 | 下载功能完整实现 |
| M4: 集成优化 | Week 4 | 可用的应用程序 |
| M5: 正式发布 | Week 5 | 正式版本 v1.0.0 |

---

### 风险评估

#### 技术风险

**1. yt-dlp 兼容性问题**
- **风险：** yt-dlp 命令输出格式变化
- **应对：** 使用稳定版本，做好错误处理

**2. 跨平台差异**
- **风险：** 不同系统的文件路径、权限问题
- **应对：** 早期进行跨平台测试

**3. 下载进度解析**
- **风险：** 实时解析命令行输出可能不准确
- **应对：** 使用 yt-dlp 的 JSON 输出模式

#### 进度风险

**1. 并行开发协调**
- **风险：** 接口变更导致返工
- **应对：** 接口定义阶段充分讨论，锁定后不轻易修改

**2. 学习曲线**
- **风险：** Rust/Tauri 不熟悉导致进度延误
- **应对：** 提前学习，使用官方模板和文档

---

### 开发资源

#### 文档
- [Tauri 官方文档](https://tauri.app/zh-cn/)
- [React 文档](https://react.dev/)
- [yt-dlp 文档](https://github.com/yt-dlp/yt-dlp#readme)

#### 工具
- [Tauri Studio](https://tauri.app/)
- [Rust Playground](https://play.rust-lang.org/)
- [TypeScript Playground](https://www.typescriptlang.org/play)

#### 社区
- [Tauri Discord](https://discord.com/invite/tauri)
- [yt-dlp GitHub Issues](https://github.com/yt-dlp/yt-dlp/issues)

---

## 附录

### A. 打包配置

#### tauri.conf.json 完整配置

```json
{
  "package": {
    "productName": "YouTube Downloader",
    "version": "1.0.0"
  },
  "build": {
    "distDir": "../dist",
    "devPath": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "execute": true,
        "sidecar": true,
        "open": true
      },
      "dialog": {
        "all": false,
        "open": true,
        "save": true
      },
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "createDir": true,
        "removeFile": true,
        "scope": ["$DOWNLOAD/**", "$HOME/Downloads/**"]
      }
    },
    "bundle": {
      "active": true,
      "identifier": "com.ytdlp.desktop",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "targets": ["dmg", "msi"],
      "macOS": {
        "minimumSystemVersion": "10.15",
        "entitlements": null,
        "exceptionDomain": "",
        "frameworks": [],
        "providerShortName": null,
        "signingIdentity": null
      },
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": "",
        "wix": {
          "language": "zh-CN"
        }
      },
      "resources": ["resources/*", "bin/yt-dlp*"],
      "externalBin": ["bin/yt-dlp"]
    },
    "security": {
      "csp": "default-src 'self'; img-src 'self' https: data:; style-src 'self' 'unsafe-inline'"
    },
    "windows": [
      {
        "title": "YouTube Downloader",
        "width": 900,
        "height": 700,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false,
        "center": true
      }
    ]
  }
}
```

### B. package.json 配置

```json
{
  "name": "yt-dlp-desktop",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build",
    "tauri:build:mac": "tauri build --target universal-apple-darwin",
    "tauri:build:win": "tauri build --target x86_64-pc-windows-msvc"
  },
  "dependencies": {
    "@tauri-apps/api": "^1.5.3",
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^1.5.9",
    "@types/react": "^18.2.45",
    "@types/react-dom": "^18.2.18",
    "@typescript-eslint/eslint-plugin": "^6.15.0",
    "@typescript-eslint/parser": "^6.15.0",
    "@vitejs/plugin-react": "^4.2.1",
    "autoprefixer": "^10.4.16",
    "eslint": "^8.56.0",
    "eslint-plugin-react": "^7.33.2",
    "postcss": "^8.4.32",
    "tailwindcss": "^3.4.0",
    "typescript": "^5.3.3",
    "vite": "^5.0.8"
  }
}
```

### C. 常见问题

**Q1: Tauri 打包后体积多大？**
A: 约 5-8MB（不包含 yt-dlp 二进制文件）

**Q2: 是否需要用户预安装 yt-dlp？**
A: 不需要，将 yt-dlp 打包进应用

**Q3: 如何处理 YouTube 链接解析失败？**
A: 显示友好的错误提示，提供重试选项

**Q4: 是否支持批量下载？**
A: v1.0 暂不支持，可在后续版本添加

**Q5: 如何更新 yt-dlp 版本？**
A: 替换 bin 目录下的 yt-dlp 二进制文件，重新打包

---

## 总结

本方案采用 **Tauri + React + TypeScript** 技术栈，实现一个轻量级、高性能的跨平台 YouTube 下载器桌面应用。

**核心优势：**
- ✅ 体积小（5-8MB）
- ✅ 性能好（低内存占用）
- ✅ 开发效率高（前后端并行）
- ✅ 易于维护（TypeScript 类型安全）
- ✅ 用户体验好（现代化 UI）

**预计开发周期：** 5 周（并行开发）

**目标平台：** macOS 10.15+, Windows 10+

---

**文档版本：** v1.0
**创建日期：** 2025-01-15
**最后更新：** 2025-01-15
