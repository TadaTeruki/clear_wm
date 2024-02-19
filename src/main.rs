use model::client::container::ClientContainer;
use wm::WindowManager;
use x11rb::protocol::xproto::Window;

mod logger;
mod model;
mod wm;

fn main() {
    logger::setup_logging(Some("wm.log")).expect("Failed to initialize logging");
    let clicont: ClientContainer<Window> = ClientContainer::new();
    let wm = WindowManager::new(clicont);
    wm.start().expect("Failed to start window manager");
}
