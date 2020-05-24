use log::LevelFilter;

#[derive(Debug)]
pub struct FileConfig {
    pub log_level: LevelFilter,
    pub path: String,
    pub max_file_size: u64,
    pub logs_to_keep: u32,
    pub log_name: String,
}

impl FileConfig {
    pub fn default() -> FileConfig {
        FileConfig {
            log_level: LevelFilter::Off,
            path: String::from("."),
            max_file_size: 1024 * 1024,
            logs_to_keep: 1,
            log_name: "log.log".to_string(),
        }
    }

    pub fn max_file_size(mut self, file_size: u64) -> FileConfig {
        self.max_file_size = file_size;
        self
    }

    pub fn logs_to_keep(mut self, log_count: u32) -> FileConfig {
        self.logs_to_keep = log_count;
        self
    }

    pub fn log_name(mut self, log_name: &str) -> FileConfig {
        self.log_name = log_name.to_string();
        self
    }

    pub fn log_level(mut self, log_level: LevelFilter) -> FileConfig {
        self.log_level = log_level;
        self
    }
}
