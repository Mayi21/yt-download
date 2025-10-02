# 🚀 快速开始指南

## 📦 项目已完成

恭喜！YouTube Downloader 项目已经配置完成，支持前后端并行开发。

### ✅ 已完成的内容

1. **✅ 前端完整实现**
   - React + TypeScript 组件
   - TailwindCSS 样式
   - Mock 数据支持
   - API 切换层

2. **✅ 后端完整实现**
   - Rust Tauri Commands
   - yt-dlp 封装
   - 类型定义
   - 所有 API 接口

3. **✅ 完整文档**
   - 技术方案文档
   - 并行开发指南
   - API 接口文档

---

## 🎯 两种启动方式

### 方式 1️⃣: 前端开发（推荐先用这个）

**适合：** 前端开发者、UI 设计、不想安装 Rust

```bash
# 确保在项目根目录
cd /Users/liuge/project/yt-download/yt-dlp-desktop

# 安装依赖（如果还没安装）
npm install

# 启动前端开发服务器（使用 Mock 数据）
npm run dev
```

✨ 浏览器会自动打开 http://localhost:5173

**功能说明：**
- ✅ 输入 YouTube URL
- ✅ 显示视频信息（Mock 数据）
- ✅ 选择格式和清晰度
- ✅ 模拟下载进度
- ✅ 完整的 UI 交互

**Mock 数据：** 所有操作都使用预设的示例数据，无需真实下载。

---

### 方式 2️⃣: 全栈开发（需要 Rust）

**适合：** 后端开发者、需要测试真实下载功能

#### Step 1: 安装 Rust

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 验证安装
rustc --version
cargo --version
```

Windows 用户请访问：https://www.rust-lang.org/tools/install

#### Step 2: 下载 yt-dlp

```bash
cd /Users/liuge/project/yt-download/yt-dlp-desktop/src-tauri/bin

# macOS/Linux
curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o yt-dlp
chmod +x yt-dlp

# 测试 yt-dlp
./yt-dlp --version
```

Windows 用户：
1. 访问 https://github.com/yt-dlp/yt-dlp/releases/latest
2. 下载 `yt-dlp.exe`
3. 放到 `src-tauri/bin/` 目录

#### Step 3: 修改环境变量

编辑 `.env.development` 文件：

```bash
# 将 VITE_USE_MOCK 改为 false
VITE_USE_MOCK=false
```

#### Step 4: 启动全栈应用

```bash
cd /Users/liuge/project/yt-download/yt-dlp-desktop

# 启动 Tauri 应用（包含前后端）
npm run tauri dev
```

⚠️ **首次启动会编译 Rust 代码，需要 5-10 分钟。**

---

## 📊 功能演示

### 前端界面预览

```
┌─────────────────────────────────────────────┐
│          📺 YouTube Downloader              │
│   快速下载 YouTube 视频，支持多种格式和清晰度 │
└─────────────────────────────────────────────┘

┌─────────────────────────────────────────────┐
│ 🔗 YouTube 链接                             │
│ ┌─────────────────────────────────────────┐ │
│ │ https://www.youtube.com/watch?v=...    │ │
│ └─────────────────────────────────────────┘ │
│                              [📋 粘贴] [→]  │
└─────────────────────────────────────────────┘

视频信息（获取后显示）：
┌─────────────────────────────────────────────┐
│ ┌────────┐  Rick Astley - Never Gonna...   │
│ │        │  ⏱️ 3:32  👤 Rick Astley         │
│ │ 缩略图 │  📅 2009-10-25  👁️ 1.4B views   │
│ └────────┘                                  │
└─────────────────────────────────────────────┘

格式选择：
┌─────────────────────────────────────────────┐
│ 清晰度                                       │
│ [2160p] [1080p] [720p] [480p] [360p]       │
│                                              │
│ 格式                                         │
│ [MP4] [WebM] [MKV]                          │
│                                              │
│ ☐ 🎵 仅音频    ☐ 📝 包含字幕                 │
│                                              │
│ [        ⬇️ 开始下载        ]                │
└─────────────────────────────────────────────┘
```

---

## 🧪 测试步骤

### 测试 Mock 模式（前端）

1. 启动：`npm run dev`
2. 输入任意 YouTube URL（如：`https://www.youtube.com/watch?v=dQw4w9WgXcQ`）
3. 点击"获取视频信息"
4. 查看 Mock 视频信息
5. 选择格式和清晰度
6. 点击"开始下载"
7. 观察模拟的下载进度

### 测试真实模式（全栈）

1. 确保 Rust 和 yt-dlp 已安装
2. 设置 `VITE_USE_MOCK=false`
3. 启动：`npm run tauri dev`
4. 输入真实的 YouTube URL
5. 等待获取真实视频信息
6. 选择保存路径
7. 开始真实下载

---

## 🐛 常见问题

### Q1: npm run dev 启动失败？

```bash
# 删除 node_modules 重新安装
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### Q2: TypeScript 报错？

```bash
# 确保所有文件都在正确位置
ls -la src/components/
ls -la src/services/
ls -la src/types/

# 重启 VSCode 或编辑器
```

### Q3: Rust 编译失败？

```bash
# 更新 Rust
rustup update

# 清理并重新构建
cd src-tauri
cargo clean
cargo build
```

### Q4: yt-dlp 无法执行？

```bash
# macOS/Linux: 确保有执行权限
chmod +x src-tauri/bin/yt-dlp

# 测试 yt-dlp
./src-tauri/bin/yt-dlp --version
```

### Q5: Mock 数据没有显示？

检查 `.env.development`：
```bash
VITE_USE_MOCK=true  # 确保是 true
```

重启开发服务器：
```bash
# Ctrl+C 停止
npm run dev  # 重新启动
```

---

## 📂 项目结构

```
yt-dlp-desktop/
├── src/                      # 前端源码
│   ├── components/           # React 组件
│   │   ├── UrlInput.tsx      ✅ 已完成
│   │   ├── VideoInfo.tsx     ✅ 已完成
│   │   ├── FormatSelector.tsx ✅ 已完成
│   │   └── DownloadProgress.tsx ✅ 已完成
│   ├── services/
│   │   ├── api.ts            ✅ API 层（支持切换）
│   │   └── mock.ts           ✅ Mock 数据
│   ├── types/
│   │   ├── index.ts          ✅ 类型定义
│   │   └── commands.ts       ✅ 命令接口
│   └── App.tsx               ✅ 主应用
│
├── src-tauri/                # Rust 后端
│   ├── src/
│   │   ├── types.rs          ✅ Rust 类型
│   │   ├── ytdlp.rs          ✅ yt-dlp 封装
│   │   ├── commands.rs       ✅ Tauri 命令
│   │   └── lib.rs            ✅ 主模块
│   └── bin/
│       └── yt-dlp            ⚠️ 需要下载
│
├── docs/
│   └── YouTube下载器技术方案.md ✅ 完整文档
│
├── .env.development          ✅ 环境配置
├── README.md                 ✅ 项目说明
├── PARALLEL_DEV_GUIDE.md     ✅ 并行开发指南
└── GETTING_STARTED.md        📄 本文档
```

---

## 🎨 下一步开发建议

### 前端优化

- [ ] 添加加载动画和骨架屏
- [ ] 优化错误提示 UI
- [ ] 实现深色模式切换
- [ ] 添加下载历史记录页面
- [ ] 实现拖拽文件选择
- [ ] 添加键盘快捷键
- [ ] 优化移动端响应式

### 后端优化

- [ ] 实现真实的下载进度监听
- [ ] 添加下载队列管理
- [ ] 实现暂停/恢复下载
- [ ] 添加下载历史数据库
- [ ] 优化大文件下载性能
- [ ] 实现多任务并发下载

### 功能增强

- [ ] 支持播放列表批量下载
- [ ] 添加视频格式转换
- [ ] 支持自定义输出文件名
- [ ] 添加下载速度限制
- [ ] 实现自动更新 yt-dlp
- [ ] 添加代理设置

---

## 📞 获取帮助

- **技术文档**: `docs/YouTube下载器技术方案.md`
- **并行开发**: `PARALLEL_DEV_GUIDE.md`
- **项目说明**: `README.md`
- **yt-dlp 文档**: https://github.com/yt-dlp/yt-dlp
- **Tauri 文档**: https://tauri.app/

---

## ✨ 快速命令参考

```bash
# 前端开发（Mock 模式）
npm run dev

# 全栈开发（真实 API）
npm run tauri dev

# 构建应用
npm run tauri build

# 仅构建前端
npm run build

# 检查 TypeScript
npx tsc --noEmit

# 检查 Rust 代码
cd src-tauri && cargo check

# 运行 Rust 测试
cd src-tauri && cargo test
```

---

**🎉 准备好了！现在就开始开发吧！**

推荐：先运行 `npm run dev` 体验前端 UI，然后再配置 Rust 环境进行全栈开发。
