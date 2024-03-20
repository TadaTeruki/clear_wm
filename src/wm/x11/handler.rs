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

use super::{client_executor::ClientExecutor, middle_executor::MiddleExecutor};

struct DraggingClient {
    pub client: Client<Window>,
    pub last_root_position: (i32, i32),
}

/// Handler processes X11 events and dispatches them to the appropriate client.
pub struct Handler<'a> {
    middle_executor: MiddleExecutor<'a>,
    client_container: ClientContainer<Window>,

    dragging_client: Option<DraggingClient>,
}

impl<'a> Handler<'a> {
    pub fn new(mid_executor: MiddleExecutor<'a>) -> Self {
        Self {
            middle_executor: mid_executor,
            dragging_client: None,
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
        // get client if the window is a frame
        let client = if let Some(client) = self.client_container.query_client_from_id(event.event) {
            if client.frame_id == event.event {
                client
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        };
        // save the start position of pointer for dragging
        let last_root_position = (event.root_x as i32, event.root_y as i32);
        self.dragging_client = Some(DraggingClient {
            client,
            last_root_position,
        });
        Ok(())
    }

    fn handle_button_release(
        &mut self,
        _event: ButtonReleaseEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.dragging_client = None;
        Ok(())
    }

    fn handle_motion_notify(
        &mut self,
        event: MotionNotifyEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get client if the window is a frame
        let client = if let Some(client) = self.client_container.query_client_from_id(event.event) {
            if client.frame_id == event.event {
                client
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        };

        // check if the client is being dragged
        let dragging_client: &DraggingClient = if let Some(dragging_client) = &self.dragging_client
        {
            if dragging_client.client != client {
                return Ok(());
            }
            dragging_client
        } else {
            return Ok(());
        };

        let root_position = (event.root_x as i32, event.root_y as i32);
        let diff_position = (
            root_position.0 - dragging_client.last_root_position.0,
            root_position.1 - dragging_client.last_root_position.1,
        );

        let client_geometry = self.middle_executor.execute(Box::new(move |session| {
            Ok(ClientExecutor::new(session)
                .get_client_geometry(client)?
                .move_relative(diff_position.0, diff_position.1))
        }))?;

        self.middle_executor
            .execute_grabbed(Box::new(move |session| {
                ClientExecutor::new(session).apply_client_geometry(client, client_geometry)
            }))?;

        self.dragging_client = Some(DraggingClient {
            client,
            last_root_position: root_position,
        });
        Ok(())
    }

    fn handle_configure_request(
        &self,
        event: ConfigureRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let values: ConfigureWindowAux =
            ConfigureWindowAux::from_configure_request(&event).stack_mode(None);

        self.middle_executor.execute(Box::new(move |session| {
            session
                .connection()
                .configure_window(event.window, &values)?;
            Ok(())
        }))?;

        Ok(())
    }

    fn handle_map_request(
        &mut self,
        event: MapRequestEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let frame_values = CreateWindowAux::default()
            .event_mask(
                EventMask::BUTTON_PRESS
                    | EventMask::BUTTON_RELEASE
                    | EventMask::POINTER_MOTION
                    | EventMask::EXPOSURE,
            )
            .background_pixel(0x888888);

        let (frame, app_geometry) = self.middle_executor.execute(Box::new(move |session| {
            let frame = session.connection().generate_id()?;

            let original_geometry = session.connection().get_geometry(event.window)?.reply()?;

            let client_geometry = ClientGeometry::from_app(
                original_geometry.x as i32 + session.config().border_width as i32,
                original_geometry.y as i32
                    + session.config().titlebar_height as i32
                    + session.config().border_width as i32,
                original_geometry.width as u32,
                original_geometry.height as u32,
            );

            let app_geometry = client_geometry.parse_as_app(
                session.config().border_width,
                session.config().titlebar_height,
            );

            let frame_geometry = client_geometry.parse_as_frame(
                session.config().border_width,
                session.config().titlebar_height,
            );

            session.connection().create_window(
                COPY_DEPTH_FROM_PARENT,
                frame,
                session.screen().root,
                frame_geometry.x as i16,
                frame_geometry.y as i16,
                frame_geometry.width as u16,
                frame_geometry.height as u16,
                0,
                WindowClass::INPUT_OUTPUT,
                0,
                &frame_values,
            )?;
            Ok((frame, app_geometry))
        }))?;

        self.middle_executor
            .execute_grabbed(Box::new(move |session| {
                session.connection().configure_window(
                    event.window,
                    &ConfigureWindowAux::default()
                        .stack_mode(x11rb::protocol::xproto::StackMode::ABOVE)
                        .x(app_geometry.x)
                        .y(app_geometry.y)
                        .width(app_geometry.width)
                        .height(app_geometry.height),
                )?;

                session.connection().map_window(frame)?;
                session.connection().map_window(event.window)?;
                Ok(())
            }))?;

        // add client to container
        self.client_container.add_client(event.window, frame);

        Ok(())
    }
}
