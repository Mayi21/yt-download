use crate::config::AppConfig;
use crate::types::{DownloadConfig, DownloadHistoryItem, VideoFormat, VideoInfo};
use crate::ytdlp;
use tauri::AppHandle;

/// 获取视频信息
#[tauri::command]
pub async fn get_video_info(url: String) -> Result<VideoInfo, String> {
    println!("[COMMAND] get_video_info called with URL: {}", url);
    let result = ytdlp::get_video_info(&url).await;
    match &result {
        Ok(_) => println!("[COMMAND] get_video_info succeeded"),
        Err(e) => println!("[COMMAND] get_video_info failed: {}", e),
    }
    result
}

/// 列出可用格式
#[tauri::command]
pub async fn list_formats(url: String) -> Result<Vec<VideoFormat>, String> {
    let video_info = ytdlp::get_video_info(&url).await?;
    Ok(video_info.formats)
}

/// 开始下载
#[tauri::command]
pub async fn start_download(config: DownloadConfig, window: tauri::Window) -> Result<String, String> {
    println!("[COMMAND] start_download called");
    println!("[COMMAND] URL: {}", config.url);
    println!("[COMMAND] Format ID: {}", config.format_id);
    println!("[COMMAND] Output path: {}", config.output_path);
    println!("[COMMAND] Audio only: {}", config.audio_only);
    println!("[COMMAND] Include subtitles: {}", config.include_subtitles);

    // 如果是仅音频下载，修改格式选择
    let final_format_id = if config.audio_only {
        // 对于仅音频，使用 bestaudio 或指定的音频格式
        if config.format_id.contains("audio") {
            config.format_id
        } else {
            "bestaudio/best".to_string()
        }
    } else {
        config.format_id
    };

    println!("[COMMAND] Final format ID: {}", final_format_id);

    // 启动下载（传递 window 用于进度事件）
    ytdlp::download_video(&config.url, &final_format_id, &config.output_path, window).await?;

    println!("[COMMAND] Download started successfully");
    
    // 返回下载任务 ID（这里简化为 URL）
    Ok(config.url)
}

/// 取消下载
#[tauri::command]
pub async fn cancel_download(download_id: String) -> Result<(), String> {
    // TODO: 实现取消下载逻辑
    // 需要维护一个下载任务列表，并能够终止进程
    println!("Cancelling download: {}", download_id);
    Ok(())
}

/// 选择保存路径
#[tauri::command]
pub async fn select_save_path(app: AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    // 使用 Tauri 的文件对话框
    let result = app.dialog()
        .file()
        .set_title("选择保存位置")
        .blocking_pick_folder();

    Ok(result.and_then(|path| path.as_path().map(|p| p.to_string_lossy().to_string())))
}

/// 获取应用配置
#[tauri::command]
pub async fn get_app_config(app: AppHandle) -> Result<AppConfig, String> {
    AppConfig::load(&app)
}

/// 保存应用配置
#[tauri::command]
pub async fn save_app_config(config: AppConfig, app: AppHandle) -> Result<(), String> {
    config.save(&app)
}

/// 获取默认保存路径
#[tauri::command]
pub async fn get_default_save_path(app: AppHandle) -> Result<Option<String>, String> {
    let config = AppConfig::load(&app)?;
    Ok(config.get_default_save_path())
}

/// 设置默认保存路径
#[tauri::command]
pub async fn set_default_save_path(path: String, app: AppHandle) -> Result<(), String> {
    let mut config = AppConfig::load(&app)?;
    config.default_save_path = Some(path);
    config.save(&app)
}

/// 获取下载历史（Mock 实现）
#[tauri::command]
pub async fn get_download_history() -> Result<Vec<DownloadHistoryItem>, String> {
    // TODO: 实现真实的历史记录存储（使用文件或数据库）
    Ok(vec![])
}

/// 清空下载历史
#[tauri::command]
pub async fn clear_download_history() -> Result<(), String> {
    // TODO: 实现清空历史记录
    Ok(())
}

/// 打开文件位置
#[tauri::command]
pub async fn open_file_location(file_path: String) -> Result<(), String> {
    use std::process::Command;

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-R")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open file location: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg("/select,")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open file location: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 上使用 xdg-open 打开父目录
        let path = std::path::Path::new(&file_path);
        if let Some(parent) = path.parent() {
            Command::new("xdg-open")
                .arg(parent)
                .spawn()
                .map_err(|e| format!("Failed to open file location: {}", e))?;
        }
    }

    Ok(())
}
