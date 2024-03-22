use x11rb::{
    connection::Connection,
    protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt, EventMask},
};

use super::{handler::Handler, session::X11Session};

/// X11WindowManager performs the main event loop and dispatches events to the handler.
pub struct X11WindowManager<'a> {
    session: &'a X11Session,
    handler: Handler<'a>,
}

impl<'a> X11WindowManager<'a> {
    pub fn new(session: &'a X11Session) -> Self {
        Self {
            session,
            handler: Handler::new(session),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let root_values = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY);

        self.session
            .connection()
            .change_window_attributes(self.session.screen().root, &root_values)?
            .check()?;

        loop {
            self.session.connection().flush()?;
            let mut event_option = Some(self.session.connection().wait_for_event()?);
            while let Some(event) = event_option {
                self.handler.handle_event(event)?;
                event_option = self.session.connection().poll_for_event()?;
            }
            self.handler.flush_queued()?;
        }
    }
}
