use crate::model::config::WindowManagerConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientGeometry {
    geometry: Geometry,
    border_width: u32,
    titlebar_height: u32,
}

impl ClientGeometry {
    pub fn from_app(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        titlebar_height: u32,
    ) -> ClientGeometry {
        ClientGeometry {
            geometry: Geometry {
                x,
                y,
                width,
                height,
            },
            border_width,
            titlebar_height,
        }
    }

    pub fn from_frame(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        titlebar_height: u32,
    ) -> ClientGeometry {
        ClientGeometry {
            geometry: Geometry {
                x: x + border_width as i32,
                y: y + (border_width + titlebar_height) as i32,
                width: width - 2 * border_width,
                height: height - (2 * border_width + titlebar_height),
            },
            border_width,
            titlebar_height,
        }
    }

    pub fn parse_as_app(&self) -> Geometry {
        Geometry {
            x: self.geometry.x,
            y: self.geometry.y,
            width: self.geometry.width,
            height: self.geometry.height,
        }
    }

    pub fn parse_as_frame(&self) -> Geometry {
        Geometry {
            x: self.geometry.x - self.border_width as i32,
            y: self.geometry.y - (self.border_width + self.titlebar_height) as i32,
            width: self.geometry.width + 2 * self.border_width,
            height: self.geometry.height + (2 * self.border_width + self.titlebar_height),
        }
    }

    pub fn move_relative(&self, x: i32, y: i32) -> ClientGeometry {
        ClientGeometry {
            geometry: Geometry {
                x: self.geometry.x + x,
                y: self.geometry.y + y,
                width: self.geometry.width,
                height: self.geometry.height,
            },
            border_width: self.border_width,
            titlebar_height: self.titlebar_height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_geometry() {
        let client_geom = ClientGeometry::from_app(0, 0, 100, 100, 4, 20);

        assert_eq!(
            client_geom.parse_as_frame(),
            Geometry {
                x: -4,
                y: -24,
                width: 108,
                height: 128
            }
        );

        assert_eq!(
            client_geom.move_relative(10, 10),
            ClientGeometry::from_app(10, 10, 100, 100, 4, 20)
        );

        let client_geom = ClientGeometry::from_frame(0, 0, 100, 100, 4, 20);

        assert_eq!(
            client_geom.parse_as_app(),
            Geometry {
                x: 4,
                y: 24,
                width: 92,
                height: 72
            }
        );

        assert_eq!(
            client_geom.move_relative(10, 10),
            ClientGeometry::from_frame(10, 10, 100, 100, 4, 20)
        );
    }
}
