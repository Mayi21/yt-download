import { useTranslation } from 'react-i18next';
import type { DownloadProgress as DownloadProgressType } from '../types';
import { formatSpeed, formatFileSize, formatTime, openFileLocation } from '../services/api';

interface DownloadProgressProps {
  progress: DownloadProgressType;
  onCancel?: () => void;
  onPause?: () => void;
  onResume?: () => void;
  isPaused?: boolean;
}

export function DownloadProgress({
  progress,
  onCancel,
  onPause,
  onResume,
  isPaused = false,
}: DownloadProgressProps) {
  const { t } = useTranslation();
  
  const getStatusText = () => {
    switch (progress.status) {
      case 'downloading':
        return t('progress.downloading');
      case 'processing':
        return t('progress.processing');
      case 'finished':
        return `✅ ${t('progress.finished')}`;
      case 'error':
        return `❌ ${t('progress.error')}`;
      default:
        return t('progress.preparing');
    }
  };

  const getStatusColor = () => {
    switch (progress.status) {
      case 'downloading':
        return 'text-primary';
      case 'processing':
        return 'text-warning';
      case 'finished':
        return 'text-success';
      case 'error':
        return 'text-error';
      default:
        return 'text-gray-600';
    }
  };

  return (
    <div className="card">
      <div className="space-y-4">
        {/* 状态和文件名 */}
        <div>
          <h3 className={`text-lg font-semibold ${getStatusColor()}`}>
            {getStatusText()}
          </h3>
          <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
            {progress.filename}
          </p>
        </div>

        {/* 进度条 */}
        <div>
          <div className="flex justify-between text-sm text-gray-600 dark:text-gray-400 mb-2">
            <span>{Math.floor(progress.percent)}%</span>
            <span>
              {formatFileSize(progress.downloaded)} / {formatFileSize(progress.total)}
            </span>
          </div>

          <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 overflow-hidden">
            <div
              className={`h-full transition-all duration-300 ${
                progress.status === 'finished'
                  ? 'bg-success'
                  : progress.status === 'error'
                  ? 'bg-error'
                  : 'bg-primary'
              }`}
              style={{ width: `${progress.percent}%` }}
            />
          </div>
        </div>

        {/* 下载信息 */}
        {progress.status === 'downloading' && (
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-gray-600 dark:text-gray-400">{t('progress.speed')}</span>
              <span className="ml-2 font-medium">{formatSpeed(progress.speed)}</span>
            </div>
            <div>
              <span className="text-gray-600 dark:text-gray-400">{t('progress.eta')}</span>
              <span className="ml-2 font-medium">{formatTime(progress.eta)}</span>
            </div>
          </div>
        )}

        {/* 控制按钮 */}
        {progress.status === 'downloading' && progress.percent < 100 && (
          <div className="flex gap-2">
            {isPaused ? (
              <button onClick={onResume} className="btn btn-primary flex-1">
                ▶️ {t('download.resume')}
              </button>
            ) : (
              <button onClick={onPause} className="btn btn-secondary flex-1">
                ⏸️ {t('download.pause')}
              </button>
            )}
            <button onClick={onCancel} className="btn bg-error text-white hover:bg-red-600 flex-1">
              ❌ {t('download.cancel')}
            </button>
          </div>
        )}

        {/* 处理中状态 */}
        {progress.status === 'processing' && (
          <div className="text-center text-sm text-gray-600 dark:text-gray-400">
            <div className="animate-spin inline-block w-4 h-4 border-2 border-primary border-t-transparent rounded-full mr-2"></div>
            {t('progress.pleaseWait')}
          </div>
        )}

        {/* 完成后的操作 */}
        {progress.status === 'finished' && (
          <div className="flex gap-2">
            <button 
              onClick={() => progress.file_path && openFileLocation(progress.file_path)}
              className="btn btn-primary flex-1"
            >
              📂 {t('download.openFolder')}
            </button>
          </div>
        )}

        {/* 错误时的重试按钮 */}
        {progress.status === 'error' && (
          <button className="btn btn-primary w-full">
            🔄 {t('download.retry')}
          </button>
        )}
      </div>
    </div>
  );
}
