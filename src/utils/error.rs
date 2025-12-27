use thiserror::Error;

/// 语音输入工具的错误类型
#[derive(Error, Debug)]
pub enum VoiceInputError {
    #[error("音频设备错误: {0}")]
    AudioDevice(String),

    #[error("未找到麦克风设备")]
    NoMicrophone,

    #[error("音频录制失败: {0}")]
    AudioRecord(String),

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

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl VoiceInputError {
    /// 获取用户友好的错误提示信息
    pub fn user_message(&self) -> String {
        match self {
            Self::NoMicrophone => {
                "未检测到麦克风设备。请检查：\n\
                 1. 麦克风是否已连接\n\
                 2. 是否授予了录音权限\n\
                 3. 运行 'arecord -l' 查看可用设备"
                    .to_string()
            }
            Self::ModelLoad(msg) => {
                format!(
                    "Whisper 模型加载失败: {}\n\
                    请确保模型文件存在且格式正确。\n\
                    可以运行以下命令下载模型：\n\
                    mkdir -p ~/.local/share/voice-input/models\n\
                    wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin \\\n\
                      -O ~/.local/share/voice-input/models/ggml-base.bin",
                    msg
                )
            }
            Self::Config(msg) => {
                format!(
                    "配置错误: {}\n\
                    请检查配置文件格式是否正确。",
                    msg
                )
            }
            Self::Clipboard(msg) => {
                format!(
                    "剪贴板操作失败: {}\n\
                    请确保系统支持剪贴板操作。",
                    msg
                )
            }
            _ => self.to_string(),
        }
    }
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, VoiceInputError>;
