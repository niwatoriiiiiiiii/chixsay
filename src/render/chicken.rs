use super::palette::{Palette, paint};

#[derive(Clone, Copy)]
pub enum Expression {
    Normal,
    Dead,
    Tired,
}

impl Expression {
    const fn eyes(self) -> (&'static str, &'static str) {
        match self {
            Self::Normal => (">", "<"),
            Self::Dead => ("x", "x"),
            Self::Tired => ("-", "-"),
        }
    }
}

pub(super) fn print(p: &Palette, expression: Expression) {
    let (left_eye, right_eye) = expression.eyes();

    println!("       {}", paint(p.border, "\\"));
    println!(
        "        {}    {}",
        paint(p.border, "\\"),
        paint(p.comb, "##")
    );
    println!(
        "          {} {} {}{} {}",
        paint(p.body, "<"),
        paint(p.eye, left_eye),
        paint(p.beak, "^"),
        paint(p.eye, right_eye),
        paint(p.body, ">"),
    );
}
