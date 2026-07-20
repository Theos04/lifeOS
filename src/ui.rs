// The UI module - handles all terminal output and styling

use std::io::{self, Write};

// Color helpers - the "how"
fn color(c: &str, text: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", c, text)
}

// Public API - the "what"
pub fn green(text: &str) -> String {
    color("32", text)
}

pub fn cyan(text: &str) -> String {
    color("36", text)
}

pub fn yellow(text: &str) -> String {
    color("33", text)
}

pub fn red(text: &str) -> String {
    color("31", text)
}

pub fn bold(text: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", text)
}

pub fn dim(text: &str) -> String {
    format!("\x1b[2m{}\x1b[0m", text)
}

// Screen utilities
pub fn clear_screen() -> io::Result<()> {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush()?;
    Ok(())
}
