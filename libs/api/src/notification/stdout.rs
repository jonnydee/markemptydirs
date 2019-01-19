use super::Error;
use super::LogLevel;
use super::Notifier;

pub struct StdoutNotifier {
    log_level: LogLevel,
}

impl StdoutNotifier {
    pub fn create(log_level: LogLevel) -> Box<Notifier> {
        Box::new(StdoutNotifier {
            log_level: log_level,
        })
    }

    fn notify(&self, log_level: LogLevel, target: &str, text: &str, error: Option<Error>) {
        if self.log_level < log_level {
            return;
        }

        let msg = if let Some(err) = error {
            format!("{}: {}: {}", target, text, err)
        } else {
            format!("{}: {}", target, text)
        };

        println!("{}", msg);
    }
}

impl std::fmt::Debug for StdoutNotifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stdout {{ log_level: {} }}", self.log_level)
    }
}

impl Notifier for StdoutNotifier {
    fn debug(&self, target: &str, text: &str, error: Option<Error>) {
        self.notify(LogLevel::Debug, target, text, error);
    }

    fn error(&self, target: &str, text: &str, error: Option<Error>) {
        self.notify(LogLevel::Error, target, text, error);
    }

    fn get_log_level(&self) -> LogLevel {
        self.log_level
    }

    fn info(&self, target: &str, text: &str, error: Option<Error>) {
        self.notify(LogLevel::Info, target, text, error);
    }

    fn trace(&self, target: &str, text: &str, error: Option<Error>) {
        self.notify(LogLevel::Trace, target, text, error);
    }

    fn warn(&self, target: &str, text: &str, error: Option<Error>) {
        self.notify(LogLevel::Warn, target, text, error);
    }
}
