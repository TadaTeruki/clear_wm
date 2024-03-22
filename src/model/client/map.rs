use super::Client;

pub struct ClientMap<WinId, T>
where
    WinId: Copy + Eq,
{
    pub item: Vec<(Client<WinId>, T)>,
}

impl<WinId, T> ClientMap<WinId, T>
where
    WinId: Copy + Eq,
{
    pub fn new() -> Self {
        Self { item: Vec::new() }
    }

    pub fn insert(&mut self, client: Client<WinId>, item: T) {
        // if already exists, update
        if let Some((_, i)) = self.item.iter_mut().find(|(c, _)| c == &client) {
            *i = item;
        } else {
            self.item.push((client, item));
        }
    }

    pub fn query(&self, client: Client<WinId>) -> Option<&T> {
        self.item
            .iter()
            .find(|(c, _)| c == &client)
            .map(|(_, item)| item)
    }

    pub fn remove(&mut self, client: Client<WinId>) {
        self.item.retain(|(c, _)| c != &client);
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Client<WinId>, T)> {
        self.item.iter()
    }

    pub fn clear(&mut self) {
        self.item.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_map() {
        let mut map = ClientMap::new();
        map.insert(
            Client {
                app_id: 1,
                frame_id: 2,
            },
            3,
        );
        map.insert(
            Client {
                app_id: 4,
                frame_id: 5,
            },
            6,
        );
        assert_eq!(
            map.query(Client {
                app_id: 1,
                frame_id: 2
            }),
            Some(&3)
        );
        assert_eq!(
            map.query(Client {
                app_id: 4,
                frame_id: 5
            }),
            Some(&6)
        );

        map.remove(Client {
            app_id: 1,
            frame_id: 2,
        });
        assert_eq!(
            map.query(Client {
                app_id: 1,
                frame_id: 2
            }),
            None
        );
        assert_eq!(
            map.query(Client {
                app_id: 4,
                frame_id: 5
            }),
            Some(&6)
        );
    }
}
