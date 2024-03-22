use log::{error, info};
use wm::x11::window_manager::X11WindowManager;

use crate::{
    config::WindowManagerConfig,
    wm::x11::{cairo::CairoSession, session::X11Session},
};

mod config;
mod logger;
mod model;
mod wm;

fn main() {
    logger::setup_logging(Some("wm.log")).expect("Failed to initialize logging");
    info!("Starting X11 window manager");
    let wmconfig = WindowManagerConfig::default();

    let session = X11Session::connect(wmconfig)
        .unwrap_or_else(|e| panic!("Failed to connect to X11 server: {}", e));

    let cairo_session = CairoSession::connect(session.connection(), session.screen_num())
        .unwrap_or_else(|e| panic!("Failed to initialize cairo session: {}", e));

    X11WindowManager::new(&session, cairo_session)
        .unwrap_or_else(|e| panic!("Failed to initialize window manager: {}", e))
        .start()
        .unwrap_or_else(|e| error!("Error: {}", e));
}
