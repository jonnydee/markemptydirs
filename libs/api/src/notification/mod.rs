mod logger;
pub use self::logger::*;

mod stdout;
pub use self::stdout::*;

pub use log::Level as LogLevel;

use std::fmt::Debug;
use Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageLength {
    Short,
    Long,
}

pub trait Notifier: Debug + Sync {
    fn get_log_level(&self) -> LogLevel;

    fn notify(
        &self,
        log_level: LogLevel,
        target: &str,
        info: &str,
        data: &str,
        error: Option<Error>,
    );

    fn debug(&self, target: &str, info: &str, data: &str, error: Option<Error>) {
        self.notify(LogLevel::Debug, target, info, data, error);
    }

    fn error(&self, target: &str, info: &str, data: &str, error: Option<Error>) {
        self.notify(LogLevel::Error, target, info, data, error);
    }

    fn info(&self, target: &str, info: &str, data: &str, error: Option<Error>) {
        self.notify(LogLevel::Info, target, info, data, error);
    }

    fn trace(&self, target: &str, info: &str, data: &str, error: Option<Error>) {
        self.notify(LogLevel::Trace, target, info, data, error);
    }

    fn warn(&self, target: &str, info: &str, data: &str, error: Option<Error>) {
        self.notify(LogLevel::Warn, target, info, data, error);
    }
}
