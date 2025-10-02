# 🚀 并行开发快速指南

## 📌 当前项目状态

✅ 已完成：
- [x] 项目初始化（Tauri + React + TypeScript）
- [x] 类型定义（TypeScript 接口契约）
- [x] Mock 数据和 API 切换层
- [x] 前端 UI 组件（使用 Mock 数据）
- [x] 项目文档和配置

⏳ 待完成：
- [ ] Rust 后端 yt-dlp 封装
- [ ] 前后端集成测试
- [ ] 性能优化和打包

---

## 👥 团队成员快速入门

### 🎨 前端开发者（可立即开始）

**无需安装 Rust！直接开始开发。**

#### 1. 克隆项目并安装依赖
```bash
cd yt-dlp-desktop
npm install
```

#### 2. 确认使用 Mock 模式
查看 `.env.development` 文件：
```bash
VITE_USE_MOCK=true  # ✅ 确保是 true
```

#### 3. 启动开发服务器
```bash
npm run dev
```

浏览器会自动打开 http://localhost:5173

#### 4. 开发说明

**已实现的组件：**
- `src/components/UrlInput.tsx` - URL 输入框
- `src/components/VideoInfo.tsx` - 视频信息展示
- `src/components/FormatSelector.tsx` - 格式选择器
- `src/components/DownloadProgress.tsx` - 下载进度条

**API 服务层：**
- `src/services/api.ts` - API 调用（自动切换 Mock/真实）
- `src/services/mock.ts` - Mock 数据

**类型定义：**
- `src/types/index.ts` - 所有数据类型
- `src/types/commands.ts` - Tauri 命令接口

#### 5. 开发任务建议

**优先级高：**
- [ ] 优化 UI 样式和布局
- [ ] 添加加载动画和过渡效果
- [ ] 改进错误提示 UI
- [ ] 添加深色模式切换
- [ ] 优化响应式布局

**优先级中：**
- [ ] 添加下载历史列表组件
- [ ] 实现键盘快捷键（如 Ctrl+V 粘贴）
- [ ] 添加设置页面（保存默认路径等）
- [ ] 优化缩略图加载

**优先级低：**
- [ ] 添加多语言支持
- [ ] 自定义主题颜色
- [ ] 添加使用教程/引导

#### 6. 调试技巧

```typescript
// 在组件中打印 Mock 数据
console.log('[Debug] Video Info:', videoInfo);

// 测试不同的 URL 格式
const testUrls = [
  'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
  'https://youtu.be/dQw4w9WgXcQ',
  'https://www.youtube.com/shorts/abc123',
];
```

---

### 🦀 后端开发者

**需要安装 Rust 环境。**

#### 1. 安装 Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 2. 下载 yt-dlp 二进制文件

**macOS/Linux:**
```bash
mkdir -p src-tauri/bin
curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o src-tauri/bin/yt-dlp
chmod +x src-tauri/bin/yt-dlp
```

**Windows:**
```powershell
mkdir src-tauri\bin
# 下载 yt-dlp.exe 到 src-tauri\bin\
```

#### 3. 测试 yt-dlp
```bash
./src-tauri/bin/yt-dlp --version
./src-tauri/bin/yt-dlp --dump-json "https://www.youtube.com/watch?v=dQw4w9WgXcQ" --skip-download
```

#### 4. Rust 项目结构

```
src-tauri/
├── src/
│   ├── main.rs       # 主入口（已有）
│   ├── commands.rs   # 需要创建：Tauri 命令
│   └── ytdlp.rs      # 需要创建：yt-dlp 封装
└── Cargo.toml        # Rust 依赖配置
```

#### 5. 需要实现的 Commands

**基于前端定义的接口 (`src/types/commands.ts`)：**

```rust
// commands.rs

#[tauri::command]
async fn get_video_info(url: String) -> Result<VideoInfo, String> {
    // 调用 yt-dlp --dump-json {url}
    // 解析 JSON 输出
}

#[tauri::command]
async fn list_formats(url: String) -> Result<Vec<VideoFormat>, String> {
    // 从 video info 中提取 formats
}

#[tauri::command]
async fn start_download(config: DownloadConfig) -> Result<String, String> {
    // 执行 yt-dlp -f {format_id} -o {output_path} {url}
    // 解析下载进度并发送事件
}

#[tauri::command]
async fn cancel_download(download_id: String) -> Result<(), String> {
    // 终止下载进程
}

#[tauri::command]
async fn select_save_path() -> Result<Option<String>, String> {
    // 使用 tauri::api::dialog 选择文件夹
}
```

#### 6. 开发流程

```bash
# 1. 进入后端目录
cd src-tauri

# 2. 添加依赖（Cargo.toml）
cargo add serde --features derive
cargo add serde_json
cargo add tokio --features full

# 3. 编写代码

# 4. 运行测试
cargo test

# 5. 启动 Tauri 应用（包含后端）
cd ..
npm run tauri dev
```

#### 7. 单元测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_video_info() {
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = get_video_info(url.to_string()).await;

        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.id, "dQw4w9WgXcQ");
        assert!(!info.title.is_empty());
    }
}
```

#### 8. 调试

```bash
# 启用详细日志
RUST_LOG=debug npm run tauri dev

# 仅构建后端
cd src-tauri
cargo build
```

---

## 🔄 集成流程

### 何时进行集成？

当前后端都完成基本功能后：

1. **前端：** 确认所有组件使用 Mock 数据正常工作
2. **后端：** 确认所有 Tauri Commands 单元测试通过
3. **集成：** 切换到真实 API

### 集成步骤

#### 1. 切换到真实 API

修改 `.env.development`：
```bash
VITE_USE_MOCK=false  # 使用真实后端
```

#### 2. 启动完整应用

```bash
npm run tauri dev
```

#### 3. 测试清单

**基础功能：**
- [ ] 输入 YouTube URL 并获取视频信息
- [ ] 显示视频缩略图和元数据
- [ ] 列出可用格式和清晰度
- [ ] 选择格式并开始下载
- [ ] 实时显示下载进度
- [ ] 取消下载功能
- [ ] 下载完成后打开文件

**异常处理：**
- [ ] 无效 URL 错误提示
- [ ] 网络错误重试
- [ ] 磁盘空间不足提示
- [ ] 下载失败错误处理

**性能：**
- [ ] 大文件下载（4K 视频）
- [ ] 多任务下载
- [ ] 内存占用监控

#### 4. Bug 修复

遇到问题时：

1. **前端问题：** 检查浏览器控制台
2. **后端问题：** 查看终端 Rust 日志
3. **接口问题：** 确认类型定义一致

---

## 📊 进度跟踪

### Week 1（已完成）
- ✅ 项目初始化
- ✅ 接口定义
- ✅ 前端 UI 实现（Mock）

### Week 2（进行中）
- ⏳ 后端 Rust 开发
- ⏳ yt-dlp 集成
- ⏳ 前后端联调

### Week 3（计划中）
- 📋 完整功能测试
- 📋 性能优化
- 📋 跨平台打包

---

## 🆘 常见问题

### Q1: 前端启动失败？
```bash
# 清理缓存重试
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### Q2: 后端编译失败？
```bash
# 更新 Rust
rustup update

# 清理构建缓存
cd src-tauri
cargo clean
cargo build
```

### Q3: Mock 数据没有显示？
检查 `.env.development` 中 `VITE_USE_MOCK=true`，并重启开发服务器。

### Q4: TypeScript 类型错误？
```bash
# 重新生成类型定义
npm run build
```

---

## 📞 联系和协作

- **代码审查：** 提交 Pull Request
- **Bug 报告：** 创建 GitHub Issue
- **讨论：** 使用团队协作工具（Slack/Discord）

---

## 🎯 成功标准

### 前端完成标准
- ✅ 所有 UI 组件正常渲染
- ✅ Mock 数据交互流畅
- ✅ 无 TypeScript 错误
- ✅ 响应式布局正常

### 后端完成标准
- ✅ 所有 Commands 实现
- ✅ 单元测试通过
- ✅ yt-dlp 调用成功
- ✅ 进度解析正确

### 集成完成标准
- ✅ 前后端数据流通
- ✅ 错误处理完善
- ✅ 性能达标
- ✅ 跨平台兼容

---

**Happy Parallel Coding! 🎉**

*最后更新：2025-01-15*
