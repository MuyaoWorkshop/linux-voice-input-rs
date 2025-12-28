pub mod audio;
pub mod config;
pub mod output;
pub mod utils;
pub mod whisper;

// 暂时先注释掉还未实现的模块
// pub mod recognizer;
// pub mod online;

pub use config::Config;
pub use utils::{Result, VoiceInputError};
