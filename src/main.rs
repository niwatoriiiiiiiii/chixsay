mod cli;
mod render;

fn main() {
    let config = cli::parse();
    render::say(
        &config.message,
        config.expression,
        config.speech,
        config.width,
    );
}
