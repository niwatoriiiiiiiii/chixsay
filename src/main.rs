use std::env;

mod render;

fn main() {
    let message = get_message();
    render::say(&message);
}

fn get_message() -> String {
    let message = env::args().skip(1).collect::<Vec<_>>().join(" ");
    if message.is_empty() {
        "Bawk Bawk!".to_string()
    } else {
        message
    }
}
