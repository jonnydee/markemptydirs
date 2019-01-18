pub mod logger;

pub mod stdout;

pub use log::Level as LogLevel;

use Error;
use std::fmt::Debug;

pub trait Notifier: Debug + Sync {

    fn debug(&self, target:  &str, text:  &str, error: Option<Error>);

    fn error(&self, target:  &str, text:  &str, error: Option<Error>);

    fn get_log_level(&self) -> LogLevel;

    fn info(&self, target:  &str, text:  &str, error: Option<Error>);

    fn trace(&self, target:  &str, text:  &str, error: Option<Error>);

    fn warn(&self, target:  &str, text:  &str, error: Option<Error>);

}
