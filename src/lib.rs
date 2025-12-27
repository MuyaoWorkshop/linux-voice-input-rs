pub mod audio;
pub mod config;
pub mod utils;

// 暂时先注释掉还未实现的模块
// pub mod recognizer;
// pub mod whisper;
// pub mod online;
// pub mod output;

pub use config::Config;
pub use utils::{Result, VoiceInputError};
