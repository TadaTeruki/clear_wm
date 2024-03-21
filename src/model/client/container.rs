use super::Client;

pub struct ClientContainer<WinId>
where
    WinId: Copy + Eq,
{
    clients: Vec<Client<WinId>>,
}

impl<WinId> ClientContainer<WinId>
where
    WinId: Copy + Eq,
{
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
        }
    }

    pub fn add_client(&mut self, app_id: WinId, frame_id: WinId) {
        self.clients.push(Client { app_id, frame_id });
    }

    pub fn query_client_from_app(&self, win_id: WinId) -> Option<Client<WinId>> {
        if let Some(app) = self.clients.iter().find(|client| client.app_id == win_id) {
            return Some(*app);
        }
        None
    }

    pub fn query_client_from_frame(&self, win_id: WinId) -> Option<Client<WinId>> {
        if let Some(frame) = self.clients.iter().find(|client| client.frame_id == win_id) {
            return Some(*frame);
        }
        None
    }

    pub fn remove_client(&mut self, client: Client<WinId>) {
        self.clients.retain(|c| c != &client);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_container() {
        let mut container = ClientContainer::new();
        container.add_client(1, 2);
        container.add_client(3, 4);
        assert_eq!(
            container.query_client_from_app(1).unwrap(),
            container.query_client_from_frame(2).unwrap()
        );
        assert_eq!(
            container.query_client_from_app(3).unwrap(),
            container.query_client_from_frame(4).unwrap()
        );
        assert_ne!(
            container.query_client_from_app(1).unwrap(),
            container.query_client_from_app(3).unwrap()
        );
        assert_eq!(container.query_client_from_app(5), None);
        assert_eq!(container.query_client_from_frame(6), None);
    }
}
