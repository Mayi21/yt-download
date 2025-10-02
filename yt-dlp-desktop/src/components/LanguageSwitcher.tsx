import { useTranslation } from 'react-i18next';

export function LanguageSwitcher() {
  const { i18n, t } = useTranslation();

  const changeLanguage = (lng: string) => {
    i18n.changeLanguage(lng);
  };

  return (
    <div className="flex items-center gap-2">
      <span className="text-sm text-gray-600 dark:text-gray-400">
        {t('settings.language')}:
      </span>
      <select
        value={i18n.language}
        onChange={(e) => changeLanguage(e.target.value)}
        className="text-sm bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded px-2 py-1"
      >
        <option value="en">English</option>
        <option value="zh">中文</option>
      </select>
    </div>
  );
}