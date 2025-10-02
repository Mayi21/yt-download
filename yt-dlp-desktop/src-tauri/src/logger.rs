use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::Local;
use once_cell::sync::Lazy;

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// 日志缓冲区大小（64KB）
const BUFFER_SIZE: usize = 64 * 1024;

/// 单个日志文件最大大小（10MB）
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024;

/// 保留的日志文件数量
const MAX_LOG_FILES: usize = 5;

struct LoggerInner {
    writer: BufWriter<fs::File>,
    log_file: PathBuf,
    current_level: LogLevel,
    current_size: u64,
}

/// 应用日志记录器（单例模式）
pub struct AppLogger {
    inner: Arc<Mutex<LoggerInner>>,
    log_dir: PathBuf,
}

/// 全局日志实例
static LOGGER: Lazy<AppLogger> = Lazy::new(|| {
    AppLogger::init().unwrap_or_else(|e| {
        eprintln!("无法初始化日志系统: {}", e);
        // 创建一个空的日志器作为降级方案
        let temp_file = std::env::temp_dir().join("yt-dlp-desktop-fallback.log");
        AppLogger::init_with_path(temp_file).expect("无法创建降级日志器")
    })
});

impl AppLogger {
    /// 获取全局日志实例
    pub fn get() -> &'static AppLogger {
        &LOGGER
    }

    /// 初始化日志系统
    fn init() -> Result<Self, Box<dyn std::error::Error>> {
        // 使用 Application Support 目录
        let app_data_dir = dirs::data_dir()
            .ok_or("无法获取应用数据目录")?
            .join("com.youtube-downloader.desktop");

        // 确保目录存在
        fs::create_dir_all(&app_data_dir)?;

        let log_file = app_data_dir.join("app.log");
        Self::init_with_path_and_dir(log_file, app_data_dir)
    }

    /// 使用指定路径初始化
    fn init_with_path(log_file: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let log_dir = log_file.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();
        Self::init_with_path_and_dir(log_file, log_dir)
    }

    fn init_with_path_and_dir(log_file: PathBuf, log_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // 检查是否需要轮转
        if log_file.exists() {
            let metadata = fs::metadata(&log_file)?;
            if metadata.len() > MAX_LOG_SIZE {
                Self::rotate_logs(&log_file, &log_dir)?;
            }
        }

        // 创建日志文件
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;

        let current_size = file.metadata()?.len();
        let writer = BufWriter::with_capacity(BUFFER_SIZE, file);

        let inner = LoggerInner {
            writer,
            log_file: log_file.clone(),
            current_level: LogLevel::Info,
            current_size,
        };

        let logger = AppLogger {
            inner: Arc::new(Mutex::new(inner)),
            log_dir,
        };

        // 写入启动信息
        logger.info(&format!("应用启动 - 日志文件: {:?}", log_file));

        Ok(logger)
    }

    /// 日志文件轮转
    fn rotate_logs(log_file: &PathBuf, log_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("app_{}.log", timestamp);
        let backup_path = log_dir.join(&backup_name);

        // 移动当前日志文件
        fs::rename(log_file, &backup_path)?;

        // 清理旧日志文件
        let mut log_files: Vec<_> = fs::read_dir(log_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_name().to_str()
                    .map(|name| name.starts_with("app_") && name.ends_with(".log"))
                    .unwrap_or(false)
            })
            .collect();

        // 按修改时间排序
        log_files.sort_by_key(|entry| {
            entry.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        // 删除超出限制的旧文件
        while log_files.len() > MAX_LOG_FILES {
            if let Some(old_file) = log_files.first() {
                let _ = fs::remove_file(old_file.path());
                log_files.remove(0);
            } else {
                break;
            }
        }

        Ok(())
    }

    /// 写入日志（内部方法）
    fn write_log(&self, level: LogLevel, message: &str) {
        // 获取当前日志级别，如果低于设定级别则不记录
        let should_log = {
            let inner = self.inner.lock().unwrap();
            level >= inner.current_level
        };

        if !should_log {
            return;
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("[{}] [{}] {}\n", timestamp, level, message);

        // 写入文件
        if let Ok(mut inner) = self.inner.lock() {
            if let Err(e) = inner.writer.write_all(log_entry.as_bytes()) {
                eprintln!("写入日志失败: {}", e);
            } else {
                if let Err(e) = inner.writer.flush() {
                    eprintln!("刷新日志缓冲失败: {}", e);
                }

                inner.current_size += log_entry.len() as u64;

                // 检查是否需要轮转
                if inner.current_size > MAX_LOG_SIZE {
                    if let Err(e) = self.rotate_current(&mut inner) {
                        eprintln!("日志轮转失败: {}", e);
                    }
                }
            }
        }

        // 同时输出到控制台（仅在调试模式）
        #[cfg(debug_assertions)]
        print!("{}", log_entry);
    }

    /// 轮转当前日志
    fn rotate_current(&self, inner: &mut LoggerInner) -> Result<(), Box<dyn std::error::Error>> {
        // 刷新并关闭当前文件
        inner.writer.flush()?;

        // 执行轮转
        Self::rotate_logs(&inner.log_file, &self.log_dir)?;

        // 创建新文件
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&inner.log_file)?;

        inner.writer = BufWriter::with_capacity(BUFFER_SIZE, file);
        inner.current_size = 0;

        Ok(())
    }

    /// 记录 INFO 级别日志
    pub fn info(&self, message: &str) {
        self.write_log(LogLevel::Info, message);
    }

    /// 记录 ERROR 级别日志
    pub fn error(&self, message: &str) {
        self.write_log(LogLevel::Error, message);
    }

    /// 记录 DEBUG 级别日志
    pub fn debug(&self, message: &str) {
        self.write_log(LogLevel::Debug, message);
    }

    /// 记录 WARNING 级别日志
    pub fn warn(&self, message: &str) {
        self.write_log(LogLevel::Warning, message);
    }

    /// 获取日志文件路径
    pub fn get_log_path(&self) -> PathBuf {
        self.inner.lock().unwrap().log_file.clone()
    }

    /// 设置日志级别
    pub fn set_level(&self, level: LogLevel) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.current_level = level;
        }
    }

    /// 刷新日志缓冲区
    pub fn flush(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            let _ = inner.writer.flush();
        }
    }
}

impl Drop for LoggerInner {
    fn drop(&mut self) {
        // 确保所有日志都被写入
        let _ = self.writer.flush();
    }
}

/// 兼容旧代码的构造函数
impl AppLogger {
    pub fn new() -> Result<&'static Self, Box<dyn std::error::Error>> {
        Ok(AppLogger::get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_singleton() {
        let logger1 = AppLogger::get();
        let logger2 = AppLogger::get();

        // 确保是同一个实例
        assert!(std::ptr::eq(logger1, logger2));
    }

    #[test]
    fn test_concurrent_logging() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let logger = AppLogger::get();
                    for j in 0..100 {
                        logger.info(&format!("Thread {} - Message {}", i, j));
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        // 刷新确保所有日志都写入
        AppLogger::get().flush();
    }

    #[test]
    fn test_log_levels() {
        let logger = AppLogger::get();

        // 设置为 Error 级别
        logger.set_level(LogLevel::Error);

        // 这些不应该被记录
        logger.debug("This is debug");
        logger.info("This is info");

        // 这个应该被记录
        logger.error("This is error");

        logger.flush();
    }
}
