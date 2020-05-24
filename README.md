# Termite
Make easy work of your ~~wooden~~ logs.

A simple logging implementation for Rust. Supports logging to console and to file. This library is designed 
for ease of use before performance and probably should not be used in anything too serious.

## Example
A more complete example can be found in `examples`.
```rust
use log::{info, LevelFilter};
use termite::{ConsoleConfig, FileConfig, Termite, TermiteConfig};

fn main() {
    // Setup console_config
    let console_config = ConsoleConfig::default()
        .log_level(LevelFilter::Info)
        .info_color(Color::Green);

    // Setup log config
    let log_config = TermiteConfig::default()
        .console_config(console_config)
        .log_level(LevelFilter::Debug)
        .log_path(true)
        .log_date(true)
        .log_time(true);

    // Start logger
    Termite::init(log_config).unwrap();
    
    info!("Logging to console!")
}
```