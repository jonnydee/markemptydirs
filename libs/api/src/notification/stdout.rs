use super::Error;
use super::Notifier;
use super::{LogLevel, MessageLength};

pub struct StdoutNotifier {
    pub log_level: LogLevel,
    pub message_length: MessageLength,
}

impl StdoutNotifier {
    pub fn create(log_level: LogLevel, message_length: MessageLength) -> Box<Notifier> {
        Box::new(StdoutNotifier {
            log_level: log_level,
            message_length: message_length,
        })
    }
}

impl std::fmt::Debug for StdoutNotifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stdout {{ log_level: {} }}", self.log_level)
    }
}

impl Notifier for StdoutNotifier {
    fn get_log_level(&self) -> LogLevel {
        self.log_level
    }

    fn notify(
        &self,
        log_level: LogLevel,
        target: &str,
        info: &str,
        data: &str,
        error: Option<Error>,
    ) {
        if self.log_level < log_level {
            return;
        }

        let msg = if let Some(err) = error {
            match self.message_length {
                MessageLength::Short => format!("{}: {}", info, data),
                MessageLength::Long => format!("[{}] {}: {}: {} ({})", log_level, target, info, data, err),
            }
        } else {
            match self.message_length {
                MessageLength::Short => format!("{}", data),
                MessageLength::Long => format!("[{}] {}: {}: {}", log_level, target, info, data),
            }
        };

        println!("{}", msg);
    }
}
