use std::process::Command;
use crate::types::{VideoInfo, YtDlpOutput};
use tauri::Emitter;

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

/// 下载视频（支持自动合并 DASH 格式和实时进度）
pub async fn download_video(
    url: &str,
    format_id: &str,
    output_path: &str,
    window: tauri::Window,
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
       .arg("--progress") // 启用进度输出
       .arg(url); // 添加 URL 参数

    println!("[DEBUG] Executing command: {:?}", cmd);

    // 配置命令以捕获输出
    cmd.stdout(std::process::Stdio::piped())
       .stderr(std::process::Stdio::piped());

    // 启动进程
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Failed to start download: {}", e))?;

    println!("[DEBUG] Download process started successfully");

    // 在后台任务中处理进度输出
    let window_clone = window.clone();
    let progress_handle = if let Some(stdout) = child.stdout.take() {
        Some(tokio::spawn(async move {
            parse_download_progress(stdout, window_clone).await;
        }))
    } else {
        None
    };

    // 等待进程完成
    let status = child.wait().map_err(|e| format!("Failed to wait for process: {}", e))?;
    
    // 等待进度解析任务完成
    if let Some(handle) = progress_handle {
        let _ = handle.await;
    }
    
    // 给进度解析一点时间完成
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    if status.success() {
        // 发送完成事件
        let _ = window.emit("download-progress", crate::types::DownloadProgress {
            status: "finished".to_string(),
            percent: 100.0,
            speed: 0.0,
            eta: 0.0,
            downloaded: 0,
            total: 0,
            filename: "下载完成！".to_string(),
        });
        println!("[DEBUG] Download completed successfully");
        Ok(())
    } else {
        let error_msg = format!("Download failed with exit code: {:?}", status.code());
        let _ = window.emit("download-progress", crate::types::DownloadProgress {
            status: "error".to_string(),
            percent: 0.0,
            speed: 0.0,
            eta: 0.0,
            downloaded: 0,
            total: 0,
            filename: error_msg.clone(),
        });
        Err(error_msg)
    }
}

/// 解析 yt-dlp 的进度输出
async fn parse_download_progress(stdout: std::process::ChildStdout, window: tauri::Window) {
    use std::io::{BufRead, BufReader};
    use regex::Regex;

    let reader = BufReader::new(stdout);
    
    // 正则表达式匹配下载进度行
    // 例如: [download]   2.3% of   10.15GiB at   49.23MiB/s ETA 03:26
    let progress_regex = Regex::new(r"\[download\]\s+(\d+\.?\d*)%\s+of\s+([\d.]+\w+)\s+at\s+([\d.]+\w+/s)\s+ETA\s+([\d:]+)").unwrap();
    
    // 匹配 100% 完成行
    // 例如: [download] 100% of  364.32MiB in 00:00:12 at 28.36MiB/s
    let complete_regex = Regex::new(r"\[download\]\s+100%\s+of\s+([\d.]+\w+)\s+in\s+([\d:]+)\s+at\s+([\d.]+\w+/s)").unwrap();
    
    let mut last_total_bytes = 0u64;
    let mut current_filename = "Downloading...".to_string();
    
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("[PROGRESS] {}", line);
            
            // 检测文件名变化（当开始下载新文件时）
            if line.contains("[download] Destination:") {
                if let Some(filename_start) = line.rfind('/') {
                    current_filename = line[filename_start + 1..].to_string();
                }
            }
            
            // 检测合并状态
            if line.contains("[Merger] Merging formats") {
                let _ = window.emit("download-progress", crate::types::DownloadProgress {
                    status: "processing".to_string(),
                    percent: 100.0,
                    speed: 0.0,
                    eta: 0.0,
                    downloaded: last_total_bytes,
                    total: last_total_bytes,
                    filename: "正在合并视频和音频...".to_string(),
                });
                continue;
            }
            
            // 检测元数据添加状态
            if line.contains("[Metadata] Adding metadata") {
                let _ = window.emit("download-progress", crate::types::DownloadProgress {
                    status: "processing".to_string(),
                    percent: 100.0,
                    speed: 0.0,
                    eta: 0.0,
                    downloaded: last_total_bytes,
                    total: last_total_bytes,
                    filename: "正在添加元数据...".to_string(),
                });
                continue;
            }
            
            // 检测删除临时文件（表示处理即将完成）
            if line.contains("Deleting original file") {
                let _ = window.emit("download-progress", crate::types::DownloadProgress {
                    status: "processing".to_string(),
                    percent: 100.0,
                    speed: 0.0,
                    eta: 0.0,
                    downloaded: last_total_bytes,
                    total: last_total_bytes,
                    filename: "正在清理临时文件...".to_string(),
                });
                continue;
            }
            
            // 检测 100% 完成
            if let Some(captures) = complete_regex.captures(&line) {
                let total_str = captures.get(1).unwrap().as_str();
                let total_bytes = parse_size_to_bytes(total_str);
                last_total_bytes = total_bytes;
                
                let progress = crate::types::DownloadProgress {
                    status: "downloading".to_string(),
                    percent: 100.0,
                    speed: 0.0,
                    eta: 0.0,
                    downloaded: total_bytes,
                    total: total_bytes,
                    filename: current_filename.clone(),
                };
                
                let _ = window.emit("download-progress", &progress);
                continue;
            }
            
            // 检测常规进度
            if let Some(captures) = progress_regex.captures(&line) {
                let percent = captures.get(1).unwrap().as_str().parse::<f64>().unwrap_or(0.0);
                let total_str = captures.get(2).unwrap().as_str();
                let speed_str = captures.get(3).unwrap().as_str();
                let eta_str = captures.get(4).unwrap().as_str();
                
                // 解析文件大小
                let total_bytes = parse_size_to_bytes(total_str);
                let downloaded_bytes = (total_bytes as f64 * percent / 100.0) as u64;
                last_total_bytes = total_bytes;
                
                // 解析速度
                let speed_bytes = parse_speed_to_bytes_per_sec(speed_str);
                
                // 解析 ETA
                let eta_seconds = parse_eta_to_seconds(eta_str);
                
                let progress = crate::types::DownloadProgress {
                    status: "downloading".to_string(),
                    percent,
                    speed: speed_bytes as f64,
                    eta: eta_seconds as f64,
                    downloaded: downloaded_bytes,
                    total: total_bytes,
                    filename: current_filename.clone(),
                };
                
                // 发送进度事件到前端
                let _ = window.emit("download-progress", &progress);
            }
        }
    }
}

/// 解析文件大小字符串为字节数
fn parse_size_to_bytes(size_str: &str) -> u64 {
    let size_str = size_str.trim();
    if let Some(pos) = size_str.find(|c: char| c.is_alphabetic()) {
        let (number_part, unit_part) = size_str.split_at(pos);
        if let Ok(number) = number_part.parse::<f64>() {
            let multiplier = match unit_part.to_lowercase().as_str() {
                "b" => 1,
                "kb" => 1_024,
                "mb" => 1_024_u64.pow(2),
                "gb" => 1_024_u64.pow(3),
                "gib" => 1_073_741_824, // 1024^3
                "mib" => 1_048_576,     // 1024^2
                "kib" => 1_024,
                _ => 1,
            };
            return (number * multiplier as f64) as u64;
        }
    }
    0
}

/// 解析速度字符串为每秒字节数
fn parse_speed_to_bytes_per_sec(speed_str: &str) -> u64 {
    let speed_str = speed_str.trim().trim_end_matches("/s");
    parse_size_to_bytes(speed_str)
}

/// 解析 ETA 字符串为秒数
fn parse_eta_to_seconds(eta_str: &str) -> u32 {
    let parts: Vec<&str> = eta_str.split(':').collect();
    match parts.len() {
        2 => {
            // MM:SS 格式
            let minutes = parts[0].parse::<u32>().unwrap_or(0);
            let seconds = parts[1].parse::<u32>().unwrap_or(0);
            minutes * 60 + seconds
        }
        3 => {
            // HH:MM:SS 格式
            let hours = parts[0].parse::<u32>().unwrap_or(0);
            let minutes = parts[1].parse::<u32>().unwrap_or(0);
            let seconds = parts[2].parse::<u32>().unwrap_or(0);
            hours * 3600 + minutes * 60 + seconds
        }
        _ => 0,
    }
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
