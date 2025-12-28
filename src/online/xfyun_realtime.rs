use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use futures::{SinkExt, StreamExt};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use urlencoding::encode;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, StreamConfig};

use crate::utils::{Result, VoiceInputError};

type HmacSha256 = Hmac<Sha256>;

/// 讯飞云实时语音识别器
pub struct XfyunRealtimeRecognizer {
    app_id: String,
    api_secret: String,
    api_key: String,
}

impl XfyunRealtimeRecognizer {
    /// 创建新的讯飞云实时识别器
    pub fn new(app_id: String, api_secret: String, api_key: String) -> Self {
        Self {
            app_id,
            api_secret,
            api_key,
        }
    }

    /// 生成鉴权 URL
    fn generate_auth_url(&self) -> Result<String> {
        let host = "iat-api.xfyun.cn";
        let path = "/v2/iat";
        let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

        // 生成签名原文
        let signature_origin = format!("host: {}\ndate: {}\nGET {} HTTP/1.1", host, date, path);

        // HMAC-SHA256 加密
        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())
            .map_err(|e| VoiceInputError::Recognition(format!("HMAC 初始化失败: {}", e)))?;
        mac.update(signature_origin.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        // 生成 authorization
        let authorization_origin = format!(
            "api_key=\"{}\", algorithm=\"hmac-sha256\", headers=\"host date request-line\", signature=\"{}\"",
            self.api_key, signature
        );
        let authorization = general_purpose::STANDARD.encode(authorization_origin);

        // 生成最终 URL
        let url = format!(
            "wss://{}{}?authorization={}&date={}&host={}",
            host,
            path,
            encode(&authorization),
            encode(&date),
            host
        );

        tracing::debug!("生成鉴权 URL: {}", url);
        Ok(url)
    }

    /// 实时流式识别（边录边发送）
    pub async fn recognize_realtime(
        &self,
        sample_rate: u32,
        silence_duration: f32,
    ) -> Result<String> {
        println!("🌐 正在连接讯飞语音识别服务...");

        // 生成鉴权 URL
        let url = self.generate_auth_url()?;
        tracing::debug!("WebSocket URL: {}", url);

        // 建立 WebSocket 连接
        println!("正在建立 WebSocket 连接...");
        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| VoiceInputError::Recognition(format!("WebSocket 连接失败: {}", e)))?;

        println!("✅ 连接成功\n");

        let (write, mut read) = ws_stream.split();
        let result = Arc::new(Mutex::new(String::new()));
        let result_clone = result.clone();
        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_for_ctrlc = is_running.clone();
        let is_running_for_receive = is_running.clone();

        // Ctrl+C 处理
        ctrlc::set_handler(move || {
            println!("\n\n⏹️  用户停止录音...");
            is_running_for_ctrlc.store(false, Ordering::SeqCst);
        })
        .ok();

        // 启动接收任务
        let receive_task = tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        tracing::debug!("收到消息: {}", text);

                        // 解析返回结果
                        if let Ok(response) = serde_json::from_str::<serde_json::Value>(&text) {
                            // 检查错误
                            if let Some(code) = response.get("code") {
                                if code.as_i64() != Some(0) {
                                    if let Some(message) = response.get("message") {
                                        tracing::error!("识别错误: {}", message);
                                    }
                                    break;
                                }
                            }

                            // 提取识别结果
                            if let Some(data) = response.get("data") {
                                if let Some(result_obj) = data.get("result") {
                                    if let Some(ws) = result_obj.get("ws") {
                                        if let Some(ws_array) = ws.as_array() {
                                            let mut text_result = String::new();
                                            for item in ws_array {
                                                if let Some(cw) = item.get("cw") {
                                                    if let Some(cw_array) = cw.as_array() {
                                                        for word in cw_array {
                                                            if let Some(w) = word.get("w") {
                                                                if let Some(w_str) = w.as_str() {
                                                                    text_result.push_str(w_str);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            if !text_result.is_empty() {
                                                let mut result_lock = result_clone.lock().await;
                                                result_lock.push_str(&text_result);
                                                // 实时显示
                                                print!("\r识别中: {}", result_lock);
                                                use std::io::Write;
                                                std::io::stdout().flush().ok();
                                            }
                                        }
                                    }
                                }

                                // 检查是否结束（讯飞云 VAD 检测到静音）
                                if let Some(status) = data.get("status") {
                                    if status.as_i64() == Some(2) {
                                        println!("\n🔇 检测到静音，自动停止录音");
                                        tracing::info!("✅ 识别完成");
                                        // 通知发送端停止
                                        is_running_for_receive.store(false, Ordering::SeqCst);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        tracing::info!("WebSocket 连接关闭");
                        break;
                    }
                    Err(e) => {
                        tracing::error!("接收消息错误: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        // 录音并发送
        let app_id = self.app_id.clone();
        let send_result = self.record_and_send(
            write,
            sample_rate,
            app_id,
            is_running,
            silence_duration,
        ).await;

        // 等待接收完成
        receive_task.await.map_err(|e| {
            VoiceInputError::Recognition(format!("接收任务失败: {}", e))
        })?;

        // 检查发送是否出错
        send_result?;

        let final_result = result.lock().await.clone();
        println!("\n");  // 换行
        Ok(final_result)
    }

    /// 录音并实时发送
    async fn record_and_send(
        &self,
        mut write: futures::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>
            >,
            Message
        >,
        sample_rate: u32,
        app_id: String,
        is_running: Arc<AtomicBool>,
        silence_duration: f32,
    ) -> Result<()> {
        // 获取音频设备
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or(VoiceInputError::NoMicrophone)?;

        tracing::info!("使用音频设备: {}", device.name().unwrap_or_else(|_| "Unknown".to_string()));

        // 配置音频流
        let config = StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        // 音频缓冲区（用于收集 40ms 的数据）
        let frame_size = (sample_rate as f32 * 0.04) as usize; // 40ms
        let buffer = Arc::new(std::sync::Mutex::new(Vec::with_capacity(frame_size * 2)));

        // 构建音频流
        let supported_config = device
            .supported_input_configs()
            .map_err(|e| VoiceInputError::AudioDevice(format!("获取设备配置失败: {}", e)))?
            .next()
            .ok_or_else(|| VoiceInputError::AudioDevice("设备不支持任何配置".to_string()))?;

        let sample_format = supported_config.sample_format();

        let stream = match sample_format {
            SampleFormat::F32 => {
                let buffer_clone = Arc::clone(&buffer);
                device.build_input_stream(
                    &config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        let mut buf = buffer_clone.lock().unwrap();
                        buf.extend_from_slice(data);
                    },
                    |err| {
                        tracing::error!("录音错误: {}", err);
                    },
                    None,
                )
            }
            SampleFormat::I16 => {
                let buffer_clone = Arc::clone(&buffer);
                device.build_input_stream(
                    &config,
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        let mut buf = buffer_clone.lock().unwrap();
                        buf.extend(data.iter().map(|&s| s.to_float_sample()));
                    },
                    |err| {
                        tracing::error!("录音错误: {}", err);
                    },
                    None,
                )
            }
            SampleFormat::U16 => {
                let buffer_clone = Arc::clone(&buffer);
                device.build_input_stream(
                    &config,
                    move |data: &[u16], _: &cpal::InputCallbackInfo| {
                        let mut buf = buffer_clone.lock().unwrap();
                        buf.extend(data.iter().map(|&s| s.to_float_sample()));
                    },
                    |err| {
                        tracing::error!("录音错误: {}", err);
                    },
                    None,
                )
            }
            SampleFormat::I8 => {
                let buffer_clone = Arc::clone(&buffer);
                device.build_input_stream(
                    &config,
                    move |data: &[i8], _: &cpal::InputCallbackInfo| {
                        let mut buf = buffer_clone.lock().unwrap();
                        buf.extend(data.iter().map(|&s| s.to_float_sample()));
                    },
                    |err| {
                        tracing::error!("录音错误: {}", err);
                    },
                    None,
                )
            }
            SampleFormat::U8 => {
                let buffer_clone = Arc::clone(&buffer);
                device.build_input_stream(
                    &config,
                    move |data: &[u8], _: &cpal::InputCallbackInfo| {
                        let mut buf = buffer_clone.lock().unwrap();
                        buf.extend(data.iter().map(|&s| s.to_float_sample()));
                    },
                    |err| {
                        tracing::error!("录音错误: {}", err);
                    },
                    None,
                )
            }
            _ => {
                return Err(VoiceInputError::AudioDevice(format!(
                    "不支持的采样格式: {:?}",
                    sample_format
                )))
            }
        }
        .map_err(|e| VoiceInputError::AudioDevice(format!("构建音频流失败: {}", e)))?;

        // 开始录音
        stream.play().map_err(|e| {
            VoiceInputError::AudioRecord(format!("启动音频流失败: {}", e))
        })?;

        println!("🎤 开始录音... (按 Ctrl+C 停止)");
        println!("💡 说完话后保持静音 {:.1} 秒即可自动停止\n", silence_duration);

        let mut status = 0; // 0: 首帧, 1: 中间帧, 2: 末帧

        // 将静音持续时间转换为毫秒（讯飞云 vad_eos 参数）
        let vad_eos = (silence_duration * 1000.0) as u32;

        // 发送音频帧
        while is_running.load(Ordering::SeqCst) {
            tokio::time::sleep(tokio::time::Duration::from_millis(40)).await;

            // 读取缓冲区数据
            let audio_data = {
                let mut buf = buffer.lock().unwrap();
                if buf.len() < frame_size {
                    continue;
                }
                let data: Vec<f32> = buf.drain(..frame_size).collect();
                data
            };

            // 转换为 16-bit PCM
            let pcm_data: Vec<i16> = audio_data.iter().map(|&s| (s * 32767.0) as i16).collect();
            let bytes: Vec<u8> = pcm_data
                .iter()
                .flat_map(|&s: &i16| s.to_le_bytes())
                .collect();

            let audio_b64 = general_purpose::STANDARD.encode(&bytes);

            // 构建消息
            let frame_msg = if status == 0 {
                // 首帧
                serde_json::json!({
                    "common": {
                        "app_id": app_id
                    },
                    "business": {
                        "language": "zh_cn",
                        "domain": "iat",
                        "accent": "mandarin",
                        "vad_eos": vad_eos  // 使用配置的静音超时时间
                    },
                    "data": {
                        "status": 0,
                        "format": "audio/L16;rate=16000",
                        "encoding": "raw",
                        "audio": audio_b64
                    }
                })
            } else {
                // 中间帧
                serde_json::json!({
                    "data": {
                        "status": 1,
                        "format": "audio/L16;rate=16000",
                        "encoding": "raw",
                        "audio": audio_b64
                    }
                })
            };

            // 发送
            if let Err(e) = write.send(Message::Text(frame_msg.to_string())).await {
                tracing::warn!("发送音频失败: {}", e);
                break;
            }

            if status == 0 {
                status = 1;
            }
        }

        // 发送结束帧
        let end_frame = serde_json::json!({
            "data": {
                "status": 2,
                "format": "audio/L16;rate=16000",
                "encoding": "raw",
                "audio": ""
            }
        });

        write.send(Message::Text(end_frame.to_string())).await.ok();
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        // 停止录音
        drop(stream);

        Ok(())
    }
}
