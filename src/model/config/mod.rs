#[derive(Debug, Clone)]
pub struct WindowManagerConfig {
    pub border_width: u16,
    pub titlebar_height: u16,
}

impl Default for WindowManagerConfig {
    fn default() -> Self {
        Self {
            border_width: 4,
            titlebar_height: 20,
        }
    }
}