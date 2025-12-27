use std::time::{Duration, Instant};

/// 静音检测器
pub struct SilenceDetector {
    /// 静音阈值（0.0-1.0）
    threshold: f32,
    /// 需要持续多久才认为是静音
    duration: Duration,
    /// 上次检测到声音的时间
    last_sound_time: Option<Instant>,
}

impl SilenceDetector {
    /// 创建新的静音检测器
    pub fn new(threshold: f32, duration: Duration) -> Self {
        Self {
            threshold,
            duration,
            last_sound_time: None,
        }
    }

    /// 检测音频块是否为静音
    /// 返回 true 表示已经持续静音超过设定时长
    pub fn detect(&mut self, samples: &[f32]) -> bool {
        let volume = Self::calculate_rms(samples);

        if volume > self.threshold {
            // 检测到声音，更新时间
            self.last_sound_time = Some(Instant::now());
            false
        } else {
            // 当前是静音
            match self.last_sound_time {
                Some(last_time) => {
                    // 检查静音持续时间
                    last_time.elapsed() >= self.duration
                }
                None => {
                    // 从未检测到声音（刚开始录音）
                    false
                }
            }
        }
    }

    /// 重置检测器状态
    pub fn reset(&mut self) {
        self.last_sound_time = None;
    }

    /// 计算音频块的 RMS（均方根）音量
    fn calculate_rms(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }

        let sum_squares: f32 = samples.iter().map(|&s| s * s).sum();
        (sum_squares / samples.len() as f32).sqrt()
    }

    /// 获取音量百分比（0-100）
    pub fn get_volume_percentage(samples: &[f32]) -> u8 {
        let rms = Self::calculate_rms(samples);
        (rms * 100.0).min(100.0) as u8
    }

    /// 显示音量条
    pub fn print_volume_bar(samples: &[f32]) {
        let percentage = Self::get_volume_percentage(samples);
        let bar_length = 50;
        let filled = (percentage as usize * bar_length) / 100;
        let empty = bar_length - filled;

        print!("\r[{}{}] {:3}%",
            "█".repeat(filled),
            "░".repeat(empty),
            percentage
        );
        use std::io::Write;
        std::io::stdout().flush().ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rms_calculation() {
        let samples = vec![0.0, 0.5, -0.5, 0.0];
        let rms = SilenceDetector::calculate_rms(&samples);
        assert!(rms > 0.0 && rms < 1.0);
    }

    #[test]
    fn test_silence_detection() {
        let mut detector = SilenceDetector::new(0.1, Duration::from_millis(100));

        // 静音样本
        let silent_samples = vec![0.01; 1000];
        detector.detect(&silent_samples); // 第一次不会返回 true

        std::thread::sleep(Duration::from_millis(150));
        assert!(detector.detect(&silent_samples)); // 超时后应返回 true
    }

    #[test]
    fn test_sound_detection() {
        let mut detector = SilenceDetector::new(0.1, Duration::from_millis(100));

        // 有声音的样本
        let loud_samples = vec![0.5; 1000];
        assert!(!detector.detect(&loud_samples)); // 有声音，不应判定为静音
    }

    #[test]
    fn test_volume_percentage() {
        let samples = vec![0.5; 1000];
        let percentage = SilenceDetector::get_volume_percentage(&samples);
        assert!(percentage > 0 && percentage <= 100);
    }
}
