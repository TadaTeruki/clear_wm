//! Some helper functions to choose a visual to use and to check if a composite manager is running.
//!
//! Most code in this module is quoted from the `x11rb` crate's examples.
//! The copied code is licensed under the MIT license.
//!
//! The original code can be found at:
//! https://github.com/psychon/x11rb/blob/master/cairo-example
//!
//! The entire repository can be found at:
//! https://github.com/psychon/x11rb/

use x11rb::protocol::xproto::{Screen, VisualClass, Window};

use super::session::X11Session;

/// A rust version of XCB's `xcb_visualtype_t` struct. This is used in a FFI-way.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XCBVisualType {
    pub visual_id: u32,
    pub class: u8,
    pub bits_per_rgb_visual_type: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad0: [u8; 4],
}

/// Find a `xcb_visualtype_t` based on its ID number
fn find_xcb_visualtype(screen: &Screen, depth_: u8) -> Option<XCBVisualType> {
    for depth in &screen.allowed_depths {
        if depth.depth != depth_ {
            continue;
        }
        for visual_type in &depth.visuals {
            if visual_type.class == VisualClass::TRUE_COLOR {
                return Some(XCBVisualType {
                    visual_id: visual_type.visual_id,
                    class: visual_type.class.into(),
                    bits_per_rgb_visual_type: visual_type.bits_per_rgb_value,
                    colormap_entries: visual_type.colormap_entries,
                    red_mask: visual_type.red_mask,
                    green_mask: visual_type.green_mask,
                    blue_mask: visual_type.blue_mask,
                    pad0: [0; 4],
                });
            }
        }
    }
    None
}

pub struct CairoSession {
    visual_type: XCBVisualType,
    depth: u8,
}

pub struct CairoSurface {
    surface: cairo::XCBSurface,
}

impl CairoSession {
    pub fn create(screen: &Screen) -> Result<Self, Box<dyn std::error::Error>> {
        let depth = 32;
        let visual_type = find_xcb_visualtype(screen, depth).ok_or("Could not find visual")?;

        Ok(Self { visual_type, depth })
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }

    pub fn visual_type(&self) -> &XCBVisualType {
        &self.visual_type
    }

    pub fn create_cairo_surface_for_window(
        &self,
        session: &X11Session,
        window: Window,
        width: i32,
        height: i32,
    ) -> Result<CairoSurface, Box<dyn std::error::Error>> {
        let cairo_connection = unsafe {
            cairo::XCBConnection::from_raw_none(session.connection().get_raw_xcb_connection() as _)
        };
        let mut visual_type = self.visual_type;
        let visual =
            unsafe { cairo::XCBVisualType::from_raw_none(&mut visual_type as *mut _ as _) };
        let surface = cairo::XCBSurface::create(
            &cairo_connection,
            &cairo::XCBDrawable(window),
            &visual,
            width,
            height,
        )?;
        Ok(CairoSurface { surface })
    }
}

impl CairoSurface {
    pub fn context(&self) -> Result<cairo::Context, Box<dyn std::error::Error>> {
        Ok(cairo::Context::new(&self.surface)?)
    }

    pub fn resize(&self, width: i32, height: i32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.surface.set_size(width, height)?)
    }

    pub fn flush(&self) {
        self.surface.flush();
    }
}
