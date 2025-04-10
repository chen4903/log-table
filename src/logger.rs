use crate::color::Color;
use indexmap::IndexMap;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct Logger {
    frame_color: Option<Color>,
    content_color: Option<Color>,
    log_file: Option<PathBuf>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            frame_color: None,
            content_color: None,
            log_file: None,
        }
    }

    pub fn with_frame_color(mut self, color: Color) -> Self {
        self.frame_color = Some(color);
        self
    }

    pub fn with_content_color(mut self, color: Color) -> Self {
        self.content_color = Some(color);
        self
    }

    pub fn with_save_log(mut self, path: impl Into<PathBuf>) -> Self {
        self.log_file = Some(path.into());
        // Create directory if it doesn't exist
        if let Some(log_file) = &self.log_file {
            if let Some(parent) = log_file.parent() {
                fs::create_dir_all(parent).ok();
            }
        }
        self
    }

    fn write_to_log(&self, content: &str) {
        if let Some(log_file) = &self.log_file {
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_file) {
                writeln!(file, "{}", content).ok();
            }
        }
    }

    fn print_and_log(&self, content: &str) {
        println!("{}", content);
        self.write_to_log(content);
    }

    fn colorize_frame(&self, s: &str) -> String {
        if let Some(color) = self.frame_color {
            color.to_colored_string(s)
        } else {
            s.to_string()
        }
    }

    fn colorize_content(&self, s: &str) -> String {
        if let Some(color) = self.content_color {
            color.to_colored_string(s)
        } else {
            s.to_string()
        }
    }

    pub fn table<'a, T: Display>(&self, title: &str, data: impl Into<IndexMap<&'a str, T>>) {
        let data_map: IndexMap<&str, T> = data.into();
        let data: IndexMap<&str, String> = data_map
            .into_iter()
            .map(|(k, v)| (k, format!("{}", v)))
            .collect();

        let max_key_len = data.keys().map(|k| k.len()).max().unwrap_or(0);
        let max_val_len = data.values().map(|v| v.len()).max().unwrap_or(0);

        let key_col_width = max_key_len + 2;
        let val_col_width = max_val_len + 2;
        let total_width = key_col_width + val_col_width + 3;

        // Frame parts
        let top_border = self.colorize_frame(&format!("┌{}┐", "─".repeat(total_width - 2)));
        let title_border_left = self.colorize_frame("│");
        let title_border_right = self.colorize_frame("│");
        let title_content =
            self.colorize_content(&format!("{:^width$}", title, width = total_width - 2));

        self.print_and_log(&top_border);
        self.print_and_log(&format!(
            "{}{}{}",
            title_border_left, title_content, title_border_right
        ));
        self.print_and_log(&self.colorize_frame(&format!(
            "├{}┬{}┤",
            "─".repeat(key_col_width),
            "─".repeat(val_col_width)
        )));

        // Headers
        let header_left = self.colorize_frame("│");
        let header_middle = self.colorize_frame("│");
        let header_right = self.colorize_frame("│");
        let key_header =
            self.colorize_content(&format!(" {:<width$} ", "Key", width = max_key_len));
        let value_header =
            self.colorize_content(&format!(" {:<width$} ", "Value", width = max_val_len));

        self.print_and_log(&format!(
            "{}{}{}{}{}",
            header_left, key_header, header_middle, value_header, header_right
        ));
        self.print_and_log(&self.colorize_frame(&format!(
            "├{}┼{}┤",
            "─".repeat(key_col_width),
            "─".repeat(val_col_width)
        )));

        let entries: Vec<_> = data.iter().collect();
        for (i, (key, value)) in entries.iter().enumerate() {
            let row_left = self.colorize_frame("│");
            let row_middle = self.colorize_frame("│");
            let row_right = self.colorize_frame("│");
            let key_content =
                self.colorize_content(&format!(" {:<width$} ", key, width = max_key_len));
            let value_content =
                self.colorize_content(&format!(" {:<width$} ", value, width = max_val_len));

            self.print_and_log(&format!(
                "{}{}{}{}{}",
                row_left, key_content, row_middle, value_content, row_right
            ));

            if i < entries.len() - 1 {
                self.print_and_log(&self.colorize_frame(&format!(
                    "├{}┼{}┤",
                    "─".repeat(key_col_width),
                    "─".repeat(val_col_width)
                )));
            }
        }

        self.print_and_log(&self.colorize_frame(&format!(
            "└{}┴{}┘",
            "─".repeat(key_col_width),
            "─".repeat(val_col_width)
        )));
        self.print_and_log("");
    }
}
