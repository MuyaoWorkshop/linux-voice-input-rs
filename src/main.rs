use linux_voice_input_rs::{
    audio::AudioRecorder, output::ClipboardOutput, whisper::WhisperEngine, Config,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> linux_voice_input_rs::Result<()> {
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
    println!("  - 模型路径: {}", config.whisper.model_path);
    println!("  - 识别语言: {}", config.whisper.language);
    println!("  - 静音阈值: {}", config.whisper.silence_threshold);
    println!("  - 静音持续: {:.1} 秒\n", config.whisper.silence_duration);

    // 步骤 1: 创建 Whisper 引擎
    println!("⏳ 正在加载 Whisper 模型...");
    let mut engine = WhisperEngine::new(&config.whisper.model_path, &config.whisper.language)?;
    println!("✅ 模型加载成功\n");

    // 步骤 2: 创建音频录制器
    let recorder = AudioRecorder::new(config.audio.sample_rate, config.audio.channels)?;

    // 步骤 3: 录制音频
    let audio_buffer = recorder.record_until_silence(
        Duration::from_secs(config.whisper.max_duration),
        config.whisper.silence_threshold,
        config.whisper.silence_duration,
    )?;

    println!("\n📊 录制统计:");
    println!("  - 样本数: {}", audio_buffer.len());
    println!(
        "  - 时长: {:.2} 秒",
        audio_buffer.len() as f32 / config.audio.sample_rate as f32
    );
    println!(
        "  - 数据大小: {:.2} KB",
        (audio_buffer.len() * 4) as f32 / 1024.0
    );

    // 步骤 4: 识别音频
    println!("\n⏳ 正在识别...");
    let text = engine.transcribe(audio_buffer).await?;

    println!("\n✅ 识别完成！");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 识别结果:");
    println!("{}", text);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // 步骤 5: 复制到剪贴板
    let mut clipboard = ClipboardOutput::new()?;
    clipboard.copy(&text)?;

    println!("✅ 已复制到剪贴板，可以直接粘贴使用！");

    Ok(())
}
