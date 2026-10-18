use std::env;
use unicode_width::UnicodeWidthStr;

const CHICKEN: &str = r"
       \
        \    ##
          < > ^< >
";

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
    let width = text.width();
    let border = "-".repeat(width + 2);

    println!("  {border}  ");
    println!(" < {text} > ");
    print!("  {border}  ");
    print!("{CHICKEN}");
}
