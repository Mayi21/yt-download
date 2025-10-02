import { useState } from 'react';
import type { VideoFormat, QualityOption, FormatOption } from '../types';
import { formatFileSize } from '../services/api';

interface FormatSelectorProps {
  formats: VideoFormat[];
  onSelect: (formatId: string, audioOnly: boolean, includeSubtitles: boolean) => void;
}

export function FormatSelector({ formats, onSelect }: FormatSelectorProps) {
  const [selectedQuality, setSelectedQuality] = useState<QualityOption>('1080p');
  const [selectedFormat, setSelectedFormat] = useState<FormatOption>('mp4');
  const [audioOnly, setAudioOnly] = useState(false);
  const [includeSubtitles, setIncludeSubtitles] = useState(false);

  // 获取可用的清晰度选项
  const availableQualities: QualityOption[] = ['2160p', '1080p', '720p', '480p', '360p'];
  const qualityFormats = formats.filter((f) =>
    availableQualities.includes(f.quality_label as QualityOption)
  );

  // 根据选择的清晰度和格式找到对应的 format
  const getSelectedFormat = () => {
    if (audioOnly) {
      // 仅音频：找最高质量的音频格式
      return formats.find((f) => f.vcodec === 'none' && f.acodec !== 'none');
    }

    // 视频：根据清晰度和格式筛选
    const filtered = formats.filter(
      (f) =>
        f.quality_label === selectedQuality &&
        f.ext === selectedFormat &&
        f.vcodec !== 'none'
    );

    if (filtered.length > 0) {
      return filtered[0];
    }

    // 如果没找到精确匹配，尝试找相同清晰度的其他格式
    const sameQuality = formats.filter(
      (f) => f.quality_label === selectedQuality && f.vcodec !== 'none'
    );

    if (sameQuality.length > 0) {
      return sameQuality[0];
    }

    // 最后回退到任何可用的视频格式
    return formats.find((f) => f.vcodec !== 'none') || formats[0];
  };

  const handleDownload = () => {
    const format = getSelectedFormat();
    if (format) {
      onSelect(format.format_id, audioOnly, includeSubtitles);
    }
  };

  const selectedFormatInfo = getSelectedFormat();

  return (
    <div className="card space-y-6">
      <h3 className="text-lg font-semibold">下载选项</h3>

      {/* 清晰度选择 */}
      <div>
        <label className="block text-sm font-medium mb-3">清晰度</label>
        <div className="grid grid-cols-5 gap-2">
          {availableQualities.map((quality) => {
            const hasQuality = qualityFormats.some((f) => f.quality_label === quality);
            return (
              <button
                key={quality}
                onClick={() => setSelectedQuality(quality)}
                disabled={!hasQuality || audioOnly}
                className={`btn ${
                  selectedQuality === quality && !audioOnly
                    ? 'btn-primary'
                    : 'btn-secondary'
                } ${!hasQuality || audioOnly ? 'opacity-50 cursor-not-allowed' : ''}`}
              >
                {quality}
              </button>
            );
          })}
        </div>
      </div>

      {/* 格式选择 */}
      <div>
        <label className="block text-sm font-medium mb-3">格式</label>
        <div className="flex gap-2">
          {(['mp4', 'webm', 'mkv'] as FormatOption[]).map((format) => (
            <button
              key={format}
              onClick={() => setSelectedFormat(format)}
              disabled={audioOnly}
              className={`btn ${
                selectedFormat === format && !audioOnly ? 'btn-primary' : 'btn-secondary'
              } ${audioOnly ? 'opacity-50 cursor-not-allowed' : ''}`}
            >
              {format.toUpperCase()}
            </button>
          ))}
        </div>
      </div>

      {/* 特殊选项 */}
      <div className="space-y-2">
        <label className="flex items-center gap-2 cursor-pointer">
          <input
            type="checkbox"
            checked={audioOnly}
            onChange={(e) => setAudioOnly(e.target.checked)}
            className="w-4 h-4 text-primary focus:ring-primary"
          />
          <span className="text-sm">🎵 仅音频（最佳音质）</span>
        </label>

        <label className="flex items-center gap-2 cursor-pointer">
          <input
            type="checkbox"
            checked={includeSubtitles}
            onChange={(e) => setIncludeSubtitles(e.target.checked)}
            className="w-4 h-4 text-primary focus:ring-primary"
          />
          <span className="text-sm">📝 包含字幕</span>
        </label>
      </div>

      {/* 选中的格式信息 */}
      {selectedFormatInfo && (
        <div className="bg-gray-100 dark:bg-gray-700 rounded-lg p-4">
          <h4 className="font-medium text-sm mb-2">选中的格式：</h4>
          <div className="text-sm space-y-1 text-gray-600 dark:text-gray-300">
            <p>
              <span className="font-medium">清晰度:</span>{' '}
              {selectedFormatInfo.quality_label}
            </p>
            <p>
              <span className="font-medium">格式:</span> {selectedFormatInfo.ext.toUpperCase()}
            </p>
            <p>
              <span className="font-medium">文件大小:</span>{' '}
              {selectedFormatInfo.filesize
                ? formatFileSize(selectedFormatInfo.filesize)
                : '未知'}
            </p>
            <p>
              <span className="font-medium">编码:</span> {selectedFormatInfo.vcodec} /{' '}
              {selectedFormatInfo.acodec}
            </p>
          </div>
        </div>
      )}

      <button onClick={handleDownload} className="btn btn-primary w-full py-3 text-lg">
        ⬇️ 开始下载
      </button>
    </div>
  );
}
