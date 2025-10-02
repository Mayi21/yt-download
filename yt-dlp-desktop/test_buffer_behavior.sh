#!/bin/bash

echo "=== 缓冲区行为测试 ==="
echo ""

LOG_FILE="$HOME/Library/Application Support/com.youtube-downloader.desktop/app.log"

echo "1. 清空现有日志文件（如果存在）"
if [ -f "$LOG_FILE" ]; then
    > "$LOG_FILE"
    echo "✅ 日志文件已清空"
else
    echo "ℹ️  日志文件不存在，将在应用启动时创建"
fi

echo ""
echo "2. 启动应用并观察日志写入行为"
echo "请按以下步骤操作："
echo "   a) 启动应用"
echo "   b) 粘贴一个视频链接"
echo "   c) 点击解析"
echo "   d) 观察日志文件变化"

echo ""
echo "3. 实时监控日志文件大小变化"
echo "在另一个终端运行以下命令："
echo "watch -n 1 'ls -lh \"$LOG_FILE\" 2>/dev/null || echo \"日志文件尚未创建\"'"

echo ""
echo "4. 实时查看日志内容"
echo "tail -f \"$LOG_FILE\""

echo ""
echo "=== 预期行为 ==="
echo ""
echo "📝 INFO/DEBUG 日志:"
echo "   - 立即出现在缓冲区中"
echo "   - 可能延迟几秒才写入磁盘文件"
echo "   - 或者在积累一定量后批量写入"

echo ""
echo "🚨 ERROR 日志:"
echo "   - 立即写入磁盘文件"
echo "   - 在 tail -f 中立即可见"

echo ""
echo "💾 程序退出时:"
echo "   - 所有缓冲区内容都会刷新到磁盘"
echo "   - 确保没有日志丢失"

echo ""
echo "测试准备完成！"