use log::info;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt, EventMask, Window},
};

use crate::model::client::container::ClientContainer;

pub struct WindowManager {
    connection: x11rb::rust_connection::RustConnection,
    screen_num: usize,
    client_container: ClientContainer<Window>,
}

impl WindowManager {
    pub fn new(client_container: ClientContainer<Window>) -> Self {
        let (connection, screen_num) = x11rb::connect(None).unwrap();
        Self {
            connection,
            screen_num,
            client_container,
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let screen = &self.connection.setup().roots[self.screen_num];
        let root_values = ChangeWindowAttributesAux::default()
            .event_mask(
                EventMask::EXPOSURE
                    | EventMask::BUTTON_PRESS
                    | EventMask::BUTTON_RELEASE
                    | EventMask::POINTER_MOTION
                    | EventMask::SUBSTRUCTURE_NOTIFY,
            )
            .background_pixel(screen.white_pixel);

        self.connection
            .change_window_attributes(screen.root, &root_values)?;

        loop {
            let event = self.connection.wait_for_event()?;
            info!("Event: {:?}", event);
        }

        Ok(())
    }
}
