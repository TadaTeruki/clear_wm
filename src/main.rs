use log::{error, info};
use wm::x11::window_manager::X11WindowManager;

use crate::{
    config::{FrameConfig, WindowManagerConfig},
    wm::x11::session::X11Session,
};

mod config;
mod logger;
mod model;
mod wm;

fn main() {
    logger::setup_logging(Some("wm.log")).expect("Failed to initialize logging");
    info!("Starting X11 window manager");
    let wmconfig = WindowManagerConfig {
        frame_config: FrameConfig {
            border_width: 18,
            titlebar_height: 20,
            corner_radius: 24,
        },
    };
    let session = X11Session::connect(wmconfig)
        .unwrap_or_else(|e| panic!("Failed to connect to X11 server: {}", e));

    X11WindowManager::new(&session)
        .start()
        .unwrap_or_else(|e| error!("Error: {}", e));
}
