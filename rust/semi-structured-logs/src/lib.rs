// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    #[cfg(feature = "add-a-variant")]
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => format!("[INFO]: {}", message),
        LogLevel::Warning => format!("[WARNING]: {}", message),
        LogLevel::Error => format!("[ERROR]: {}", message),
        #[cfg(feature = "add-a-variant")]
        LogLevel::Debug => format!("[DEBUG]: {}", message),
    }
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
#[cfg(feature = "add-a-variant")]
pub fn debug(message: &str) -> String {
    log(LogLevel::Debug, message)
}
