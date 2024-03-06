use x11rb::{connection::Connection, protocol::xproto::Screen, rust_connection::RustConnection};

use crate::model::config::WindowManagerConfig;

pub struct X11Session {
    connection: x11rb::rust_connection::RustConnection,
    screen_num: usize,
    window_manager_config: WindowManagerConfig,
}

impl X11Session {
    pub fn connect(
        window_manager_config: WindowManagerConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (connection, screen_num) = RustConnection::connect(None)?;
        Ok(Self {
            connection,
            screen_num,
            window_manager_config,
        })
    }

    pub fn connection(&self) -> &RustConnection {
        &self.connection
    }

    pub fn screen(&self) -> &Screen {
        &self.connection.setup().roots[self.screen_num]
    }

    pub fn config(&self) -> &WindowManagerConfig {
        &self.window_manager_config
    }
}
