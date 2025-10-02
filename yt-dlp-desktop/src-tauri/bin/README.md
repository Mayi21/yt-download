# yt-dlp 二进制文件

这个目录需要包含 yt-dlp 可执行文件。

## 下载 yt-dlp

### macOS / Linux

```bash
# 下载 yt-dlp
curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o yt-dlp

# 添加执行权限
chmod +x yt-dlp

# 测试
./yt-dlp --version
```

### Windows

1. 访问 https://github.com/yt-dlp/yt-dlp/releases/latest
2. 下载 `yt-dlp.exe`
3. 将文件放置到当前目录

## 验证安装

运行以下命令验证 yt-dlp 是否正常工作：

```bash
# macOS/Linux
./yt-dlp --version

# Windows
.\yt-dlp.exe --version
```

## 测试获取视频信息

```bash
./yt-dlp --dump-json "https://www.youtube.com/watch?v=dQw4w9WgXcQ" --skip-download
```

如果能看到 JSON 输出，说明 yt-dlp 工作正常！
