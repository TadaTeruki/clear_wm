#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn move_relative(&self, x: i32, y: i32) -> ClientGeometry {
        match self {
            ClientGeometry::App(geom) => ClientGeometry::App(Geometry {
                x: geom.x + x,
                y: geom.y + y,
                width: geom.width,
                height: geom.height,
            }),
            ClientGeometry::Frame(geom) => ClientGeometry::Frame(Geometry {
                x: geom.x + x,
                y: geom.y + y,
                width: geom.width,
                height: geom.height,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_geometry() {
        let client_geom = ClientGeometry::from_app(0, 0, 100, 100);

        assert_eq!(
            client_geom.parse_as_frame(4, 20),
            Geometry {
                x: -4,
                y: -24,
                width: 108,
                height: 128
            }
        );

        assert_eq!(
            client_geom.move_relative(10, 10),
            ClientGeometry::from_app(10, 10, 100, 100)
        );

        let client_geom = ClientGeometry::from_frame(0, 0, 100, 100);

        assert_eq!(
            client_geom.parse_as_app(4, 20),
            Geometry {
                x: 4,
                y: 24,
                width: 92,
                height: 72
            }
        );

        assert_eq!(
            client_geom.move_relative(10, 10),
            ClientGeometry::from_frame(10, 10, 100, 100)
        );
    }
}
