use crate::levels::Level::{DEBUG, ERROR, FATAL, INFO, TRACE, WARN};
use crate::terminal;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum Level {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
    NONE = 1337,
}

impl Level {
    pub fn get_level_color(level: Level, force: bool) -> String {
        if terminal::Terminal::are_colors_supported() || force {
            return match level {
                TRACE => terminal::Color::GRAY.to_string(),
                DEBUG => format!("{}{}", terminal::Color::GRAY, terminal::Effect::BOLD),
                INFO => format!("{}{}", terminal::Color::BLUE, terminal::Effect::BOLD),
                WARN => format!("{}{}", terminal::Color::YELLOW, terminal::Effect::BOLD),
                ERROR => format!("{}{}", terminal::Color::RED, terminal::Effect::BOLD),
                FATAL => format!(
                    "{}{}{}",
                    terminal::Color::BRIGHT_WHITE,
                    terminal::Effect::BOLD,
                    terminal::Background::RED
                ),
                _ => "".to_string(),
            };
        }
        "".to_string()
    }

    pub fn get_level_name(level: Level) -> String {
        match level {
            TRACE => "TRACE".to_string(),
            DEBUG => "DEBUG".to_string(),
            INFO => "INFO".to_string(),
            WARN => "WARN".to_string(),
            ERROR => "ERROR".to_string(),
            FATAL => "FATAL".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn get_short_level_name(level: Level) -> String {
        match level {
            TRACE => "tra".to_string(),
            DEBUG => "dbg".to_string(),
            INFO => "inf".to_string(),
            WARN => "war".to_string(),
            ERROR => "err".to_string(),
            FATAL => "fat".to_string(),
            _ => "".to_string(),
        }
    }
}
