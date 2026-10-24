use super::chicken::{self, Expression};
use super::palette::{Palette, paint};
use super::speech::Speech;
use super::text::wrap_text;
use unicode_width::UnicodeWidthStr;

const MAX_WIDTH: usize = 40;

pub fn say(message: &str, expression: Expression, speech: Speech) {
    let p = Palette::new();

    if speech != Speech::Quiet {
        print_bubble(&p, message, speech);
    }
    chicken::print(&p, expression, speech);
}

pub fn print_bubble(p: &Palette, message: &str, speech: Speech) {
    let lines = wrap_text(message, MAX_WIDTH);

    let max_line_width = lines.iter().map(|l| l.width()).max().unwrap_or(0);
    let border = paint(p.border, &"-".repeat(max_line_width + 2));

    println!("  {border}  ");

    let total_lines = lines.len();
    for (i, line) in lines.iter().enumerate() {
        let padding = " ".repeat(max_line_width - line.width());
        let (left, right) = match (speech, total_lines, i) {
            (Speech::Think, _, _) => ('(', ')'),
            (_, 1, _) => ('<', '>'),
            (_, _, 0) => ('/', '\\'),
            (_, n, _) if i == n - 1 => ('\\', '/'),
            _ => ('|', '|'),
        };
        let left = paint(p.border, &left.to_string());
        let right = paint(p.border, &right.to_string());
        let msg = paint(p.text, line);
        println!(" {left} {msg}{padding} {right} ");
    }

    println!("  {border}  ");
}
