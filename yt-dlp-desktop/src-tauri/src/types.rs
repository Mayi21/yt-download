use serde::{Deserialize, Serialize};

/// 视频信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub duration: u32,
    pub thumbnail: String,
    pub uploader: String,
    pub upload_date: String,
    pub description: String,
    pub view_count: u64,
    pub formats: Vec<VideoFormat>,
}

/// 视频格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFormat {
    pub format_id: String,
    pub ext: String,
    pub resolution: String,
    pub quality_label: String,
    pub filesize: Option<u64>,
    pub fps: Option<f64>,  // fps 可能是浮点数或 null
    pub vcodec: String,
    pub acodec: String,
    pub tbr: Option<f64>,  // tbr 也可能是 null
    pub format_note: String,
    pub hdr: bool,         // 是否支持 HDR
    pub is_dash: bool,     // 是否是 DASH 格式
}

/// 下载配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    pub url: String,
    pub format_id: String,
    pub output_path: String,
    pub audio_only: bool,
    pub include_subtitles: bool,
    pub prefer_hdr: bool,
}

/// 下载进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub status: String,
    pub percent: f64,
    pub speed: f64,
    pub eta: f64,
    pub downloaded: u64,
    pub total: u64,
    pub filename: String,
}

/// 下载历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadHistoryItem {
    pub id: String,
    pub title: String,
    pub url: String,
    pub file_path: String,
    pub download_date: String,
    pub thumbnail: String,
}

/// yt-dlp JSON 输出格式（部分字段）
#[derive(Debug, Deserialize)]
pub struct YtDlpOutput {
    pub id: String,
    pub title: String,
    pub duration: Option<u32>,
    pub thumbnail: Option<String>,
    pub uploader: Option<String>,
    pub upload_date: Option<String>,
    pub description: Option<String>,
    pub view_count: Option<u64>,
    pub formats: Option<Vec<YtDlpFormat>>,
}

/// yt-dlp 格式信息
#[derive(Debug, Deserialize)]
pub struct YtDlpFormat {
    pub format_id: String,
    pub ext: String,
    pub resolution: Option<String>,
    pub height: Option<u32>,
    pub filesize: Option<u64>,
    pub fps: Option<f64>,  // fps 可能是浮点数
    pub vcodec: Option<String>,
    pub acodec: Option<String>,
    pub tbr: Option<f64>,
    pub format_note: Option<String>,
    pub hdr: Option<String>,  // HDR 信息
}

impl From<YtDlpOutput> for VideoInfo {
    fn from(output: YtDlpOutput) -> Self {
        let formats = output
            .formats
            .unwrap_or_default()
            .into_iter()
            .filter_map(|f| {
                println!("[DEBUG] Processing format: {} - {}x{:?} - {} - {}", 
                    f.format_id, 
                    f.ext, 
                    f.height,
                    f.vcodec.as_deref().unwrap_or("none"),
                    f.acodec.as_deref().unwrap_or("none")
                );
                
                // 过滤掉无用的格式（如 storyboard）
                if f.ext == "mhtml" || f.format_id.starts_with("sb") {
                    return None;
                }
                
                // 只保留有视频或音频的格式（排除两者都为 none 的格式）
                if f.vcodec.as_deref() == Some("none") && f.acodec.as_deref() == Some("none") {
                    return None;
                }

                // 检查是否是 HDR
                let is_hdr = f.hdr.as_deref() == Some("10") || 
                            f.format_note.as_deref().unwrap_or("").contains("HDR");
                
                // 检查是否是 DASH 格式
                let is_dash = f.format_note.as_deref().unwrap_or("").contains("dash") ||
                             (f.vcodec.as_deref() != Some("none") && f.acodec.as_deref() == Some("none")) ||
                             (f.vcodec.as_deref() == Some("none") && f.acodec.as_deref() != Some("none"));

                Some(VideoFormat {
                    format_id: f.format_id,
                    ext: f.ext,
                    resolution: f.resolution.clone().unwrap_or_else(|| {
                        if let Some(height) = f.height {
                            format!("{}x{}", (height * 16 / 9), height)
                        } else if f.vcodec.as_deref() == Some("none") {
                            "audio only".to_string()
                        } else {
                            "unknown".to_string()
                        }
                    }),
                    quality_label: f.height.map(|h| format!("{}p", h)).unwrap_or_else(|| {
                        if f.vcodec.as_deref() == Some("none") {
                            "audio".to_string()
                        } else {
                            "unknown".to_string()
                        }
                    }),
                    filesize: f.filesize,
                    fps: f.fps,
                    vcodec: f.vcodec.unwrap_or_else(|| "unknown".to_string()),
                    acodec: f.acodec.unwrap_or_else(|| "unknown".to_string()),
                    tbr: f.tbr,
                    format_note: f.format_note.unwrap_or_default(),
                    hdr: is_hdr,
                    is_dash,
                })
            })
            .collect();

        VideoInfo {
            id: output.id,
            title: output.title,
            duration: output.duration.unwrap_or(0),
            thumbnail: output.thumbnail.unwrap_or_default(),
            uploader: output.uploader.unwrap_or_default(),
            upload_date: output.upload_date.unwrap_or_default(),
            description: output.description.unwrap_or_default(),
            view_count: output.view_count.unwrap_or(0),
            formats,
        }
    }
}
