use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::audio::AudioBuffer;
use crate::utils::{Result, VoiceInputError};

/// Whisper 引擎
pub struct WhisperEngine {
    context: WhisperContext,
    language: String,
}

impl WhisperEngine {
    /// 创建新的 Whisper 引擎
    pub fn new(model_path: impl AsRef<Path>, language: &str) -> Result<Self> {
        tracing::info!("正在加载 Whisper 模型: {}", model_path.as_ref().display());

        // 创建上下文参数
        let params = WhisperContextParameters::default();

        // 加载模型
        let context = WhisperContext::new_with_params(
            model_path
                .as_ref()
                .to_str()
                .ok_or_else(|| {
                    VoiceInputError::ModelLoad("模型路径包含无效字符".to_string())
                })?,
            params,
        )
        .map_err(|e| VoiceInputError::ModelLoad(format!("加载模型失败: {:?}", e)))?;

        tracing::info!("Whisper 模型加载成功");

        Ok(Self {
            context,
            language: language.to_string(),
        })
    }

    /// 识别音频
    pub fn transcribe_sync(&mut self, audio: &AudioBuffer) -> Result<String> {
        // 创建状态
        let mut state = self
            .context
            .create_state()
            .map_err(|e| VoiceInputError::Recognition(format!("创建状态失败: {:?}", e)))?;

        // 配置识别参数
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        params.set_language(Some(&self.language));
        params.set_translate(false);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // 执行识别
        state
            .full(params, audio)
            .map_err(|e| VoiceInputError::Recognition(format!("识别失败: {:?}", e)))?;

        // 提取结果
        let num_segments = state
            .full_n_segments()
            .map_err(|e| VoiceInputError::Recognition(format!("获取结果失败: {:?}", e)))?;

        let mut result = String::new();
        for i in 0..num_segments {
            let segment = state
                .full_get_segment_text(i)
                .map_err(|e| VoiceInputError::Recognition(format!("获取文本失败: {:?}", e)))?;
            result.push_str(&segment);
        }

        Ok(result.trim().to_string())
    }

    /// 异步识别音频
    pub async fn transcribe(&mut self, audio: AudioBuffer) -> Result<String> {
        // 使用 spawn_blocking 在线程池中执行 CPU 密集任务
        // 但由于生命周期限制，这里直接调用同步方法
        // TODO: 未来可以考虑将 WhisperContext 包装为 Arc 来支持真正的异步
        self.transcribe_sync(&audio)
    }

    /// 获取引擎信息
    pub fn info(&self) -> String {
        format!("Whisper Engine (language: {})", self.language)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_info() {
        // 注意：这个测试需要实际的模型文件，可能在 CI 环境失败
        // 只测试基本功能
        let info = WhisperEngine {
            context: Arc::new(unsafe { std::mem::zeroed() }),
            language: "zh".to_string(),
        }
        .info();

        assert!(info.contains("zh"));
    }
}
