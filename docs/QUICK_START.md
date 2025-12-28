# 快速开始指南

本指南帮助你在 5 分钟内配置好语音输入工具。

## 📋 前置条件

- Linux 系统（推荐 Debian/Ubuntu）
- GNOME 桌面环境
- 已安装 Rust（如未安装，见下方）

## 🚀 一键安装

### 1. 安装 Rust（如果还没有）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. 安装系统依赖

```bash
sudo apt install xclip libssl-dev libasound2-dev build-essential
```

### 3. 编译项目

```bash
cd linux-voice-input-rs
cargo build --release
```

### 4. 运行安装脚本

```bash
./scripts/install.sh
```

### 5. 配置 API 密钥

```bash
# 编辑配置文件
nano ~/.config/voice-input/config.toml

# 或使用 VS Code
code ~/.config/voice-input/config.toml
```

填入你的讯飞云 API 密钥：

```toml
[xfyun]
app_id = "你的APP_ID"
api_secret = "你的API_SECRET"
api_key = "你的API_KEY"
```

> 💡 到 https://console.xfyun.cn/ 注册并获取 API 密钥

### 6. 配置快捷键

```bash
./scripts/configure-shortcut.sh
```

## ✅ 完成！

现在按 **Super + Shift + B**（Super 键即 Windows 键）就可以开始语音输入了！

## 🎯 使用流程

1. **按快捷键** Super + Shift + B
2. **看到麦克风图标亮起**（系统右上角）
3. **说话**："你好，这是一个测试"
4. **停止说话，等待 3 秒**
5. **自动停止**，文字已复制到剪贴板
6. **粘贴** Ctrl + V

## ⚙️ 调整静音时长

如果觉得 3 秒太长或太短：

```bash
nano ~/.config/voice-input/config.toml
```

修改这一行：

```toml
silence_duration = 1.5  # 改成 1.5 秒，更快响应
# 或
silence_duration = 5.0  # 改成 5 秒，允许更长停顿
```

## 🐛 遇到问题？

### 快捷键不工作

1. 手动运行测试：
   ```bash
   ~/.local/bin/voice-input
   ```

2. 查看日志：
   ```bash
   tail -f ~/.config/voice-input/voice-input.log
   ```

### 无法识别

1. 检查 API 密钥是否正确
2. 检查网络连接
3. 确认麦克风工作正常：
   ```bash
   arecord -d 3 test.wav && aplay test.wav
   ```

## 📚 更多帮助

- 完整文档：[README.md](../README.md)
- 卸载：`./scripts/uninstall.sh`
- 删除快捷键：`./scripts/remove-shortcut.sh`
