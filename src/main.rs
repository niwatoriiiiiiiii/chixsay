use render::{Expression, color_enabled, paint};
use std::{env, process};

mod render;

const HEAD: &str = "\x1b[1;92m";
const LIT: &str = "\x1b[1;96m";

fn main() {
    let args = parse_args();
    render::say(&args.message, args.expression);
}

struct Config {
    message: String,
    expression: Expression,
}

fn parse_args() -> Config {
    let mut message_parts = Vec::new();
    let mut expression = Expression::Normal;

    for arg in env::args().skip(1) {
        if matches!(arg.as_str(), "-h" | "--help") {
            print_help();
            process::exit(0);
        }
        match Expression::from_flag(&arg) {
            Some(e) => expression = e,
            None => message_parts.push(arg),
        }
    }

    let message = if message_parts.is_empty() {
        "Bawk Bawk!".to_string()
    } else {
        message_parts.join(" ")
    };

    Config {
        message,
        expression,
    }
}

fn print_help() {
    let (head, lit) = if color_enabled() {
        (HEAD, LIT)
    } else {
        ("", "")
    };

    println!("chixsay - cowsay, but for chickens.");
    println!();
    println!("{}", paint(head, "Usage:"));
    println!("  {}", paint(lit, "chixsay [OPTIONS] [MESSAGE]..."));
    println!();
    println!("{}", paint(head, "Options:"));

    print_option(lit, "-h, --help", "Show this help");
    for (short, long, face) in Expression::options() {
        print_option(lit, &format!("{short}, {long}"), &face);
    }

    println!();
}

fn print_option(lit: &str, flags: &str, desc: &str) {
    let pad = " ".repeat(16usize.saturating_sub(flags.len()));
    println!("  {}{pad}{desc}", paint(lit, flags));
}
