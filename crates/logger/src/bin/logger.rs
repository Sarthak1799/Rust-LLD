use std::sync::Arc;

use logger::{
    logger::Logger,
    logger_types::{FileLogger, LogLevel},
};
fn main() {
    let mut logger = Logger::new(None);

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    logger
        .log_message(&input.trim())
        .expect("Failed to log message");

    logger.set_log_level(LogLevel::Error);

    input.clear();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    logger
        .log_message(&input.trim())
        .expect("Failed to log message");

    let file_logger = FileLogger::new("log.txt".to_string());

    logger.set_output(Arc::new(file_logger));
    logger.set_log_level(LogLevel::Debug);

    input.clear();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    logger
        .log_message(&input.trim())
        .expect("Failed to log message");
}
