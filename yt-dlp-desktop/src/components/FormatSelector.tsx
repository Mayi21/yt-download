import { useState, useEffect } from 'react';
import type { VideoFormat, QualityOption, FormatOption } from '../types';
import { formatFileSize } from '../services/api';

interface FormatSelectorProps {
  formats: VideoFormat[];
  onSelect: (formatId: string, audioOnly: boolean, includeSubtitles: boolean, preferHdr?: boolean) => void;
}

export function FormatSelector({ formats, onSelect }: FormatSelectorProps) {
  const [selectedQuality, setSelectedQuality] = useState<QualityOption>('1080p');
  const [selectedFormat, setSelectedFormat] = useState<FormatOption>('mp4');
  const [audioOnly, setAudioOnly] = useState(false);
  const [includeSubtitles, setIncludeSubtitles] = useState(false);
  const [preferHdr, setPreferHdr] = useState(false);
  const [isInitialized, setIsInitialized] = useState(false);

  // 获取可用的清晰度选项
  const availableQualities: QualityOption[] = ['2160p', '1440p', '1080p', '720p', '480p', '360p', '240p', '144p'];
  const qualityFormats = formats.filter((f) =>
    availableQualities.includes(f.quality_label as QualityOption)
  );

  // 获取最佳可用清晰度（按优先级排序）
  const getBestAvailableQuality = (): QualityOption => {
    const preferredQualities: QualityOption[] = ['1080p', '720p', '2160p', '480p', '360p'];
    
    for (const quality of preferredQualities) {
      const hasQuality = formats.some(
        (f) => f.quality_label === quality && f.vcodec !== 'none'
      );
      if (hasQuality) {
        return quality;
      }
    }
    
    // 如果没有找到预设的清晰度，返回第一个可用的
    const firstAvailable = formats.find((f) => f.vcodec !== 'none');
    return (firstAvailable?.quality_label as QualityOption) || '360p';
  };

  // 获取指定清晰度下的最佳格式
  const getBestAvailableFormat = (quality: QualityOption): FormatOption => {
    const preferredFormats: FormatOption[] = ['mp4', 'webm', 'mkv'];
    
    for (const format of preferredFormats) {
      const hasFormat = formats.some(
        (f) => f.quality_label === quality && f.ext === format && f.vcodec !== 'none'
      );
      if (hasFormat) {
        return format;
      }
    }
    
    // 如果没有找到预设的格式，返回第一个可用的
    const firstAvailable = formats.find(
      (f) => f.quality_label === quality && f.vcodec !== 'none'
    );
    return (firstAvailable?.ext as FormatOption) || 'mp4';
  };

  // 当格式数据变化时，自动选择最佳默认选项
  useEffect(() => {
    if (formats.length > 0 && !isInitialized) {
      const bestQuality = getBestAvailableQuality();
      const bestFormat = getBestAvailableFormat(bestQuality);
      
      setSelectedQuality(bestQuality);
      setSelectedFormat(bestFormat);
      setIsInitialized(true);
      
      console.log(`[FormatSelector] 自动选择最佳选项: ${bestQuality} ${bestFormat.toUpperCase()}`);
    }
  }, [formats, isInitialized]);

  // 根据选择的清晰度和格式找到对应的 format
  const getSelectedFormat = () => {
    if (audioOnly) {
      // 仅音频：找最高质量的音频格式
      return formats.find((f) => f.vcodec === 'none' && f.acodec !== 'none');
    }

    // 使用改进的格式选择逻辑
    const bestFormat = getBestFormatForQuality(selectedQuality, selectedFormat, preferHdr);
    if (bestFormat) {
      return bestFormat;
    }

    // 如果没找到，尝试其他格式
    const formatPriority: FormatOption[] = ['mp4', 'webm', 'mkv'];
    for (const fmt of formatPriority) {
      const format = getBestFormatForQuality(selectedQuality, fmt, preferHdr);
      if (format) {
        return format;
      }
    }

    // 最后回退到任何可用的视频格式
    return formats.find((f) => f.vcodec !== 'none') || formats[0];
  };

  // 创建 DASH 格式组合（视频+音频）
  const createDashFormat = (videoFormat: VideoFormat, audioFormat: VideoFormat): VideoFormat => {
    return {
      format_id: `${videoFormat.format_id}+${audioFormat.format_id}`,
      ext: videoFormat.ext,
      resolution: videoFormat.resolution,
      quality_label: videoFormat.quality_label,
      filesize: (videoFormat.filesize || 0) + (audioFormat.filesize || 0),
      fps: videoFormat.fps,
      vcodec: videoFormat.vcodec,
      acodec: audioFormat.acodec,
      tbr: (videoFormat.tbr || 0) + (audioFormat.tbr || 0),
      format_note: `DASH video + audio (${videoFormat.format_note}, ${audioFormat.format_note})`,
      hdr: videoFormat.hdr,
      is_dash: true,
    };
  };

  // 检查指定清晰度是否有 HDR 版本
  const hasHdrForQuality = (quality: QualityOption): boolean => {
    return formats.some(f => f.quality_label.includes(quality.replace('p', '')) && f.hdr);
  };

  // 获取最佳格式（包括 DASH 合并和 HDR 支持）
  const getBestFormatForQuality = (quality: QualityOption, format: FormatOption, preferHdr: boolean = false): VideoFormat | null => {
    // 首先尝试找到完整的格式（包含视频和音频）
    let completeFormats = formats.filter(
      (f) =>
        f.quality_label.includes(quality.replace('p', '')) &&
        f.ext === format &&
        f.vcodec !== 'none' &&
        f.acodec !== 'none'
    );

    // 如果偏好 HDR，优先选择 HDR 格式
    if (preferHdr && completeFormats.some(f => f.hdr)) {
      completeFormats = completeFormats.filter(f => f.hdr);
    }

    if (completeFormats.length > 0) {
      return completeFormats[0];
    }

    // 如果没有完整格式，尝试创建 DASH 组合
    let videoFormats = formats.filter(
      (f) =>
        f.quality_label.includes(quality.replace('p', '')) &&
        f.vcodec !== 'none' &&
        f.acodec === 'none' // 纯视频流
    );

    // HDR 偏好处理
    if (preferHdr && videoFormats.some(f => f.hdr)) {
      videoFormats = videoFormats.filter(f => f.hdr);
    }

    // 选择最佳视频格式（优先选择指定的容器格式）
    const videoFormat = videoFormats.find(f => f.ext === format) || videoFormats[0];

    // 选择最佳音频格式
    const audioFormats = formats.filter(f => f.vcodec === 'none' && f.acodec !== 'none');
    const audioFormat = audioFormats
      .sort((a, b) => (b.tbr || 0) - (a.tbr || 0))[0]; // 最高质量音频

    if (videoFormat && audioFormat) {
      console.log(`[FormatSelector] Creating DASH format: ${videoFormat.format_id}+${audioFormat.format_id} (HDR: ${videoFormat.hdr})`);
      return createDashFormat(videoFormat, audioFormat);
    }

    return null;
  };

  // 当清晰度改变时，自动选择该清晰度下可用的格式
  const getAvailableFormatsForQuality = (quality: QualityOption): FormatOption[] => {
    const availableFormats = formats
      .filter((f) => f.quality_label === quality && f.vcodec !== 'none')
      .map((f) => f.ext as FormatOption);
    
    return [...new Set(availableFormats)]; // 去重
  };

  // 当清晰度改变时，检查当前选择的格式是否可用，如果不可用则自动切换
  const handleQualityChange = (quality: QualityOption) => {
    setSelectedQuality(quality);
    
    const availableFormats = getAvailableFormatsForQuality(quality);
    if (availableFormats.length > 0 && !availableFormats.includes(selectedFormat)) {
      // 如果当前格式不可用，优先选择 mp4，其次 webm，最后 mkv
      const preferredOrder: FormatOption[] = ['mp4', 'webm', 'mkv'];
      const bestFormat = preferredOrder.find(f => availableFormats.includes(f)) || availableFormats[0];
      setSelectedFormat(bestFormat);
    }
  };

  const handleDownload = () => {
    const format = getSelectedFormat();
    if (format) {
      onSelect(format.format_id, audioOnly, includeSubtitles, preferHdr);
    }
  };

  const selectedFormatInfo = getSelectedFormat();

  return (
    <div className="card space-y-6">
      <h3 className="text-lg font-semibold">下载选项</h3>

      {/* 清晰度选择 */}
      <div>
        <label className="block text-sm font-medium mb-3">清晰度</label>
        <div className="grid grid-cols-4 gap-2">
          {availableQualities.map((quality) => {
            const hasQuality = qualityFormats.some((f) => f.quality_label === quality);
            return (
              <button
                key={quality}
                onClick={() => handleQualityChange(quality)}
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
          {(['mp4', 'webm', 'mkv'] as FormatOption[]).map((format) => {
            // 检查当前清晰度下是否有这种格式可用
            const hasFormat = formats.some(
              (f) => f.quality_label === selectedQuality && f.ext === format && f.vcodec !== 'none'
            );
            
            return (
              <button
                key={format}
                onClick={() => setSelectedFormat(format)}
                disabled={audioOnly || !hasFormat}
                className={`btn ${
                  selectedFormat === format && !audioOnly ? 'btn-primary' : 'btn-secondary'
                } ${audioOnly || !hasFormat ? 'opacity-50 cursor-not-allowed' : ''}`}
                title={!hasFormat ? `${selectedQuality} 清晰度下没有 ${format.toUpperCase()} 格式` : ''}
              >
                {format.toUpperCase()}
              </button>
            );
          })}
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

        {hasHdrForQuality(selectedQuality) && !audioOnly && (
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={preferHdr}
              onChange={(e) => setPreferHdr(e.target.checked)}
              className="w-4 h-4 text-primary focus:ring-primary"
            />
            <span className="text-sm">🌈 优先选择 HDR（高动态范围）</span>
          </label>
        )}
      </div>

      {/* 选中的格式信息 */}
      {selectedFormatInfo && (
        <div className="bg-gray-100 dark:bg-gray-700 rounded-lg p-4">
          <h4 className="font-medium text-sm mb-2">选中的格式：</h4>
          <div className="text-sm space-y-1 text-gray-600 dark:text-gray-300">
            <p>
              <span className="font-medium">清晰度:</span>{' '}
              {selectedFormatInfo.quality_label}
              {isInitialized && selectedQuality !== '1080p' && (
                <span className="ml-2 text-blue-600 dark:text-blue-400 text-xs">
                  (自动选择最佳可用清晰度)
                </span>
              )}
            </p>
            <p>
              <span className="font-medium">格式:</span> {selectedFormatInfo.ext.toUpperCase()}
              {selectedFormatInfo.ext !== selectedFormat && (
                <span className="ml-2 text-orange-600 dark:text-orange-400 text-xs">
                  (自动选择，{selectedFormat.toUpperCase()} 不可用)
                </span>
              )}
              {isInitialized && selectedFormat !== 'mp4' && selectedFormatInfo.ext === selectedFormat && (
                <span className="ml-2 text-blue-600 dark:text-blue-400 text-xs">
                  (自动选择最佳可用格式)
                </span>
              )}
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
              {selectedFormatInfo.hdr && (
                <span className="ml-2 text-purple-600 dark:text-purple-400 text-xs">
                  HDR
                </span>
              )}
              {selectedFormatInfo.format_id.includes('+') && (
                <span className="ml-2 text-green-600 dark:text-green-400 text-xs">
                  (DASH 格式，将自动合并)
                </span>
              )}
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
