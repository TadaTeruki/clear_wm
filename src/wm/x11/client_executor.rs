use x11rb::protocol::xproto::{ConfigureWindowAux, ConnectionExt, Window};

use crate::model::client::{geometry::ClientGeometry, Client};

use super::session::X11Session;

pub struct ClientExecutor<'a> {
    session: &'a X11Session,
}

impl<'a> ClientExecutor<'a> {
    pub fn new(session: &'a X11Session) -> Self {
        Self { session }
    }

    pub fn focus_client(&self, client: Client<Window>) -> Result<(), Box<dyn std::error::Error>> {
        self.session.connection().set_input_focus(
            x11rb::protocol::xproto::InputFocus::POINTER_ROOT,
            client.app_id,
            x11rb::CURRENT_TIME,
        )?;

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
            self.session.config().border_width,
            self.session.config().titlebar_height,
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
