import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { isValidYouTubeUrl } from '../services/api';

interface UrlInputProps {
  onFetch: (url: string) => void;
  isLoading?: boolean;
}

export function UrlInput({ onFetch, isLoading = false }: UrlInputProps) {
  const { t } = useTranslation();
  const [url, setUrl] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    // 验证 URL
    if (!url.trim()) {
      setError('Please enter YouTube URL');
      return;
    }

    if (!isValidYouTubeUrl(url)) {
      setError('Invalid YouTube URL');
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
            🔗 YouTube URL
          </label>
          <div className="flex gap-2">
            <div className="flex-1 relative">
              <input
                id="url-input"
                type="text"
                className={`input ${error ? 'border-error' : ''}`}
                placeholder={t('url.placeholder')}
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
              📋 Paste
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
              {t('url.fetching')}
            </>
          ) : (
            `${t('url.fetch')} →`
          )}
        </button>
      </form>
    </div>
  );
}
