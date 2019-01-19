use super::Error;
use super::LogLevel;
use super::Notifier;

#[derive(PartialEq, Debug)]
pub struct Logger {
    pub log_level: LogLevel,
}

impl Notifier for Logger {
    fn debug(&self, target: &str, text: &str, error: Option<Error>) {
        if self.log_level < LogLevel::Debug {
            return;
        }

        if let Some(err) = error {
            debug!(target: &target, "{}: {}", err, text)
        } else {
            debug!(target: &target, "{}", text)
        }
    }

    fn error(&self, target: &str, text: &str, error: Option<Error>) {
        if self.log_level < LogLevel::Error {
            return;
        }

        if let Some(err) = error {
            error!(target: &target, "{}: {}", err, text)
        } else {
            error!(target: &target, "{}", text)
        }
    }

    fn get_log_level(&self) -> LogLevel {
        self.log_level
    }

    fn info(&self, target: &str, text: &str, error: Option<Error>) {
        if self.log_level < LogLevel::Info {
            return;
        }

        if let Some(err) = error {
            info!(target: &target, "{}: {}", err, text)
        } else {
            info!(target: &target, "{}", text)
        }
    }

    fn trace(&self, target: &str, text: &str, error: Option<Error>) {
        if self.log_level < LogLevel::Trace {
            return;
        }

        if let Some(err) = error {
            trace!(target: &target, "{}: {}", err, text)
        } else {
            trace!(target: &target, "{}", text)
        }
    }

    fn warn(&self, target: &str, text: &str, error: Option<Error>) {
        if self.log_level < LogLevel::Warn {
            return;
        }

        if let Some(err) = error {
            warn!(target: &target, "{}: {}", err, text)
        } else {
            warn!(target: &target, "{}", text)
        }
    }
}
