use log::{info, warn};
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{
            ButtonPressEvent, ButtonReleaseEvent, ConfigureRequestEvent, ConfigureWindowAux,
            ConnectionExt, CreateWindowAux, EventMask, MapRequestEvent, MotionNotifyEvent, Window,
            WindowClass,
        },
        Event,
    },
    COPY_DEPTH_FROM_PARENT,
};

use crate::model::client::{container::ClientContainer, geometry::ClientGeometry, Client};

use super::session::X11Session;

/// Handler processes X11 events and dispatches them to the appropriate client.
pub struct Handler<'a> {
    session: &'a X11Session,
    grabbed_client: Option<Client<Window>>,
    client_container: ClientContainer<Window>,
}

impl<'a> Handler<'a> {
    pub fn new(session: &'a X11Session) -> Self {
        Self {
            session,
            grabbed_client: None,
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
            Event::ButtonPress(event) => self.handle_button_press(event)?,
            Event::ButtonRelease(event) => self.handle_button_release(event)?,
            Event::MotionNotify(event) => self.handle_motion_notify(event)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_button_press(
        &mut self,
        event: ButtonPressEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = if let Some(client) = self.client_container.query_client_from_id(event.event) {
            if client.frame_id == event.event {
                client
            } else {
                // client is not a frame
                return Ok(());
            }
        } else {
            // client not found
            return Ok(());
        };
        info!("Client found: {:?}", client);
        self.grabbed_client = Some(client);
        Ok(())
    }

    fn handle_button_release(
        &mut self,
        _event: ButtonReleaseEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Button release");
        self.grabbed_client = None;
        Ok(())
    }

    fn handle_motion_notify(
        &self,
        _event: MotionNotifyEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Motion notify");
        Ok(())
    }

    fn handle_configure_request(
        &self,
        event: ConfigureRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let values: ConfigureWindowAux =
            ConfigureWindowAux::from_configure_request(&event).stack_mode(None);
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
            .event_mask(
                EventMask::BUTTON_PRESS
                    | EventMask::BUTTON_RELEASE
                    | EventMask::POINTER_MOTION
                    | EventMask::EXPOSURE,
            )
            .background_pixel(0x888888);

        let original_geometry = self
            .session
            .connection()
            .get_geometry(event.window)?
            .reply()?;

        let client_geometry = ClientGeometry::from_app(
            original_geometry.x as i32 + self.session.config().border_width as i32,
            original_geometry.y as i32
                + self.session.config().titlebar_height as i32
                + self.session.config().border_width as i32,
            original_geometry.width as u32,
            original_geometry.height as u32,
        );

        let app_geometry = client_geometry.parse_as_app(
            self.session.config().border_width,
            self.session.config().titlebar_height,
        );

        let frame_geometry = client_geometry.parse_as_frame(
            self.session.config().border_width,
            self.session.config().titlebar_height,
        );

        self.session.connection().create_window(
            COPY_DEPTH_FROM_PARENT,
            frame,
            self.session.screen().root,
            frame_geometry.x as i16,
            frame_geometry.y as i16,
            frame_geometry.width as u16,
            frame_geometry.height as u16,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &frame_values,
        )?;

        self.session.connection().grab_server()?;

        self.session.connection().configure_window(
            event.window,
            &ConfigureWindowAux::default()
                .stack_mode(x11rb::protocol::xproto::StackMode::ABOVE)
                .x(app_geometry.x as i32)
                .y(app_geometry.y as i32)
                .width(app_geometry.width)
                .height(app_geometry.height),
        )?;

        self.session.connection().map_window(frame)?;
        self.session.connection().map_window(event.window)?;
        self.session.connection().ungrab_server()?;

        // add client to container
        self.client_container.add_client(event.window, frame);

        Ok(())
    }
    /*
    fn resize_client(
        &self,
        client: Client<Window>,
        geometry: ClientGeometry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.session.config();
        let (app_geometry, frame_geometry) = match geometry {
            ClientGeometry::App(x, y, width, height) => {
                let frame = geometry.to_frame(config.border_width, config.titlebar_height);
                (geometry.to_tuple(), frame.to_tuple())
            }
            ClientGeometry::Frame(x, y, width, height) => {
                let app = geometry.to_app(config.border_width, config.titlebar_height);
                (app.to_tuple(), geometry.to_tuple())
            }
        };

        self.session.connection().grab_server()?;

        self.session.connection().configure_window(
            client.app_id,
            &ConfigureWindowAux::default()
                .x(app_geometry.0 as i32)
                .y(app_geometry.1 as i32)
                .width(app_geometry.2)
                .height(app_geometry.3),
        )?;

        self.session.connection().configure_window(
            client.frame_id,
            &ConfigureWindowAux::default()
                .x(frame_geometry.0 as i32)
                .y(frame_geometry.1 as i32)
                .width(frame_geometry.2)
                .height(frame_geometry.3),
        )?;

        self.session.connection().ungrab_server()?;

        Ok(())
    }
    */
}
