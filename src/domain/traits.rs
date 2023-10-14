use x11rb::{
    protocol::xproto::{ConfigureWindowAux, Window},
    rust_connection::{ConnectionError, RustConnection},
};

use super::types::ClientId;

pub trait ClientConfigurationImpl {
    fn req_move(&self, x: i32, y: i32);
    fn req_resize(&self, width: i32, height: i32);
}

pub trait ClientImpl<'a> {
    fn create(conn: &'a RustConnection, app_id: Window, frame_id: Window) -> Self;
    fn commit_to_app(&self, conf: &ConfigureWindowAux) -> Result<(), ConnectionError>;
}

pub trait ClientManagerImpl<'a, C>
where
    C: ClientImpl<'a>,
{
    fn new(conn: &'a RustConnection) -> Self;
    fn bind(&mut self, app_id: Window) -> Option<ClientId>;
    fn release(&mut self, client_id: ClientId);
    fn find_from_window(&self, window_id: Window) -> Option<ClientId>;
    fn query_client(&self, client_id: ClientId) -> Option<&C>;
}
