import type {
  VideoInfo,
  VideoFormat,
  DownloadConfig,
  DownloadResult,
  DownloadHistoryItem,
} from './index';

/**
 * Tauri 后端命令接口
 * 这些是前端调用后端的所有可用命令
 */
export interface TauriCommands {
  /**
   * 获取视频信息
   * @param url YouTube 视频 URL
   * @returns 视频信息对象
   * @throws 如果 URL 无效或无法获取信息
   */
  get_video_info(url: string): Promise<VideoInfo>;

  /**
   * 列出可用格式
   * @param url YouTube 视频 URL
   * @returns 格式列表
   */
  list_formats(url: string): Promise<VideoFormat[]>;

  /**
   * 开始下载
   * @param config 下载配置
   * @returns 下载任务 ID
   */
  start_download(config: DownloadConfig): Promise<string>;

  /**
   * 取消下载
   * @param download_id 下载任务 ID
   */
  cancel_download(download_id: string): Promise<void>;

  /**
   * 选择保存路径
   * @returns 选择的路径，如果取消则返回 null
   */
  select_save_path(): Promise<string | null>;

  /**
   * 获取下载历史
   * @returns 历史记录列表
   */
  get_download_history(): Promise<DownloadHistoryItem[]>;

  /**
   * 清空下载历史
   */
  clear_download_history(): Promise<void>;

  /**
   * 打开文件位置
   * @param file_path 文件路径
   */
  open_file_location(file_path: string): Promise<void>;
}
