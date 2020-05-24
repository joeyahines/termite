pub mod console_config;
pub mod file_config;

use chrono::offset::Local;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::cmp::Ordering;
use std::fs::{read_dir, remove_file, rename, DirEntry, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;
use std::sync::Mutex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub use console_config::ConsoleConfig;
pub use file_config::FileConfig;

type TermiteLogResult = Result<(), TermiteLogError>;

pub enum TermiteLogError {
    FileError(std::io::Error),
}

impl std::convert::From<std::io::Error> for TermiteLogError {
    fn from(e: Error) -> Self {
        TermiteLogError::FileError(e)
    }
}

#[derive(Debug)]
pub struct TermiteConfig {
    pub global_log_level: LevelFilter,
    pub console_config: Option<ConsoleConfig>,
    pub file_config: Option<FileConfig>,
    pub log_time: bool,
    pub log_date: bool,
    pub log_path: bool,
}

impl TermiteConfig {
    pub fn default() -> TermiteConfig {
        TermiteConfig {
            global_log_level: LevelFilter::Off,
            console_config: Some(ConsoleConfig::default()),
            file_config: None,
            log_time: false,
            log_date: false,
            log_path: false,
        }
    }

    pub fn console_config(mut self, config: ConsoleConfig) -> TermiteConfig {
        self.console_config = Some(config);
        self
    }

    pub fn file_config(mut self, config: FileConfig) -> TermiteConfig {
        self.file_config = Some(config);
        self
    }

    pub fn log_level(mut self, log_level: LevelFilter) -> TermiteConfig {
        self.global_log_level = log_level;
        self
    }

    pub fn log_time(mut self, set: bool) -> TermiteConfig {
        self.log_time = set;
        self
    }

    pub fn log_date(mut self, set: bool) -> TermiteConfig {
        self.log_date = set;
        self
    }

    pub fn log_path(mut self, set: bool) -> TermiteConfig {
        self.log_path = set;
        self
    }
}

#[derive(Debug)]
pub struct Termite {
    config: TermiteConfig,
    output_lock: Mutex<()>,
}

impl Termite {
    pub fn new(config: TermiteConfig) -> Box<Termite> {
        Box::new(Termite {
            config,
            output_lock: Mutex::new(()),
        })
    }

    pub fn init(config: TermiteConfig) -> Result<(), SetLoggerError> {
        let termite = Termite::new(config);
        log::set_max_level(termite.config.global_log_level);
        log::set_boxed_logger(termite)?;
        Ok(())
    }

    pub fn log_to_console(&self, log_msg: &str, record: &Record) -> Result<(), TermiteLogError> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        if let Some(console_config) = &self.config.console_config {
            let mut color_spec = ColorSpec::new();
            stdout.set_color(match record.level() {
                Level::Info => color_spec.set_fg(Some(console_config.info_color)),
                Level::Debug => color_spec.set_fg(Some(console_config.info_color)),
                Level::Warn => color_spec.set_fg(Some(console_config.info_color)),
                _ => color_spec.set_fg(Some(Color::White)),
            })?;
            writeln!(&mut stdout, "{}", log_msg)?;
        }

        Ok(())
    }

    pub fn log_to_file(&self, log_msg: &str, record: &Record) -> TermiteLogResult {
        if let Some(file_config) = &self.config.file_config {
            if record.level() <= file_config.log_level {}
            let path = Path::new(&file_config.path);
            let log_path = path.join(&file_config.log_name);
            let file_size = {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&log_path)?;

                writeln!(file, "{}", log_msg)?;
                file.metadata()?.len()
            };

            if file_size > file_config.max_file_size {
                let time_str = Local::now().format("%F.%I:%M:%S%p").to_string();
                let new_name = path.join(format!("{}-{}", time_str, file_config.log_name));
                rename(&log_path, &new_name)?;

                let mut log_files: Vec<DirEntry> = Vec::new();
                for file in read_dir(path)? {
                    let file = file?;

                    if file.file_type()?.is_file() {
                        let file_name = file.file_name();
                        let file_name = match file_name.to_str() {
                            Some(str) => str,
                            None => continue,
                        };

                        if file_name.contains(&file_config.log_name) {
                            log_files.push(file);
                        }
                    }
                }

                if log_files.len() > file_config.logs_to_keep as usize {
                    log_files.sort_by(|a, b| {
                        let a_name = a.file_name();
                        let b_name = b.file_name();

                        let a_name = a_name.to_str().unwrap();
                        let b_name = b_name.to_str().unwrap();

                        let a_time_str =
                            a_name.replace(format!("-{}", &file_config.log_name).as_str(), "");
                        let b_time_str =
                            b_name.replace(format!("-{}", &file_config.log_name).as_str(), "");

                        if a_time_str.len() == 0 {
                            Ordering::Greater
                        } else if b_time_str.len() == 0 {
                            Ordering::Less
                        } else {
                            let a_time = chrono::NaiveDateTime::parse_from_str(
                                a_time_str.as_str(),
                                "%F.%I:%M:%S%p",
                            )
                            .unwrap();
                            let b_time = chrono::NaiveDateTime::parse_from_str(
                                b_time_str.as_str(),
                                "%F.%I:%M:%S%p",
                            )
                            .unwrap();

                            b_time.cmp(&a_time)
                        }
                    });

                    if let Some(oldest_log) = log_files.pop() {
                        remove_file(oldest_log.path())?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Log for Termite {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.config.global_log_level
    }

    fn log(&self, record: &Record) {
        let _lock = self.output_lock.lock().unwrap();
        if self.enabled(record.metadata()) {
            let time = Local::now();
            let time_str = time.format("%F.%I:%M:%S%p").to_string();
            let mut msg = String::new();

            if self.config.log_date {
                msg.push_str(&time_str);
                msg.push_str(" ");
            }
            if self.config.log_path {
                msg.push_str(record.module_path().unwrap());
                msg.push_str(" ");
            }
            msg.push_str(&record.level().to_string());
            msg.push_str("::");

            msg.push_str(&format!("{:?}", record.args()));

            self.log_to_console(msg.as_str(), record).ok();
            self.log_to_file(msg.as_str(), record).ok();
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::{ConsoleConfig, Termite, TermiteConfig};
    use log::{info, warn, LevelFilter};
    use std::io::{self, Write};
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    #[test]
    fn do_log() {
        let console_config = ConsoleConfig::default()
            .log_level(LevelFilter::Info)
            .info_color(Color::Green);

        let log_config = TermiteConfig::new(LevelFilter::Info, Some(console_config));
        Termite::init(log_config);

        info!("A nice test");
    }
}
