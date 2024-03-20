#[derive(Debug, Clone, Copy)]

pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub enum ClientGeometry {
    App(Geometry),
    Frame(Geometry),
}

impl ClientGeometry {
    pub fn from_app(x: i32, y: i32, width: u32, height: u32) -> ClientGeometry {
        ClientGeometry::App(Geometry {
            x,
            y,
            width,
            height,
        })
    }

    pub fn from_frame(x: i32, y: i32, width: u32, height: u32) -> ClientGeometry {
        ClientGeometry::Frame(Geometry {
            x,
            y,
            width,
            height,
        })
    }

    pub fn parse_as_app(&self, border_width: u32, titlebar_height: u32) -> Geometry {
        match self {
            ClientGeometry::App(geom) => *geom,
            ClientGeometry::Frame(geom) => Geometry {
                x: geom.x + border_width as i32,
                y: geom.y + (border_width + titlebar_height) as i32,
                width: geom.width - 2 * border_width,
                height: geom.height - (2 * border_width + titlebar_height),
            },
        }
    }

    pub fn parse_as_frame(&self, border_width: u32, titlebar_height: u32) -> Geometry {
        match self {
            ClientGeometry::Frame(geom) => *geom,
            ClientGeometry::App(geom) => Geometry {
                x: geom.x - border_width as i32,
                y: geom.y - (border_width + titlebar_height) as i32,
                width: geom.width + 2 * border_width,
                height: geom.height + (2 * border_width + titlebar_height),
            },
        }
    }
}
