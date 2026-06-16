use chrono::Local;
use colored::Colorize;

/// Represent the severity levels of log messages.
///
/// The `LogLevel` enumeration defines various categories of log severity.
///
/// - `Trace`: The lowest level, typically used for tracing program flow.
/// - `Debug`: Used for development and debugging purposes.
/// - `Info`: General information messages concerning normal application operations.
/// - `Notice`: Used for significant events that are neither warnings nor errors.
/// - `Warning`: Used for warning messages that do not interrupt the application.
/// - `Error`: Used to report failures during operations that are not fatal.
/// - `Critical`: Represents severe conditions that may cause certain features to fail.
/// - `Alert`: Denotes critical situations requiring immediate human intervention.
/// - `Emergency`: The highest level, indicating that the system is unusable.
/// - `System`: Custom level for system-wide initialization and service events.
/// - `Operation`: Custom level for tracing operational actions.
/// - `Success`: Custom level for indicating successful execution of commands.
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
    System,
    Operation,
    Success,
}

impl LogLevel {
    /// Return the prefix string corresponding to the log level.
    ///
    /// This method returns a static string representing the prefix for the log level,
    /// which is used for formatting the log output.
    pub fn prefix(&self) -> &str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Notice => "NOTICE",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRITICAL",
            LogLevel::Alert => "ALERT",
            LogLevel::Emergency => "EMERGENCY",
            LogLevel::System => "SYSTEM",
            LogLevel::Operation => "OPERATION",
            LogLevel::Success => "SUCCESS",
        }
    }
    /// Apply color and style formatting to the log message.
    ///
    /// This method formats the MESSAGE string with colors and styles corresponding to
    /// the log level.
    pub fn colorize_message(&self, message: &str) -> colored::ColoredString {
        match self {
            LogLevel::Trace => message.dimmed(),
            LogLevel::Debug => message.blue(),
            LogLevel::Info => message.white(),
            LogLevel::Notice => message.magenta(),
            LogLevel::Warning => message.yellow(),
            LogLevel::Error => message.red(),
            LogLevel::Critical => message.red().bold(),
            LogLevel::Alert => message.red().bold().underline(),
            LogLevel::Emergency => message.red().bold().underline().on_white(),
            LogLevel::System => message.cyan(),
            LogLevel::Operation => message.green(),
            LogLevel::Success => message.green().bold(),
        }
    }
}

/// Logger utility.
///
/// This struct provides interfaces for recording log messages with varying severity levels.
pub struct Log;

impl Log {
    /// Record a log message.
    ///
    /// This method logs the MESSAGE under the specified LEVEL, automatically printing
    /// it with a timestamp and the level's prefix string.
    pub fn log(level: LogLevel, message: String) {
        let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let colored_message = level.colorize_message(&message);
        println!("[{}] {}: {}", timestamp, level.prefix(), colored_message);
    }
    /// Record a log message at the trace level.
    pub fn trace(message: String) {
        Log::log(LogLevel::Trace, message);
    }
    /// Record a log message at the debug level.
    pub fn debug(message: String) {
        Log::log(LogLevel::Debug, message);
    }
    /// Record a log message at the info level.
    pub fn info(message: String) {
        Log::log(LogLevel::Info, message);
    }
    /// Record a log message at the notice level.
    pub fn notice(message: String) {
        Log::log(LogLevel::Notice, message);
    }
    /// Record a log message at the warning level.
    pub fn warning(message: String) {
        Log::log(LogLevel::Warning, message);
    }

    /// Record a log message at the error level.
    pub fn error(message: String) {
        Log::log(LogLevel::Error, message);
    }

    /// Record a log message at the critical level.
    pub fn critical(message: String) {
        Log::log(LogLevel::Critical, message);
    }

    /// Record a log message at the alert level.
    pub fn alert(message: String) {
        Log::log(LogLevel::Alert, message);
    }

    /// Record a log message at the emergency level.
    pub fn emergency(message: String) {
        Log::log(LogLevel::Emergency, message);
    }

    /// Record a log message at the system level.
    pub fn system(message: String) {
        Log::log(LogLevel::System, message);
    }

    /// Record a log message at the operation level.
    pub fn operation(message: String) {
        Log::log(LogLevel::Operation, message);
    }

    /// Record a log message at the success level.
    pub fn success(message: String) {
        Log::log(LogLevel::Success, message);
    }
}

#[cfg(test)]
mod tests {
    use super::Log;

    #[test]
    fn test_logging() {
        Log::trace("This is a trace message.".to_string());
        Log::debug("This is a debug message.".to_string());
        Log::info("This is an info message.".to_string());
        Log::notice("This is a notice message.".to_string());
        Log::warning("This is a warning message.".to_string());
        Log::error("This is an error message.".to_string());
        Log::critical("This is a critical error message.".to_string());
        Log::alert("This is an alert message.".to_string());
        Log::emergency("This is an emergency message.".to_string());
        Log::system("System is starting.".to_string());
        Log::operation("Performing an operation.".to_string());
        Log::success("Operation was successful.".to_string());
    }
}
