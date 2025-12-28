#!/bin/bash
set -e

echo "🗑️  移除 GNOME 快捷键"
echo "===================="
echo ""

BINDING_PATH="/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/voice-input/"

# 获取当前自定义快捷键列表
CUSTOM_KEYBINDINGS=$(gsettings get org.gnome.settings-daemon.plugins.media-keys custom-keybindings)

# 检查是否存在
if [[ "$CUSTOM_KEYBINDINGS" != *"$BINDING_PATH"* ]]; then
    echo "⚠️  快捷键不存在，无需移除"
    exit 0
fi

# 从列表中移除
NEW_LIST=$(echo "$CUSTOM_KEYBINDINGS" | sed "s|, '$BINDING_PATH'||g" | sed "s|'$BINDING_PATH', ||g" | sed "s|'$BINDING_PATH'|@as []|g")
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "$NEW_LIST"

# 清除快捷键详情
gsettings reset org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:$BINDING_PATH name 2>/dev/null || true
gsettings reset org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:$BINDING_PATH command 2>/dev/null || true
gsettings reset org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:$BINDING_PATH binding 2>/dev/null || true

echo "✅ 快捷键已移除"
