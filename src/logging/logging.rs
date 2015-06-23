extern crate log;
extern crate time;

use log::{set_logger, LogRecord, LogLevel, LogMetadata, SetLoggerError, LogLevelFilter};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let module_path = &record.location().module_path();
            let line_number = &record.location().line();
            let filename = &record.location().file();
            println!("{time} {level} [{module_path}:{filename}:{line_number}] - {args}",
                     time = now(), level = record.level(), args = record.args(),
                     module_path = module_path, filename = filename, line_number = line_number);
        }
    }
}

fn now() -> String {
    return time::strftime("%F %T", &time::now()).unwrap();
}

pub fn init() -> Result<(), SetLoggerError> {
    set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(SimpleLogger)
    })
}
