use render::{Expression, Speech, color_enabled, paint, stderr_color_enabled};
use std::{
    env,
    io::{self, IsTerminal, Read},
    process,
};

mod render;

const HEAD: &str = "\x1b[1;92m";
const LIT: &str = "\x1b[1;96m";
const ERR: &str = "\x1b[1;91m";
const INVALID: &str = "\x1b[1;93m";

const USAGE: &str = "chixsay [OPTIONS] [MESSAGE]...";

fn main() {
    let args = parse_args();
    render::say(&args.message, args.expression, args.speech);
}

struct Config {
    message: String,
    expression: Expression,
    speech: Speech,
}

fn parse_args() -> Config {
    let mut message_parts = Vec::new();
    let mut expression: Option<(Expression, String)> = None;
    let mut speech: Option<(Speech, String)> = None;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--" {
            message_parts.extend(args.by_ref());
            break;
        }
        if matches!(arg.as_str(), "-h" | "--help") {
            print_help();
            process::exit(0);
        }
        if matches!(arg.as_str(), "-V" | "--version") {
            println!("chixsay {}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        if let Some(e) = Expression::from_flag(&arg) {
            set_once(&mut expression, e, arg);
            continue;
        }
        if let Some(s) = Speech::from_flag(&arg) {
            set_once(&mut speech, s, arg);
            continue;
        }
        if looks_like_option(&arg) {
            unexpected_argument(&arg);
        }
        message_parts.push(arg);
    }

    let expression = expression.map_or(Expression::Normal, |(e, _)| e);
    let speech = speech.map_or(Speech::Say, |(s, _)| s);

    let message = if speech == Speech::Quiet {
        String::new()
    } else if message_parts.is_empty() {
        read_stdin().unwrap_or_else(|| "Bawk Bawk!".to_string())
    } else {
        message_parts.join(" ")
    };

    Config {
        message,
        expression,
        speech,
    }
}

fn set_once<T: PartialEq>(slot: &mut Option<(T, String)>, value: T, arg: String) {
    if let Some((prev, prev_arg)) = slot.as_ref() {
        conflicting_option(prev_arg, &arg, *prev == value);
    }
    *slot = Some((value, arg));
}

fn looks_like_option(arg: &str) -> bool {
    arg.len() > 1 && arg.starts_with('-') && !arg.contains(char::is_whitespace)
}

struct Styles {
    head: &'static str,
    lit: &'static str,
    err: &'static str,
    invalid: &'static str,
}

impl Styles {
    fn stderr() -> Self {
        if stderr_color_enabled() {
            Self {
                head: HEAD,
                lit: LIT,
                err: ERR,
                invalid: INVALID,
            }
        } else {
            Self {
                head: "",
                lit: "",
                err: "",
                invalid: "",
            }
        }
    }
}

fn unexpected_argument(arg: &str) -> ! {
    let s = Styles::stderr();
    let tip = format!(
        "to pass '{}' as a value, use '{}'",
        paint(s.invalid, arg),
        paint(s.lit, &format!("chixsay -- {arg}"))
    );
    fail(
        &s,
        &format!("unexpected argument '{}' found", paint(s.invalid, arg)),
        Some(tip),
    )
}

fn conflicting_option(prev: &str, arg: &str, same: bool) -> ! {
    let s = Styles::stderr();
    let message = if same {
        format!(
            "the argument '{}' cannot be used multiple times",
            paint(s.invalid, arg)
        )
    } else {
        format!(
            "the argument '{}' cannot be used with '{}'",
            paint(s.invalid, arg),
            paint(s.invalid, prev)
        )
    };
    fail(&s, &message, None)
}

fn fail(s: &Styles, message: &str, tip: Option<String>) -> ! {
    eprintln!("{} {message}", paint(s.err, "error:"));
    if let Some(tip) = tip {
        eprintln!();
        eprintln!("  {} {tip}", paint(s.lit, "tip:"));
    }
    eprintln!();
    eprintln!("{} {}", paint(s.head, "Usage:"), paint(s.lit, USAGE));
    eprintln!();
    eprintln!("For more information, try '{}'.", paint(s.lit, "--help"));
    process::exit(2);
}

fn read_stdin() -> Option<String> {
    let mut stdin = io::stdin();
    if stdin.is_terminal() {
        return None;
    }

    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf).ok()?;

    let text = String::from_utf8_lossy(&buf);
    let text = text.trim_end_matches(['\r', '\n']);

    (!text.trim().is_empty()).then(|| text.to_string())
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
    println!("  {}", paint(lit, USAGE));
    println!("  If no MESSAGE is given, it is read from stdin when piped.");
    println!();
    println!("{}", paint(head, "Options:"));

    print_option(lit, "-h, --help", "Show this help");
    print_option(lit, "-V, --version", "Show version");
    print_option(lit, "-q, --quiet", "Hide the speech bubble");
    print_option(lit, "--think", "Use a thought bubble");
    print_option(lit, "--", "Treat the rest as the message");
    for (short, long, face) in Expression::options() {
        print_option(lit, &format!("{short}, {long}"), &face);
    }

    println!();
}

fn print_option(lit: &str, flags: &str, desc: &str) {
    let pad = " ".repeat(16usize.saturating_sub(flags.len()));
    println!("  {}{pad}{desc}", paint(lit, flags));
}
