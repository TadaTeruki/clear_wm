use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt, Window};

use crate::model::client::{
    container::ClientContainer, geometry::ClientGeometry, map::ClientMap, Client,
};

use super::{graphics::CairoSurface, session::X11Session};

pub struct ClientExecutor<'a> {
    session: &'a X11Session,
    client_container: ClientContainer<Window>,
    surface_container: ClientMap<Window, CairoSurface>,
}

impl<'a> ClientExecutor<'a> {
    pub fn new(session: &'a X11Session) -> Self {
        Self {
            session,
            client_container: ClientContainer::new(),
            surface_container: ClientMap::new(),
        }
    }

    pub fn container(&self) -> &ClientContainer<Window> {
        &self.client_container
    }

    pub fn add_client(
        &mut self,
        app_id: Window,
        frame_id: Window,
        client_geometry: ClientGeometry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let frame_geometry = client_geometry.parse_as_frame();
        let surface = self
            .session
            .cairo_session()
            .create_cairo_surface_for_window(
                self.session,
                frame_id,
                frame_geometry.width as i32,
                frame_geometry.height as i32,
            )?;

        let client = self.client_container.add_client(app_id, frame_id);

        self.surface_container.insert(client, surface);

        Ok(())
    }

    pub fn draw_client(&self, client: Client<Window>) -> Result<(), Box<dyn std::error::Error>> {
        let surface = if let Some(surface) = self.surface_container.query(client) {
            surface
        } else {
            return Ok(());
        };

        let ctx = surface.context()?;
        ctx.set_source_rgb(1.0, 0.5, 0.5);
        ctx.paint()?;
        surface.flush();

        Ok(())
    }

    pub fn remove_client(&mut self, client: Client<Window>) {
        self.client_container.remove_client(client);
        self.surface_container.remove(client);
    }

    fn get_focused_client(&self) -> Result<Option<Client<Window>>, Box<dyn std::error::Error>> {
        let focused_window = self.session.connection().get_input_focus()?.reply()?.focus;

        Ok(self.client_container.query_client_from_app(focused_window))
    }

    pub fn raise_client(&self, client: Client<Window>) -> Result<(), Box<dyn std::error::Error>> {
        // If there is a previous client, move the frame to the above of the stack to hide application window.
        if let Some(previous_client) = self.get_focused_client()? {
            if previous_client == client {
                return Ok(());
            }
            self.session.connection().configure_window(
                previous_client.frame_id,
                &ConfigureWindowAux::default()
                    .stack_mode(x11rb::protocol::xproto::StackMode::ABOVE),
            )?;
        }

        // Focus the client's application window.
        self.session.connection().set_input_focus(
            x11rb::protocol::xproto::InputFocus::POINTER_ROOT,
            client.app_id,
            x11rb::CURRENT_TIME,
        )?;

        // Move the frame and the application window to the above of the stack.
        self.session.connection().configure_window(
            client.frame_id,
            &ConfigureWindowAux::default().stack_mode(x11rb::protocol::xproto::StackMode::ABOVE),
        )?;
        self.session.connection().configure_window(
            client.app_id,
            &ConfigureWindowAux::default().stack_mode(x11rb::protocol::xproto::StackMode::ABOVE),
        )?;

        Ok(())
    }

    pub fn get_client_geometry(
        &self,
        client: Client<Window>,
    ) -> Result<ClientGeometry, Box<dyn std::error::Error>> {
        let x11_app_geometry = self
            .session
            .connection()
            .get_geometry(client.app_id)?
            .reply()?;

        Ok(ClientGeometry::from_app(
            x11_app_geometry.x as i32,
            x11_app_geometry.y as i32,
            x11_app_geometry.width as u32,
            x11_app_geometry.height as u32,
            self.session.config().frame_config,
        ))
    }

    pub fn apply_client_geometry(
        &self,
        client: Client<Window>,
        client_geometry: ClientGeometry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let app_geometry = client_geometry.parse_as_app();

        let frame_geometry = client_geometry.parse_as_frame();

        self.session.connection().configure_window(
            client.app_id,
            &ConfigureWindowAux::default()
                .x(app_geometry.x)
                .y(app_geometry.y)
                .width(app_geometry.width)
                .height(app_geometry.height),
        )?;

        self.session.connection().configure_window(
            client.frame_id,
            &ConfigureWindowAux::default()
                .x(frame_geometry.x)
                .y(frame_geometry.y)
                .width(frame_geometry.width)
                .height(frame_geometry.height),
        )?;

        Ok(())
    }
}
