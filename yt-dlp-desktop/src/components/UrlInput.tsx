import { useState } from 'react';
import { isValidYouTubeUrl } from '../services/api';

interface UrlInputProps {
  onFetch: (url: string) => void;
  isLoading?: boolean;
}

export function UrlInput({ onFetch, isLoading = false }: UrlInputProps) {
  const [url, setUrl] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    // 验证 URL
    if (!url.trim()) {
      setError('请输入 YouTube 链接');
      return;
    }

    if (!isValidYouTubeUrl(url)) {
      setError('无效的 YouTube 链接');
      return;
    }

    setError('');
    onFetch(url);
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setUrl(e.target.value);
    if (error) setError(''); // 清除错误
  };

  const handlePaste = async () => {
    try {
      const text = await navigator.clipboard.readText();
      setUrl(text);
      if (error) setError('');
    } catch (err) {
      console.error('Failed to read clipboard:', err);
    }
  };

  return (
    <div className="card">
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label htmlFor="url-input" className="block text-sm font-medium mb-2">
            🔗 YouTube 链接
          </label>
          <div className="flex gap-2">
            <div className="flex-1 relative">
              <input
                id="url-input"
                type="text"
                className={`input ${error ? 'border-error' : ''}`}
                placeholder="https://www.youtube.com/watch?v=..."
                value={url}
                onChange={handleChange}
                disabled={isLoading}
              />
              {error && (
                <p className="text-error text-sm mt-1">{error}</p>
              )}
            </div>
            <button
              type="button"
              onClick={handlePaste}
              className="btn btn-secondary"
              disabled={isLoading}
            >
              📋 粘贴
            </button>
          </div>
        </div>

        <button
          type="submit"
          className="btn btn-primary w-full"
          disabled={isLoading}
        >
          {isLoading ? (
            <>
              <span className="inline-block animate-spin mr-2">⏳</span>
              获取中...
            </>
          ) : (
            '获取视频信息 →'
          )}
        </button>
      </form>
    </div>
  );
}
