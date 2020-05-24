use log::LevelFilter;
use termcolor::Color;

#[derive(Debug)]
pub struct ConsoleConfig {
    pub log_level: LevelFilter,
    pub warn_color: Color,
    pub info_color: Color,
    pub debug_color: Color,
}

impl ConsoleConfig {
    pub fn default() -> ConsoleConfig {
        ConsoleConfig {
            log_level: LevelFilter::Off,
            warn_color: Color::Red,
            info_color: Color::White,
            debug_color: Color::Green,
        }
    }

    pub fn warn_color(mut self, color: Color) -> ConsoleConfig {
        self.warn_color = color;
        self
    }

    pub fn info_color(mut self, color: Color) -> ConsoleConfig {
        self.info_color = color;
        self
    }

    pub fn debug_color(mut self, color: Color) -> ConsoleConfig {
        self.debug_color = color;
        self
    }

    pub fn log_level(mut self, log_level: LevelFilter) -> ConsoleConfig {
        self.log_level = log_level;
        self
    }
}
