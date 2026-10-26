mod chicken;
mod palette;
mod scene;
mod speech;
mod text;

pub use chicken::{Expression, Face};
pub use palette::{color_enabled, paint, stderr_color_enabled};
pub use scene::say;
pub use speech::Speech;
