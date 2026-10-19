use std::env;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

const CHICKEN: &str = r"       \
        \    ##
          < > ^< >";

const MAX_WIDTH: usize = 40;

fn main() {
    let message = get_message();
    print_character(&message);
}

fn get_message() -> String {
    let message = env::args().skip(1).collect::<Vec<_>>().join(" ");
    if message.is_empty() {
        "I'm a chicken!".to_string()
    } else {
        message
    }
}

fn print_character(text: &str) {
    let lines = wrap_text(text, MAX_WIDTH);

    let max_line_width = lines.iter().map(|l| l.width()).max().unwrap_or(0);
    let border = "-".repeat(max_line_width + 2);

    println!("  {border}  ");

    let total_lines = lines.len();
    for (i, line) in lines.iter().enumerate() {
        let padding = " ".repeat(max_line_width - line.width());
        let (left, right) = match (total_lines, i) {
            (1, _) => ('<', '>'),
            (_, 0) => ('/', '\\'),
            (n, i) if i == n - 1 => ('\\', '/'),
            _ => ('|', '|'),
        };
        println!(" {left} {line}{padding} {right} ");
    }

    println!("  {border}  ");
    println!("{CHICKEN}");
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    // Replace tabs with spaces and remove control characters
    let cleaned: String = text
        .chars()
        .map(|c| if c == '\t' { ' ' } else { c })
        .filter(|c| !c.is_control() || *c == '\n')
        .collect();

    let mut lines = Vec::new();

    for paragraph in cleaned.trim_end_matches('\n').split('\n') {
        let mut current = String::new();
        let mut current_width = 0;

        for g in paragraph.graphemes(true) {
            let w = g.width();
            if current_width + w > max_width && !current.is_empty() {
                lines.push(std::mem::take(&mut current));
                current_width = 0;
            }
            current.push_str(g);
            current_width += w;
        }
        lines.push(current);
    }

    lines
}
