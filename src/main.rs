use model::client::{container::ClientContainer};
use x11rb::protocol::xproto::Window;

mod logger;
mod model;

fn main() {
    logger::setup_logging(Some("wm.log")).expect("Failed to initialize logging");
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let clicont: ClientContainer<Window> = ClientContainer::new();
}
