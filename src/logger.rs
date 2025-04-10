use crate::color::Color;
use indexmap::IndexMap;
use std::fmt::Display;

pub struct Logger {
    chain_id: Option<u64>,
    provider: Option<String>,
    frame_color: Option<Color>,
    content_color: Option<Color>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            chain_id: None,
            provider: None,
            frame_color: None,
            content_color: None,
        }
    }

    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    pub fn with_frame_color(mut self, color: Color) -> Self {
        self.frame_color = Some(color);
        self
    }

    pub fn with_content_color(mut self, color: Color) -> Self {
        self.content_color = Some(color);
        self
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

        println!("{}", top_border);
        println!(
            "{}{}{}",
            title_border_left, title_content, title_border_right
        );
        println!(
            "{}",
            self.colorize_frame(&format!(
                "├{}┬{}┤",
                "─".repeat(key_col_width),
                "─".repeat(val_col_width)
            ))
        );

        // Headers
        let header_left = self.colorize_frame("│");
        let header_middle = self.colorize_frame("│");
        let header_right = self.colorize_frame("│");
        let key_header =
            self.colorize_content(&format!(" {:<width$} ", "Key", width = max_key_len));
        let value_header =
            self.colorize_content(&format!(" {:<width$} ", "Value", width = max_val_len));

        println!(
            "{}{}{}{}{}",
            header_left, key_header, header_middle, value_header, header_right
        );
        println!(
            "{}",
            self.colorize_frame(&format!(
                "├{}┼{}┤",
                "─".repeat(key_col_width),
                "─".repeat(val_col_width)
            ))
        );

        let entries: Vec<_> = data.iter().collect();
        for (i, (key, value)) in entries.iter().enumerate() {
            let row_left = self.colorize_frame("│");
            let row_middle = self.colorize_frame("│");
            let row_right = self.colorize_frame("│");
            let key_content =
                self.colorize_content(&format!(" {:<width$} ", key, width = max_key_len));
            let value_content =
                self.colorize_content(&format!(" {:<width$} ", value, width = max_val_len));

            println!(
                "{}{}{}{}{}",
                row_left, key_content, row_middle, value_content, row_right
            );

            if i < entries.len() - 1 {
                println!(
                    "{}",
                    self.colorize_frame(&format!(
                        "├{}┼{}┤",
                        "─".repeat(key_col_width),
                        "─".repeat(val_col_width)
                    ))
                );
            }
        }

        println!(
            "{}",
            self.colorize_frame(&format!(
                "└{}┴{}┘",
                "─".repeat(key_col_width),
                "─".repeat(val_col_width)
            ))
        );
        println!();
    }
}
