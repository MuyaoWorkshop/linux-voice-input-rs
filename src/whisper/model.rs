use std::path::{Path, PathBuf};

use crate::utils::{Result, VoiceInputError};

/// Whisper 模型类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhisperModel {
    /// 75MB, <1秒识别, 低准确性
    Tiny,
    /// 142MB, 1-2秒识别, 平衡（默认）
    Base,
    /// 466MB, 2-4秒识别, 高准确性
    Small,
    /// 1.5GB, 5-8秒识别, 更高准确性
    Medium,
    /// 3GB, 10-15秒识别, 最高准确性
    Large,
}

impl WhisperModel {
    /// 获取模型文件名
    pub fn filename(&self) -> &'static str {
        match self {
            Self::Tiny => "ggml-tiny.bin",
            Self::Base => "ggml-base.bin",
            Self::Small => "ggml-small.bin",
            Self::Medium => "ggml-medium.bin",
            Self::Large => "ggml-large.bin",
        }
    }

    /// 获取完整路径
    pub fn path(&self, base_dir: &Path) -> PathBuf {
        base_dir.join(self.filename())
    }

    /// 获取下载 URL（Hugging Face）
    pub fn download_url(&self) -> &'static str {
        match self {
            Self::Tiny => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
            Self::Base => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
            Self::Small => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
            Self::Medium => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
            Self::Large => "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3.bin",
        }
    }

    /// 从文件名解析模型类型
    pub fn from_filename(filename: &str) -> Option<Self> {
        match filename {
            name if name.contains("tiny") => Some(Self::Tiny),
            name if name.contains("base") => Some(Self::Base),
            name if name.contains("small") => Some(Self::Small),
            name if name.contains("medium") => Some(Self::Medium),
            name if name.contains("large") => Some(Self::Large),
            _ => None,
        }
    }

    /// 获取模型大小（MB）
    pub fn size_mb(&self) -> u32 {
        match self {
            Self::Tiny => 75,
            Self::Base => 142,
            Self::Small => 466,
            Self::Medium => 1500,
            Self::Large => 3000,
        }
    }
}

/// 查找可用的 Whisper 模型
pub fn find_model(model_path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(shellexpand::tilde(model_path).to_string());

    if path.exists() {
        tracing::info!("找到 Whisper 模型: {}", path.display());
        Ok(path)
    } else {
        Err(VoiceInputError::ModelLoad(format!(
            "模型文件不存在: {}",
            path.display()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_filename() {
        assert_eq!(WhisperModel::Base.filename(), "ggml-base.bin");
        assert_eq!(WhisperModel::Tiny.filename(), "ggml-tiny.bin");
    }

    #[test]
    fn test_from_filename() {
        assert_eq!(
            WhisperModel::from_filename("ggml-base.bin"),
            Some(WhisperModel::Base)
        );
        assert_eq!(
            WhisperModel::from_filename("/path/to/ggml-tiny.bin"),
            Some(WhisperModel::Tiny)
        );
    }

    #[test]
    fn test_model_size() {
        assert_eq!(WhisperModel::Base.size_mb(), 142);
        assert_eq!(WhisperModel::Tiny.size_mb(), 75);
    }
}
