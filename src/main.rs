use log::{error, info};
use model::client::container::ClientContainer;
use wm::WindowManager;
use x11rb::protocol::xproto::Window;

mod logger;
mod model;
mod wm;

fn main() {
    logger::setup_logging(Some("wm.log")).expect("Failed to initialize logging");
    info!("Starting window manager");
    let clicont: ClientContainer<Window> = ClientContainer::new();
    let wmconfig = wm::WindowManagerConfig::default();
    WindowManager::new(clicont, wmconfig)
        .start()
        .unwrap_or_else(|e| error!("Error: {}", e));
}
