import { invoke } from '@tauri-apps/api/core';
import type {
  VideoInfo,
  VideoFormat,
  DownloadConfig,
  DownloadProgress,
  DownloadHistoryItem,
} from '../types';

/**
 * 获取视频信息
 * @param url YouTube 视频 URL
 * @returns 视频信息对象
 */
export async function getVideoInfo(url: string): Promise<VideoInfo> {
  return invoke<VideoInfo>('get_video_info', { url });
}

/**
 * 列出可用格式
 * @param url YouTube 视频 URL
 * @returns 格式列表
 */
export async function listFormats(url: string): Promise<VideoFormat[]> {
  return invoke<VideoFormat[]>('list_formats', { url });
}

/**
 * 开始下载
 * @param config 下载配置
 * @param onProgress 进度回调
 * @returns 清理函数
 */
export function startDownload(
  config: DownloadConfig,
  onProgress: (progress: DownloadProgress) => void
): () => void {
  // 真实实现：使用 Tauri Event 监听进度
  // TODO: 实现真实的下载和进度监听
  invoke('start_download', { config });

  return () => {
    console.log('Download cleanup');
  };
}

/**
 * 取消下载
 * @param downloadId 下载任务 ID
 */
export async function cancelDownload(downloadId: string): Promise<void> {
  return invoke('cancel_download', { downloadId });
}

/**
 * 选择保存路径
 * @returns 选择的路径，如果取消则返回 null
 */
export async function selectSavePath(): Promise<string | null> {
  return invoke<string | null>('select_save_path');
}

/**
 * 获取下载历史
 * @returns 历史记录列表
 */
export async function getDownloadHistory(): Promise<DownloadHistoryItem[]> {
  return invoke<DownloadHistoryItem[]>('get_download_history');
}

/**
 * 清空下载历史
 */
export async function clearDownloadHistory(): Promise<void> {
  return invoke('clear_download_history');
}

/**
 * 打开文件位置
 * @param filePath 文件路径
 */
export async function openFileLocation(filePath: string): Promise<void> {
  return invoke('open_file_location', { filePath });
}

/**
 * 验证 YouTube URL
 * @param url 待验证的 URL
 * @returns 是否有效
 */
export function isValidYouTubeUrl(url: string): boolean {
  const patterns = [
    /^https?:\/\/(www\.)?youtube\.com\/watch\?v=[\w-]+/,
    /^https?:\/\/youtu\.be\/[\w-]+/,
    /^https?:\/\/(www\.)?youtube\.com\/shorts\/[\w-]+/,
  ];

  return patterns.some((pattern) => pattern.test(url));
}

/**
 * 格式化文件大小
 * @param bytes 字节数
 * @returns 格式化后的字符串
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';

  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${units[i]}`;
}

/**
 * 格式化下载速度
 * @param bytesPerSecond 每秒字节数
 * @returns 格式化后的字符串
 */
export function formatSpeed(bytesPerSecond: number): string {
  return `${formatFileSize(bytesPerSecond)}/s`;
}

/**
 * 格式化时间（秒转为可读格式）
 * @param seconds 秒数
 * @returns 格式化后的字符串
 */
export function formatTime(seconds: number): string {
  if (seconds < 60) {
    return `${Math.floor(seconds)} 秒`;
  }

  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = Math.floor(seconds % 60);

  if (minutes < 60) {
    return `${minutes} 分 ${remainingSeconds} 秒`;
  }

  const hours = Math.floor(minutes / 60);
  const remainingMinutes = Math.floor(minutes % 60);

  return `${hours} 小时 ${remainingMinutes} 分`;
}
