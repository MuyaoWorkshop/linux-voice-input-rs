use linux_voice_input_rs::{online::XfyunRealtimeRecognizer, output::ClipboardOutput, Config};

#[tokio::main]
async fn main() -> linux_voice_input_rs::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    println!("🎙️  Linux Voice Input - 讯飞云在线版");
    println!("=====================================\n");

    // 加载配置
    let config = Config::load()?;

    println!("📝 配置信息:");
    println!("  - 采样率: {} Hz", config.audio.sample_rate);
    println!("  - 声道数: {}", config.audio.channels);
    println!("  - 静音阈值: {}", config.whisper.silence_threshold);
    println!("  - 静音持续: {:.1} 秒", config.whisper.silence_duration);
    println!("  - 讯飞云 App ID: {}\n", config.xfyun.app_id);

    // 检查讯飞云配置
    if config.xfyun.app_id.is_empty()
        || config.xfyun.api_secret.is_empty()
        || config.xfyun.api_key.is_empty()
    {
        eprintln!("❌ 错误: 请在 config.toml 中配置讯飞云 API 密钥");
        eprintln!("   需要设置: app_id, api_secret, api_key");
        std::process::exit(1);
    }

    // 创建讯飞云实时识别器
    let recognizer = XfyunRealtimeRecognizer::new(
        config.xfyun.app_id.clone(),
        config.xfyun.api_secret.clone(),
        config.xfyun.api_key.clone(),
    );

    // 实时识别（边录边发送）
    let text = recognizer
        .recognize_realtime(config.audio.sample_rate)
        .await?;

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 识别结果:");
    println!("{}", text);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 复制到剪贴板
    let mut clipboard = ClipboardOutput::new()?;
    clipboard.copy(&text)?;

    println!("✅ 已复制到剪贴板，可以直接粘贴使用！");

    Ok(())
}
