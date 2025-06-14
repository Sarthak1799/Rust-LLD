#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Error,
}
impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Error => "ERROR",
        }
    }
}
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
    pub created_at: std::time::SystemTime,
}

impl LogMessage {
    pub fn new(level: &LogLevel, message: String) -> Self {
        LogMessage {
            level: level.clone(),
            message,
            created_at: std::time::SystemTime::now(),
        }
    }

    pub fn as_str(&self) -> String {
        format!(
            "[{:?}] - [{}] - {}",
            self.created_at
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap(),
            self.level.as_str(),
            self.message
        )
    }
}

#[derive(Debug)]
pub enum LoggingError {
    IoError(std::io::Error),
    InternalError(String),
}

pub trait LoggingInterface {
    fn log(&self, level: &LogLevel, message: &str) -> Result<(), LoggingError>;
}

#[derive(Debug)]
pub struct ConsoleLogger;
impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger
    }
}
impl LoggingInterface for ConsoleLogger {
    fn log(&self, level: &LogLevel, message: &str) -> Result<(), LoggingError> {
        let log = LogMessage::new(level, message.to_string());
        println!("{}", log.as_str());
        Ok(())
    }
}

#[derive(Debug)]
pub struct FileLogger {
    file_path: String,
}
impl FileLogger {
    pub fn new(file_path: String) -> Self {
        FileLogger { file_path }
    }
}

impl LoggingInterface for FileLogger {
    fn log(&self, level: &LogLevel, message: &str) -> Result<(), LoggingError> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(|err| LoggingError::IoError(err))?;

        let log = LogMessage::new(level, message.to_string()).as_str();
        let log_with_newline = format!("{}\n", log);
        let mut buf = log_with_newline.as_bytes();

        file.write_all(&mut buf)
            .map_err(|err| LoggingError::IoError(err))
    }
}
