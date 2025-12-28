#!/bin/bash
set -e

echo "🗑️  Linux Voice Input - 卸载脚本"
echo "================================"
echo ""

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="voice-input"
LAUNCHER_SCRIPT="voice-input-launcher"
CONFIG_DIR="$HOME/.config/voice-input"

echo "将要删除以下内容："
echo "  - 二进制文件: $INSTALL_DIR/$BINARY_NAME"
echo "  - 启动脚本: $INSTALL_DIR/$LAUNCHER_SCRIPT"
echo "  - 配置目录: $CONFIG_DIR (包含配置文件和日志)"
echo ""
read -p "确认卸载? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "已取消"
    exit 0
fi

# 删除二进制文件
if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    rm "$INSTALL_DIR/$BINARY_NAME"
    echo "✓ 已删除二进制文件"
else
    echo "⚠️  二进制文件不存在，跳过"
fi

# 删除启动脚本
if [ -f "$INSTALL_DIR/$LAUNCHER_SCRIPT" ]; then
    rm "$INSTALL_DIR/$LAUNCHER_SCRIPT"
    echo "✓ 已删除启动脚本"
else
    echo "⚠️  启动脚本不存在，跳过"
fi

# 删除配置目录（可选）
if [ -d "$CONFIG_DIR" ]; then
    echo ""
    read -p "是否删除配置目录? (包含配置文件和日志) (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$CONFIG_DIR"
        echo "✓ 已删除配置目录"
    else
        echo "⚠️  保留配置目录: $CONFIG_DIR"
    fi
else
    echo "⚠️  配置目录不存在，跳过"
fi

echo ""
echo "✅ 卸载完成！"
echo ""
echo "注意："
echo "  - 快捷键配置未删除"
echo "  - 如需删除快捷键，请运行: ./scripts/remove-shortcut.sh"
echo ""
