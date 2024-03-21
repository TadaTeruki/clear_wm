use super::Client;

#[derive(Debug, Clone, Copy)]
pub struct DragDetail<WinId>
where
    WinId: Copy + Eq,
{
    client: Client<WinId>,
    last_root_position: (i32, i32),
}

impl<WinId> DragDetail<WinId>
where
    WinId: Copy + Eq,
{
    pub fn client(&self) -> Client<WinId> {
        self.client
    }

    pub fn last_root_position(&self) -> (i32, i32) {
        self.last_root_position
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DragState<WinId>
where
    WinId: Copy + Eq,
{
    Dragging(DragDetail<WinId>),
    Dragged(DragDetail<WinId>),
    None,
}

impl<WinId> DragState<WinId>
where
    WinId: Copy + Eq,
{
    pub fn new_as_dragging(client: Client<WinId>, last_root_position: (i32, i32)) -> Self {
        DragState::Dragging(DragDetail {
            client,
            last_root_position,
        })
    }

    pub fn release_from_dragging(&mut self) {
        if let DragState::Dragging(drag_detail) = self {
            *self = DragState::Dragged(*drag_detail);
        }
    }

    pub fn parse_with_check_dragging(&self, client: Client<WinId>) -> Option<DragDetail<WinId>> {
        if let DragState::Dragging(drag_state) = self {
            if drag_state.client == client {
                return Some(*drag_state);
            }
        }
        None
    }
}
