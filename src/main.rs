use log::{error, info};
use wm::x11::X11WindowManager;

use crate::model::config::WindowManagerConfig;

mod logger;
mod model;
mod wm;

fn main() {
    logger::setup_logging(Some("wm.log")).expect("Failed to initialize logging");
    info!("Starting X11 window manager");
    let wmconfig = WindowManagerConfig::default();
    X11WindowManager::new(wmconfig)
        .start()
        .unwrap_or_else(|e| error!("Error: {}", e));
}
