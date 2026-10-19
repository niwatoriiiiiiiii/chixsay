use std::env;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

const CHICKEN: &str = r"
       \
        \    ##
          < > ^< >
";

const MAX_WIDTH: usize = 40;

fn main() {
    let message = get_message();
    print_character(&message);
}

fn get_message() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return args[1..].join(" ");
    }

    // Fallback
    "I'm a chicken!".to_string()
}

fn print_character(text: &str) {
    let lines = wrap_text(text, MAX_WIDTH);

    let max_line_width = lines.iter().map(|l| l.width()).max().unwrap_or(0);
    let border = "-".repeat(max_line_width + 2);

    println!("  {border}  ");

    let total_lines = lines.len();
    for (i, line) in lines.iter().enumerate() {
        let padding = " ".repeat(max_line_width - line.width());

        if total_lines == 1 {
            println!(" < {line}{padding} > ");
        } else if i == 0 {
            println!(" / {line}{padding} \\ ");
        } else if i == total_lines - 1 {
            println!(" \\ {line}{padding} / ");
        } else {
            println!(" | {line}{padding} | ");
        }
    }

    print!("  {border}  ");
    print!("{CHICKEN}");
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for ch in text.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);

        if current_width + ch_width > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0;
        }

        current_line.push(ch);
        current_width += ch_width;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
