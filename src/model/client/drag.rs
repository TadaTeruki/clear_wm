use super::{geometry::GeometryControl, Client};

#[derive(Debug, Clone, Copy)]
pub struct DragDetail<WinId>
where
    WinId: Copy + Eq,
{
    client: Client<WinId>,
    geometry_control: GeometryControl,
    last_root_position: (i32, i32),
}

impl<WinId> DragDetail<WinId>
where
    WinId: Copy + Eq,
{
    pub fn last_root_position(&self) -> (i32, i32) {
        self.last_root_position
    }

    pub fn geometry_control(&self) -> GeometryControl {
        self.geometry_control
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
    pub fn new_as_dragging(
        client: Client<WinId>,
        geometry_control: GeometryControl,
        last_root_position: (i32, i32),
    ) -> Self {
        DragState::Dragging(DragDetail {
            client,
            geometry_control,
            last_root_position,
        })
    }

    pub fn change_root_position(&mut self, new_root_position: (i32, i32)) {
        if let DragState::Dragging(drag_detail) = self {
            *self = DragState::Dragging(DragDetail {
                client: drag_detail.client,
                geometry_control: drag_detail.geometry_control,
                last_root_position: new_root_position,
            });
        }
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
