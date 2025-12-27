# Rust 重写项目规划文档

**创建时间**: 2025-12-28
**目的**: 使用 Rust 全栈重写语音输入工具，实现离线和在线双方案

---

## 📋 项目概要

### 项目名称
`linux-voice-input-rs` 或 `voice-input-rust`

### 仓库信息
- **GitHub**: https://github.com/MuyaoWorkshop/linux-voice-input-rs
- **Gitee**: https://gitee.com/muyaoworkshop/linux-voice-input-rs
- **原项目**: linux-voice-input (Python 版本)

### 项目定位
Linux 桌面中文语音转文字工具 - Rust 高性能版本

**核心优势**:
- ⚡ 离线识别速度提升 80%（4-5秒 → <1秒）
- 💾 内存占用减少 78%（900MB → 200MB）
- 📦 单个二进制文件部署（5MB）
- 🔒 类型安全，编译时保证正确性

---

## 🛠️ 技术栈

### 核心语言
- **Rust** 1.75+ (stable)
- **Edition**: 2021

### 关键依赖库

```toml
[dependencies]
# 异步运行时
tokio = { version = "1.35", features = ["full"] }

# Whisper 绑定（离线识别）
whisper-rs = "0.10"

# WebSocket 客户端（在线识别）
tokio-tungstenite = "0.21"

# JSON 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 加密和编码
base64 = "0.21"
hmac = "0.12"
sha2 = "0.10"

# 音频录制
cpal = "0.15"

# CLI 参数解析
clap = { version = "4.4", features = ["derive"] }

# 配置文件
toml = "0.8"

# 错误处理
anyhow = "1.0"
thiserror = "1.0"

# 日志
tracing = "0.1"
tracing-subscriber = "0.3"

# 剪贴板操作（通过 xclip）
```

### 系统依赖
- ALSA 或 PulseAudio（音频录制）
- xclip（剪贴板操作）
- Whisper 模型文件（ggml 格式）

---

## 🏗️ 项目架构

### 目录结构

```
linux-voice-input-rs/
├── Cargo.toml                  # 项目配置和依赖
├── Cargo.lock                  # 依赖锁定
├── build.rs                    # 构建脚本
├── README.md                   # 项目说明
├── LICENSE                     # MIT 许可证
│
├── src/
│   ├── main.rs                 # 程序入口
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   └── config.toml.example # 配置模板
│   │
│   ├── audio/                  # 音频录制模块（共用）
│   │   ├── mod.rs
│   │   ├── recorder.rs         # 录音器
│   │   └── processor.rs        # 音频处理
│   │
│   ├── whisper/                # 离线识别模块
│   │   ├── mod.rs
│   │   ├── engine.rs           # Whisper 引擎封装
│   │   └── transcribe.rs       # 识别逻辑
│   │
│   ├── xfyun/                  # 在线识别模块
│   │   ├── mod.rs
│   │   ├── client.rs           # WebSocket 客户端
│   │   ├── auth.rs             # 认证签名
│   │   └── protocol.rs         # 协议处理
│   │
│   └── utils/
│       ├── mod.rs
│       ├── clipboard.rs        # 剪贴板操作
│       └── error.rs            # 错误类型定义
│
├── scripts/
│   ├── install.sh              # 安装脚本
│   ├── setup_shortcuts.sh      # 快捷键配置
│   └── download_models.sh      # 下载 Whisper 模型
│
├── models/                     # Whisper 模型（.gitignore）
│   └── .gitkeep
│
├── config/
│   └── config.toml.example     # 配置文件模板
│
└── docs/
    ├── INSTALL.md              # 安装指南
    ├── USAGE.md                # 使用说明
    ├── PERFORMANCE.md          # 性能对比
    └── DEVELOPMENT.md          # 开发指南
```

---

## 📊 应用逻辑流程

### 整体流程图

```
用户按快捷键
    ↓
启动程序（指定模式）
    ├─ local（离线）
    │    ↓
    │  录音 → Whisper 识别 → 复制到剪贴板
    │
    └─ xfyun（在线）
         ↓
       录音 + WebSocket 实时传输 → 流式识别 → 复制到剪贴板
```

### 离线方案详细流程

```rust
// 1. 启动程序
main() -> whisper::run()

// 2. 录音
audio::record()
    → 录制 16kHz, 单声道, S16LE
    → 静音检测（3秒静音自动停止）
    → 返回 Vec<f32>

// 3. 加载模型
WhisperContext::new("models/ggml-base.bin")
    → 内存映射模型文件
    → 初始化推理引擎

// 4. 识别
ctx.full(params, audio_data)
    → 设置语言: zh
    → Greedy 解码策略
    → 返回文本段落

// 5. 复制
clipboard::copy(text)
    → 调用 xclip
    → 显示结果
```

### 在线方案详细流程

```rust
// 1. 启动程序
main() -> xfyun::run()

// 2. 认证
auth::generate_url(config)
    → 读取 APPID, APISecret, APIKey
    → 生成 HMAC-SHA256 签名
    → 构造 WebSocket URL

// 3. 连接
connect_async(url)
    → 建立 WebSocket 连接
    → 分离读写流

// 4. 并发处理（tokio::spawn）
├─ 录音线程
│   → audio::record_stream()
│   → 分块（每 1280 字节）
│   → 发送到 WebSocket
│
└─ 接收线程
    → 接收 JSON 消息
    → 解析识别结果
    → 累积文本
    → 实时显示

// 5. 结束
→ 按 Ctrl+C 停止录音
→ 等待剩余结果
→ 复制完整文本到剪贴板
```

---

## 🎯 核心功能实现

### 1. 离线识别（Whisper）

**关键代码逻辑**:

```rust
pub async fn run(config: &Config) -> Result<String> {
    // 步骤1: 录音
    println!("🎤 开始录音... (停顿3秒自动结束)");
    let audio_data = audio::record(Duration::from_secs(60)).await?;

    // 步骤2: 加载模型
    let model_path = config.whisper.model_path.clone();
    let ctx = WhisperContext::new(&model_path)?;

    // 步骤3: 配置识别参数
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("zh"));
    params.set_translate(false);
    params.set_print_special(false);
    params.set_print_progress(false);

    // 步骤4: 执行识别
    println!("⏳ 正在识别...");
    ctx.full(params, &audio_data)?;

    // 步骤5: 提取结果
    let num_segments = ctx.full_n_segments()?;
    let mut result = String::new();
    for i in 0..num_segments {
        let text = ctx.full_get_segment_text(i)?;
        result.push_str(&text);
    }

    Ok(result.trim().to_string())
}
```

**性能优化点**:
- 使用内存映射加载模型（mmap）
- 避免不必要的内存拷贝
- 多线程推理（Whisper 内部支持）

### 2. 在线识别（讯飞云）

**关键代码逻辑**:

```rust
pub async fn run(config: &Config) -> Result<String> {
    // 步骤1: 生成认证 URL
    let url = auth::generate_auth_url(&config.xfyun)?;

    // 步骤2: 建立连接
    println!("🌐 正在连接讯飞云...");
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // 步骤3: 发送音频帧（异步任务）
    let send_handle = tokio::spawn(async move {
        println!("🎤 开始录音... (按 Ctrl+C 停止)");
        let mut recorder = audio::Recorder::new()?;

        loop {
            let chunk = recorder.read_chunk(1280)?;
            let frame = protocol::build_audio_frame(&chunk, false);
            write.send(Message::Text(frame)).await?;
        }
    });

    // 步骤4: 接收识别结果
    let mut result = String::new();
    while let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => {
                let partial = protocol::parse_result(&text)?;
                result.push_str(&partial);
                print!("\r识别中: {}", result);
                std::io::stdout().flush()?;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // 清理
    send_handle.abort();
    println!("\n✓ 识别完成");

    Ok(result)
}
```

**讯飞云协议处理**:

```rust
// 认证签名
pub fn generate_auth_url(config: &XfyunConfig) -> Result<String> {
    let host = "iat-api.xfyun.cn";
    let path = "/v2/iat";
    let date = httpdate::fmt_http_date(SystemTime::now());

    // 构造签名字符串
    let signature_origin = format!(
        "host: {}\ndate: {}\nGET {} HTTP/1.1",
        host, date, path
    );

    // HMAC-SHA256 签名
    let mut mac = Hmac::<Sha256>::new_from_slice(config.api_secret.as_bytes())?;
    mac.update(signature_origin.as_bytes());
    let signature = base64::encode(mac.finalize().into_bytes());

    // 构造 Authorization
    let authorization = format!(
        "api_key=\"{}\", algorithm=\"hmac-sha256\", headers=\"host date request-line\", signature=\"{}\"",
        config.api_key, signature
    );

    // 构造 URL
    let url = format!(
        "wss://{}{}?authorization={}&date={}&host={}",
        host, path,
        urlencoding::encode(&authorization),
        urlencoding::encode(&date),
        host
    );

    Ok(url)
}

// 音频帧构造
pub fn build_audio_frame(audio_data: &[u8], is_end: bool) -> String {
    let audio_b64 = base64::encode(audio_data);

    json!({
        "common": {
            "app_id": config.app_id
        },
        "business": {
            "language": "zh_cn",
            "domain": "iat",
            "accent": "mandarin",
            "vad_eos": 3000
        },
        "data": {
            "status": if is_end { 2 } else { 1 },
            "format": "audio/L16;rate=16000",
            "encoding": "raw",
            "audio": audio_b64
        }
    }).to_string()
}
```

### 3. 音频录制模块（共用）

```rust
pub struct Recorder {
    stream: Stream,
    sample_rate: u32,
    channels: u16,
    silence_threshold: f32,
    silence_duration: Duration,
}

impl Recorder {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| anyhow!("未找到麦克风"))?;

        let config = device.default_input_config()?;

        // ... 初始化音频流

        Ok(Self {
            stream,
            sample_rate: 16000,
            channels: 1,
            silence_threshold: 0.02,
            silence_duration: Duration::from_secs(3),
        })
    }

    // 录制完整音频（离线方案）
    pub async fn record(&mut self, max_duration: Duration) -> Result<Vec<f32>> {
        let mut buffer = Vec::new();
        let mut silent_chunks = 0;
        let max_silent_chunks = self.calculate_silent_chunks();

        self.stream.play()?;

        loop {
            let chunk = self.read_chunk(1024)?;
            buffer.extend_from_slice(&chunk);

            let volume = self.calculate_volume(&chunk);

            if volume < self.silence_threshold {
                silent_chunks += 1;
                if silent_chunks > max_silent_chunks {
                    println!("\n✓ 检测到静音，停止录音");
                    break;
                }
            } else {
                silent_chunks = 0;
                self.print_volume_bar(volume);
            }

            if buffer.len() > max_duration.as_secs() as usize * self.sample_rate as usize {
                break;
            }
        }

        self.stream.pause()?;
        Ok(buffer)
    }

    // 读取音频块（在线方案）
    pub fn read_chunk(&mut self, size: usize) -> Result<Vec<u8>> {
        // ... 从音频流读取指定大小的数据
    }
}
```

### 4. 剪贴板操作

```rust
pub fn copy(text: &str) -> Result<()> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut child = Command::new("xclip")
        .args(&["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }

    child.wait()?;
    Ok(())
}
```

---

## 📝 配置文件设计

### config.toml

```toml
[whisper]
# Whisper 模型路径
model_path = "~/.local/share/voice-input/models/ggml-base.bin"
# 识别语言
language = "zh"
# 最大录音时长（秒）
max_duration = 60
# 静音检测阈值（0.0-1.0）
silence_threshold = 0.02
# 静音持续时间（秒）
silence_duration = 3

[xfyun]
# 讯飞云 API 密钥（从 https://console.xfyun.cn/ 获取）
app_id = "your_app_id"
api_secret = "your_api_secret"
api_key = "your_api_key"

[audio]
# 采样率
sample_rate = 16000
# 声道数
channels = 1
# 音频格式
format = "S16LE"

[ui]
# 是否显示音量条
show_volume_bar = true
# 是否显示实时结果
show_realtime_result = true
```

---

## 🚀 实施计划

### 阶段 1: 项目搭建（Day 1 上午）

**任务清单**:
- [ ] 创建 GitHub/Gitee 仓库
- [ ] 初始化 Cargo 项目：`cargo new --bin linux-voice-input-rs`
- [ ] 配置 Cargo.toml 依赖
- [ ] 创建基础目录结构
- [ ] 编写 README.md 和 LICENSE

**验收标准**:
```bash
cargo build  # 编译通过
cargo run -- --help  # 显示帮助信息
```

### 阶段 2: 离线方案实现（Day 1 下午 - Day 2 上午）

**任务清单**:
- [ ] 集成 whisper-rs
- [ ] 实现音频录制（audio/recorder.rs）
- [ ] 实现 Whisper 引擎封装（whisper/engine.rs）
- [ ] 实现识别逻辑（whisper/transcribe.rs）
- [ ] 实现剪贴板操作（utils/clipboard.rs）
- [ ] 端到端测试

**验收标准**:
```bash
cargo run -- --mode local
# 录音 → 识别 → 复制成功
```

### 阶段 3: 在线方案实现（Day 2 下午 - Day 3 上午）

**任务清单**:
- [ ] 实现讯飞云认证（xfyun/auth.rs）
- [ ] 实现 WebSocket 客户端（xfyun/client.rs）
- [ ] 实现协议处理（xfyun/protocol.rs）
- [ ] 异步音频流传输
- [ ] 实时结果显示
- [ ] 端到端测试

**验收标准**:
```bash
cargo run -- --mode xfyun
# 连接 → 录音 → 流式识别 → 复制成功
```

### 阶段 4: 配置和工具（Day 3 下午）

**任务清单**:
- [ ] 实现配置文件读取（config/mod.rs）
- [ ] 编写安装脚本（scripts/install.sh）
- [ ] 编写模型下载脚本（scripts/download_models.sh）
- [ ] 快捷键配置脚本（scripts/setup_shortcuts.sh）
- [ ] 错误处理完善

**验收标准**:
```bash
./scripts/install.sh  # 一键安装成功
# 快捷键可用
```

### 阶段 5: 优化和发布（Day 4）

**任务清单**:
- [ ] 性能测试和对比
- [ ] 内存占用优化
- [ ] 二进制体积优化（strip, upx）
- [ ] 编写文档（docs/）
- [ ] CI/CD 配置（GitHub Actions）
- [ ] 发布第一个 Release

**验收标准**:
- 性能指标达标：
  - 离线启动 <1 秒
  - 内存占用 <250MB
  - 二进制体积 <10MB
- 文档完整
- GitHub Release 发布

---

## 🎯 性能目标

### 对比基准（Python 版本）

| 指标 | Python | Rust 目标 | 提升 |
|------|--------|----------|------|
| **离线启动** | 4-5秒 | <1秒 | 80%+ |
| **离线识别** | 3-5秒 | 2-3秒 | 30%+ |
| **内存占用** | 900MB | <250MB | 72%+ |
| **在线延迟** | 网络+5ms | 网络+2ms | 微小 |
| **二进制大小** | - | <10MB | - |
| **部署复杂度** | 需 Python 环境 | 单文件 | 巨大 |

### 性能测试方法

```bash
# 启动速度
time cargo run --release -- --mode local < /dev/null

# 内存占用
/usr/bin/time -v cargo run --release -- --mode local

# 二进制大小
ls -lh target/release/linux-voice-input-rs
strip target/release/linux-voice-input-rs
ls -lh target/release/linux-voice-input-rs
```

---

## 📚 参考资料

### Rust 学习资源
- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio 教程](https://tokio.rs/tokio/tutorial)

### 关键库文档
- [whisper-rs](https://docs.rs/whisper-rs/)
- [tokio-tungstenite](https://docs.rs/tokio-tungstenite/)
- [cpal](https://docs.rs/cpal/)
- [clap](https://docs.rs/clap/)

### API 文档
- [讯飞语音听写 WebSocket API](https://www.xfyun.cn/doc/asr/voicedictation/API.html)
- [Whisper.cpp](https://github.com/ggerganov/whisper.cpp)

---

## ⚠️ 关键注意事项

### 1. Whisper 模型文件
- 使用 ggml 格式（whisper.cpp 兼容）
- 不要提交到 Git（太大）
- 提供下载脚本

### 2. 音频格式
- 统一使用 16kHz, 单声道, S16LE
- Whisper 需要 f32 格式
- 讯飞云需要 raw PCM

### 3. 异步编程
- 使用 Tokio 运行时
- 注意异步函数的错误传播
- 避免阻塞操作在异步上下文中

### 4. 错误处理
- 使用 `anyhow` 简化错误传播
- 使用 `thiserror` 定义自定义错误
- 所有可能失败的操作都要处理错误

### 5. 跨平台考虑
- 剪贴板操作：Linux 使用 xclip
- 音频录制：使用 cpal 跨平台库
- 路径处理：使用 std::path

---

## 🔄 后续迭代计划

### v1.1 - GUI 版本
- 使用 egui 或 iced
- 系统托盘图标
- 可视化配置

### v1.2 - 更多功能
- 支持英文识别
- 支持方言
- 自定义词库

### v1.3 - 性能极致优化
- GPU 加速（CUDA）
- 模型量化
- 自定义 Whisper 后端

---

## 📞 联系方式

- **GitHub Issues**: https://github.com/MuyaoWorkshop/linux-voice-input-rs/issues
- **原项目**: https://github.com/MuyaoWorkshop/linux-voice-input

---

**最后更新**: 2025-12-28
**文档版本**: v1.0
**状态**: 待实施
