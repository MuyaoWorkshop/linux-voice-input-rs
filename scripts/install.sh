#!/bin/bash
set -e

# 切换到项目根目录
cd "$(dirname "$0")/.."

echo "🚀 Linux Voice Input - 安装脚本"
echo "================================"
echo ""

# 检查是否已编译
if [ ! -f "target/release/linux-voice-input-rs" ]; then
    echo "❌ 错误: 未找到编译后的二进制文件"
    echo "请先运行: cargo build --release"
    exit 1
fi

# 安装目录
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="voice-input"
CONFIG_DIR="$HOME/.config/voice-input"

# 创建安装目录
mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"

# 复制二进制文件
echo "📦 正在安装二进制文件..."
cp target/release/linux-voice-input-rs "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"
echo "   ✓ 已安装到: $INSTALL_DIR/$BINARY_NAME"

# 复制配置文件示例
if [ -f "config.toml.example" ]; then
    if [ ! -f "$CONFIG_DIR/config.toml" ]; then
        echo "📝 正在创建配置文件..."
        cp config.toml.example "$CONFIG_DIR/config.toml"
        echo "   ✓ 配置文件: $CONFIG_DIR/config.toml"
        echo "   ⚠️  请编辑配置文件，填入讯飞云 API 密钥"
    else
        echo "⚠️  配置文件已存在，跳过: $CONFIG_DIR/config.toml"
    fi
fi

# 检查 PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "⚠️  警告: $INSTALL_DIR 不在 PATH 中"
    echo ""
    echo "请将以下内容添加到 ~/.bashrc 或 ~/.zshrc："
    echo ""
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
fi

# 创建启动脚本（用于快捷键）
LAUNCHER_SCRIPT="$INSTALL_DIR/voice-input-launcher"
cat > "$LAUNCHER_SCRIPT" << 'EOF'
#!/bin/bash

# 切换到配置文件目录
cd ~/.config/voice-input

# 运行程序（使用绝对路径，输出到日志）
$HOME/.local/bin/voice-input 2>&1 | tee -a ~/.config/voice-input/voice-input.log

# 如果程序异常退出，显示通知
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    notify-send "语音输入" "程序异常退出，请查看日志" -u critical
fi
EOF

chmod +x "$LAUNCHER_SCRIPT"
echo "✓ 已创建启动脚本: $LAUNCHER_SCRIPT"

echo ""
echo "✅ 安装完成！"
echo ""
echo "下一步："
echo "1. 编辑配置文件: $CONFIG_DIR/config.toml"
echo "2. 填入讯飞云 API 密钥"
echo "3. 测试运行: $BINARY_NAME"
echo "4. 配置快捷键（见下方说明）"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "配置 GNOME 快捷键："
echo ""
echo "方法1 - 图形界面："
echo "  1. 打开 设置 > 键盘 > 查看和自定义快捷键"
echo "  2. 点击底部 '自定义快捷键'"
echo "  3. 点击 '+' 添加新快捷键"
echo "  4. 名称: 语音输入"
echo "  5. 命令: $LAUNCHER_SCRIPT"
echo "  6. 快捷键: 按你想要的组合键（推荐 Super+Shift+B）"
echo ""
echo "方法2 - 命令行："
echo "  运行以下命令自动配置（快捷键: Super+Shift+B）："
echo ""
echo "  ./scripts/configure-shortcut.sh"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
