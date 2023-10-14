use x11rb::{
    protocol::xproto::{ConfigureWindowAux, ConnectionExt, Window},
    rust_connection::{ConnectionError, RustConnection},
};

use crate::domain::traits::ClientImpl;

pub struct Client<'a> {
    conn: &'a RustConnection,
    app_id: Window,
    frame_id: Window,
}

impl<'a> ClientImpl<'a> for Client<'a> {
    fn create(conn: &'a RustConnection, app_id: Window, frame_id: Window) -> Self {
        Self {
            conn,
            app_id,
            frame_id,
        }
    }

    fn commit_to_app(&self, conf: &ConfigureWindowAux) -> Result<(), ConnectionError> {
        self.conn.configure_window(self.app_id, conf)?;
        Ok(())
    }
}
