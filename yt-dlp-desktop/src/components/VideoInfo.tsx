import type { VideoInfo as VideoInfoType } from '../types';
import { formatTime, formatFileSize } from '../services/api';

interface VideoInfoProps {
  video: VideoInfoType;
}

export function VideoInfo({ video }: VideoInfoProps) {
  // 格式化上传日期
  const formatUploadDate = (dateStr: string) => {
    if (!dateStr || dateStr.length !== 8) return dateStr;
    const year = dateStr.substring(0, 4);
    const month = dateStr.substring(4, 6);
    const day = dateStr.substring(6, 8);
    return `${year}-${month}-${day}`;
  };

  // 格式化观看次数
  const formatViewCount = (count: number) => {
    if (count >= 1000000000) {
      return `${(count / 1000000000).toFixed(1)}B`;
    }
    if (count >= 1000000) {
      return `${(count / 1000000).toFixed(1)}M`;
    }
    if (count >= 1000) {
      return `${(count / 1000).toFixed(1)}K`;
    }
    return count.toString();
  };

  // 获取最佳格式（用于显示总大小）
  const bestFormat = video.formats.find((f) => f.quality_label === '1080p') || video.formats[0];

  return (
    <div className="card">
      <div className="flex gap-4">
        {/* 缩略图 */}
        <div className="flex-shrink-0">
          <img
            src={video.thumbnail}
            alt={video.title}
            className="w-48 h-36 object-cover rounded-lg"
          />
        </div>

        {/* 视频信息 */}
        <div className="flex-1 space-y-2">
          <h2 className="text-xl font-bold text-gray-900 dark:text-gray-100">
            {video.title}
          </h2>

          <div className="grid grid-cols-2 gap-x-4 gap-y-1 text-sm text-gray-600 dark:text-gray-400">
            <div className="flex items-center gap-2">
              <span className="font-medium">⏱️ 时长:</span>
              <span>{formatTime(video.duration)}</span>
            </div>

            <div className="flex items-center gap-2">
              <span className="font-medium">👤 上传者:</span>
              <span>{video.uploader}</span>
            </div>

            <div className="flex items-center gap-2">
              <span className="font-medium">📅 日期:</span>
              <span>{formatUploadDate(video.upload_date)}</span>
            </div>

            <div className="flex items-center gap-2">
              <span className="font-medium">👁️ 观看:</span>
              <span>{formatViewCount(video.view_count)} 次</span>
            </div>

            <div className="flex items-center gap-2">
              <span className="font-medium">📦 大小:</span>
              <span>{bestFormat.filesize ? formatFileSize(bestFormat.filesize) : '未知'}</span>
            </div>

            <div className="flex items-center gap-2">
              <span className="font-medium">🎬 格式:</span>
              <span>{video.formats.length} 个可用</span>
            </div>
          </div>

          {/* 描述（可折叠） */}
          {video.description && (
            <details className="text-sm text-gray-600 dark:text-gray-400 mt-2">
              <summary className="cursor-pointer font-medium hover:text-primary">
                查看描述
              </summary>
              <p className="mt-2 whitespace-pre-wrap">
                {video.description.length > 200
                  ? `${video.description.substring(0, 200)}...`
                  : video.description}
              </p>
            </details>
          )}
        </div>
      </div>
    </div>
  );
}
