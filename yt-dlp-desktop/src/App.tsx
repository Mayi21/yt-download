import { useState } from 'react';
import type { VideoInfo, DownloadProgress as DownloadProgressType, DownloadConfig } from './types';
import { UrlInput } from './components/UrlInput';
import { VideoInfo as VideoInfoComponent } from './components/VideoInfo';
import { FormatSelector } from './components/FormatSelector';
import { DownloadProgress } from './components/DownloadProgress';
import { getVideoInfo, startDownload, selectSavePath } from './services/api';
import './styles/index.css';

function App() {
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [savePath, setSavePath] = useState<string>('');
  const [downloadProgress, setDownloadProgress] = useState<DownloadProgressType | null>(null);
  const [cleanupDownload, setCleanupDownload] = useState<(() => void) | null>(null);

  // 获取视频信息
  const handleFetchVideo = async (url: string) => {
    setIsLoading(true);
    setError(null);
    setVideoInfo(null);
    setDownloadProgress(null);

    try {
      const info = await getVideoInfo(url);
      setVideoInfo(info);

      // 自动选择默认保存路径
      const defaultPath = await selectSavePath();
      if (defaultPath) {
        setSavePath(defaultPath);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : '获取视频信息失败');
      console.error('Failed to fetch video info:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // 开始下载
  const handleStartDownload = (formatId: string, audioOnly: boolean, includeSubtitles: boolean) => {
    if (!videoInfo || !savePath) {
      alert('请先选择保存路径');
      return;
    }

    const config: DownloadConfig = {
      url: `https://www.youtube.com/watch?v=${videoInfo.id}`,
      format_id: formatId,
      output_path: savePath,
      audio_only: audioOnly,
      include_subtitles: includeSubtitles,
    };

    // 开始下载并监听进度
    const cleanup = startDownload(config, (progress) => {
      setDownloadProgress(progress);
    });

    setCleanupDownload(() => cleanup);
  };

  // 取消下载
  const handleCancelDownload = () => {
    if (cleanupDownload) {
      cleanupDownload();
      setCleanupDownload(null);
    }
    setDownloadProgress(null);
  };

  // 选择保存路径
  const handleSelectPath = async () => {
    const path = await selectSavePath();
    if (path) {
      setSavePath(path);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 p-6">
      <div className="max-w-4xl mx-auto space-y-6">
        {/* 标题 */}
        <header className="text-center py-6">
          <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-2">
            📺 YouTube Downloader
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            快速下载 YouTube 视频，支持多种格式和清晰度
          </p>
        </header>

        {/* URL 输入 */}
        <UrlInput onFetch={handleFetchVideo} isLoading={isLoading} />

        {/* 错误提示 */}
        {error && (
          <div className="card bg-error text-white">
            <p>❌ {error}</p>
          </div>
        )}

        {/* 视频信息 */}
        {videoInfo && (
          <>
            <VideoInfoComponent video={videoInfo} />

            {/* 保存路径选择 */}
            <div className="card">
              <label className="block text-sm font-medium mb-2">保存位置</label>
              <div className="flex gap-2">
                <input
                  type="text"
                  value={savePath}
                  readOnly
                  className="input flex-1"
                  placeholder="选择保存位置..."
                />
                <button onClick={handleSelectPath} className="btn btn-secondary">
                  📁 浏览
                </button>
              </div>
            </div>

            {/* 格式选择器 */}
            {!downloadProgress && (
              <FormatSelector formats={videoInfo.formats} onSelect={handleStartDownload} />
            )}
          </>
        )}

        {/* 下载进度 */}
        {downloadProgress && (
          <DownloadProgress progress={downloadProgress} onCancel={handleCancelDownload} />
        )}

        {/* 页脚 */}
        <footer className="text-center text-sm text-gray-500 dark:text-gray-400 py-4">
          <p>基于 yt-dlp 构建 | Tauri + React</p>
        </footer>
      </div>
    </div>
  );
}

export default App;
