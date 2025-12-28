#!/bin/bash
set -e

echo "⌨️  配置 GNOME 快捷键"
echo "===================="
echo ""

LAUNCHER_SCRIPT="$HOME/.local/bin/voice-input-launcher"

# 检查启动脚本是否存在
if [ ! -f "$LAUNCHER_SCRIPT" ]; then
    echo "❌ 错误: 未找到启动脚本"
    echo "请先运行: ./scripts/install.sh"
    exit 1
fi

# 获取当前自定义快捷键列表
CUSTOM_KEYBINDINGS=$(gsettings get org.gnome.settings-daemon.plugins.media-keys custom-keybindings)

# 创建新的快捷键路径
NEW_BINDING="/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/voice-input/"

# 检查是否已经存在
if [[ "$CUSTOM_KEYBINDINGS" == *"$NEW_BINDING"* ]]; then
    echo "⚠️  快捷键已存在，将更新配置..."
else
    # 添加到列表
    if [ "$CUSTOM_KEYBINDINGS" == "@as []" ]; then
        # 列表为空
        NEW_LIST="['$NEW_BINDING']"
    else
        # 列表不为空，添加到末尾
        NEW_LIST="${CUSTOM_KEYBINDINGS%]*}, '$NEW_BINDING']"
    fi
    gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "$NEW_LIST"
    echo "✓ 已添加到自定义快捷键列表"
fi

# 配置快捷键详情
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:$NEW_BINDING name '语音输入'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:$NEW_BINDING command "$LAUNCHER_SCRIPT"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:$NEW_BINDING binding '<Super><Shift>b'

echo "✓ 快捷键已配置"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ 配置完成！"
echo ""
echo "快捷键: Super + Shift + B"
echo "（Super 键通常是 Windows 键）"
echo ""
echo "现在可以按快捷键测试了！"
echo ""
echo "如果想修改快捷键："
echo "  - 打开 设置 > 键盘 > 查看和自定义快捷键"
echo "  - 找到 '语音输入'"
echo "  - 点击快捷键，按新的组合键"
echo ""
echo "卸载快捷键："
echo "  ./scripts/remove-shortcut.sh"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
