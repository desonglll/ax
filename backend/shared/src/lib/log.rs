use chrono::Local;
use colored::Colorize;

/// 定义日志等级
///
/// LogLevel 枚举用于表示不同的日志等级。
///
/// - `Trace`：最低级别，通常用于跟踪程序的每一步执行，提供最详细的日志信息。
/// - `Debug`：用于开发和调试阶段，记录调试信息，便于开发者查找问题。
/// - `Info`：一般信息日志，记录应用的正常操作，如启动、停止或重要状态变化。
/// - `Notice`：记录一些重要的事件信息，但并非错误或警告。
/// - `Warning`：用于记录可能导致问题的警告信息，但并不影响程序的正常运行。
/// - `Error`：记录错误信息，表示程序在某些操作上失败，但通常不致命。
/// - `Critical`：表示严重错误，可能导致某些功能无法继续工作，需要立即关注。
/// - `Alert`：表示需要立即处理的紧急情况，通常用于需要人工介入的情况。
/// - `Emergency`：最高级别，表示系统无法正常运行，需要立即采取行动。
/// - `System`：自定义等级，通常用于系统级别的日志。
/// - `Operation`：自定义等级，表示某些操作过程中的日志信息。
/// - `Success`：自定义等级，用于记录成功的操作或状态。
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
    /// 返回日志等级的前缀字符串
    ///
    /// 返回不同日志等级对应的前缀，用于格式化日志输出。
    ///
    /// # Examples
    ///
    /// ```
    /// use shared::log::LogLevel;
    ///
    /// let level = LogLevel::Info;
    /// assert_eq!(level.prefix(), "INFO");
    /// ```
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
    /// 为日志消息应用颜色和样式
    ///
    /// 根据日志等级，为日志消息应用相应的颜色和样式。
    ///
    /// # Examples
    ///
    /// ```
    /// use shared::log::LogLevel;
    ///
    /// let level = LogLevel::Error;
    /// let message = "An error occurred".to_string();
    /// let colored_message = level.colorize_message(&message);
    /// assert_eq!(colored_message.to_string(), "An error occurred".red().to_string());
    /// ```
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

/// 日志记录器
///
/// 提供了日志记录的功能，可以记录不同等级的日志信息。
///
/// # Examples
///
/// ```
/// use shared::log::Log;
///
/// Log::info("This is an info message.".to_string());
/// ```
pub struct Log;

impl Log {
    /// 记录日志信息
    ///
    /// 根据日志等级记录日志消息，并输出时间戳和前缀。
    ///
    /// # Examples
    ///
    /// ```
    /// use shared::log::{Log, LogLevel};
    ///
    /// Log::log(LogLevel::Info, "This is an info message.".to_string());
    /// ```
    pub fn log(level: LogLevel, message: String) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let colored_message = level.colorize_message(&message);
        println!("[{}] {}: {}", timestamp, level.prefix(), colored_message);
    }
    /// 记录 trace 级别的日志
    pub fn trace(message: String) {
        Log::log(LogLevel::Trace, message);
    }
    /// 记录 debug 级别的日志
    pub fn debug(message: String) {
        Log::log(LogLevel::Debug, message);
    }
    /// 记录 info 级别的日志
    pub fn info(message: String) {
        Log::log(LogLevel::Info, message);
    }
    /// 记录 notice 级别的日志
    pub fn notice(message: String) {
        Log::log(LogLevel::Notice, message);
    }
    /// 记录 warning 级别的日志
    pub fn warning(message: String) {
        Log::log(LogLevel::Warning, message);
    }

    /// 记录 error 级别的日志
    pub fn error(message: String) {
        Log::log(LogLevel::Error, message);
    }

    /// 记录 critical 级别的日志
    pub fn critical(message: String) {
        Log::log(LogLevel::Critical, message);
    }

    /// 记录 alert 级别的日志
    pub fn alert(message: String) {
        Log::log(LogLevel::Alert, message);
    }

    /// 记录 emergency 级别的日志
    pub fn emergency(message: String) {
        Log::log(LogLevel::Emergency, message);
    }

    /// 记录 system 级别的日志
    pub fn system(message: String) {
        Log::log(LogLevel::System, message);
    }

    /// 记录 operation 级别的日志
    pub fn operation(message: String) {
        Log::log(LogLevel::Operation, message);
    }

    /// 记录 success 级别的日志
    pub fn success(message: String) {
        Log::log(LogLevel::Success, message);
    }
}

#[cfg(test)]
mod tests {
    use crate::log::Log;

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