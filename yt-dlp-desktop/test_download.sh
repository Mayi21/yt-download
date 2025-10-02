#!/bin/bash

# 测试下载脚本
YTDLP_PATH="/Users/liuge/project/yt-download/yt-dlp-desktop/src-tauri/bin/yt-dlp"
TEST_URL="https://youtu.be/njX2bu-_Vw4"
OUTPUT_DIR="/tmp/yt_test"

echo "=== YouTube Downloader 下载测试 ==="
echo "yt-dlp 路径: $YTDLP_PATH"
echo "测试 URL: $TEST_URL"
echo "输出目录: $OUTPUT_DIR"
echo ""

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

echo "1. 测试获取视频信息..."
$YTDLP_PATH -J --no-playlist "$TEST_URL" > /tmp/video_info.json
if [ $? -eq 0 ]; then
    echo "✅ 视频信息获取成功"
    echo "视频标题: $(cat /tmp/video_info.json | jq -r '.title')"
    echo "可用格式数量: $(cat /tmp/video_info.json | jq '.formats | length')"
else
    echo "❌ 视频信息获取失败"
    exit 1
fi

echo ""
echo "2. 测试下载 720p 格式..."
$YTDLP_PATH -f "best[height<=720]" --no-playlist -o "$OUTPUT_DIR/%(title)s.%(ext)s" "$TEST_URL" --embed-metadata --write-thumbnail --convert-thumbnails jpg --newline --no-colors --progress

if [ $? -eq 0 ]; then
    echo "✅ 下载成功"
    echo "下载的文件:"
    ls -la "$OUTPUT_DIR"
else
    echo "❌ 下载失败"
    echo "错误代码: $?"
fi

echo ""
echo "3. 清理测试文件..."
rm -rf "$OUTPUT_DIR"
rm -f /tmp/video_info.json
echo "测试完成"