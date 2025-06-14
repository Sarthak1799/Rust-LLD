use std::sync::Arc;

use super::logger_types::{ConsoleLogger, LogLevel, LoggingInterface};
const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

pub struct LoggerConfig {
    pub log_level: LogLevel,
    pub output: Arc<dyn LoggingInterface>,
}

impl LoggerConfig {
    pub fn new(log_level: LogLevel, output: Arc<dyn LoggingInterface>) -> Self {
        LoggerConfig { log_level, output }
    }
    pub fn set_log_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }
    pub fn set_output(&mut self, output: Arc<dyn LoggingInterface>) {
        self.output = output;
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig {
            log_level: DEFAULT_LOG_LEVEL,
            output: Arc::new(ConsoleLogger::new()),
        }
    }
}
