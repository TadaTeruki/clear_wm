use x11rb::{connection::Connection, protocol::xproto::Screen, xcb_ffi::XCBConnection};

use crate::config::WindowManagerConfig;

/// X11Session connects to the X11 server and provides static information about the X11 server and the window manager configuration.
pub struct X11Session {
    connection: XCBConnection,
    screen_num: usize,
    window_manager_config: WindowManagerConfig,
}

impl X11Session {
    pub fn connect(
        window_manager_config: WindowManagerConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (connection, screen_num) = XCBConnection::connect(None)?;
        Ok(Self {
            connection,
            screen_num,
            window_manager_config,
        })
    }

    pub fn connection(&self) -> &XCBConnection {
        &self.connection
    }

    pub fn screen(&self) -> &Screen {
        &self.connection.setup().roots[self.screen_num]
    }

    pub fn screen_num(&self) -> usize {
        self.screen_num
    }

    pub fn config(&self) -> &WindowManagerConfig {
        &self.window_manager_config
    }
}
