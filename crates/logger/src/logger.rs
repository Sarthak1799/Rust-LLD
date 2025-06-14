use super::{config, logger_types as types};

pub type LoggerResult<T> = Result<T, types::LoggingError>;
pub struct Logger {
    config: config::LoggerConfig,
}

impl Logger {
    pub fn new(config: Option<config::LoggerConfig>) -> Self {
        let config = config.unwrap_or_default();
        Logger { config }
    }

    pub fn log_message(&self, message: &str) -> LoggerResult<()> {
        let level = &self.config.log_level;
        let output = self.config.output.clone();
        output.log(level, message)
    }
    pub fn set_log_level(&mut self, log_level: types::LogLevel) {
        self.config.set_log_level(log_level);
    }
    pub fn set_output(&mut self, output: std::sync::Arc<dyn types::LoggingInterface>) {
        self.config.set_output(output);
    }
}
