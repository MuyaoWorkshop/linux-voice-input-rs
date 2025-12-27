# Linux Voice Input - Rust 重写详细规划

**项目仓库**: https://github.com/MuyaoWorkshop/linux-voice-input-rs
**创建时间**: 2025-12-28
**文档状态**: 详细技术规划

---

## 📋 目录

1. [项目目标与核心需求](#1-项目目标与核心需求)
2. [功能列表与优先级](#2-功能列表与优先级)
3. [技术架构设计](#3-技术架构设计)
4. [核心模块设计](#4-核心模块设计)
5. [关键技术决策](#5-关键技术决策)
6. [实施计划](#6-实施计划)
7. [性能指标](#7-性能指标)
8. [风险与挑战](#8-风险与挑战)

---

## 1. 项目目标与核心需求

### 1.1 核心目标

**解决 Python 版本的主要痛点**：
- ✅ 降低内存占用：从 900MB → <250MB （目标减少 70%+）
- ✅ 简化部署：单二进制文件，无需 Python 虚拟环境
- ✅ 提升用户体验：更快的启动速度和更低的延迟

**不降低现有能力**：
- ✅ 保持离线识别准确性
- ✅ 保持在线识别的流式体验
- ✅ 保持配置的灵活性

### 1.2 使用场景分析

| 场景 | 特点 | 优先级 | 推荐方案 |
|------|------|--------|----------|
| **编写文档/代码注释** | 需要高准确性，可容忍 1-2 秒延迟 | P0 | 离线 base 模型 |
| **即时聊天/邮件回复** | 需要低延迟，实时反馈 | P0 | 在线讯飞云 |
| **会议记录/长文本** | 持续录音，稳定性要求高 | P1 | 在线讯飞云 |
| **命令控制/快速输入** | 极低延迟（<1 秒），短语音 | P2 | 离线 tiny 模型 |

### 1.3 功能需求

#### 必须功能（MVP）
1. **离线识别**：
   - 基于 Whisper base 模型
   - 支持中文普通话
   - 静音检测自动停止录音
   - 识别结果复制到剪贴板

2. **在线识别**：
   - 讯飞云 WebSocket API
   - 流式实时识别
   - 结果复制到剪贴板

3. **音频录制**：
   - 16kHz, 单声道, S16LE
   - 音量可视化
   - 跨平台麦克风支持

4. **配置管理**：
   - TOML 配置文件
   - 环境变量支持（API 密钥）
   - 多路径配置查找

5. **命令行界面**：
   - 模式选择：`--mode local|xfyun`
   - 模型选择：`--model base|small|tiny`
   - 详细日志：`--verbose`

#### 计划功能（v1.1+）
1. 结果保存到文件（`--output file.txt`）
2. 自动键盘输入（`--auto-type`）
3. 全局快捷键支持（守护进程模式）
4. 多 Whisper 模型支持（tiny/small/medium）
5. 更多在线 API（阿里云、腾讯云、百度）

#### 未来功能（v2.0+）
1. GUI 系统托盘
2. 自定义词库和热词
3. 实时转写（边录边识别）
4. GPU 加速（CUDA）
5. 多语言支持（英文、粤语）

---

## 2. 功能列表与优先级

### 2.1 MVP 功能拆解（v0.1.0）

**Phase 1: 离线方案（Week 1）**

| 功能点 | 描述 | 优先级 | 预计工作量 |
|--------|------|--------|------------|
| 音频录制 | 基于 cpal，支持静音检测 | P0 | 1 天 |
| Whisper 集成 | whisper-rs 绑定，base 模型 | P0 | 1 天 |
| 剪贴板操作 | 基于 arboard，跨平台 | P0 | 0.5 天 |
| 命令行参数 | clap 解析，基础参数 | P0 | 0.5 天 |
| 配置文件 | TOML 读取，默认配置 | P1 | 0.5 天 |
| 错误处理 | 统一错误类型，友好提示 | P1 | 0.5 天 |

**Phase 2: 在线方案（Week 2）**

| 功能点 | 描述 | 优先级 | 预计工作量 |
|--------|------|--------|------------|
| 讯飞云认证 | HMAC-SHA256 签名 | P0 | 0.5 天 |
| WebSocket 客户端 | tokio-tungstenite | P0 | 1 天 |
| 协议处理 | JSON 序列化/反序列化 | P0 | 1 天 |
| 流式识别 | 异步音频流传输 | P0 | 1 天 |
| 断线重连 | 自动重连机制 | P1 | 0.5 天 |
| 实时显示 | 终端进度条和结果 | P1 | 0.5 天 |

**Phase 3: 完善与优化（Week 3）**

| 功能点 | 描述 | 优先级 | 预计工作量 |
|--------|------|--------|------------|
| 单元测试 | 核心模块测试覆盖 | P0 | 1 天 |
| 集成测试 | 端到端测试 | P1 | 0.5 天 |
| 性能优化 | 内存和启动速度 | P0 | 1 天 |
| 文档编写 | README, INSTALL, USAGE | P0 | 1 天 |
| 安装脚本 | 一键安装和配置 | P1 | 0.5 天 |
| CI/CD | GitHub Actions | P1 | 0.5 天 |

### 2.2 版本规划

**v0.1.0 - MVP**（3 周）
- ✅ 离线识别（Whisper base）
- ✅ 在线识别（讯飞云）
- ✅ 命令行工具
- ✅ 剪贴板集成
- ✅ 基本配置管理

**v0.2.0 - 功能增强**（1-2 周）
- ✅ 结果保存到文件
- ✅ 多 Whisper 模型支持
- ✅ 更好的错误处理和重试
- ✅ 性能基准测试
- ✅ 打包脚本（deb/rpm）

**v1.0.0 - 稳定版本**（2-3 周）
- ✅ 自动键盘输入
- ✅ 全局快捷键（守护进程）
- ✅ 完善的文档和示例
- ✅ 性能调优
- ✅ 跨架构编译（x86_64/aarch64）

**v1.1.0+ - 扩展功能**（按需）
- 更多在线 API（插件化架构）
- GUI 系统托盘
- 自定义词库
- 实时转写

---

## 3. 技术架构设计

### 3.1 整体架构

```
┌─────────────────────────────────────────────────┐
│              CLI Entry Point (main.rs)          │
│  - 参数解析 (clap)                               │
│  - 配置加载 (config)                             │
│  - 模式分发 (local/online)                       │
└─────────────────┬───────────────────────────────┘
                  │
        ┌─────────┴─────────┐
        │                   │
        ▼                   ▼
┌───────────────┐   ┌───────────────┐
│  Local Mode   │   │  Online Mode  │
│   (Whisper)   │   │  (XFyun API)  │
└───────┬───────┘   └───────┬───────┘
        │                   │
        └─────────┬─────────┘
                  │
        ┌─────────┴─────────┐
        ▼                   ▼
┌───────────────┐   ┌───────────────┐
│ Audio Module  │   │ Output Module │
│  (recorder)   │   │ (clipboard)   │
└───────────────┘   └───────────────┘
```

### 3.2 模块划分

```
src/
├── main.rs                 # 程序入口，CLI 参数解析
├── lib.rs                  # 库入口，导出公共 API
│
├── config/                 # 配置模块
│   ├── mod.rs
│   ├── loader.rs           # 配置加载器（多路径查找）
│   └── types.rs            # 配置结构定义
│
├── audio/                  # 音频模块（共用）
│   ├── mod.rs
│   ├── recorder.rs         # 音频录制器
│   ├── processor.rs        # 音频处理（格式转换、重采样）
│   └── silence.rs          # 静音检测
│
├── recognizer/             # 识别引擎（多态设计）
│   ├── mod.rs              # Recognizer trait 定义
│   ├── local.rs            # 离线识别实现
│   └── online.rs           # 在线识别实现
│
├── whisper/                # Whisper 引擎（离线）
│   ├── mod.rs
│   ├── engine.rs           # Whisper 上下文封装
│   ├── model.rs            # 模型加载和管理
│   └── params.rs           # 识别参数配置
│
├── online/                 # 在线 API 模块（可扩展）
│   ├── mod.rs              # OnlineProvider trait
│   ├── xfyun/              # 讯飞云实现
│   │   ├── mod.rs
│   │   ├── client.rs       # WebSocket 客户端
│   │   ├── auth.rs         # 认证签名
│   │   └── protocol.rs     # 协议编解码
│   ├── aliyun/             # 阿里云（预留）
│   └── tencent/            # 腾讯云（预留）
│
├── output/                 # 输出模块
│   ├── mod.rs              # Output trait 定义
│   ├── clipboard.rs        # 剪贴板输出
│   ├── file.rs             # 文件输出
│   └── keyboard.rs         # 键盘输入（未来）
│
└── utils/                  # 工具模块
    ├── mod.rs
    ├── error.rs            # 错误类型定义
    ├── logger.rs           # 日志初始化
    └── platform.rs         # 平台检测（X11/Wayland）
```

### 3.3 核心数据流

**离线模式数据流**：
```
用户命令
  ↓
录音器 (cpal)
  ↓ Vec<f32> (16kHz, mono)
静音检测
  ↓ (检测到静音)
Whisper 引擎 (spawn_blocking)
  ↓ String (识别结果)
剪贴板输出 (arboard)
  ↓
用户粘贴使用
```

**在线模式数据流**：
```
用户命令
  ↓
认证 (HMAC-SHA256)
  ↓ WebSocket URL
连接 XFyun API
  ↓ (连接成功)
并发任务：
  ├─ 录音线程: 音频块 → WebSocket (发送)
  └─ 接收线程: WebSocket (接收) → 解析 JSON → 累积文本
  ↓ (Ctrl+C 或静音)
关闭连接
  ↓ 完整文本
剪贴板输出
```

---

## 4. 核心模块设计

### 4.1 配置模块

**设计目标**：
- 支持多路径配置查找（优先级：CLI > 环境变量 > 配置文件）
- 支持环境变量覆盖敏感信息（API 密钥）
- 提供合理的默认值

**配置文件结构**：
```toml
[whisper]
model_path = "~/.local/share/voice-input/models/ggml-base.bin"
language = "zh"
max_duration = 60
silence_threshold = 0.02
silence_duration = 3.0

[xfyun]
app_id = "${XFYUN_APP_ID}"       # 支持环境变量
api_secret = "${XFYUN_API_SECRET}"
api_key = "${XFYUN_API_KEY}"

[audio]
sample_rate = 16000
channels = 1
chunk_size = 1280

[output]
default = "clipboard"             # clipboard | file | keyboard
file_path = "~/voice-output.txt"
```

**配置查找顺序**：
1. `./voice-input.toml` （当前目录）
2. `~/.config/voice-input/config.toml` （用户配置）
3. `/etc/voice-input/config.toml` （系统配置）
4. 内置默认配置

### 4.2 音频模块

**关键设计**：

```rust
pub struct AudioRecorder {
    device: Device,
    config: StreamConfig,
    silence_detector: SilenceDetector,
}

impl AudioRecorder {
    /// 录制音频直到静音（离线模式）
    pub async fn record_until_silence(&mut self) -> Result<AudioBuffer> {
        // ...
    }

    /// 流式录制音频（在线模式）
    pub fn stream(&mut self) -> impl Stream<Item = AudioChunk> {
        // ...
    }
}

pub struct SilenceDetector {
    threshold: f32,
    duration: Duration,
    // ...
}

impl SilenceDetector {
    pub fn is_silent(&mut self, chunk: &[f32]) -> bool {
        // 计算音量 RMS
        // 检查是否低于阈值且持续足够时间
    }
}
```

### 4.3 识别引擎（多态设计）

**Trait 定义**：
```rust
#[async_trait]
pub trait Recognizer {
    /// 识别音频
    async fn recognize(&mut self, audio: AudioBuffer) -> Result<String>;

    /// 流式识别（可选）
    async fn recognize_stream(
        &mut self,
        audio_stream: impl Stream<Item = AudioChunk>
    ) -> Result<String> {
        Err(Error::NotSupported)
    }
}

pub struct LocalRecognizer {
    whisper_engine: Arc<WhisperEngine>,
}

pub struct OnlineRecognizer {
    provider: Box<dyn OnlineProvider>,
}
```

### 4.4 在线 API 抽象（可扩展）

```rust
#[async_trait]
pub trait OnlineProvider: Send + Sync {
    /// 连接到服务
    async fn connect(&mut self) -> Result<()>;

    /// 发送音频数据
    async fn send_audio(&mut self, chunk: AudioChunk) -> Result<()>;

    /// 接收识别结果（流式）
    async fn receive(&mut self) -> Result<Option<String>>;

    /// 关闭连接
    async fn close(&mut self) -> Result<String>;
}

// 讯飞云实现
pub struct XFyunProvider {
    config: XFyunConfig,
    ws_stream: Option<WebSocketStream>,
    result_buffer: String,
}

// 未来可以添加：
// pub struct AliyunProvider { ... }
// pub struct TencentProvider { ... }
```

### 4.5 输出模块

```rust
#[async_trait]
pub trait OutputHandler {
    async fn output(&mut self, text: &str) -> Result<()>;
}

pub struct ClipboardOutput;
pub struct FileOutput { path: PathBuf }
pub struct KeyboardOutput;  // 未来实现

// 组合输出
pub struct MultiOutput {
    handlers: Vec<Box<dyn OutputHandler>>,
}
```

---

## 5. 关键技术决策

### 5.1 依赖选型

| 功能 | 库 | 版本 | 理由 |
|------|-----|------|------|
| **异步运行时** | tokio | 1.35+ | 成熟稳定，生态丰富 |
| **Whisper 绑定** | whisper-rs | 0.10+ | 官方推荐，性能好 |
| **WebSocket** | tokio-tungstenite | 0.21+ | 异步支持，活跃维护 |
| **音频录制** | cpal | 0.15+ | 跨平台，ALSA/PulseAudio |
| **剪贴板** | arboard | 3.3+ | 跨平台，支持 X11/Wayland |
| **CLI 参数** | clap | 4.4+ | 强大，类型安全 |
| **配置文件** | config | 0.14+ | 支持多格式，环境变量 |
| **序列化** | serde + serde_json | 1.0 | 标准方案 |
| **错误处理** | anyhow + thiserror | 1.0 | 灵活且类型安全 |
| **日志** | tracing | 0.1 | 结构化日志，性能好 |
| **加密** | hmac + sha2 | 0.12, 0.10 | 讯飞云签名 |
| **编码** | base64 | 0.21 | 音频数据编码 |

### 5.2 Whisper 模型管理

**模型来源**：
- 官方预训练模型（GGML 格式）
- 下载地址：https://huggingface.co/ggerganov/whisper.cpp/tree/main

**模型选择策略**：
```rust
pub enum WhisperModel {
    Tiny,    // 75MB,  <1s,  低准确性
    Base,    // 142MB, 1-2s, 平衡（默认）
    Small,   // 466MB, 2-4s, 高准确性
    Medium,  // 1.5GB, 5-8s, 更高准确性
}

impl WhisperModel {
    pub fn path(&self, base_dir: &Path) -> PathBuf {
        match self {
            Self::Tiny => base_dir.join("ggml-tiny.bin"),
            Self::Base => base_dir.join("ggml-base.bin"),
            // ...
        }
    }

    pub fn download_url(&self) -> &'static str {
        // Hugging Face 镜像
    }
}
```

**模型加载优化**：
```rust
// 使用 once_cell 全局单例，避免重复加载
static WHISPER_CONTEXT: OnceCell<Arc<WhisperContext>> = OnceCell::new();

pub fn get_whisper_context(model_path: &Path) -> Result<Arc<WhisperContext>> {
    WHISPER_CONTEXT
        .get_or_try_init(|| {
            let ctx = WhisperContext::new(model_path)?;
            Ok(Arc::new(ctx))
        })
        .map(Arc::clone)
}
```

### 5.3 异步与同步的边界

**原则**：
- I/O 密集操作：异步（网络、文件）
- CPU 密集操作：`spawn_blocking`（Whisper 推理）
- 音频录制：同步（硬件 I/O，但用 async channel 桥接）

**示例**：
```rust
// ❌ 错误：阻塞异步运行时
pub async fn recognize(audio: Vec<f32>) -> Result<String> {
    let ctx = WhisperContext::new("model.bin")?;
    ctx.full(params, &audio)?;  // 这会阻塞整个运行时！
    // ...
}

// ✅ 正确：放到线程池
pub async fn recognize(audio: Vec<f32>) -> Result<String> {
    let ctx = get_whisper_context()?;

    tokio::task::spawn_blocking(move || {
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("zh"));
        ctx.full(params, &audio)?;

        let mut result = String::new();
        for i in 0..ctx.full_n_segments()? {
            result.push_str(&ctx.full_get_segment_text(i)?);
        }
        Ok(result)
    })
    .await?
}
```

### 5.4 错误处理策略

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoiceInputError {
    #[error("音频设备错误: {0}")]
    AudioDevice(String),

    #[error("未找到麦克风设备")]
    NoMicrophone,

    #[error("Whisper 模型加载失败: {0}")]
    ModelLoad(String),

    #[error("Whisper 识别失败: {0}")]
    Recognition(String),

    #[error("WebSocket 连接失败: {0}")]
    WebSocket(String),

    #[error("API 认证失败: {0}")]
    Authentication(String),

    #[error("剪贴板操作失败: {0}")]
    Clipboard(String),

    #[error("配置错误: {0}")]
    Config(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

// 用户友好的错误提示
impl VoiceInputError {
    pub fn user_message(&self) -> String {
        match self {
            Self::NoMicrophone => {
                "未检测到麦克风设备。请检查：\n\
                 1. 麦克风是否已连接\n\
                 2. 是否授予了录音权限\n\
                 3. 运行 'arecord -l' 查看可用设备".to_string()
            }
            Self::ModelLoad(msg) => {
                format!("Whisper 模型加载失败: {}\n\
                        请运行 'voice-input download-model' 下载模型", msg)
            }
            _ => self.to_string(),
        }
    }
}
```

### 5.5 配置环境变量优先级

```rust
impl Config {
    pub fn load() -> Result<Self> {
        // 1. 加载配置文件
        let mut config = Self::from_file_or_default()?;

        // 2. 环境变量覆盖
        if let Ok(app_id) = env::var("XFYUN_APP_ID") {
            config.xfyun.app_id = app_id;
        }

        // 3. CLI 参数覆盖（在 main.rs 中处理）

        Ok(config)
    }
}
```

---

## 6. 实施计划

### 6.1 分阶段实施（3 周 MVP）

#### Week 1: 离线方案核心功能

**Day 1-2: 项目搭建和音频录制**
- [ ] 初始化 Cargo 项目
- [ ] 配置依赖和 Cargo.toml
- [ ] 实现音频录制模块（`audio/recorder.rs`）
- [ ] 实现静音检测（`audio/silence.rs`）
- [ ] 单元测试：音频录制和静音检测

**Day 3-4: Whisper 集成**
- [ ] 下载和测试 Whisper base 模型
- [ ] 实现 Whisper 引擎封装（`whisper/engine.rs`）
- [ ] 实现模型加载和管理（`whisper/model.rs`）
- [ ] 实现识别逻辑（`recognizer/local.rs`）
- [ ] 端到端测试：录音 → 识别

**Day 5: 输出和 CLI**
- [ ] 实现剪贴板输出（`output/clipboard.rs`）
- [ ] 实现 CLI 参数解析（`main.rs`）
- [ ] 集成测试：完整离线流程
- [ ] 修复 bug 和优化

#### Week 2: 在线方案

**Day 6-7: 讯飞云认证和 WebSocket**
- [ ] 实现讯飞云认证签名（`online/xfyun/auth.rs`）
- [ ] 实现 WebSocket 客户端（`online/xfyun/client.rs`）
- [ ] 测试连接和断线重连

**Day 8-9: 协议处理和流式识别**
- [ ] 实现讯飞云协议编解码（`online/xfyun/protocol.rs`）
- [ ] 实现音频流传输
- [ ] 实现结果接收和累积
- [ ] 端到端测试：在线识别

**Day 10: 配置管理**
- [ ] 实现配置加载器（`config/loader.rs`）
- [ ] 支持环境变量
- [ ] 创建配置文件模板
- [ ] 配置验证和错误提示

#### Week 3: 完善和发布

**Day 11-12: 测试和优化**
- [ ] 编写单元测试（目标：核心模块 70% 覆盖率）
- [ ] 编写集成测试
- [ ] 性能测试和优化（内存、启动速度）
- [ ] 错误处理完善

**Day 13-14: 文档和工具**
- [ ] 编写 README.md
- [ ] 编写 INSTALL.md
- [ ] 编写 USAGE.md
- [ ] 创建安装脚本（`scripts/install.sh`）
- [ ] 创建模型下载脚本（`scripts/download_models.sh`）

**Day 15: CI/CD 和发布**
- [ ] 配置 GitHub Actions（编译、测试、发布）
- [ ] 配置 release profile 优化
- [ ] 创建第一个 Release（v0.1.0）
- [ ] 更新文档和示例

### 6.2 开发环境准备

**系统要求**：
- Rust 1.75+ (stable)
- ALSA 或 PulseAudio 开发库
- 测试用麦克风

**安装依赖（Ubuntu/Debian）**：
```bash
sudo apt install build-essential pkg-config libasound2-dev
```

**安装依赖（Fedora）**：
```bash
sudo dnf install alsa-lib-devel
```

**下载 Whisper 模型**：
```bash
mkdir -p ~/.local/share/voice-input/models
cd ~/.local/share/voice-input/models
wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
```

### 6.3 持续集成配置

**GitHub Actions 工作流**：
```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: sudo apt install -y libasound2-dev
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check

  build:
    strategy:
      matrix:
        os: [ubuntu-latest]
        arch: [x86_64]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: sudo apt install -y libasound2-dev
      - run: cargo build --release
      - run: strip target/release/voice-input
      - uses: actions/upload-artifact@v3
        with:
          name: voice-input-${{ matrix.arch }}
          path: target/release/voice-input
```

---

## 7. 性能指标

### 7.1 性能目标

| 指标 | Python 版本 | Rust 目标 | 测量方法 |
|------|-------------|-----------|----------|
| **内存占用（离线）** | 900MB | <250MB | `/usr/bin/time -v` |
| **内存占用（在线）** | 200MB | <100MB | `/usr/bin/time -v` |
| **冷启动时间** | 4-5 秒 | <1 秒 | `time ./voice-input ...` |
| **识别延迟（10s 音频）** | 3-5 秒 | 2-3 秒 | 手动计时 |
| **二进制大小** | - | <10MB | `ls -lh` + `strip` |

### 7.2 优化技巧

**Cargo.toml 优化**：
```toml
[profile.release]
lto = "fat"              # 链接时优化
codegen-units = 1        # 单编译单元，更激进优化
opt-level = 3            # 最高优化级别
strip = true             # 去除符号表
panic = "abort"          # 减小二进制体积
```

**进一步压缩**（可选）：
```bash
# 使用 upx 压缩（可能影响启动速度）
upx --best --lzma target/release/voice-input
```

**Whisper 模型优化**：
- 使用 mmap 加载模型（`whisper-rs` 默认支持）
- 考虑模型量化（如 Q5_0 量化版本）

### 7.3 基准测试

创建 `benches/recognition.rs`：
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use voice_input::whisper::WhisperEngine;

fn benchmark_recognition(c: &mut Criterion) {
    let engine = WhisperEngine::new("models/ggml-base.bin").unwrap();
    let audio = generate_test_audio(10.0); // 10 秒测试音频

    c.bench_function("whisper_10s", |b| {
        b.iter(|| {
            engine.recognize(black_box(&audio))
        });
    });
}

criterion_group!(benches, benchmark_recognition);
criterion_main!(benches);
```

---

## 8. 风险与挑战

### 8.1 技术风险

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| **whisper-rs 兼容性问题** | 高 | 早期验证，准备降级或直接调用 whisper.cpp |
| **音频设备兼容性** | 中 | 使用 cpal（已测试多平台），提供详细错误信息 |
| **Wayland 剪贴板问题** | 中 | 使用 arboard（原生支持），或提供手动 fallback |
| **WebSocket 稳定性** | 中 | 实现断线重连，添加超时和重试机制 |
| **模型下载失败** | 低 | 提供国内镜像，支持手动下载 |

### 8.2 性能风险

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| **内存目标未达成** | 中 | 性能分析（valgrind, heaptrack），优化模型加载 |
| **启动速度未达标** | 中 | 模型懒加载，守护进程模式 |
| **识别延迟过高** | 低 | 优化音频处理流程，考虑 GPU 加速 |

### 8.3 用户体验风险

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| **配置复杂** | 中 | 提供合理默认值，一键安装脚本 |
| **错误提示不友好** | 中 | 统一错误处理，提供修复建议 |
| **快捷键冲突** | 低 | 允许自定义，提供配置向导 |

### 8.4 维护风险

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| **依赖库更新** | 中 | 定期更新，CI 自动化测试 |
| **模型更新** | 低 | 支持多版本，提供升级工具 |
| **API 变更** | 中 | 抽象接口，版本兼容处理 |

---

## 9. 下一步行动

### 9.1 立即行动（Day 1）

1. **验证环境**：
   ```bash
   # 安装 Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # 安装系统依赖
   sudo apt install libasound2-dev

   # 测试麦克风
   arecord -l
   ```

2. **下载 Whisper 模型**：
   ```bash
   mkdir -p ~/.local/share/voice-input/models
   wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin \
     -O ~/.local/share/voice-input/models/ggml-base.bin
   ```

3. **创建项目结构**：
   ```bash
   # 已有基础项目，补充目录
   mkdir -p src/{config,audio,recognizer,whisper,online/xfyun,output,utils}
   mkdir -p scripts docs tests benches
   ```

4. **更新 Cargo.toml**（添加依赖）

5. **实现第一个功能**：音频录制和静音检测

### 9.2 验收标准（MVP）

**Week 1 End**：
```bash
# 离线识别可用
./voice-input --mode local
# → 录音 → 识别 → 剪贴板 ✓
```

**Week 2 End**：
```bash
# 在线识别可用
./voice-input --mode xfyun
# → 连接 → 流式识别 → 剪贴板 ✓
```

**Week 3 End**：
```bash
# 发布 v0.1.0
# - 完整功能 ✓
# - 文档齐全 ✓
# - CI 通过 ✓
# - GitHub Release ✓
```

---

## 附录

### A. 参考资料

**Rust 学习**：
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Async Rust Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

**关键依赖文档**：
- [whisper-rs](https://github.com/tazz4843/whisper-rs)
- [whisper.cpp](https://github.com/ggerganov/whisper.cpp)
- [cpal](https://docs.rs/cpal/)
- [arboard](https://docs.rs/arboard/)
- [clap](https://docs.rs/clap/)

**API 文档**：
- [讯飞语音听写 API](https://www.xfyun.cn/doc/asr/voicedictation/API.html)

### B. 常见问题（预期）

**Q: 为什么选择 Whisper 而不是其他离线模型？**
A: Whisper 准确性高，开源免费，有成熟的 Rust 绑定，社区活跃。

**Q: 是否支持 GPU 加速？**
A: MVP 不支持，v2.0 考虑 CUDA 加速（需要 NVIDIA 显卡）。

**Q: 是否支持 Windows/macOS？**
A: 理论上支持（cpal 和 arboard 都是跨平台的），但未经测试，v1.1 考虑。

**Q: 如何切换离线和在线模式？**
A: 使用 `--mode` 参数或配置文件中的 `default_mode`。

---

**文档维护**：
- 随着开发进展更新本文档
- 重大变更需记录在 CHANGELOG.md
- 技术决策变更需更新"关键技术决策"章节

**最后更新**: 2025-12-28
**文档版本**: v1.0
**状态**: 待审核和实施
