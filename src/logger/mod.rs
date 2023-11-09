use chrono::Local;
use fern::Dispatch;
use log::{Level, LevelFilter};
use std::{fs::File, io};

pub fn setup_logging(log_file: Option<&str>) -> Result<(), fern::InitError> {
    const WM_LOGGER_STYLE_ERROR: &str = "\x1b[1;31m";
    const WM_LOGGER_STYLE_WARN: &str = "\x1b[1;33m";
    const WM_LOGGER_STYLE_DEFAULT: &str = "\x1b[1;38m";
    const WM_LOGGER_STYLE_END: &str = "\x1b[0m";

    let base_config = Dispatch::new()
        .format(|out, message, record| {
            let level_style = match record.level() {
                Level::Error => WM_LOGGER_STYLE_ERROR,
                Level::Warn => WM_LOGGER_STYLE_WARN,
                _ => WM_LOGGER_STYLE_DEFAULT,
            };

            out.finish(format_args!(
                "[ {} ] {}| {} |{} ( at {} ) - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_style,
                record.level(),
                WM_LOGGER_STYLE_END,
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info);

    let final_config = if let Some(file) = log_file {
        let log_file = File::create(file)?;
        base_config.chain(log_file)
    } else {
        base_config.chain(io::stdout())
    };

    final_config.apply()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{error, info, warn};

    #[test]
    fn test_logging() {
        setup_logging(None).expect("Failed to initialize logging");

        info!("this is an info message");
        warn!("this is a warning message");
        error!("this is an error message");
    }
}
