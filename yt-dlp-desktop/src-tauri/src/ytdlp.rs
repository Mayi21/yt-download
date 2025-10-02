use std::process::Command;
use crate::types::{VideoInfo, YtDlpOutput};
use tauri::Emitter;

/// 获取 yt-dlp 可执行文件路径
fn get_ytdlp_path() -> String {
    // 在调试模式下使用开发路径
    #[cfg(debug_assertions)]
    {
        // 开发环境：使用绝对路径
        let dev_path = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            "/Users/liuge/project/yt-download/yt-dlp-desktop/src-tauri/bin/yt-dlp"
        } else {
            "C:\\Users\\liuge\\project\\yt-download\\yt-dlp-desktop\\src-tauri\\bin\\yt-dlp.exe"
        };

        // 检查开发路径是否存在
        if std::path::Path::new(dev_path).exists() {
            crate::logger::AppLogger::get().info(&format!("使用开发环境路径: {}", dev_path));
            return dev_path.to_string();
        }
    }

    // 生产环境：获取可执行文件路径并推导资源目录
    #[cfg(not(debug_assertions))]
    {
        if let Ok(exe_path) = std::env::current_exe() {
            if cfg!(target_os = "macos") {
                // macOS: .app/Contents/MacOS/exe -> .app/Contents/Resources/bin/yt-dlp
                if let Some(resources_path) = exe_path
                    .parent() // MacOS 目录
                    .and_then(|p| p.parent()) // Contents 目录
                    .map(|p| p.join("Resources").join("bin").join("yt-dlp"))
                {
                    if resources_path.exists() {
                        let path = resources_path.to_string_lossy().to_string();
                        crate::logger::AppLogger::get().info(&format!("使用生产环境路径: {}", path));
                        return path;
                    }
                }
            } else if cfg!(target_os = "windows") {
                // Windows: exe 同级目录下的 bin/yt-dlp.exe
                if let Some(parent) = exe_path.parent() {
                    let yt_dlp_path = parent.join("bin").join("yt-dlp.exe");
                    if yt_dlp_path.exists() {
                        let path = yt_dlp_path.to_string_lossy().to_string();
                        crate::logger::AppLogger::get().info(&format!("使用生产环境路径: {}", path));
                        return path;
                    }
                }
            } else if cfg!(target_os = "linux") {
                // Linux: exe 同级目录下的 bin/yt-dlp
                if let Some(parent) = exe_path.parent() {
                    let yt_dlp_path = parent.join("bin").join("yt-dlp");
                    if yt_dlp_path.exists() {
                        let path = yt_dlp_path.to_string_lossy().to_string();
                        crate::logger::AppLogger::get().info(&format!("使用生产环境路径: {}", path));
                        return path;
                    }
                }
            }
        }
    }

    // 备用方案（不应该到达这里）
    let fallback = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        "./bin/yt-dlp"
    } else {
        ".\\bin\\yt-dlp.exe"
    };
    crate::logger::AppLogger::get().warn(&format!("使用备用路径: {}", fallback));
    fallback.to_string()
}

/// 获取 ffmpeg 目录路径（用于 --ffmpeg-location 参数）
fn get_ffmpeg_path() -> String {
    // 在调试模式下使用开发路径
    #[cfg(debug_assertions)]
    {
        // 开发环境：使用绝对路径
        let dev_path = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            "/Users/liuge/project/yt-download/yt-dlp-desktop/src-tauri/bin"
        } else {
            "C:\\Users\\liuge\\project\\yt-download\\yt-dlp-desktop\\src-tauri\\bin"
        };

        // 检查开发路径是否存在
        if std::path::Path::new(dev_path).exists() {
            crate::logger::AppLogger::get().info(&format!("使用开发环境 ffmpeg 路径: {}", dev_path));
            return dev_path.to_string();
        }
    }

    // 生产环境：获取可执行文件路径并推导资源目录
    #[cfg(not(debug_assertions))]
    {
        if let Ok(exe_path) = std::env::current_exe() {
            if cfg!(target_os = "macos") {
                // macOS: .app/Contents/MacOS/exe -> .app/Contents/Resources/bin/
                if let Some(resources_path) = exe_path
                    .parent() // MacOS 目录
                    .and_then(|p| p.parent()) // Contents 目录
                    .map(|p| p.join("Resources").join("bin"))
                {
                    if resources_path.exists() {
                        let path = resources_path.to_string_lossy().to_string();
                        crate::logger::AppLogger::get().info(&format!("使用生产环境 ffmpeg 路径: {}", path));
                        return path;
                    }
                }
            } else if cfg!(target_os = "windows") {
                // Windows: exe 同级目录下的 bin/
                if let Some(parent) = exe_path.parent() {
                    let ffmpeg_path = parent.join("bin");
                    if ffmpeg_path.exists() {
                        let path = ffmpeg_path.to_string_lossy().to_string();
                        crate::logger::AppLogger::get().info(&format!("使用生产环境 ffmpeg 路径: {}", path));
                        return path;
                    }
                }
            } else if cfg!(target_os = "linux") {
                // Linux: exe 同级目录下的 bin/
                if let Some(parent) = exe_path.parent() {
                    let ffmpeg_path = parent.join("bin");
                    if ffmpeg_path.exists() {
                        let path = ffmpeg_path.to_string_lossy().to_string();
                        crate::logger::AppLogger::get().info(&format!("使用生产环境 ffmpeg 路径: {}", path));
                        return path;
                    }
                }
            }
        }
    }

    // 备用方案（不应该到达这里）
    let fallback = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        "./bin"
    } else {
        ".\\bin"
    };
    crate::logger::AppLogger::get().warn(&format!("使用备用 ffmpeg 路径: {}", fallback));
    fallback.to_string()
}

/// 获取视频信息
pub async fn get_video_info(url: &str) -> Result<VideoInfo, String> {
    let ytdlp_path = get_ytdlp_path();
    let ffmpeg_path = get_ffmpeg_path();

    // 使用自定义日志记录
    let logger = crate::logger::AppLogger::get();
    logger.info(&format!("开始获取视频信息: {}", url));
    logger.debug(&format!("yt-dlp 路径: {}", ytdlp_path));
    logger.debug(&format!("ffmpeg 路径: {}", ffmpeg_path));


    // 只在调试模式下打印
    #[cfg(debug_assertions)]
    {
        println!("[DEBUG] yt-dlp path: {}", ytdlp_path);
        println!("[DEBUG] ffmpeg path: {}", ffmpeg_path);
        println!("[DEBUG] Video URL: {}", url);
        println!("[DEBUG] Executing yt-dlp command...");
    }

    // 执行 yt-dlp 获取 JSON，包含所有格式
    let output = Command::new(&ytdlp_path)
        .arg("-J")  // 等同于 --dump-single-json
        .arg("--no-playlist")
        .arg("--all-formats")  // 获取所有可用格式
        .arg("--format-sort")
        .arg("res,fps,hdr:12,vcodec:vp9.2,acodec")  // 按分辨率、帧率、HDR排序
        .arg("--ffmpeg-location")
        .arg(&ffmpeg_path)  // 指定 ffmpeg 位置
        .arg(url)
        .output()
        .map_err(|e| {
            let error_msg = format!("Failed to execute yt-dlp: {}", e);
            logger.error(&error_msg);
            error_msg
        })?;

    #[cfg(debug_assertions)]
    println!("[DEBUG] yt-dlp execution completed");

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp error: {}", error));
    }

    // 解析 JSON 输出
    let json_str = String::from_utf8_lossy(&output.stdout);
    
    // 记录获取到的格式数量
    logger.debug(&format!("yt-dlp 输出长度: {} 字符", json_str.len()));
    
    let ytdlp_output: YtDlpOutput = serde_json::from_str(&json_str)
        .map_err(|e| {
            logger.error(&format!("解析 yt-dlp 输出失败: {}", e));
            logger.debug(&format!("原始输出: {}", &json_str[..json_str.len().min(1000)]));
            format!("Failed to parse yt-dlp output: {}", e)
        })?;

    let video_info: VideoInfo = ytdlp_output.into();
    
    // 记录找到的格式
    logger.info(&format!("找到 {} 个可用格式", video_info.formats.len()));
    // 只记录4K格式，避免过多日志
    for format in &video_info.formats {
        if format.quality_label.contains("2160") {
            logger.debug(&format!("发现 4K 格式: {} - {}", format.format_id, format.quality_label));
        }
    }

    Ok(video_info)
}

/// 下载视频（支持自动合并 DASH 格式和实时进度）
pub async fn download_video(
    url: &str,
    format_id: &str,
    output_path: &str,
    window: tauri::Window,
) -> Result<(), String> {
    let ytdlp_path = get_ytdlp_path();
    let ffmpeg_path = get_ffmpeg_path();

    let logger = crate::logger::AppLogger::get();

    // 记录下载开始
    logger.info(&format!("开始下载: URL={}, 格式={}, 输出路径={}", url, format_id, output_path));
    logger.debug(&format!("使用 ffmpeg 路径: {}", ffmpeg_path));

    // 如果是4K格式，给出提示
    if format_id.contains("701") || format_id.contains("315") || format_id.contains("337") {
        logger.info("检测到4K格式下载，将使用增强的网络重试策略");
    }

    #[cfg(debug_assertions)]
    {
        println!("[DEBUG] Starting download with format: {}", format_id);
        println!("[DEBUG] Output path: {}", output_path);
    }

    // 检查输出目录是否存在
    if !std::path::Path::new(output_path).exists() {
        if let Err(e) = std::fs::create_dir_all(output_path) {
            let error_msg = format!("无法创建输出目录: {}", e);
            logger.error(&error_msg);
            return Err(error_msg);
        }
    }

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
        #[cfg(debug_assertions)]
        println!("[DEBUG] DASH format detected, enabling merge");
        cmd.arg("--merge-output-format").arg("mp4");
    }

    // 添加网络和重试相关参数
    cmd.arg("--retries").arg("10")  // 重试10次
       .arg("--fragment-retries").arg("10")  // 片段重试10次
       .arg("--retry-sleep").arg("linear=1:5:10")  // 重试间隔：线性增长1-5-10秒
       .arg("--socket-timeout").arg("30")  // Socket 超时30秒
       .arg("--no-check-certificates")  // 跳过SSL证书检查（临时解决方案）
       .arg("--user-agent").arg("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")  // 设置用户代理
       .arg("--ffmpeg-location").arg(&ffmpeg_path)  // 指定 ffmpeg 位置
       .arg("--embed-metadata")  // 嵌入元数据
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

    // 发送初始进度状态
    let _ = window.emit("download-progress", crate::types::DownloadProgress {
        status: "downloading".to_string(),
        percent: 0.0,
        speed: 0.0,
        eta: 0.0,
        downloaded: 0,
        total: 0,
        filename: "Initializing download...".to_string(),
        file_path: None,
    });

    // 在后台任务中处理进度输出
    let window_clone = window.clone();
    let progress_handle = if let Some(stdout) = child.stdout.take() {
        Some(tokio::spawn(async move {
            parse_download_progress(stdout, window_clone).await;
        }))
    } else {
        None
    };

    // 捕获 stderr 输出用于错误诊断
    let stderr_handle = if let Some(stderr) = child.stderr.take() {
        Some(tokio::spawn(async move {
            use std::io::{BufRead, BufReader};
            let reader = BufReader::new(stderr);
            let mut error_output = String::new();
            let mut error_lines = Vec::new();

            for line in reader.lines() {
                if let Ok(line) = line {
                    // 只在调试模式下打印到控制台
                    #[cfg(debug_assertions)]
                    println!("[STDERR] {}", line);

                    error_output.push_str(&line);
                    error_output.push('\n');

                    // 只记录重要的错误信息，过滤进度和调试信息
                    if line.contains("ERROR") ||
                       line.contains("error") ||
                       line.contains("failed") ||
                       line.contains("SSL") ||
                       line.contains("certificate") ||
                       line.contains("timeout") {
                        error_lines.push(line.clone());
                    }
                }
            }

            // 批量记录错误（最多记录前10条重要错误）
            if !error_lines.is_empty() {
                let logger = crate::logger::AppLogger::get();
                let error_summary = if error_lines.len() > 10 {
                    format!("yt-dlp 发生 {} 个错误，前10个：\n{}",
                            error_lines.len(),
                            error_lines.iter().take(10).map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("\n"))
                } else {
                    format!("yt-dlp 错误：\n{}",
                            error_lines.iter().map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("\n"))
                };
                logger.error(&error_summary);
            }

            error_output
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
    
    // 获取错误输出
    let error_output = if let Some(handle) = stderr_handle {
        handle.await.unwrap_or_default()
    } else {
        String::new()
    };
    
    // 给进度解析一点时间完成
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    if status.success() {
        // 尝试找到下载的文件
        let file_path = find_downloaded_file(&output_path);
        
        // 获取文件大小
        let (downloaded_size, total_size) = if let Some(ref path) = file_path {
            match std::fs::metadata(path) {
                Ok(metadata) => {
                    let size = metadata.len();
                    (size, size)
                },
                Err(_) => {
                    // 如果无法获取文件大小，尝试使用一个合理的默认值
                    // 对于 DASH 格式，通常视频 + 音频文件会比较大
                    (100_000_000, 100_000_000) // 100MB 作为默认值
                }
            }
        } else {
            (100_000_000, 100_000_000) // 100MB 作为默认值
        };
        
        // 发送完成事件
        let _ = window.emit("download-progress", crate::types::DownloadProgress {
            status: "finished".to_string(),
            percent: 100.0,
            speed: 0.0,
            eta: 0.0,
            downloaded: downloaded_size,
            total: total_size,
            filename: "Download completed!".to_string(),
            file_path,
        });

        #[cfg(debug_assertions)]
        println!("[DEBUG] Download completed successfully");

        Ok(())
    } else {
        let error_msg = if !error_output.is_empty() {
            format!("Download failed with exit code: {:?}\nError details: {}", status.code(), error_output)
        } else {
            format!("Download failed with exit code: {:?}", status.code())
        };
        
        // 记录详细错误信息
        logger.error(&format!("下载失败: {}", error_msg));

        // 检查是否是SSL错误，如果是，尝试备用策略
        if error_output.contains("SSL") || error_output.contains("ssl") {
            logger.info("检测到SSL错误，建议尝试以下解决方案：");
            logger.info("1. 检查网络连接");
            logger.info("2. 尝试选择较低分辨率的格式");
            logger.info("3. 稍后重试");
        }
        
        let _ = window.emit("download-progress", crate::types::DownloadProgress {
            status: "error".to_string(),
            percent: 0.0,
            speed: 0.0,
            eta: 0.0,
            downloaded: 0,
            total: 0,
            filename: error_msg.clone(),
            file_path: None,
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
    let mut _total_downloaded = 0u64; // 跟踪总下载量
    
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
                    filename: "Merging video and audio...".to_string(),
                    file_path: None,
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
                    filename: "Adding metadata...".to_string(),
                    file_path: None,
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
                    filename: "Cleaning up temporary files...".to_string(),
                    file_path: None,
                });
                continue;
            }
            
            // 检测 100% 完成
            if let Some(captures) = complete_regex.captures(&line) {
                let total_str = captures.get(1).unwrap().as_str();
                let total_bytes = parse_size_to_bytes(total_str);
                last_total_bytes = total_bytes;
                _total_downloaded += total_bytes; // 累加下载量
                
                let progress = crate::types::DownloadProgress {
                    status: "downloading".to_string(),
                    percent: 100.0,
                    speed: 0.0,
                    eta: 0.0,
                    downloaded: total_bytes,
                    total: total_bytes,
                    filename: current_filename.clone(),
                    file_path: None,
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
                    file_path: None,
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

/// 查找下载的文件
fn find_downloaded_file(output_path: &str) -> Option<String> {
    use std::fs;

    
    // 尝试读取输出目录
    if let Ok(entries) = fs::read_dir(output_path) {
        let mut video_files = Vec::new();
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    // 查找视频文件
                    if matches!(ext_str.as_str(), "mp4" | "mkv" | "webm" | "avi" | "mov" | "m4a" | "mp3") {
                        // 排除缩略图文件
                        if let Some(name) = path.file_name() {
                            let name_str = name.to_string_lossy();
                            if !name_str.contains("thumbnail") && !name_str.contains("thumb") && !name_str.contains(".jpg") {
                                if let Ok(metadata) = path.metadata() {
                                    if let Ok(modified) = metadata.modified() {
                                        video_files.push((path.to_string_lossy().to_string(), modified));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // 返回最新修改的文件
        video_files.sort_by(|a, b| b.1.cmp(&a.1));
        video_files.first().map(|(path, _)| path.clone())
    } else {
        None
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
/// 网络连接诊断
pub async fn diagnose_network_issue(url: &str) -> Result<String, String> {
    let ytdlp_path = get_ytdlp_path();
    let ffmpeg_path = get_ffmpeg_path();

    // 测试基本连接
    let output = Command::new(&ytdlp_path)
        .arg("--simulate")
        .arg("--no-playlist")
        .arg("--verbose")
        .arg("--ffmpeg-location")
        .arg(&ffmpeg_path)
        .arg(url)
        .output()
        .map_err(|e| format!("Failed to run diagnostic: {}", e))?;
    
    let _stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    let mut diagnosis = String::new();
    
    if stderr.contains("SSL") {
        diagnosis.push_str("🔍 SSL连接问题检测到\n");
        diagnosis.push_str("建议解决方案:\n");
        diagnosis.push_str("• 检查网络连接稳定性\n");
        diagnosis.push_str("• 尝试使用VPN或更换网络\n");
        diagnosis.push_str("• 选择较低分辨率格式\n");
    }
    
    if stderr.contains("timeout") || stderr.contains("Timeout") {
        diagnosis.push_str("⏱️ 网络超时问题\n");
        diagnosis.push_str("建议:\n");
        diagnosis.push_str("• 检查网络速度\n");
        diagnosis.push_str("• 稍后重试\n");
    }
    
    if diagnosis.is_empty() {
        diagnosis.push_str("✅ 网络连接正常，可能是临时问题，请重试");
    }
    
    Ok(diagnosis)
}

/// 获取推荐的备用格式
pub fn get_fallback_formats(original_format: &str) -> Vec<String> {
    let mut fallbacks = Vec::new();
    
    // 如果原格式是4K，提供降级选项
    if original_format.contains("701") || original_format.contains("315") || original_format.contains("337") {
        // 4K -> 1440p
        fallbacks.push("308+258".to_string()); // 1440p60 webm + audio
        fallbacks.push("299+258".to_string()); // 1080p60 mp4 + audio
        fallbacks.push("136+258".to_string()); // 720p mp4 + audio
        fallbacks.push("best[height<=1080]".to_string()); // 最佳1080p或以下
    } else if original_format.contains("308") {
        // 1440p -> 1080p
        fallbacks.push("299+258".to_string());
        fallbacks.push("136+258".to_string());
        fallbacks.push("best[height<=720]".to_string());
    }
    
    // 通用备用选项
    fallbacks.push("best".to_string());
    fallbacks.push("worst".to_string());
    
    fallbacks
}
