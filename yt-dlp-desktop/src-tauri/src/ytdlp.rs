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

/// 下载视频（简化版本，仅启动下载）
pub async fn download_video(
    url: &str,
    format_id: &str,
    output_path: &str,
) -> Result<(), String> {
    let ytdlp_path = get_ytdlp_path();

    // 构建输出模板：output_path/%(title)s.%(ext)s
    let output_template = format!("{}/%(title)s.%(ext)s", output_path);

    // 执行下载命令
    let _output = Command::new(&ytdlp_path)
        .arg("-f")
        .arg(format_id)
        .arg("-o")
        .arg(&output_template)
        .arg(url)
        .spawn()
        .map_err(|e| format!("Failed to start download: {}", e))?;

    Ok(())
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
