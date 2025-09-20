use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VisualBellMode {
    Overlay,
    Inverse,
    Border,
}

impl Default for VisualBellMode {
    fn default() -> Self {
        VisualBellMode::Overlay
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bell {
    #[serde(default = "default_visual_bell")]
    pub visual: bool,
    #[serde(default = "default_audio_bell")]
    pub audio: bool,
    #[serde(default = "default_visual_bell_opacity")]
    pub visual_bell_opacity: f32,
    #[serde(default = "default_visual_bell_color")]
    pub visual_bell_color: [f32; 3],
    #[serde(default = "VisualBellMode::default")]
    pub visual_bell_mode: VisualBellMode,
    #[serde(default = "default_visual_bell_duration")]
    pub visual_bell_duration: u64,
}

impl Default for Bell {
    fn default() -> Self {
        Bell {
            visual: default_visual_bell(),
            audio: default_audio_bell(),
            visual_bell_opacity: default_visual_bell_opacity(),
            visual_bell_color: default_visual_bell_color(),
            visual_bell_mode: VisualBellMode::default(),
            visual_bell_duration: default_visual_bell_duration(),
        }
    }
}

fn default_visual_bell() -> bool {
    false
}

fn default_audio_bell() -> bool {
    // Enable audio bell by default on macOS and Windows since they use the system sound
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        true
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

fn default_visual_bell_opacity() -> f32 {
    0.15
}

fn default_visual_bell_color() -> [f32; 3] {
    [1.0, 1.0, 1.0] // White by default
}

fn default_visual_bell_duration() -> u64 {
    125 // 125 milliseconds by default
}
