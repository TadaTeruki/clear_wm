use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;
use std::{fs::File, io};

pub fn setup_logging(log_file_name: Option<&str>) -> Result<(), fern::InitError> {
    let base_config = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[ {} ] | {} | ( at {} ) - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info);

    let final_config = if let Some(file) = log_file_name {
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
