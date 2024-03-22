pub mod container;
pub mod drag;
pub mod geometry;
pub mod hints;
pub mod map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Client<WinId>
where
    WinId: Copy + Eq,
{
    pub app_id: WinId,
    pub frame_id: WinId,
}
