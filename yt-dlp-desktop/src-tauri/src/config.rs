use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_save_path: Option<String>,
    pub language: String,
    pub prefer_hdr: bool,
    pub include_subtitles: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_save_path: None,
            language: "auto".to_string(),
            prefer_hdr: false,
            include_subtitles: false,
        }
    }
}

impl AppConfig {
    pub fn load(app: &AppHandle) -> Result<Self, String> {
        let config_dir = app.path().app_config_dir()
            .map_err(|e| format!("Failed to get config dir: {}", e))?;
        
        let config_file = config_dir.join("config.json");
        
        if config_file.exists() {
            let content = std::fs::read_to_string(&config_file)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            
            serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse config: {}", e))
        } else {
            // 创建默认配置
            let config = Self::default();
            config.save(app)?;
            Ok(config)
        }
    }
    
    pub fn save(&self, app: &AppHandle) -> Result<(), String> {
        let config_dir = app.path().app_config_dir()
            .map_err(|e| format!("Failed to get config dir: {}", e))?;
        
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config dir: {}", e))?;
        
        let config_file = config_dir.join("config.json");
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        std::fs::write(&config_file, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        
        Ok(())
    }
    
    pub fn get_default_save_path(&self) -> Option<String> {
        self.default_save_path.clone().or_else(|| {
            // 如果没有设置默认路径，使用系统下载目录
            dirs::download_dir().and_then(|path| {
                path.to_str().map(|s| s.to_string())
            })
        })
    }
}