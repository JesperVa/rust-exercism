// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", get_caps_name(level), message)
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

fn get_caps_name(level: LogLevel) -> String {
    match level {
        LogLevel::Error => "ERROR".to_string(),
        LogLevel::Info => "INFO".to_string(),
        LogLevel::Warning => "WARNING".to_string(),

    }
}
