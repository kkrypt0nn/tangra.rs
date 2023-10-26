extern crate chrono;
extern crate whoami;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::panic::Location;

use chrono::Local;

use crate::levels::Level;

mod constants;
pub mod levels;
pub mod terminal;

#[derive(Debug)]
pub struct Logger {
    prefix: String,
    styling: bool,
    log_file: Option<File>,
    force_styling: bool,
    date_format: String,
    datetime_format: String,
    time_format: String,
    current_logging_level: Level,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            prefix: "$[datetime] $[level:color]$[level:name]$[reset]: ".to_string(),
            styling: true,
            log_file: None,
            force_styling: false,
            date_format: "%b %d, %Y".to_string(),
            datetime_format: "%b %d, %Y %H:%M:%S".to_string(),
            time_format: "%H:%M:%S".to_string(),
            current_logging_level: Level::NONE,
        }
    }

    pub fn new_with_prefix(prefix: impl Into<String>) -> Self {
        Logger {
            prefix: prefix.into(),
            ..Logger::default()
        }
    }

    #[track_caller]
    fn variables(&self) -> HashMap<&'static str, String> {
        let caller = Location::caller();
        let split_path = caller
            .file()
            .split(std::path::MAIN_SEPARATOR)
            .collect::<Vec<&str>>();
        let file = if !split_path.is_empty() {
            split_path.last().unwrap()
        } else {
            ""
        };
        HashMap::from([
            // Caller
            ("$[caller:line]", caller.line().to_string()),
            ("$[caller:file]", file.to_string()),
            // Logging Level
            (
                "$[level:color]",
                Level::get_level_color(self.current_logging_level, self.force_styling),
            ),
            (
                "$[level:lowername]",
                Level::get_level_name(self.current_logging_level).to_lowercase(),
            ),
            (
                "$[level:name]",
                Level::get_level_name(self.current_logging_level),
            ),
            (
                "$[level:shortname]",
                Level::get_short_level_name(self.current_logging_level),
            ),
            // Date & Time Now
            (
                "$[now:date]",
                Local::now().format(&self.date_format).to_string(),
            ),
            (
                "$[now:datetime]",
                Local::now().format(&self.datetime_format).to_string(),
            ),
            (
                "$[now:time]",
                Local::now().format(&self.time_format).to_string(),
            ),
            // System
            ("$[sys:architecture]", whoami::arch().to_string()),
            ("$[sys:hostname]", whoami::hostname().to_string()),
            ("$[sys:operating_system]", whoami::devicename().to_string()),
            ("$[sys:username]", whoami::username().to_string()),
            ("$[sys:username]", whoami::username().to_string()),
        ])
    }

    pub fn set_prefix(&mut self, prefix: &'static str) {
        self.prefix = prefix.to_string();
    }

    pub fn set_styling(&mut self, styling: bool) {
        self.styling = styling;
    }

    pub fn set_logging_level(&mut self, level: Level) {
        self.current_logging_level = level
    }

    pub fn set_force_styling(&mut self, force_styling: bool) {
        self.force_styling = force_styling;
    }

    pub fn set_log_file_path(&mut self, path: &'static str) {
        self.log_file = Option::from(
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .unwrap(),
        );
    }

    pub fn set_date_format(&mut self, format: String) {
        self.date_format = format;
    }

    pub fn set_datetime_format(&mut self, format: String) {
        self.datetime_format = format;
    }

    pub fn set_time_format(&mut self, format: String) {
        self.time_format = format;
    }

    fn replace_aliases(&self, mut message: String) -> String {
        for (k, v) in constants::Get::aliases() {
            message = message.replace(k, v);
        }
        message
    }

    fn add_styling(&self, mut message: String) -> String {
        for (k, v) in constants::Get::styling() {
            message = message.replace(k, v);
        }
        message
    }

    fn remove_styling(&self, mut message: String) -> String {
        for (k, v) in constants::Get::styling() {
            message = message.replace(k, "");
            message = message.replace(v, "");
        }
        message
    }

    fn add_variables(&self, mut message: String) -> String {
        message = self.replace_aliases(message);
        for (k, v) in self.variables() {
            message = message.replace(k, &v);
        }
        message
    }

    fn do_log(&mut self, level: Level, mut message: String) {
        self.current_logging_level = level;

        message = format!("{}{}{}", self.prefix, message, terminal::Color::RESET);
        message = self.add_variables(message);
        if (self.styling && terminal::Terminal::are_colors_supported()) || self.force_styling {
            message = self.add_styling(message)
        } else {
            message = self.remove_styling(message)
        }
        println!("{}", message);

        if self.log_file.is_some() {
            message = self.remove_styling(message);
            writeln!(self.log_file.as_ref().unwrap(), "{}", message)
                .expect("Failed writing to the log file");
        }

        self.current_logging_level = Level::NONE
    }

    pub fn trace(&mut self, message: impl Into<String>) {
        self.do_log(Level::TRACE, message.into());
    }

    pub fn debug(&mut self, message: impl Into<String>) {
        self.do_log(Level::DEBUG, message.into());
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.do_log(Level::INFO, message.into());
    }

    pub fn warn(&mut self, message: impl Into<String>) {
        self.do_log(Level::WARN, message.into());
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.do_log(Level::ERROR, message.into());
    }

    pub fn fatal(&mut self, message: impl Into<String>) {
        self.do_log(Level::FATAL, message.into());
    }

    pub fn log(&mut self, level: Level, message: impl Into<String>) {
        self.do_log(level, message.into());
    }

    pub fn print(&self, message: impl Into<String>) {
        let mut message = message.into();
        message = self.add_variables(message);
        if (self.styling && terminal::Terminal::are_colors_supported()) || self.force_styling {
            message = self.add_styling(message)
        }
        print!("{}{}", message, terminal::Color::RESET)
    }

    pub fn println(&self, message: impl Into<String>) {
        let mut message = message.into();
        message = self.add_variables(message);
        if (self.styling && terminal::Terminal::are_colors_supported()) || self.force_styling {
            message = self.add_styling(message)
        }
        println!("{}{}", message, terminal::Color::RESET)
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
