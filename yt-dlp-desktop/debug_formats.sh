#!/bin/bash

echo "=== 格式数量诊断 ==="
echo ""

YTDLP_PATH="/Users/liuge/project/yt-download/yt-dlp-desktop/src-tauri/bin/yt-dlp"
TEST_URL="https://www.youtube.com/watch?v=d5pKWokfKPY"

echo "测试URL: $TEST_URL"
echo "yt-dlp路径: $YTDLP_PATH"
echo ""

echo "1. 获取所有格式信息..."
$YTDLP_PATH -J --no-playlist --all-formats --format-sort "res,fps,hdr:12,vcodec:vp9.2,acodec" "$TEST_URL" > /tmp/formats_debug.json

if [ $? -eq 0 ]; then
    echo "✅ 成功获取格式信息"
    
    # 统计总格式数
    TOTAL_FORMATS=$(cat /tmp/formats_debug.json | jq '.formats | length')
    echo "总格式数: $TOTAL_FORMATS"
    
    # 统计各类格式
    VIDEO_FORMATS=$(cat /tmp/formats_debug.json | jq '[.formats[] | select(.vcodec != null and .vcodec != "none" and .vcodec != "images")] | length')
    AUDIO_FORMATS=$(cat /tmp/formats_debug.json | jq '[.formats[] | select(.acodec != null and .acodec != "none")] | length')
    COMBINED_FORMATS=$(cat /tmp/formats_debug.json | jq '[.formats[] | select(.vcodec != null and .vcodec != "none" and .acodec != null and .acodec != "none")] | length')
    
    echo "视频格式数: $VIDEO_FORMATS"
    echo "音频格式数: $AUDIO_FORMATS" 
    echo "合并格式数: $COMBINED_FORMATS"
    
    echo ""
    echo "2. 按分辨率统计:"
    cat /tmp/formats_debug.json | jq -r '.formats[] | select(.height != null) | "\(.height)p - \(.format_id) - \(.ext)"' | sort -nr | uniq -c
    
    echo ""
    echo "3. 过滤后的格式 (排除 mhtml, sb*, jpg, webp):"
    FILTERED_COUNT=$(cat /tmp/formats_debug.json | jq '[.formats[] | select(.ext != "mhtml" and (.format_id | startswith("sb") | not) and .ext != "jpg" and .ext != "webp")] | length')
    echo "过滤后格式数: $FILTERED_COUNT"
    
    echo ""
    echo "4. 有效格式 (有视频或音频):"
    VALID_COUNT=$(cat /tmp/formats_debug.json | jq '[.formats[] | select(.ext != "mhtml" and (.format_id | startswith("sb") | not) and .ext != "jpg" and .ext != "webp") | select((.vcodec != null and .vcodec != "none" and .vcodec != "images") or (.acodec != null and .acodec != "none"))] | length')
    echo "有效格式数: $VALID_COUNT"
    
else
    echo "❌ 获取格式信息失败"
fi

echo ""
echo "5. 清理临时文件..."
rm -f /tmp/formats_debug.json

echo "诊断完成"