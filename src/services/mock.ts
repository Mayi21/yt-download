import type {
  VideoInfo,
  DownloadProgress,
  DownloadHistoryItem,
} from '../types';

/**
 * Mock 视频信息数据
 */
export const mockVideoInfo: VideoInfo = {
  id: 'dQw4w9WgXcQ',
  title: 'Rick Astley - Never Gonna Give You Up (Official Video)',
  duration: 212,
  thumbnail: 'https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg',
  uploader: 'Rick Astley',
  upload_date: '20091025',
  description:
    'The official video for "Never Gonna Give You Up" by Rick Astley. The new album 'Are We There Yet?' is out now...',
  view_count: 1400000000,
  formats: [
    {
      format_id: '137+140',
      ext: 'mp4',
      resolution: '1920x1080',
      quality_label: '1080p',
      filesize: 52428800,
      fps: 30,
      vcodec: 'avc1.640028',
      acodec: 'mp4a.40.2',
      tbr: 2500,
      format_note: 'DASH video',
    },
    {
      format_id: '136+140',
      ext: 'mp4',
      resolution: '1280x720',
      quality_label: '720p',
      filesize: 32428800,
      fps: 30,
      vcodec: 'avc1.4d401f',
      acodec: 'mp4a.40.2',
      tbr: 1500,
      format_note: 'DASH video',
    },
    {
      format_id: '135+140',
      ext: 'mp4',
      resolution: '854x480',
      quality_label: '480p',
      filesize: 18428800,
      fps: 30,
      vcodec: 'avc1.4d401e',
      acodec: 'mp4a.40.2',
      tbr: 800,
      format_note: 'DASH video',
    },
    {
      format_id: '134+140',
      ext: 'mp4',
      resolution: '640x360',
      quality_label: '360p',
      filesize: 12428800,
      fps: 30,
      vcodec: 'avc1.4d401e',
      acodec: 'mp4a.40.2',
      tbr: 500,
      format_note: 'DASH video',
    },
    {
      format_id: '140',
      ext: 'm4a',
      resolution: 'audio only',
      quality_label: '128k',
      filesize: 3428800,
      fps: 0,
      vcodec: 'none',
      acodec: 'mp4a.40.2',
      tbr: 128,
      format_note: 'DASH audio',
    },
  ],
};

/**
 * Mock 下载历史数据
 */
export const mockDownloadHistory: DownloadHistoryItem[] = [
  {
    id: '1',
    title: 'Sample Video 1',
    url: 'https://www.youtube.com/watch?v=example1',
    file_path: '/Users/username/Downloads/video1.mp4',
    download_date: '2024-01-15T10:30:00Z',
    thumbnail: 'https://i.ytimg.com/vi/example1/default.jpg',
  },
  {
    id: '2',
    title: 'Sample Video 2',
    url: 'https://www.youtube.com/watch?v=example2',
    file_path: '/Users/username/Downloads/video2.mp4',
    download_date: '2024-01-14T15:20:00Z',
    thumbnail: 'https://i.ytimg.com/vi/example2/default.jpg',
  },
];

/**
 * 模拟下载进度
 * @param callback 进度回调函数
 * @returns 清理函数
 */
export function mockDownloadProgress(
  callback: (progress: DownloadProgress) => void
): () => void {
  let percent = 0;
  let isCancelled = false;

  const interval = setInterval(() => {
    if (isCancelled) {
      clearInterval(interval);
      return;
    }

    // 每次增加 2-7% 的进度
    percent += Math.random() * 5 + 2;

    if (percent >= 100) {
      percent = 100;
      callback({
        status: 'finished',
        percent: 100,
        speed: 0,
        eta: 0,
        downloaded: 52428800,
        total: 52428800,
        filename: 'Rick_Astley_Never_Gonna_Give_You_Up.mp4',
      });
      clearInterval(interval);
      return;
    }

    // 模拟下载中
    callback({
      status: 'downloading',
      percent: Math.floor(percent),
      speed: 1024 * 1024 * (2 + Math.random()), // 2-3 MB/s
      eta: Math.floor((100 - percent) * 0.8),
      downloaded: Math.floor(52428800 * (percent / 100)),
      total: 52428800,
      filename: 'Rick_Astley_Never_Gonna_Give_You_Up.mp4',
    });
  }, 500);

  // 返回清理函数
  return () => {
    isCancelled = true;
    clearInterval(interval);
  };
}

/**
 * 模拟网络延迟
 */
export const delay = (ms: number): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, ms));
