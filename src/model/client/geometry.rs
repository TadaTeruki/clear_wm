use crate::config::FrameConfig;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge {
    OnBorder,
    OnCornerRadius,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalResize {
    Top,
    Bottom,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalResize {
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryControl {
    Move,
    Resize(VerticalResize, HorizontalResize),
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

    pub fn move_relative(&self, rel_x: i32, rel_y: i32) -> ClientGeometry {
        ClientGeometry {
            geometry: Geometry {
                x: self.geometry.x + rel_x,
                y: self.geometry.y + rel_y,
                width: self.geometry.width,
                height: self.geometry.height,
            },
            frame_config: self.frame_config,
        }
    }

    pub fn check_control_by_position_on_frame(
        &self,
        x_on_frame: i32,
        y_on_frame: i32,
    ) -> GeometryControl {
        let frame_geom = self.parse_as_frame();

        let vertical_control = {
            let on_border_vertically = y_on_frame < self.frame_config.border_width as i32
                || y_on_frame > frame_geom.height as i32 - self.frame_config.border_width as i32;

            let on_corner_radius_vertically = y_on_frame < self.frame_config.corner_radius as i32
                || y_on_frame > frame_geom.height as i32 - self.frame_config.corner_radius as i32;

            if y_on_frame < (frame_geom.height / 2) as i32 {
                if on_border_vertically {
                    Some((VerticalResize::Top, Edge::OnBorder))
                } else if on_corner_radius_vertically {
                    Some((VerticalResize::Top, Edge::OnCornerRadius))
                } else {
                    None
                }
            } else {
                if on_border_vertically {
                    Some((VerticalResize::Bottom, Edge::OnBorder))
                } else if on_corner_radius_vertically {
                    Some((VerticalResize::Bottom, Edge::OnCornerRadius))
                } else {
                    None
                }
            }
        };

        let horizontal_control = {
            let on_border_horizontally = x_on_frame < self.frame_config.border_width as i32
                || x_on_frame > frame_geom.width as i32 - self.frame_config.border_width as i32;
            let on_corner_radius_horizontally = x_on_frame < self.frame_config.corner_radius as i32
                || x_on_frame > frame_geom.width as i32 - self.frame_config.corner_radius as i32;

            if x_on_frame < (frame_geom.width / 2) as i32 {
                if on_border_horizontally {
                    Some((HorizontalResize::Left, Edge::OnBorder))
                } else if on_corner_radius_horizontally {
                    Some((HorizontalResize::Left, Edge::OnCornerRadius))
                } else {
                    None
                }
            } else {
                if on_border_horizontally {
                    Some((HorizontalResize::Right, Edge::OnBorder))
                } else if on_corner_radius_horizontally {
                    Some((HorizontalResize::Right, Edge::OnCornerRadius))
                } else {
                    None
                }
            }
        };

        if let (Some((vertical, _)), Some((horizontal, _))) = (vertical_control, horizontal_control)
        {
            GeometryControl::Resize(vertical, horizontal)
        } else if let Some((vertical, Edge::OnBorder)) = vertical_control {
            GeometryControl::Resize(vertical, HorizontalResize::None)
        } else if let Some((horizontal, Edge::OnBorder)) = horizontal_control {
            GeometryControl::Resize(VerticalResize::None, horizontal)
        } else {
            GeometryControl::Move
        }
    }

    pub fn move_resize_on_control(
        &self,
        cursor_move_x: i32,
        cursor_move_y: i32,
        control: GeometryControl,
    ) -> ClientGeometry {
        match control {
            GeometryControl::Move => self.move_relative(cursor_move_x, cursor_move_y),
            GeometryControl::Resize(vertical, horizontal) => {
                let mut new_geom = *self;
                match vertical {
                    VerticalResize::Top => {
                        new_geom.geometry.y += cursor_move_y;
                        new_geom.geometry.height -= cursor_move_y as u32;
                    }
                    VerticalResize::Bottom => {
                        new_geom.geometry.height += cursor_move_y as u32;
                    }
                    VerticalResize::None => {}
                }
                match horizontal {
                    HorizontalResize::Left => {
                        new_geom.geometry.x += cursor_move_x;
                        new_geom.geometry.width -= cursor_move_x as u32;
                    }
                    HorizontalResize::Right => {
                        new_geom.geometry.width += cursor_move_x as u32;
                    }
                    HorizontalResize::None => {}
                }
                new_geom
            }
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

    #[test]
    fn test_control() {
        let frame_config = FrameConfig {
            border_width: 4,
            titlebar_height: 20,
            corner_radius: 6,
        };
        let client_geom = ClientGeometry::from_app(0, 0, 100, 100, frame_config);

        // top left
        {
            // on border
            assert_eq!(
                client_geom.check_control_by_position_on_frame(1, 1),
                GeometryControl::Resize(VerticalResize::Top, HorizontalResize::Left)
            );

            // on corner radius
            assert_eq!(
                client_geom.check_control_by_position_on_frame(5, 5),
                GeometryControl::Resize(VerticalResize::Top, HorizontalResize::Left)
            );

            // outside of corner radius
            assert_eq!(
                client_geom.check_control_by_position_on_frame(9, 9),
                GeometryControl::Move
            );

            // on border but only vertical
            assert_eq!(
                client_geom.check_control_by_position_on_frame(9, 1),
                GeometryControl::Resize(VerticalResize::Top, HorizontalResize::None)
            );

            // on border but only horizontal
            assert_eq!(
                client_geom.check_control_by_position_on_frame(1, 9),
                GeometryControl::Resize(VerticalResize::None, HorizontalResize::Left)
            );
        }
        // top right
        {
            // on border
            assert_eq!(
                client_geom.check_control_by_position_on_frame(107, 1),
                GeometryControl::Resize(VerticalResize::Top, HorizontalResize::Right)
            );

            // on corner radius
            assert_eq!(
                client_geom.check_control_by_position_on_frame(103, 5),
                GeometryControl::Resize(VerticalResize::Top, HorizontalResize::Right)
            );

            // outside of corner radius
            assert_eq!(
                client_geom.check_control_by_position_on_frame(99, 9),
                GeometryControl::Move
            );

            // on border but only vertical
            assert_eq!(
                client_geom.check_control_by_position_on_frame(99, 1),
                GeometryControl::Resize(VerticalResize::Top, HorizontalResize::None)
            );

            // on border but only horizontal
            assert_eq!(
                client_geom.check_control_by_position_on_frame(107, 9),
                GeometryControl::Resize(VerticalResize::None, HorizontalResize::Right)
            );
        }

        // bottom left
        {
            // on border
            assert_eq!(
                client_geom.check_control_by_position_on_frame(1, 127),
                GeometryControl::Resize(VerticalResize::Bottom, HorizontalResize::Left)
            );

            // on corner radius
            assert_eq!(
                client_geom.check_control_by_position_on_frame(5, 123),
                GeometryControl::Resize(VerticalResize::Bottom, HorizontalResize::Left)
            );

            // outside of corner radius
            assert_eq!(
                client_geom.check_control_by_position_on_frame(9, 119),
                GeometryControl::Move
            );

            // on border but only vertical
            assert_eq!(
                client_geom.check_control_by_position_on_frame(9, 127),
                GeometryControl::Resize(VerticalResize::Bottom, HorizontalResize::None)
            );

            // on border but only horizontal
            assert_eq!(
                client_geom.check_control_by_position_on_frame(1, 119),
                GeometryControl::Resize(VerticalResize::None, HorizontalResize::Left)
            );
        }

        // bottom right
        {
            // on border
            assert_eq!(
                client_geom.check_control_by_position_on_frame(107, 127),
                GeometryControl::Resize(VerticalResize::Bottom, HorizontalResize::Right)
            );

            // on corner radius
            assert_eq!(
                client_geom.check_control_by_position_on_frame(103, 123),
                GeometryControl::Resize(VerticalResize::Bottom, HorizontalResize::Right)
            );

            // outside of corner radius
            assert_eq!(
                client_geom.check_control_by_position_on_frame(99, 119),
                GeometryControl::Move
            );

            // on border but only vertical
            assert_eq!(
                client_geom.check_control_by_position_on_frame(99, 127),
                GeometryControl::Resize(VerticalResize::Bottom, HorizontalResize::None)
            );

            // on border but only horizontal
            assert_eq!(
                client_geom.check_control_by_position_on_frame(107, 119),
                GeometryControl::Resize(VerticalResize::None, HorizontalResize::Right)
            );
        }
    }
}
