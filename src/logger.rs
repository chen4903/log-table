use indexmap::IndexMap;

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Logger
    }

    pub fn table<'a>(&self, title: &str, data: impl Into<IndexMap<&'a str, String>>) {
        let data: IndexMap<&str, String> = data.into();
        
        // Find the longest key and value for proper spacing
        let max_key_len = data.keys().map(|k| k.len()).max().unwrap_or(0);
        let max_val_len = data.values().map(|v| v.len()).max().unwrap_or(0);
        
        // Calculate total widths including padding
        let key_col_width = max_key_len + 2; // Add 2 for padding
        let val_col_width = max_val_len + 2; // Add 2 for padding
        let total_width = key_col_width + val_col_width + 3; // Add 3 for borders (│├┤)

        // Print title centered in a box
        println!("┌{}┐", "─".repeat(total_width - 2));
        println!("│{:^width$}│", title, width = total_width - 2);
        println!("├{}┬{}┤", "─".repeat(key_col_width), "─".repeat(val_col_width));

        // Print headers
        println!("│ {:<width$} │ {:<width2$} │", 
            "Key", 
            "Value",
            width = max_key_len,
            width2 = max_val_len
        );

        // Print header separator
        println!("├{}┼{}┤", "─".repeat(key_col_width), "─".repeat(val_col_width));

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
                println!("├{}┼{}┤", "─".repeat(key_col_width), "─".repeat(val_col_width));
            }
        }

        // Print bottom border
        println!("└{}┴{}┘", "─".repeat(key_col_width), "─".repeat(val_col_width));
        println!();
    }
} 