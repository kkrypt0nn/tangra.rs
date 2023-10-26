use std::env;

pub struct Terminal;

impl Terminal {
    pub fn are_colors_supported() -> bool {
        let mut supported: bool = false;
        if env::var("TERM").is_ok() {
            supported = env::var("TERM").unwrap() != ""
        }
        if env::var("WT_SESSION").is_ok() {
            supported = env::var("WT_SESSION").unwrap() != ""
        }
        if env::var("SESSIONNAME").is_ok() {
            supported = env::var("SESSIONNAME").unwrap() == "Console"
        }
        supported
    }
}

pub struct Color;

impl Color {
    pub const RESET: &'static str = "\x1b[0m";

    pub const BLACK: &'static str = "\x1b[30m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const PURPLE: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";
    pub const GRAY: &'static str = "\x1b[90m";
    pub const BRIGHT_RED: &'static str = "\x1b[91m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    pub const BRIGHT_PURPLE: &'static str = "\x1b[95m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";
    pub const BRIGHT_WHITE: &'static str = "\x1b[97m";
}

pub struct Background;

impl Background {
    pub const RESET: &'static str = "\x1b[0m";

    pub const BLACK: &'static str = "\x1b[40m";
    pub const RED: &'static str = "\x1b[41m";
    pub const GREEN: &'static str = "\x1b[42m";
    pub const YELLOW: &'static str = "\x1b[43m";
    pub const BLUE: &'static str = "\x1b[44m";
    pub const PURPLE: &'static str = "\x1b[45m";
    pub const CYAN: &'static str = "\x1b[46m";
    pub const WHITE: &'static str = "\x1b[47m";
    pub const GRAY: &'static str = "\x1b[100m";
    pub const BRIGHT_RED: &'static str = "\x1b[101m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[102m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[103m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[104m";
    pub const BRIGHT_PURPLE: &'static str = "\x1b[105m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[106m";
    pub const BRIGHT_WHITE: &'static str = "\x1b[107m";
}

pub struct Effect;

impl Effect {
    pub const RESET: &'static str = "\x1b[0m";

    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const UNDERLINE: &'static str = "\x1b[4m";
    pub const BLINK: &'static str = "\x1b[5m";
    pub const INVERSE: &'static str = "\x1b[7m";
    pub const STRIKETHROUGH: &'static str = "\x1b[9m";
}
