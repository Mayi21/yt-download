#!/bin/bash

# 测试SSL修复的脚本
YTDLP_PATH="/Users/liuge/project/yt-download/yt-dlp-desktop/src-tauri/bin/yt-dlp"
TEST_URL="https://www.youtube.com/watch?v=njX2bu-_Vw4"
OUTPUT_DIR="/tmp/yt_ssl_test"

echo "=== SSL 连接问题修复测试 ==="
echo "测试 URL: $TEST_URL"
echo "输出目录: $OUTPUT_DIR"
echo ""

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

echo "1. 测试原始4K格式 (可能失败)..."
$YTDLP_PATH -f "701+258" --no-playlist -o "$OUTPUT_DIR/4k_%(title)s.%(ext)s" "$TEST_URL" \
    --retries 3 --fragment-retries 3 --socket-timeout 30 --simulate

if [ $? -eq 0 ]; then
    echo "✅ 4K格式测试通过"
else
    echo "❌ 4K格式测试失败，这是预期的"
fi

echo ""
echo "2. 测试1440p备用格式..."
$YTDLP_PATH -f "308+258" --no-playlist -o "$OUTPUT_DIR/1440p_%(title)s.%(ext)s" "$TEST_URL" \
    --retries 5 --fragment-retries 5 --socket-timeout 30 --simulate

if [ $? -eq 0 ]; then
    echo "✅ 1440p格式测试通过"
else
    echo "❌ 1440p格式测试失败"
fi

echo ""
echo "3. 测试1080p备用格式..."
$YTDLP_PATH -f "299+258" --no-playlist -o "$OUTPUT_DIR/1080p_%(title)s.%(ext)s" "$TEST_URL" \
    --retries 5 --fragment-retries 5 --socket-timeout 30 --simulate

if [ $? -eq 0 ]; then
    echo "✅ 1080p格式测试通过"
else
    echo "❌ 1080p格式测试失败"
fi

echo ""
echo "4. 测试通用最佳格式..."
$YTDLP_PATH -f "best[height<=720]" --no-playlist -o "$OUTPUT_DIR/best_%(title)s.%(ext)s" "$TEST_URL" \
    --retries 5 --fragment-retries 5 --socket-timeout 30 --simulate

if [ $? -eq 0 ]; then
    echo "✅ 通用格式测试通过"
else
    echo "❌ 通用格式测试失败"
fi

echo ""
echo "5. 实际下载测试 (720p)..."
$YTDLP_PATH -f "best[height<=720]" --no-playlist -o "$OUTPUT_DIR/actual_%(title)s.%(ext)s" "$TEST_URL" \
    --retries 10 --fragment-retries 10 --retry-sleep "linear=1:5:10" \
    --socket-timeout 30 --user-agent "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36" \
    --embed-metadata --write-thumbnail --convert-thumbnails jpg

if [ $? -eq 0 ]; then
    echo "✅ 实际下载成功"
    echo "下载的文件:"
    ls -la "$OUTPUT_DIR"
else
    echo "❌ 实际下载失败"
fi

echo ""
echo "6. 清理测试文件..."
rm -rf "$OUTPUT_DIR"
echo "测试完成"