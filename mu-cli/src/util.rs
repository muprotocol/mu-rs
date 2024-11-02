use colored::Colorize;
use terminal_size::{terminal_size, Width};

pub fn print_full_line(message: &str) {
    let width = match terminal_size() {
        Some((Width(w), _)) => w as usize,
        None => 80, // Fallback width if terminal size can't be detected
    };
    let message = format!("[μ]: {} ", message);
    let padded_message = format!("{:<width$}", message, width = width)
        .white()
        .on_green()
        .bold();

    println!("{}", padded_message);
}
