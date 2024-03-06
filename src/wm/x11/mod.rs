use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{
            ChangeWindowAttributesAux, ConfigureRequestEvent, ConfigureWindowAux, ConnectionExt,
            CreateWindowAux, EventMask, MapRequestEvent, Screen, Window, WindowClass,
        },
        Event,
    },
    COPY_DEPTH_FROM_PARENT,
};

use crate::model::{client::container::ClientContainer, config::WindowManagerConfig};

pub struct X11WindowManager {
    connection: x11rb::rust_connection::RustConnection,
    screen_num: usize,
    client_container: ClientContainer<Window>,
    window_manager_config: WindowManagerConfig,
}

impl X11WindowManager {
    pub fn new(window_manager_config: WindowManagerConfig) -> Self {
        let (connection, screen_num) = x11rb::connect(None).unwrap();
        Self {
            connection,
            screen_num,
            client_container: ClientContainer::new(),
            window_manager_config,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let root_values = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY);

        self.connection
            .change_window_attributes(self.screen().root, &root_values)?
            .check()?;

        loop {
            self.connection.flush()?;
            let event = self.connection.wait_for_event()?;
            self.handle_event(event)?;
        }
    }

    fn screen(&self) -> &Screen {
        &self.connection.setup().roots[self.screen_num]
    }

    fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::ClientMessage(_) => {
                return Ok(());
            }
            Event::ConfigureRequest(event) => self.handle_configure_request(event)?,
            Event::MapRequest(event) => self.handle_map_request(event)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_configure_request(
        &self,
        event: ConfigureRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let values: ConfigureWindowAux = ConfigureWindowAux::from_configure_request(&event);
        self.connection.configure_window(event.window, &values)?;
        Ok(())
    }

    fn handle_map_request(
        &mut self,
        event: MapRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let frame = self.connection.generate_id()?;
        let frame_values = CreateWindowAux::default()
            .event_mask(EventMask::BUTTON_PRESS | EventMask::EXPOSURE)
            .background_pixel(0x888888);

        let geometry = self.connection.get_geometry(event.window)?.reply()?;

        self.connection.create_window(
            COPY_DEPTH_FROM_PARENT + 1,
            frame,
            self.screen().root,
            geometry.x,
            geometry.y,
            geometry.width,
            geometry.height,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &frame_values,
        )?;

        // map window
        self.connection.grab_server()?;
        self.connection.map_window(frame)?;
        self.connection.map_window(event.window)?;
        self.connection.ungrab_server()?;

        // add client to container
        self.client_container.add_client(event.window, frame);

        Ok(())
    }
}
