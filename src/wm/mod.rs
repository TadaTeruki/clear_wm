use log::info;
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{
            ChangeWindowAttributesAux, ConfigureRequestEvent, ConfigureWindowAux, ConnectionExt,
            CreateWindowAux, EventMask, MapRequestEvent, Window, WindowClass,
        },
        Event,
    },
    COPY_DEPTH_FROM_PARENT,
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

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let screen = &self.connection.setup().roots[self.screen_num];
        let root_values = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY);

        self.connection
            .change_window_attributes(screen.root, &root_values)?
            .check()?;

        loop {
            let event = self.connection.wait_for_event()?;
            info!("Event: {:?}", event);
            match event {
                Event::ConfigureRequest(event) => self.handle_configure_request(event)?,
                Event::MapRequest(event) => self.handle_map_request(event)?,
                _ => {}
            }
        }
    }

    fn root_window(&self) -> Window {
        self.connection.setup().roots[self.screen_num].root
    }

    fn handle_configure_request(
        &self,
        event: ConfigureRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let values = ConfigureWindowAux::from_configure_request(&event);
        self.connection.configure_window(event.window, &values)?;
        Ok(())
    }

    fn handle_map_request(
        &mut self,
        event: MapRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // create simple window for debug
        let frame = self.connection.generate_id()?;
        let frame_values = CreateWindowAux::default().background_pixel(0x00ff00);

        self.connection.create_window(
            COPY_DEPTH_FROM_PARENT,
            frame,
            self.root_window(),
            300,
            300,
            100,
            100,
            10,
            WindowClass::INPUT_OUTPUT,
            0,
            &frame_values,
        )?;

        self.connection.grab_server()?;

        // map window
        self.connection.map_window(event.window)?;

        // map frame
        self.connection.map_window(frame)?;

        self.connection.ungrab_server()?;
        Ok(())
    }
}
