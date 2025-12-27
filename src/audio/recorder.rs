use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Sample, SampleFormat, Stream, StreamConfig};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::audio::silence::SilenceDetector;
use crate::utils::{Result, VoiceInputError};

/// 音频缓冲区
pub type AudioBuffer = Vec<f32>;

/// 音频录制器
pub struct AudioRecorder {
    device: Device,
    config: StreamConfig,
    sample_rate: u32,
}

impl AudioRecorder {
    /// 创建新的音频录制器
    pub fn new(sample_rate: u32, channels: u16) -> Result<Self> {
        // 获取默认音频主机
        let host = cpal::default_host();

        // 获取默认输入设备（麦克风）
        let device = host
            .default_input_device()
            .ok_or(VoiceInputError::NoMicrophone)?;

        tracing::info!("使用音频设备: {}", device.name().unwrap_or_else(|_| "Unknown".to_string()));

        // 配置音频流
        let config = StreamConfig {
            channels,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        Ok(Self {
            device,
            config,
            sample_rate,
        })
    }

    /// 录制音频直到静音（离线模式使用）
    pub fn record_until_silence(
        &self,
        max_duration: Duration,
        silence_threshold: f32,
        silence_duration: f32,
    ) -> Result<AudioBuffer> {
        let mut silence_detector = SilenceDetector::new(
            silence_threshold,
            Duration::from_secs_f32(silence_duration),
        );

        let buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&buffer);

        let err_occurred = Arc::new(Mutex::new(None));
        let err_clone = Arc::clone(&err_occurred);

        // 构建音频流
        let stream = self.build_input_stream(
            move |data: &[f32]| {
                let mut buf = buffer_clone.lock().unwrap();
                buf.extend_from_slice(data);

                // 显示音量条
                SilenceDetector::print_volume_bar(data);
            },
            move |err| {
                *err_clone.lock().unwrap() = Some(err);
            },
        )?;

        // 开始录音
        stream.play().map_err(|e| {
            VoiceInputError::AudioRecord(format!("启动音频流失败: {}", e))
        })?;

        println!("🎤 开始录音... (停顿 {:.1} 秒自动结束)", silence_duration);

        let start_time = std::time::Instant::now();
        let chunk_duration = Duration::from_millis(100); // 每 100ms 检查一次

        loop {
            std::thread::sleep(chunk_duration);

            // 检查是否有错误
            if let Some(err) = err_occurred.lock().unwrap().take() {
                return Err(VoiceInputError::AudioRecord(format!("录音错误: {}", err)));
            }

            // 检查是否超时
            if start_time.elapsed() > max_duration {
                println!("\n⏱️  达到最大录音时长，停止录音");
                break;
            }

            // 检查静音
            let buf = buffer.lock().unwrap();
            if buf.len() > self.sample_rate as usize {
                // 取最后 0.5 秒的数据检测静音
                let check_size = (self.sample_rate as f32 * 0.5) as usize;
                let start_idx = buf.len().saturating_sub(check_size);
                let recent_samples = &buf[start_idx..];

                if silence_detector.detect(recent_samples) {
                    println!("\n🔇 检测到静音，停止录音");
                    break;
                }
            }
        }

        // 停止录音
        drop(stream);

        let result = Arc::try_unwrap(buffer)
            .unwrap_or_else(|arc| {
                let mutex = Mutex::new(arc.lock().unwrap().clone());
                mutex
            })
            .into_inner()
            .unwrap();

        println!("✅ 录音完成，时长: {:.2} 秒", result.len() as f32 / self.sample_rate as f32);

        Ok(result)
    }

    /// 构建输入音频流
    fn build_input_stream<D, E>(
        &self,
        data_callback: D,
        error_callback: E,
    ) -> Result<Stream>
    where
        D: FnMut(&[f32]) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        let supported_config = self
            .device
            .supported_input_configs()
            .map_err(|e| {
                VoiceInputError::AudioDevice(format!("获取设备配置失败: {}", e))
            })?
            .next()
            .ok_or_else(|| {
                VoiceInputError::AudioDevice("设备不支持任何配置".to_string())
            })?;

        let sample_format = supported_config.sample_format();

        tracing::debug!(
            "使用采样格式: {:?}, 采样率: {}",
            sample_format,
            self.sample_rate
        );

        // 根据样本格式构建流
        match sample_format {
            SampleFormat::F32 => self.build_stream_f32(data_callback, error_callback),
            SampleFormat::I16 => self.build_stream_i16(data_callback, error_callback),
            SampleFormat::U16 => self.build_stream_u16(data_callback, error_callback),
            SampleFormat::I8 => self.build_stream_i8(data_callback, error_callback),
            SampleFormat::U8 => self.build_stream_u8(data_callback, error_callback),
            _ => Err(VoiceInputError::AudioDevice(format!(
                "不支持的采样格式: {:?}",
                sample_format
            ))),
        }
    }

    /// 构建 f32 类型的音频流
    fn build_stream_f32<D, E>(
        &self,
        mut data_callback: D,
        error_callback: E,
    ) -> Result<Stream>
    where
        D: FnMut(&[f32]) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    data_callback(data);
                },
                error_callback,
                None,
            )
            .map_err(|e| {
                VoiceInputError::AudioDevice(format!("构建音频流失败: {}", e))
            })?;

        Ok(stream)
    }

    /// 构建 i16 类型的音频流
    fn build_stream_i16<D, E>(
        &self,
        mut data_callback: D,
        error_callback: E,
    ) -> Result<Stream>
    where
        D: FnMut(&[f32]) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let samples: Vec<f32> = data.iter().map(|&s| s.to_float_sample()).collect();
                    data_callback(&samples);
                },
                error_callback,
                None,
            )
            .map_err(|e| {
                VoiceInputError::AudioDevice(format!("构建音频流失败: {}", e))
            })?;

        Ok(stream)
    }

    /// 构建 u16 类型的音频流
    fn build_stream_u16<D, E>(
        &self,
        mut data_callback: D,
        error_callback: E,
    ) -> Result<Stream>
    where
        D: FnMut(&[f32]) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    let samples: Vec<f32> = data.iter().map(|&s| s.to_float_sample()).collect();
                    data_callback(&samples);
                },
                error_callback,
                None,
            )
            .map_err(|e| {
                VoiceInputError::AudioDevice(format!("构建音频流失败: {}", e))
            })?;

        Ok(stream)
    }

    /// 构建 i8 类型的音频流
    fn build_stream_i8<D, E>(
        &self,
        mut data_callback: D,
        error_callback: E,
    ) -> Result<Stream>
    where
        D: FnMut(&[f32]) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data: &[i8], _: &cpal::InputCallbackInfo| {
                    let samples: Vec<f32> = data.iter().map(|&s| s.to_float_sample()).collect();
                    data_callback(&samples);
                },
                error_callback,
                None,
            )
            .map_err(|e| {
                VoiceInputError::AudioDevice(format!("构建音频流失败: {}", e))
            })?;

        Ok(stream)
    }

    /// 构建 u8 类型的音频流
    fn build_stream_u8<D, E>(
        &self,
        mut data_callback: D,
        error_callback: E,
    ) -> Result<Stream>
    where
        D: FnMut(&[f32]) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data: &[u8], _: &cpal::InputCallbackInfo| {
                    let samples: Vec<f32> = data.iter().map(|&s| s.to_float_sample()).collect();
                    data_callback(&samples);
                },
                error_callback,
                None,
            )
            .map_err(|e| {
                VoiceInputError::AudioDevice(format!("构建音频流失败: {}", e))
            })?;

        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_creation() {
        // 这个测试可能在 CI 环境中失败（没有音频设备）
        // 仅在本地开发环境运行
        if std::env::var("CI").is_err() {
            let recorder = AudioRecorder::new(16000, 1);
            // 如果有麦克风应该成功
            if let Ok(rec) = recorder {
                assert_eq!(rec.sample_rate, 16000);
            }
        }
    }
}
