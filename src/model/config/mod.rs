#[derive(Debug, Clone)]
pub struct WindowManagerConfig {
    pub frame_config: FrameConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameConfig {
    pub border_width: u32,
    pub titlebar_height: u32,
    pub corner_radius: u32,
}

impl Default for WindowManagerConfig {
    fn default() -> Self {
        Self {
            frame_config: FrameConfig {
                border_width: 18,
                titlebar_height: 20,
                corner_radius: 24,
            },
        }
    }
}
