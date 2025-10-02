import type { DownloadProgress as DownloadProgressType } from '../types';
import { formatSpeed, formatFileSize, formatTime } from '../services/api';

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
  const getStatusText = () => {
    switch (progress.status) {
      case 'downloading':
        return '下载中...';
      case 'processing':
        return '处理中...';
      case 'finished':
        return '✅ 下载完成！';
      case 'error':
        return '❌ 下载失败';
      default:
        return '准备中...';
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
              <span className="text-gray-600 dark:text-gray-400">速度:</span>
              <span className="ml-2 font-medium">{formatSpeed(progress.speed)}</span>
            </div>
            <div>
              <span className="text-gray-600 dark:text-gray-400">剩余时间:</span>
              <span className="ml-2 font-medium">{formatTime(progress.eta)}</span>
            </div>
          </div>
        )}

        {/* 控制按钮 */}
        {progress.status === 'downloading' && (
          <div className="flex gap-2">
            {isPaused ? (
              <button onClick={onResume} className="btn btn-primary flex-1">
                ▶️ 继续
              </button>
            ) : (
              <button onClick={onPause} className="btn btn-secondary flex-1">
                ⏸️ 暂停
              </button>
            )}
            <button onClick={onCancel} className="btn bg-error text-white hover:bg-red-600 flex-1">
              ❌ 取消
            </button>
          </div>
        )}

        {/* 完成后的操作 */}
        {progress.status === 'finished' && (
          <div className="flex gap-2">
            <button className="btn btn-primary flex-1">
              📂 打开文件夹
            </button>
            <button className="btn btn-secondary flex-1">
              ▶️ 播放
            </button>
          </div>
        )}

        {/* 错误时的重试按钮 */}
        {progress.status === 'error' && (
          <button className="btn btn-primary w-full">
            🔄 重试
          </button>
        )}
      </div>
    </div>
  );
}
