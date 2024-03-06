#[derive(Debug, Clone, Copy)]
pub enum ClientGeometry {
    App(i32, i32, u32, u32),
    Frame(i32, i32, u32, u32),
}

impl ClientGeometry {
    pub fn to_frame(self, border_width: u32, titlebar_height: u32) -> ClientGeometry {
        match self {
            ClientGeometry::App(x, y, width, height) => {
                ClientGeometry::Frame(
                    x - border_width as i32,
                    y - (border_width + titlebar_height) as i32,
                    width + 2 * border_width,
                    height + 2 * (border_width + titlebar_height),
                )
            }
            ClientGeometry::Frame(_, _, _, _) => self,
        }
    }

    pub fn to_app(self, border_width: u32, titlebar_height: u32) -> ClientGeometry {
        match self {
            ClientGeometry::Frame(x, y, width, height) => {
                ClientGeometry::App(
                    x + border_width as i32,
                    y + (border_width + titlebar_height) as i32,
                    width - 2 * border_width,
                    height - 2 * (border_width + titlebar_height),
                )
            }
            ClientGeometry::App(_, _, _, _) => self,
        }
    }

    pub fn to_tuple(self) -> (i32, i32, u32, u32) {
        match self {
            ClientGeometry::App(x, y, width, height) => (x, y, width, height),
            ClientGeometry::Frame(x, y, width, height) => (x, y, width, height),
        }
    }
}