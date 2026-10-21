use std::{
    env,
    io::{self, IsTerminal},
};

#[derive(Default)]
pub(super) struct Palette {
    pub(super) border: &'static str,
    pub(super) text: &'static str,
    pub(super) comb: &'static str,
    pub(super) body: &'static str,
    pub(super) eye: &'static str,
    pub(super) beak: &'static str,
}

pub fn color_enabled() -> bool {
    io::stdout().is_terminal() && env::var_os("NO_COLOR").is_none()
}

impl Palette {
    pub(super) fn new() -> Self {
        if color_enabled() {
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

pub fn paint(code: &str, s: &str) -> String {
    if code.is_empty() {
        s.to_string()
    } else {
        format!("{code}{s}\x1b[0m")
    }
}
