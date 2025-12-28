use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub whisper: WhisperConfig,
    #[serde(default)]
    pub xfyun: XFyunConfig,
    #[serde(default)]
    pub audio: AudioConfig,
    #[serde(default)]
    pub output: OutputConfig,
}

/// Whisper 离线识别配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperConfig {
    /// 模型文件路径
    pub model_path: String,
    /// 识别语言
    pub language: String,
    /// 最大录音时长（秒）
    pub max_duration: u64,
    /// 静音检测阈值（0.0-1.0）
    pub silence_threshold: f32,
    /// 静音持续时间（秒）
    pub silence_duration: f32,
}

/// 讯飞云在线识别配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XFyunConfig {
    pub app_id: String,
    pub api_secret: String,
    pub api_key: String,
}

/// 音频配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// 采样率
    pub sample_rate: u32,
    /// 声道数
    pub channels: u16,
    /// 音频块大小（在线模式）
    pub chunk_size: usize,
}

/// 输出配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// 默认输出方式：clipboard | file
    pub default: String,
    /// 文件输出路径（可选）
    pub file_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            whisper: WhisperConfig::default(),
            xfyun: XFyunConfig::default(),
            audio: AudioConfig::default(),
            output: OutputConfig::default(),
        }
    }
}

impl Default for WhisperConfig {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

        // 多路径查找模型文件（优先级顺序）
        let model_paths = vec![
            format!("{}/.local/share/whisper/ggml-base.bin", home),  // XDG 规范
            format!("{}/.cache/whisper/ggml-base.bin", home),        // 兼容性
            format!("./models/ggml-base.bin"),                        // 本地目录
        ];

        let model_path = model_paths
            .iter()
            .find(|p| std::path::Path::new(p).exists())
            .cloned()
            .unwrap_or_else(|| {
                // 如果都不存在，返回推荐的默认路径
                format!("{}/.local/share/whisper/ggml-base.bin", home)
            });

        Self {
            model_path,
            language: "zh".to_string(),
            max_duration: 60,
            silence_threshold: 0.02,
            silence_duration: 3.0,
        }
    }
}

impl Default for XFyunConfig {
    fn default() -> Self {
        Self {
            app_id: std::env::var("XFYUN_APP_ID").unwrap_or_default(),
            api_secret: std::env::var("XFYUN_API_SECRET").unwrap_or_default(),
            api_key: std::env::var("XFYUN_API_KEY").unwrap_or_default(),
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            channels: 1,
            chunk_size: 1280,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            default: "clipboard".to_string(),
            file_path: None,
        }
    }
}

impl Config {
    /// 从文件加载配置
    pub fn from_file(path: impl Into<PathBuf>) -> crate::utils::Result<Self> {
        let path = path.into();
        let content = std::fs::read_to_string(&path)
            .map_err(|e| crate::utils::VoiceInputError::Config(format!("读取配置文件失败: {}", e)))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| crate::utils::VoiceInputError::Config(format!("解析配置文件失败: {}", e)))?;

        Ok(config)
    }

    /// 加载配置（多路径查找）
    pub fn load() -> crate::utils::Result<Self> {
        let config_paths = vec![
            "./config.toml".to_string(),
            "./voice-input.toml".to_string(),
            format!("{}/.config/voice-input/config.toml",
                std::env::var("HOME").unwrap_or_else(|_| ".".to_string())),
            "/etc/voice-input/config.toml".to_string(),
        ];

        for path in config_paths {
            if std::path::Path::new(&path).exists() {
                tracing::info!("从配置文件加载: {}", path);
                return Self::from_file(path);
            }
        }

        tracing::info!("未找到配置文件，使用默认配置");
        Ok(Self::default())
    }

    /// 保存配置到文件
    pub fn save(&self, path: impl Into<PathBuf>) -> crate::utils::Result<()> {
        let path = path.into();
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::utils::VoiceInputError::Config(format!("序列化配置失败: {}", e)))?;

        // 确保目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.audio.sample_rate, 16000);
        assert_eq!(config.audio.channels, 1);
        assert_eq!(config.whisper.language, "zh");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("sample_rate"));

        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.audio.sample_rate, config.audio.sample_rate);
    }
}
