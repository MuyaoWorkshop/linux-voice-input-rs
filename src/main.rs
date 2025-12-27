use linux_voice_input_rs::{audio::AudioRecorder, Config};
use std::time::Duration;

fn main() -> linux_voice_input_rs::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    println!("🎙️  Linux Voice Input - Rust 版本");
    println!("=====================================\n");

    // 加载配置
    let config = Config::load()?;

    println!("📝 配置信息:");
    println!("  - 采样率: {} Hz", config.audio.sample_rate);
    println!("  - 声道数: {}", config.audio.channels);
    println!("  - 静音阈值: {}", config.whisper.silence_threshold);
    println!("  - 静音持续: {:.1} 秒\n", config.whisper.silence_duration);

    // 创建音频录制器
    let recorder = AudioRecorder::new(config.audio.sample_rate, config.audio.channels)?;

    // 录制音频
    let audio_buffer = recorder.record_until_silence(
        Duration::from_secs(config.whisper.max_duration),
        config.whisper.silence_threshold,
        config.whisper.silence_duration,
    )?;

    println!("\n📊 录制统计:");
    println!("  - 样本数: {}", audio_buffer.len());
    println!("  - 时长: {:.2} 秒", audio_buffer.len() as f32 / config.audio.sample_rate as f32);
    println!("  - 数据大小: {:.2} KB", (audio_buffer.len() * 4) as f32 / 1024.0);

    println!("\n✅ 测试成功！音频录制功能正常工作。");
    println!("提示: 后续将实现 Whisper 识别功能。");

    Ok(())
}
