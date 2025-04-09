use std::collections::HashMap;

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Logger
    }

    pub fn table(&self, title: &str, data: HashMap<&str, String>) {
        // Print title
        println!("\n{}\n", title);

        // Find the longest key and value for proper spacing
        let max_key_len = data.keys().map(|k| k.len()).max().unwrap_or(0);
        let max_val_len = data.values().map(|v| v.len()).max().unwrap_or(0);

        // Print top border
        println!("┌{}┬{}┐", "─".repeat(max_key_len + 2), "─".repeat(max_val_len + 2));

        // Print headers
        println!("│ {:<width$} │ {:<width2$} │", 
            "Key", 
            "Value",
            width = max_key_len,
            width2 = max_val_len
        );

        // Print header separator
        println!("├{}┼{}┤", "─".repeat(max_key_len + 2), "─".repeat(max_val_len + 2));

        // Print data rows
        let entries: Vec<_> = data.iter().collect();
        for (i, (key, value)) in entries.iter().enumerate() {
            println!("│ {:<width$} │ {:<width2$} │",
                key,
                value,
                width = max_key_len,
                width2 = max_val_len
            );
            
            // Add separator line between rows (except for the last row)
            if i < entries.len() - 1 {
                println!("├{}┼{}┤", "─".repeat(max_key_len + 2), "─".repeat(max_val_len + 2));
            }
        }

        // Print bottom border
        println!("└{}┴{}┘", "─".repeat(max_key_len + 2), "─".repeat(max_val_len + 2));
        println!();
    }
} 