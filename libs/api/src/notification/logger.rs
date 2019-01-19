use super::Error;
use super::Notifier;
use super::{LogLevel, MessageLength};

#[derive(PartialEq, Debug)]
pub struct LoggerNotifier {
    pub log_level: LogLevel,
    pub message_length: MessageLength,
}

impl LoggerNotifier {
    pub fn create(log_level: LogLevel, message_length: MessageLength) -> Box<Notifier> {
        Box::new(LoggerNotifier {
            log_level: log_level,
            message_length: message_length,
        })
    }
}

impl Notifier for LoggerNotifier {
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
                MessageLength::Long => format!("{}: {} ({})", info, data, err),
            }
        } else {
            match self.message_length {
                MessageLength::Short => format!("{}", data),
                MessageLength::Long => format!("{}: {}", info, data),
            }
        };

        match log_level {
            LogLevel::Error => error!(target: &target, "{}", &msg),
            LogLevel::Warn => warn!(target: &target, "{}", &msg),
            LogLevel::Info => info!(target: &target, "{}", &msg),
            LogLevel::Debug => debug!(target: &target, "{}", &msg),
            LogLevel::Trace => trace!(target: &target, "{}", &msg),
        }
    }
}
