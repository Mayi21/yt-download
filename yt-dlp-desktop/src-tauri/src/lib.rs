// 模块声明
mod commands;
mod config;
mod types;
mod ytdlp;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
