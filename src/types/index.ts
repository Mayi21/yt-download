/**
 * 视频信息
 */
export interface VideoInfo {
  id: string;                    // 视频 ID
  title: string;                 // 标题
  duration: number;              // 时长（秒）
  thumbnail: string;             // 缩略图 URL
  uploader: string;              // 上传者
  upload_date: string;           // 上传日期 (YYYYMMDD)
  description: string;           // 描述
  view_count: number;            // 观看次数
  formats: VideoFormat[];        // 可用格式列表
}

/**
 * 视频格式
 */
export interface VideoFormat {
  format_id: string;             // 格式 ID
  ext: string;                   // 扩展名 (mp4, webm, mkv)
  resolution: string;            // 分辨率 (1920x1080)
  quality_label: string;         // 清晰度标签 (1080p, 720p)
  filesize: number | null;       // 文件大小（字节）
  fps: number;                   // 帧率
  vcodec: string;                // 视频编码
  acodec: string;                // 音频编码
  tbr: number;                   // 比特率
  format_note: string;           // 格式说明
}

/**
 * 下载进度状态
 */
export type DownloadStatus = 'downloading' | 'processing' | 'finished' | 'error';

/**
 * 下载进度
 */
export interface DownloadProgress {
  status: DownloadStatus;
  percent: number;               // 进度百分比 (0-100)
  speed: number;                 // 下载速度（bytes/s）
  eta: number;                   // 预计剩余时间（秒）
  downloaded: number;            // 已下载字节数
  total: number;                 // 总字节数
  filename: string;              // 文件名
}

/**
 * 下载配置
 */
export interface DownloadConfig {
  url: string;                   // 视频 URL
  format_id: string;             // 选择的格式 ID
  output_path: string;           // 输出路径
  audio_only: boolean;           // 是否仅下载音频
  include_subtitles: boolean;    // 是否包含字幕
}

/**
 * 下载结果
 */
export interface DownloadResult {
  success: boolean;
  file_path: string;
  error?: string;
}

/**
 * 下载历史记录
 */
export interface DownloadHistoryItem {
  id: string;
  title: string;
  url: string;
  file_path: string;
  download_date: string;
  thumbnail: string;
}

/**
 * 清晰度选项
 */
export type QualityOption = '2160p' | '1080p' | '720p' | '480p' | '360p';

/**
 * 格式选项
 */
export type FormatOption = 'mp4' | 'webm' | 'mkv';
