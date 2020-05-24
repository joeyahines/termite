use log::{info, warn, LevelFilter};
use std::thread::sleep;
use std::time::Duration;
use termcolor::Color;
use termite::{ConsoleConfig, FileConfig, Termite, TermiteConfig};

fn main() {
    // Setup console_config
    let console_config = ConsoleConfig::default()
        .log_level(LevelFilter::Info)
        .info_color(Color::Green);

    // Setup file logging
    let file_config = FileConfig::default()
        .log_level(LevelFilter::Info)
        .log_name("example.log")
        .logs_to_keep(5)
        .max_file_size(1024);

    // Setup log config
    let log_config = TermiteConfig::default()
        .console_config(console_config)
        .file_config(file_config)
        .log_level(LevelFilter::Debug)
        .log_path(true)
        .log_date(true)
        .log_time(true);

    // Start logger
    Termite::init(log_config).unwrap();

    // Test logger
    for i in 0..10 {
        for j in 0..1024 {
            info!("{}", i * 1024 + j);
        }

        sleep(Duration::from_secs(1));
    }
}
