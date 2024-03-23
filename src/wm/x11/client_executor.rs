use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt, Window};

use crate::model::{
    client::{
        container::ClientContainer, geometry::ClientGeometry, hints::ClientHints, map::ClientMap,
        Client,
    },
    draw::FrameDrawContext,
};

use super::{graphics::CairoSurface, session::X11Session};

pub struct ClientExecutor<'a> {
    session: &'a X11Session,
    client_container: ClientContainer<Window>,
    surface_container: ClientMap<Window, CairoSurface>,
    draw_queue: ClientMap<Window, ()>,
    move_resize_queue: ClientMap<Window, ClientGeometry>,
    hints_cache: ClientMap<Window, ClientHints>,
}

pub enum ClientRaisedResult {
    Raised,
    NotChanged,
}

impl<'a> ClientExecutor<'a> {
    pub fn new(session: &'a X11Session) -> Self {
        Self {
            session,
            client_container: ClientContainer::new(),
            surface_container: ClientMap::new(),
            draw_queue: ClientMap::new(),
            move_resize_queue: ClientMap::new(),
            hints_cache: ClientMap::new(),
        }
    }

    pub fn container(&self) -> &ClientContainer<Window> {
        &self.client_container
    }

    pub fn update_hints(
        &mut self,
        client: Client<Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hints_cache = self.fetch_hints(client)?;
        self.hints_cache.insert(client, hints_cache);
        Ok(())
    }

    fn fetch_hints(
        &self,
        client: Client<Window>,
    ) -> Result<ClientHints, Box<dyn std::error::Error>> {
        let title = {
            let title = self
                .session
                .connection()
                .get_property(
                    false,
                    client.app_id,
                    self.session.atoms().WM_NAME,
                    self.session.atoms().UTF8_STRING,
                    0,
                    1024,
                )?
                .reply()?;

            if title.value_len != 0 {
                String::from_utf8(title.value)?
            } else {
                let title = self
                    .session
                    .connection()
                    .get_property(
                        false,
                        client.app_id,
                        self.session.atoms().WM_NAME,
                        self.session.atoms().STRING,
                        0,
                        1024,
                    )?
                    .reply()?;

                if title.value_len != 0 {
                    String::from_utf8(title.value)?
                } else {
                    String::from("")
                }
            }
        };
        Ok(ClientHints { title })
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

    pub fn remove_client(&mut self, client: Client<Window>) {
        self.client_container.remove_client(client);
        self.surface_container.remove(client);
        self.draw_queue.remove(client);
        self.move_resize_queue.remove(client);
        self.hints_cache.remove(client);
    }

    fn get_focused_client(&self) -> Result<Option<Client<Window>>, Box<dyn std::error::Error>> {
        let focused_window = self.session.connection().get_input_focus()?.reply()?.focus;

        Ok(self.client_container.query_client_from_app(focused_window))
    }

    // If the client is already raised, return false
    pub fn raise_client(
        &self,
        client: Client<Window>,
    ) -> Result<ClientRaisedResult, Box<dyn std::error::Error>> {
        if let Some(previous_client) = self.get_focused_client()? {
            if previous_client == client {
                return Ok(ClientRaisedResult::NotChanged);
            }
            // If there is a previous client, move the frame to the above of the stack to hide application window.
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

        Ok(ClientRaisedResult::Raised)
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

    pub fn flush_queued(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for (client, client_geometry) in self.move_resize_queue.iter() {
            self.move_resize_with_client_geometry(*client, *client_geometry)?;
        }

        for (client, _) in self.draw_queue.iter() {
            self.draw_client(*client)?;
        }
        self.draw_queue.clear();
        self.move_resize_queue.clear();
        Ok(())
    }

    pub fn queue_draw(&mut self, client: Client<Window>) {
        self.draw_queue.insert(client, ());
    }

    fn draw_client(&self, client: Client<Window>) -> Result<(), Box<dyn std::error::Error>> {
        let surface = if let Some(surface) = self.surface_container.query(client) {
            surface
        } else {
            return Ok(());
        };

        let hint_default = ClientHints::default();
        let hints = if let Some(hints) = self.hints_cache.query(client) {
            hints
        } else {
            &hint_default
        };

        let ctx = surface.context()?;
        FrameDrawContext::new(ctx).draw(
            &self.get_client_geometry(client)?,
            &self.session.config().frame_config,
            hints,
        )?;
        surface.flush();

        Ok(())
    }

    pub fn apply_geometry(
        &mut self,
        client: Client<Window>,
        client_geometry: ClientGeometry,
        can_be_resized: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if can_be_resized {
            self.queue_move_resize(client, client_geometry);
        } else {
            self.move_with_client_geometry(client, client_geometry)?;
        }
        Ok(())
    }

    fn queue_move_resize(&mut self, client: Client<Window>, client_geometry: ClientGeometry) {
        self.move_resize_queue.insert(client, client_geometry);
        self.queue_draw(client);
    }

    fn move_with_client_geometry(
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
                .y(app_geometry.y),
        )?;

        self.session.connection().configure_window(
            client.frame_id,
            &ConfigureWindowAux::default()
                .x(frame_geometry.x)
                .y(frame_geometry.y),
        )?;

        Ok(())
    }

    fn move_resize_with_client_geometry(
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

        if let Some(surface) = self.surface_container.query(client) {
            surface.resize(frame_geometry.width as i32, frame_geometry.height as i32)?;
        }

        Ok(())
    }
}
