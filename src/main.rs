mod cli;
mod render;

fn main() {
    let config = cli::parse();
    render::say(&config.message, &config.face, config.speech, config.width);
}
