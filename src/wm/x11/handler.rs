use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{
            ButtonPressEvent, ButtonReleaseEvent, ColormapAlloc, ConfigureRequestEvent,
            ConfigureWindowAux, ConnectionExt, CreateWindowAux, EventMask, MapNotifyEvent,
            MapRequestEvent, MotionNotifyEvent, UnmapNotifyEvent, Window, WindowClass,
        },
        Event,
    },
};

use crate::model::client::{drag::DragState, geometry::ClientGeometry};

use super::{cairo::CairoSession, client_executor::ClientExecutor, session::X11Session};

/// Handler processes X11 events and dispatches them to the appropriate client.
pub struct Handler<'a> {
    session: &'a X11Session,
    drag_state: DragState<Window>,
    client_exec: ClientExecutor<'a>,
}

impl<'a> Handler<'a> {
    pub fn new(session: &'a X11Session, cairo_session: CairoSession) -> Self {
        Self {
            session,
            drag_state: DragState::None,
            client_exec: ClientExecutor::new(session, cairo_session),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("event: {:?}", event);
        match event {
            Event::ClientMessage(_) => {
                return Ok(());
            }
            Event::ConfigureRequest(event) => self.handle_configure_request(event)?,
            Event::MapRequest(event) => self.handle_map_request(event)?,
            Event::MapNotify(event) => self.handle_map_notify(event)?,
            Event::ButtonPress(event) => self.handle_button_press(event)?,
            Event::ButtonRelease(event) => self.handle_button_release(event)?,
            Event::MotionNotify(event) => self.handle_motion_notify(event)?,
            Event::UnmapNotify(event) => self.handle_unmap_notify(event)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_button_press(
        &mut self,
        event: ButtonPressEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get client if the window is a frame
        let client = if let Some(client) = self
            .client_exec
            .container()
            .query_client_from_frame(event.event)
        {
            client
        } else {
            return Ok(());
        };

        self.client_exec.raise_client(client)?;

        let geometry_control = self
            .client_exec
            .get_client_geometry(client)?
            .check_control_by_position_on_frame(event.event_x as i32, event.event_y as i32);

        // save the start position of cursor for dragging
        let last_root_position = (event.root_x as i32, event.root_y as i32);
        self.drag_state = DragState::new_as_dragging(client, geometry_control, last_root_position);
        Ok(())
    }

    fn handle_button_release(
        &mut self,
        _event: ButtonReleaseEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.drag_state.release_from_dragging();
        Ok(())
    }

    fn handle_motion_notify(
        &mut self,
        event: MotionNotifyEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get client if the window is a frame
        let client = if let Some(client) = self
            .client_exec
            .container()
            .query_client_from_frame(event.event)
        {
            client
        } else {
            return Ok(());
        };

        let drag_state = if let Some(drag_state) = self.drag_state.parse_with_check_dragging(client)
        {
            drag_state
        } else {
            return Ok(());
        };

        let root_position = (event.root_x as i32, event.root_y as i32);
        let diff_position = (
            root_position.0 - drag_state.last_root_position().0,
            root_position.1 - drag_state.last_root_position().1,
        );

        let client_geometry = self
            .client_exec
            .get_client_geometry(client)?
            .move_resize_on_control(
                diff_position.0,
                diff_position.1,
                drag_state.geometry_control(),
            );

        self.execute_grabbed(|| {
            self.client_exec
                .apply_client_geometry(client, client_geometry)
        })?;

        self.drag_state.change_root_position(root_position);
        Ok(())
    }

    fn handle_configure_request(
        &self,
        event: ConfigureRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // just configure the window
        let values: ConfigureWindowAux =
            ConfigureWindowAux::from_configure_request(&event).stack_mode(None);
        self.session
            .connection()
            .configure_window(event.window, &values)?;

        if let Some(client) = self
            .client_exec
            .container()
            .query_client_from_app(event.window)
        {
            // if the window is binded to a frame, move the frame and configure the window
            let client_geometry = ClientGeometry::from_app(
                event.x as i32,
                event.y as i32,
                event.width as u32,
                event.height as u32,
                self.session.config().frame_config,
            );

            self.execute_grabbed(|| {
                self.client_exec
                    .apply_client_geometry(client, client_geometry)
            })?;
        }
        Ok(())
    }

    fn handle_map_request(
        &mut self,
        event: MapRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let frame = self.session.connection().generate_id()?;

        let frame_colormap = self.session.connection().generate_id()?;

        self.session.connection().create_colormap(
            ColormapAlloc::NONE,
            frame_colormap,
            self.session.screen().root,
            self.client_exec.cairo_session().visual_type().visual_id,
        )?;

        let frame_values = CreateWindowAux::default()
            .event_mask(
                EventMask::BUTTON_PRESS
                    | EventMask::BUTTON_RELEASE
                    | EventMask::POINTER_MOTION
                    | EventMask::EXPOSURE,
            )
            .border_pixel(0)
            .background_pixel(0)
            .colormap(frame_colormap);

        let original_geometry = self
            .session
            .connection()
            .get_geometry(event.window)?
            .reply()?;

        let frame_config = self.session.config().frame_config;

        let client_geometry: ClientGeometry = ClientGeometry::from_app(
            original_geometry.x as i32,
            original_geometry.y as i32 + frame_config.titlebar_height as i32,
            original_geometry.width as u32,
            original_geometry.height as u32,
            frame_config,
        );

        let app_geometry = client_geometry.parse_as_app();

        let frame_geometry = client_geometry.parse_as_frame();

        log::info!("creating window...");

        self.session.connection().create_window(
            self.client_exec.cairo_session().depth(),
            frame,
            self.session.screen().root,
            frame_geometry.x as i16,
            frame_geometry.y as i16,
            frame_geometry.width as u16,
            frame_geometry.height as u16,
            0,
            WindowClass::INPUT_OUTPUT,
            self.client_exec.cairo_session().visual_type().visual_id,
            &frame_values,
        )?;

        self.execute_grabbed(|| {
            self.session.connection().configure_window(
                event.window,
                &ConfigureWindowAux::default()
                    .stack_mode(x11rb::protocol::xproto::StackMode::ABOVE)
                    .x(app_geometry.x)
                    .y(app_geometry.y)
                    .width(app_geometry.width)
                    .height(app_geometry.height),
            )?;

            self.session.connection().map_window(frame)?;
            self.session.connection().map_window(event.window)?;
            Ok(())
        })?;

        log::info!("created window");

        self.client_exec
            .add_client(event.window, frame, client_geometry)?;

        Ok(())
    }

    fn handle_map_notify(
        &mut self,
        event: MapNotifyEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = if let Some(client) = self
            .client_exec
            .container()
            .query_client_from_app(event.window)
        {
            client
        } else {
            return Ok(());
        };

        self.client_exec.raise_client(client)?;
        Ok(())
    }

    fn handle_unmap_notify(
        &mut self,
        event: UnmapNotifyEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = if let Some(client) = self
            .client_exec
            .container()
            .query_client_from_app(event.window)
        {
            client
        } else {
            return Ok(());
        };

        self.execute_grabbed(|| {
            self.session.connection().destroy_window(client.frame_id)?;
            Ok(())
        })?;

        //self.client_exec.container_as_mut().remove_client(client);

        self.client_exec.remove_client(client);
        Ok(())
    }

    fn execute_grabbed<T, F: FnOnce() -> Result<T, Box<dyn std::error::Error>>>(
        &self,
        f: F,
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.session.connection().grab_server()?;
        let result = f();
        self.session.connection().ungrab_server()?;
        result
    }
}
