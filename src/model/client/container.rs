use super::client::Client;

pub struct ClientContainer<WinId>
where
    WinId: Copy + PartialEq,
{
    clients: Vec<Client<WinId>>,
}

#[derive(Debug, PartialEq)]
pub enum WindowType {
    App,
    Frame,
}

impl<WinId> ClientContainer<WinId>
where
    WinId: Copy + PartialEq,
{
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
        }
    }

    pub fn add_client(&mut self, client: Client<WinId>) {
        self.clients.push(client);
    }

    pub fn query_client_from_id(&self, win_id: WinId) -> Option<(&Client<WinId>, WindowType)> {
        if let Some(app) = self.clients.iter().find(|client| client.app_id == win_id) {
            return Some((app, WindowType::App));
        } else if let Some(frame) = self.clients.iter().find(|client| client.frame_id == win_id) {
            return Some((frame, WindowType::Frame));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_container() {
        let mut container = ClientContainer::new();
        container.add_client(Client::new(1, 2));
        container.add_client(Client::new(3, 4));
        assert_eq!(
            container.query_client_from_id(1).unwrap().1,
            WindowType::App
        );
        assert_eq!(
            container.query_client_from_id(2).unwrap().1,
            WindowType::Frame
        );
        assert_eq!(
            container.query_client_from_id(3).unwrap().1,
            WindowType::App
        );
        assert_eq!(
            container.query_client_from_id(4).unwrap().1,
            WindowType::Frame
        );
        assert_eq!(container.query_client_from_id(5), None);
    }
}
