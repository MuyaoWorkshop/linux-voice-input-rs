# Linux Voice Input - Rust 版本

基于讯飞云 API 的 Linux 桌面语音输入工具，使用 Rust 开发。

## ✨ 特性

- 🎤 **实时流式识别**：边录音边发送，响应快速
- 🔇 **智能静音检测**：说完话自动停止（可配置时长）
- 📋 **自动复制到剪贴板**：识别完成自动复制，随处粘贴
- ⌨️ **快捷键启动**：按键即用，无需打开终端
- 🚀 **高性能**：Rust 编写，启动迅速，资源占用低
- 🎯 **中文优化**：专为中文语音识别优化

## 📋 系统要求

- **操作系统**：Linux (测试于 Debian 13)
- **桌面环境**：GNOME (Wayland)
- **依赖**：
  - `xclip` - 剪贴板操作
  - `libssl-dev` - TLS 支持
  - `libasound2-dev` - 音频录制

## 🚀 快速开始

### 1. 安装依赖

```bash
sudo apt install xclip libssl-dev libasound2-dev build-essential
```

### 2. 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. 克隆并编译

```bash
git clone <repository-url>
cd linux-voice-input-rs
cargo build --release
```

### 4. 安装

```bash
./scripts/install.sh
```

### 5. 配置 API 密钥

编辑 `~/.config/voice-input/config.toml`，填入讯飞云 API 密钥：

```toml
[xfyun]
app_id = "你的_APP_ID"
api_secret = "你的_API_SECRET"
api_key = "你的_API_KEY"
```

> 💡 从 [讯飞开放平台](https://console.xfyun.cn/) 获取 API 密钥

### 6. 配置快捷键

```bash
./scripts/configure-shortcut.sh
```

默认快捷键：**Super + Shift + B**

## 🎯 使用方法

### 方式一：快捷键（推荐）

1. 按 **Super + Shift + B**（Super 键即 Windows 键）
2. 看到右上角麦克风图标亮起
3. 开始说话...
4. 停止说话，等待自动停止（默认 3 秒）
5. 识别结果自动复制到剪贴板
6. 在任何地方按 **Ctrl + V** 粘贴

### 方式二：命令行

```bash
voice-input
```

或在项目目录：

```bash
./target/release/linux-voice-input-rs
```

## ⚙️ 配置说明

配置文件位置：`~/.config/voice-input/config.toml`

### 静音时长配置

```toml
[whisper]
# 说完话后保持静音多久会自动停止录音
# 建议范围：1.5-5.0 秒
silence_duration = 3.0
```

推荐值：
- **1.5 秒**：快速响应，适合短句子
- **2.5 秒**：平衡选择
- **3.0 秒**：默认，允许句子间短暂停顿
- **5.0 秒**：较慢，适合说话时有长停顿

### 讯飞云配置

```toml
[xfyun]
app_id = "你的_APP_ID"
api_secret = "你的_API_SECRET"
api_key = "你的_API_KEY"
```

### 音频配置

```toml
[audio]
sample_rate = 16000  # 采样率
channels = 1         # 声道数
```

## 🔧 常用命令

```bash
# 安装
./scripts/install.sh

# 配置快捷键
./scripts/configure-shortcut.sh

# 卸载
./scripts/uninstall.sh

# 删除快捷键
./scripts/remove-shortcut.sh

# 查看日志
tail -f ~/.config/voice-input/voice-input.log
```

## 🐛 故障排查

### 快捷键不工作

1. 检查是否正确安装：`ls ~/.local/bin/voice-input`
2. 手动运行测试：`~/.local/bin/voice-input-launcher`
3. 查看日志：`tail -f ~/.config/voice-input/voice-input.log`

### 无法录音

1. 检查麦克风权限
2. 测试麦克风：`arecord -d 3 test.wav`
3. 检查音频设备：`arecord -l`

### 识别不准确

1. 检查网络连接
2. 确认 API 密钥正确
3. 调整 `silence_duration` 参数

### 启动慢或卡住

1. 检查网络连接到讯飞云服务器
2. 查看日志中的错误信息
3. 尝试重新编译：`cargo clean && cargo build --release`

## 📁 项目结构

```
linux-voice-input-rs/
├── src/
│   ├── main.rs              # 程序入口
│   ├── audio/               # 音频录制模块
│   │   └── silence.rs       # 静音检测
│   ├── config/              # 配置管理
│   ├── online/              # 在线识别
│   │   └── xfyun_realtime.rs # 讯飞云实时识别
│   ├── output/              # 输出处理
│   │   └── clipboard.rs     # 剪贴板操作
│   └── utils/               # 工具函数
├── config.toml.example      # 配置文件示例
├── install.sh               # 安装脚本
├── uninstall.sh             # 卸载脚本
├── configure-shortcut.sh    # 快捷键配置脚本
└── remove-shortcut.sh       # 快捷键删除脚本
```

## 🔄 从 Python 版本迁移

如果你之前使用 Python 版本：

1. Rust 版本更快，启动更迅速
2. 配置文件格式不同，需要重新配置
3. 快捷键建议使用不同的组合（避免冲突）
4. API 密钥可以继续使用

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

## 🙏 致谢

- [讯飞开放平台](https://www.xfyun.cn/) - 提供语音识别 API
- [cpal](https://github.com/RustAudio/cpal) - 跨平台音频库
- [tokio](https://tokio.rs/) - 异步运行时

## 📝 更新日志

### v0.1.0 (2025-12-28)

- ✨ 初始版本
- 🎤 实时流式语音识别
- 🔇 智能静音检测（使用讯飞云 VAD）
- 📋 自动复制到剪贴板
- ⌨️ GNOME 快捷键支持
- 🚀 支持所有音频采样格式

---

**Made with ❤️ using Rust**
