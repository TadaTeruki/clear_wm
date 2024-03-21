use crate::model::config::FrameConfig;

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
    frame_config: FrameConfig,
}

pub enum ClientResizeVertical {
    Top,
    Bottom,
    None,
}

pub enum ClientResizeHorizontal {
    Left,
    Right,
    None,
}

pub enum ClientControl {
    Move,
    Resize(ClientResizeVertical, ClientResizeHorizontal),
}

impl ClientGeometry {
    pub fn from_app(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        frame_config: FrameConfig,
    ) -> ClientGeometry {
        ClientGeometry {
            geometry: Geometry {
                x,
                y,
                width,
                height,
            },
            frame_config,
        }
    }

    #[allow(dead_code)]
    pub fn from_frame(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        frame_config: FrameConfig,
    ) -> ClientGeometry {
        ClientGeometry {
            geometry: Geometry {
                x: x + frame_config.border_width as i32,
                y: y + (frame_config.border_width + frame_config.titlebar_height) as i32,
                width: width - 2 * frame_config.border_width,
                height: height - (2 * frame_config.border_width + frame_config.titlebar_height),
            },
            frame_config,
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
            x: self.geometry.x - self.frame_config.border_width as i32,
            y: self.geometry.y
                - (self.frame_config.border_width + self.frame_config.titlebar_height) as i32,
            width: self.geometry.width + 2 * self.frame_config.border_width,
            height: self.geometry.height
                + (2 * self.frame_config.border_width + self.frame_config.titlebar_height),
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
            frame_config: self.frame_config,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_geometry() {
        let frame_config = FrameConfig {
            border_width: 4,
            titlebar_height: 20,
            corner_radius: 6,
        };
        let client_geom = ClientGeometry::from_app(0, 0, 100, 100, frame_config);

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
            ClientGeometry::from_app(10, 10, 100, 100, frame_config)
        );

        let client_geom = ClientGeometry::from_frame(0, 0, 100, 100, frame_config);

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
            ClientGeometry::from_frame(10, 10, 100, 100, frame_config)
        );
    }
}
