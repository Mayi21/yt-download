// 模块声明
mod commands;
mod config;
mod types;
mod ytdlp;
mod logger;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化自定义日志（单例模式会自动初始化）
    let logger = logger::AppLogger::get();
    logger.info("YouTube Downloader 应用启动");
    println!("日志文件位置: {:?}", logger.get_log_path());

    // 在调试模式下同时初始化 env_logger
    #[cfg(debug_assertions)]
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_video_info,
            list_formats,
            start_download,
            cancel_download,
            select_save_path,
            get_app_config,
            save_app_config,
            get_default_save_path,
            set_default_save_path,
            get_download_history,
            clear_download_history,
            open_file_location,
            get_log_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
