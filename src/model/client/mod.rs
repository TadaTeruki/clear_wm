pub mod container;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientId<WinId>
where
    WinId: Copy + Eq,
{
    pub app_id: WinId,
    pub frame_id: WinId,
}
