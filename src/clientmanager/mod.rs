use x11rb::{protocol::xproto::Window, rust_connection::RustConnection};

use crate::domain::{
    traits::{ClientImpl, ClientManagerImpl},
    types::ClientId,
};

pub struct ClientManager<'a, C>
where
    C: ClientImpl<'a>,
{
    conn: &'a RustConnection,
    dictionary: Vec<(Window, ClientId)>,
    clients: Vec<(ClientId, C)>,
}

impl<'a, C> ClientManagerImpl<'a, C> for ClientManager<'a, C>
where
    C: ClientImpl<'a>,
{
    fn new(conn: &'a RustConnection) -> Self {
        Self {
            conn,
            clients: Vec::new(),
            dictionary: Vec::new(),
        }
    }

    fn bind(&mut self, app_id: Window) -> Option<ClientId> {
        let client = C::create(self.conn, app_id, 0 as Window);
        let client_id = rand::random::<ClientId>();
        if self.find_from_window(app_id).is_some() {
            return None;
        }
        if self.query_client(client_id).is_some() {
            return None;
        }
        self.clients.push((client_id, client));
        self.dictionary.push((app_id, client_id));
        Some(client_id)
    }

    fn release(&mut self, client_id: ClientId) {
        self.clients.retain(|(cli_id, _)| {
            *cli_id != client_id
        });
        self.dictionary.retain(|(_, cli_id)| {
            *cli_id != client_id
        });
    }

    fn find_from_window(&self, window_id: Window) -> Option<ClientId> {
        if let Some((_, cli_id)) = self.dictionary.iter().find(|(win_id, _)| {
            *win_id == window_id
        }) {
            return Some(*cli_id);
        }
        None
    }

    fn query_client(&self, client_id: ClientId) -> Option<&C> {
        if let Some((_, cli)) = self.clients.iter().find(|(cli_id, _)| {
            *cli_id == client_id
        }) {
            return Some(cli);
        }
        None
    }
}
