use std::process::Command;
use crate::types::{VideoInfo, YtDlpOutput};

/// 获取 yt-dlp 可执行文件路径
fn get_ytdlp_path() -> String {
    // 开发环境：使用绝对路径
    let dev_path = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        "/Users/liuge/project/yt-download/yt-dlp-desktop/src-tauri/bin/yt-dlp"
    } else {
        "C:\\Users\\liuge\\project\\yt-download\\yt-dlp-desktop\\src-tauri\\bin\\yt-dlp.exe"
    };

    // 检查开发路径
    if std::path::Path::new(dev_path).exists() {
        println!("[DEBUG] Using dev path: {}", dev_path);
        return dev_path.to_string();
    }

    // 回退到相对路径（生产环境）
    let fallback = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        "./bin/yt-dlp"
    } else {
        ".\\bin\\yt-dlp.exe"
    };
    println!("[DEBUG] Using fallback path: {}", fallback);
    fallback.to_string()
}

/// 获取视频信息
pub async fn get_video_info(url: &str) -> Result<VideoInfo, String> {
    let ytdlp_path = get_ytdlp_path();

    println!("[DEBUG] yt-dlp path: {}", ytdlp_path);
    println!("[DEBUG] Video URL: {}", url);
    println!("[DEBUG] Executing yt-dlp command...");

    // 执行 yt-dlp 获取 JSON
    let output = Command::new(&ytdlp_path)
        .arg("-J")  // 等同于 --dump-single-json
        .arg("--no-playlist")
        .arg(url)
        .output()
        .map_err(|e| {
            println!("[ERROR] Failed to execute yt-dlp: {}", e);
            format!("Failed to execute yt-dlp: {}", e)
        })?;

    println!("[DEBUG] yt-dlp execution completed");

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp error: {}", error));
    }

    // 解析 JSON 输出
    let json_str = String::from_utf8_lossy(&output.stdout);
    let ytdlp_output: YtDlpOutput = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse yt-dlp output: {}", e))?;

    Ok(ytdlp_output.into())
}

/// 下载视频（支持自动合并 DASH 格式）
pub async fn download_video(
    url: &str,
    format_id: &str,
    output_path: &str,
) -> Result<(), String> {
    let ytdlp_path = get_ytdlp_path();

    println!("[DEBUG] Starting download with format: {}", format_id);
    println!("[DEBUG] Output path: {}", output_path);

    // 构建输出模板：output_path/%(title)s.%(ext)s
    let output_template = format!("{}/%(title)s.%(ext)s", output_path);

    // 构建 yt-dlp 命令
    let mut cmd = Command::new(&ytdlp_path);
    
    // 基本参数
    cmd.arg("-f").arg(format_id)
       .arg("-o").arg(&output_template)
       .arg("--no-playlist");

    // 对于 DASH 格式，确保启用合并
    if format_id.contains('+') {
        println!("[DEBUG] DASH format detected, enabling merge");
        cmd.arg("--merge-output-format").arg("mp4");
    }

    // 添加其他有用的参数
    cmd.arg("--embed-metadata")  // 嵌入元数据
       .arg("--write-thumbnail")  // 下载缩略图
       .arg("--convert-thumbnails").arg("jpg") // 转换缩略图为 JPG
       .arg("--newline")  // 每行输出进度
       .arg("--no-colors") // 禁用颜色输出
       .arg(url); // 添加 URL 参数

    println!("[DEBUG] Executing command: {:?}", cmd);

    // 执行下载命令
    let _output = cmd
        .spawn()
        .map_err(|e| format!("Failed to start download: {}", e))?;

    println!("[DEBUG] Download process started successfully");
    Ok(())
}

/// 获取最佳格式（自动选择需要合并的 DASH 格式）
pub fn get_best_format_for_quality(formats: &[crate::types::VideoFormat], quality: &str, preferred_ext: &str) -> Option<String> {
    // 首先尝试找到完整的格式（不需要合并）
    if let Some(format) = formats.iter().find(|f| 
        f.quality_label == quality && 
        f.ext == preferred_ext && 
        f.vcodec != "none" && 
        f.acodec != "none"
    ) {
        println!("[DEBUG] Found complete format: {}", format.format_id);
        return Some(format.format_id.clone());
    }

    // 如果没有完整格式，寻找需要合并的 DASH 格式
    let video_format = formats.iter().find(|f| 
        f.quality_label == quality && 
        f.vcodec != "none" && 
        f.acodec == "none"
    );

    let audio_format = formats.iter().find(|f| 
        f.vcodec == "none" && 
        f.acodec != "none"
    ).or_else(|| {
        // 如果没有纯音频格式，找最佳音频格式
        formats.iter()
            .filter(|f| f.acodec != "none")
            .max_by_key(|f| f.tbr.unwrap_or(0.0) as u32)
    });

    if let (Some(video), Some(audio)) = (video_format, audio_format) {
        let combined_format = format!("{}+{}", video.format_id, audio.format_id);
        println!("[DEBUG] Created DASH combined format: {}", combined_format);
        return Some(combined_format);
    }

    println!("[DEBUG] No suitable format found for quality: {}", quality);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // 跳过需要网络的测试
    async fn test_get_video_info() {
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = get_video_info(url).await;

        if let Ok(info) = result {
            assert_eq!(info.id, "dQw4w9WgXcQ");
            assert!(!info.title.is_empty());
            println!("Video title: {}", info.title);
        }
    }
}
