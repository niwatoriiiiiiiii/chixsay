use render::Expression;
use std::env;

mod render;

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
        match arg.as_str() {
            "-d" => expression = Expression::Dead,
            "-t" => expression = Expression::Tired,
            other => message_parts.push(other.to_string()),
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
