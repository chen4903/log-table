use colored::*;

#[derive(Clone, Copy)]
pub enum Color {
    Blue,
    Green,
    Red,
    Orange,
    White,
    Gray,
    Black,
    Yellow,
    Purple,
}

impl Color {
    pub fn to_colored_string(&self, s: &str) -> String {
        match self {
            Color::Blue => s.blue().to_string(),
            Color::Green => s.green().to_string(),
            Color::Red => s.red().to_string(),
            Color::Orange => s.truecolor(255, 165, 0).to_string(),
            Color::White => s.white().to_string(),
            Color::Gray => s.truecolor(128, 128, 128).to_string(),
            Color::Black => s.black().to_string(),
            Color::Yellow => s.yellow().to_string(),
            Color::Purple => s.purple().to_string(),
        }
    }
}
