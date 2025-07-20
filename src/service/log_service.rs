use crate::environment::server_config::SERVER_CONFIG;

pub fn log(level: LogType, message: &str) {
    if is_allowed_log(level.clone()) {
        let level_text = level.to_text();
        let spaces = " ".repeat(8 - level_text.0);
        println!(
            "{}:{}: {}",
            level_text.1,
            spaces,
            message
        );
    }
}

#[derive(PartialEq, Clone)]
pub enum LogType {
    INFO,
    DEBUG
}

impl LogType {
    pub fn to_text(&self) -> (usize, String) {
        match self {
            LogType::INFO => (8, "[ \x1b[32mINFO\x1b[0m ]".to_string()),
            LogType::DEBUG => (8, "[ DEBG ]".to_string()),
        }
    }
}

fn is_allowed_log(level: LogType) -> bool {
    if SERVER_CONFIG.LOG_LEVEL == "ALL" {
        true
    } else if SERVER_CONFIG.LOG_LEVEL == "DEBUG" {
        level == LogType::INFO
            || level == LogType::DEBUG
    } else if SERVER_CONFIG.LOG_LEVEL == "NONE" {
        false
    } else {
        level == LogType::INFO
    }
}