use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{
            ConfigureRequestEvent, ConfigureWindowAux, ConnectionExt, CreateWindowAux, EventMask,
            MapRequestEvent, Window, WindowClass,
        },
        Event,
    },
    COPY_DEPTH_FROM_PARENT,
};

use crate::model::client::container::ClientContainer;

use super::session::X11Session;

pub struct Handler<'a> {
    session: &'a X11Session,
    client_container: ClientContainer<Window>,
}

impl<'a> Handler<'a> {
    pub fn new(session: &'a X11Session) -> Self {
        Self {
            session,
            client_container: ClientContainer::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
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
        self.session
            .connection()
            .configure_window(event.window, &values)?;
        Ok(())
    }

    fn handle_map_request(
        &mut self,
        event: MapRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let frame = self.session.connection().generate_id()?;
        let frame_values = CreateWindowAux::default()
            .event_mask(EventMask::BUTTON_PRESS | EventMask::EXPOSURE)
            .background_pixel(0x888888);

        let geometry = self
            .session
            .connection()
            .get_geometry(event.window)?
            .reply()?;

        self.session.connection().create_window(
            COPY_DEPTH_FROM_PARENT + 1,
            frame,
            self.session.screen().root,
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
        self.session.connection().grab_server()?;
        self.session.connection().map_window(frame)?;
        self.session.connection().map_window(event.window)?;
        self.session.connection().ungrab_server()?;

        // add client to container
        self.client_container.add_client(event.window, frame);

        Ok(())
    }
}
