pub mod container;
pub mod geometry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Client<WinId>
where
    WinId: Copy + Eq,
{
    pub app_id: WinId,
    pub frame_id: WinId,
}
