use x11rb::protocol::xproto::ConnectionExt;

use super::session::X11Session;

pub struct MiddleExecutor<'a> {
    session: &'a X11Session,
}

pub type MiddleExecutorFunc<T> = Box<dyn Fn(&X11Session) -> Result<T, Box<dyn std::error::Error>>>;

impl<'a> MiddleExecutor<'a> {
    pub fn new(session: &'a X11Session) -> Self {
        Self { session }
    }

    pub fn execute<T>(&self, func: MiddleExecutorFunc<T>) -> Result<T, Box<dyn std::error::Error>> {
        func(self.session)
    }

    pub fn execute_grabbed<T>(
        &self,
        func: MiddleExecutorFunc<T>,
    ) -> Result<T, Box<dyn std::error::Error>> {
        self.session.connection().grab_server()?;
        match func(self.session) {
            Ok(result) => {
                self.session.connection().ungrab_server()?;
                Ok(result)
            }
            Err(err) => {
                self.session.connection().ungrab_server()?;
                Err(err)
            }
        }
    }
}
