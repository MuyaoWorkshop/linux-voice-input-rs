use std::process::{Command, Stdio};
use std::io::Write;

use crate::utils::{Result, VoiceInputError};

/// 剪贴板输出
pub struct ClipboardOutput;

impl ClipboardOutput {
    /// 创建新的剪贴板输出
    pub fn new() -> Result<Self> {
        // 检查 xclip 是否安装
        Command::new("xclip")
            .arg("-version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|_| VoiceInputError::Clipboard(
                "未找到 xclip 命令，请安装: sudo apt install xclip".to_string()
            ))?;

        Ok(Self)
    }

    /// 复制文本到剪贴板（使用 xclip 命令）
    pub fn copy(&mut self, text: &str) -> Result<()> {
        let mut child = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| VoiceInputError::Clipboard(format!("启动 xclip 失败: {}", e)))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())
                .map_err(|e| VoiceInputError::Clipboard(format!("写入到 xclip 失败: {}", e)))?;
        }

        let status = child.wait()
            .map_err(|e| VoiceInputError::Clipboard(format!("等待 xclip 完成失败: {}", e)))?;

        if !status.success() {
            return Err(VoiceInputError::Clipboard("xclip 执行失败".to_string()));
        }

        tracing::info!("已复制 {} 个字符到剪贴板", text.chars().count());

        Ok(())
    }

    /// 获取剪贴板内容（用于测试）
    #[allow(dead_code)]
    pub fn get(&mut self) -> Result<String> {
        let output = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .arg("-o")
            .output()
            .map_err(|e| VoiceInputError::Clipboard(format!("读取剪贴板失败: {}", e)))?;

        if !output.status.success() {
            return Err(VoiceInputError::Clipboard("读取剪贴板失败".to_string()));
        }

        String::from_utf8(output.stdout)
            .map_err(|e| VoiceInputError::Clipboard(format!("剪贴板内容不是有效的 UTF-8: {}", e)))
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
