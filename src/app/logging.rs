//! Timestamped runtime logging helpers.

use std::fmt;

use chrono::Local;

/// Formats a log line with the reducer's timestamp prefix.
pub fn format_line(timestamp: &str, message: &str) -> String {
    format!("[{timestamp}] {message}")
}

/// Returns the current local timestamp used in runtime logs.
pub fn current_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Writes one timestamped log line to stdout.
pub fn info(args: fmt::Arguments<'_>) {
    println!("[{}] {}", current_timestamp(), args);
}

/// Writes one timestamped log line to stderr.
pub fn error(args: fmt::Arguments<'_>) {
    eprintln!("[{}] {}", current_timestamp(), args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_line_adds_timestamp_prefix() {
        assert_eq!(
            format_line("2026-06-29 13:45:07", "trial 1: accepted"),
            "[2026-06-29 13:45:07] trial 1: accepted"
        );
    }
}
