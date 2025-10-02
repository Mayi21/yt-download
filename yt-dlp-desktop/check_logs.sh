#!/bin/bash

LOG_DIR="$HOME/Library/Application Support/com.youtube-downloader.desktop"
LOG_FILE="$LOG_DIR/app.log"

echo "=== YouTube Downloader 日志检查 ==="
echo "日志目录: $LOG_DIR"
echo "日志文件: $LOG_FILE"
echo ""

if [ -d "$LOG_DIR" ]; then
    echo "✅ 日志目录存在"
    ls -la "$LOG_DIR"
    echo ""
    
    if [ -f "$LOG_FILE" ]; then
        echo "✅ 日志文件存在"
        echo "文件大小: $(ls -lh "$LOG_FILE" | awk '{print $5}')"
        echo ""
        echo "=== 最近的日志内容 ==="
        tail -20 "$LOG_FILE"
        echo ""
        echo "=== 实时监控日志 (按 Ctrl+C 退出) ==="
        tail -f "$LOG_FILE"
    else
        echo "❌ 日志文件不存在"
        echo "请先运行应用以生成日志文件"
    fi
else
    echo "❌ 日志目录不存在"
    echo "请先运行应用以创建日志目录"
fi