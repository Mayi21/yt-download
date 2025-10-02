import { invoke } from '@tauri-apps/api/tauri';
import type {
  VideoInfo,
  VideoFormat,
  DownloadConfig,
  DownloadProgress,
  DownloadHistoryItem,
} from '../types';
import { mockVideoInfo, mockDownloadProgress, mockDownloadHistory, delay } from './mock';

// 通过环境变量控制是否使用 Mock 数据
// 开发环境默认使用 Mock，生产环境使用真实 API
const USE_MOCK = import.meta.env.DEV && import.meta.env.VITE_USE_MOCK !== 'false';

/**
 * 获取视频信息
 * @param url YouTube 视频 URL
 * @returns 视频信息对象
 */
export async function getVideoInfo(url: string): Promise<VideoInfo> {
  if (USE_MOCK) {
    console.log('[Mock API] Getting video info for:', url);
    await delay(1000); // 模拟网络延迟
    return mockVideoInfo;
  }

  // 真实 API 调用
  return invoke<VideoInfo>('get_video_info', { url });
}

/**
 * 列出可用格式
 * @param url YouTube 视频 URL
 * @returns 格式列表
 */
export async function listFormats(url: string): Promise<VideoFormat[]> {
  if (USE_MOCK) {
    console.log('[Mock API] Listing formats for:', url);
    await delay(500);
    return mockVideoInfo.formats;
  }

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
  if (USE_MOCK) {
    console.log('[Mock API] Starting download:', config);
    return mockDownloadProgress(onProgress);
  }

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
  if (USE_MOCK) {
    console.log('[Mock API] Cancelling download:', downloadId);
    await delay(200);
    return;
  }

  return invoke('cancel_download', { downloadId });
}

/**
 * 选择保存路径
 * @returns 选择的路径，如果取消则返回 null
 */
export async function selectSavePath(): Promise<string | null> {
  if (USE_MOCK) {
    console.log('[Mock API] Selecting save path');
    await delay(500);
    return '/Users/username/Downloads';
  }

  return invoke<string | null>('select_save_path');
}

/**
 * 获取下载历史
 * @returns 历史记录列表
 */
export async function getDownloadHistory(): Promise<DownloadHistoryItem[]> {
  if (USE_MOCK) {
    console.log('[Mock API] Getting download history');
    await delay(300);
    return mockDownloadHistory;
  }

  return invoke<DownloadHistoryItem[]>('get_download_history');
}

/**
 * 清空下载历史
 */
export async function clearDownloadHistory(): Promise<void> {
  if (USE_MOCK) {
    console.log('[Mock API] Clearing download history');
    await delay(200);
    return;
  }

  return invoke('clear_download_history');
}

/**
 * 打开文件位置
 * @param filePath 文件路径
 */
export async function openFileLocation(filePath: string): Promise<void> {
  if (USE_MOCK) {
    console.log('[Mock API] Opening file location:', filePath);
    await delay(100);
    alert(`Would open: ${filePath}`);
    return;
  }

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
