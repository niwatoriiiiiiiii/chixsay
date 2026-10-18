use unicode_width::UnicodeWidthStr;

const CHICKEN: &str = r#"
          \
           \   ##
            < > ^< >
"#;

fn main() {
    print_speech_bubble("I'm a chicken!");
    print!("{CHICKEN}");
}

fn print_speech_bubble(text: &str) {
    let width = text.width();
    let border = "-".repeat(width + 2);

    println!("  {}  ", border);
    println!(" < {} > ", text);
    print!("  {}  ", border);
}
