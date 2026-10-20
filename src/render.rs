use std::{
    env,
    io::{self, IsTerminal},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

const MAX_WIDTH: usize = 40;

pub fn say(text: &str) {
    let p = Palette::new();
    let lines = wrap_text(text, MAX_WIDTH);

    let max_line_width = lines.iter().map(|l| l.width()).max().unwrap_or(0);
    let border = paint(p.border, &"-".repeat(max_line_width + 2));

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
        let left = paint(p.border, &left.to_string());
        let right = paint(p.border, &right.to_string());
        let msg = paint(p.text, line);
        println!(" {left} {msg}{padding} {right} ");
    }

    println!("  {border}  ");
    print_chicken(&p);
}

fn print_chicken(p: &Palette) {
    println!("       {}", paint(p.border, "\\"));
    println!(
        "        {}    {}",
        paint(p.border, "\\"),
        paint(p.comb, "##")
    );
    println!(
        "          {} {} {}{} {}",
        paint(p.body, "<"),
        paint(p.eye, ">"),
        paint(p.beak, "^"),
        paint(p.eye, "<"),
        paint(p.body, ">"),
    );
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

struct Palette {
    border: &'static str,
    text: &'static str,
    comb: &'static str,
    body: &'static str,
    eye: &'static str,
    beak: &'static str,
}

impl Palette {
    fn new() -> Self {
        let use_color = io::stdout().is_terminal() && env::var_os("NO_COLOR").is_none();
        if use_color {
            Self {
                border: "\x1b[93m",
                text: "\x1b[37m",
                comb: "\x1b[91m",
                body: "\x1b[37m",
                eye: "\x1b[97m",
                beak: "\x1b[91m",
            }
        } else {
            Self {
                border: "",
                text: "",
                comb: "",
                body: "",
                eye: "",
                beak: "",
            }
        }
    }
}

fn paint(code: &str, s: &str) -> String {
    if code.is_empty() {
        s.to_string()
    } else {
        format!("{code}{s}\x1b[0m")
    }
}
