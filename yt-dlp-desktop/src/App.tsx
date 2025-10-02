import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import type { VideoInfo, DownloadProgress as DownloadProgressType, DownloadConfig } from './types';
import { UrlInput } from './components/UrlInput';
import { VideoInfo as VideoInfoComponent } from './components/VideoInfo';
import { FormatSelector } from './components/FormatSelector';
import { DownloadProgress } from './components/DownloadProgress';
import { LanguageSwitcher } from './components/LanguageSwitcher';
import { getVideoInfo, startDownload, selectSavePath, getDefaultSavePath } from './services/api';
import './styles/index.css';

function App() {
  const { t } = useTranslation();
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [savePath, setSavePath] = useState<string>('');
  const [downloadProgress, setDownloadProgress] = useState<DownloadProgressType | null>(null);
  const [cleanupDownload, setCleanupDownload] = useState<(() => void) | null>(null);
  const [isDownloadStarting, setIsDownloadStarting] = useState(false);

  // 初始化默认保存路径
  useEffect(() => {
    const initDefaultPath = async () => {
      try {
        const defaultPath = await getDefaultSavePath();
        if (defaultPath) {
          setSavePath(defaultPath);
        }
      } catch (error) {
        console.error('Failed to get default save path:', error);
      }
    };
    initDefaultPath();
  }, []);

  // 获取视频信息
  const handleFetchVideo = async (url: string) => {
    setIsLoading(true);
    setError(null);
    setVideoInfo(null);
    setDownloadProgress(null);

    try {
      const info = await getVideoInfo(url);
      setVideoInfo(info);

      // 如果没有设置保存路径，尝试获取默认路径
      if (!savePath) {
        const defaultPath = await getDefaultSavePath();
        if (defaultPath) {
          setSavePath(defaultPath);
        }
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : t('errors.fetchFailed'));
      console.error('Failed to fetch video info:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // 开始下载
  const handleStartDownload = (formatId: string, audioOnly: boolean, includeSubtitles: boolean, preferHdr: boolean = false) => {
    if (!videoInfo || !savePath) {
      alert(t('errors.selectPathFirst'));
      return;
    }

    setIsDownloadStarting(true);

    const config: DownloadConfig = {
      url: `https://www.youtube.com/watch?v=${videoInfo.id}`,
      format_id: formatId,
      output_path: savePath,
      audio_only: audioOnly,
      include_subtitles: includeSubtitles,
      prefer_hdr: preferHdr,
    };

    // 开始下载并监听进度
    const cleanup = startDownload(config, (progress) => {
      setIsDownloadStarting(false);
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
    setIsDownloadStarting(false);
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
          <div className="flex justify-between items-start mb-4">
            <div></div>
            <div className="text-center">
              <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-2">
                📺 {t('app.title')}
              </h1>
              <p className="text-gray-600 dark:text-gray-400">
                {t('app.subtitle')}
              </p>
            </div>
            <LanguageSwitcher />
          </div>
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
              <label className="block text-sm font-medium mb-2">{t('download.savePath')}</label>
              <div className="flex gap-2">
                <input
                  type="text"
                  value={savePath}
                  readOnly
                  className="input flex-1"
                  placeholder={t('download.selectPath')}
                />
                <button onClick={handleSelectPath} className="btn btn-secondary">
                  📁 {t('download.browse')}
                </button>
              </div>
            </div>

            {/* 格式选择器 */}
            {!downloadProgress && !isDownloadStarting && (
              <FormatSelector 
                formats={videoInfo.formats} 
                onSelect={handleStartDownload}
                isDownloading={isDownloadStarting}
              />
            )}

            {/* 下载启动中状态 */}
            {isDownloadStarting && (
              <div className="card">
                <div className="flex items-center justify-center gap-3 py-8">
                  <div className="animate-spin w-8 h-8 border-4 border-primary border-t-transparent rounded-full"></div>
                  <span className="text-lg font-medium">{t('progress.preparingDownload')}</span>
                </div>
              </div>
            )}
          </>
        )}

        {/* 下载进度 */}
        {downloadProgress && (
          <DownloadProgress progress={downloadProgress} onCancel={handleCancelDownload} />
        )}

        {/* 页脚 */}
        <footer className="text-center text-sm text-gray-500 dark:text-gray-400 py-4">
          <p>{t('footer.poweredBy')}</p>
        </footer>
      </div>
    </div>
  );
}

export default App;
