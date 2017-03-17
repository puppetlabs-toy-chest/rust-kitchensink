use rust_log::{self, LogRecord, LogMetadata, LogLevelFilter, SetLoggerError};
use std::io::Write;

struct SimpleLogger;

impl rust_log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= rust_log::LogLevel::Trace
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println_stderr!("{} - {}", record.level(), record.args());
        }
    }
}

#[derive(RustcDecodable, Debug)]
pub enum LogLevel { None, Trace, Debug, Info, Warn, Error }

pub fn init_logging(level: LogLevel) {
    let filter = match level {
        LogLevel::None => LogLevelFilter::Off,
        LogLevel::Trace => LogLevelFilter::Trace,
        LogLevel::Debug => LogLevelFilter::Debug,
        LogLevel::Info => LogLevelFilter::Info,
        LogLevel::Warn => LogLevelFilter::Warn,
        LogLevel::Error => LogLevelFilter::Error,
    };
    init(filter).unwrap_or_else(|e| pretty_panic!("Failed to initialize logging: {}", e));
}

pub fn init(level: rust_log::LogLevelFilter) -> Result<(), SetLoggerError> {
    rust_log::set_logger(|max_log_level| {
        max_log_level.set(level);
        Box::new(SimpleLogger)
    })
}
