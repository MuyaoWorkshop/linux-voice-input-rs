use arboard::Clipboard;

use crate::utils::{Result, VoiceInputError};

/// 剪贴板输出
pub struct ClipboardOutput {
    clipboard: Clipboard,
}

impl ClipboardOutput {
    /// 创建新的剪贴板输出
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new()
            .map_err(|e| VoiceInputError::Clipboard(format!("初始化剪贴板失败: {}", e)))?;

        Ok(Self { clipboard })
    }

    /// 复制文本到剪贴板
    pub fn copy(&mut self, text: &str) -> Result<()> {
        self.clipboard
            .set_text(text)
            .map_err(|e| VoiceInputError::Clipboard(format!("复制到剪贴板失败: {}", e)))?;

        tracing::info!("已复制 {} 个字符到剪贴板", text.chars().count());

        Ok(())
    }

    /// 获取剪贴板内容（用于测试）
    #[allow(dead_code)]
    pub fn get(&mut self) -> Result<String> {
        self.clipboard
            .get_text()
            .map_err(|e| VoiceInputError::Clipboard(format!("读取剪贴板失败: {}", e)))
    }
}

impl Default for ClipboardOutput {
    fn default() -> Self {
        Self::new().expect("无法初始化剪贴板")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_basic() {
        // 这个测试可能在无显示服务器的环境（如 CI）中失败
        if std::env::var("CI").is_err() {
            let mut clipboard = ClipboardOutput::new();
            if let Ok(mut clip) = clipboard {
                let test_text = "测试文本";
                assert!(clip.copy(test_text).is_ok());

                if let Ok(content) = clip.get() {
                    assert_eq!(content, test_text);
                }
            }
        }
    }
}
