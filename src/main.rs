mod logger;
mod model;

fn main() {
    logger::setup_logging(Some("wm.log")).expect("Failed to initialize logging");
    log::info!("this is an info message");
}
