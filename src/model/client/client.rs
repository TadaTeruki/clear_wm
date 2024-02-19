#[derive(Debug, PartialEq)]
pub struct Client<WinId>
where
    WinId: Copy + PartialEq,
{
    pub app_id: WinId,
    pub frame_id: WinId,
}

impl<WinId> Client<WinId>
where
    WinId: Copy + PartialEq,
{
    pub fn new(app_id: WinId, frame_id: WinId) -> Self {
        Self { app_id, frame_id }
    }
}
