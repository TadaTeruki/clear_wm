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
