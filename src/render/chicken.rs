use super::palette::{Palette, paint};

#[derive(Clone, Copy)]
pub enum Expression {
    Normal,
    Crying,
    Dead,
    Greedy,
    Meh,
    Paranoid,
    Stoned,
    Tired,
    Wired,
}

impl Expression {
    pub fn from_frag(flag: &str) -> Option<Self> {
        match flag {
            "-c" | "--crying" => Some(Self::Crying),
            "-d" | "--dead" => Some(Self::Dead),
            "-g" | "--greedy" => Some(Self::Greedy),
            "-m" | "--meh" => Some(Self::Meh),
            "-p" | "--paranoid" => Some(Self::Paranoid),
            "-s" | "--stoned" => Some(Self::Stoned),
            "-t" | "--tired" => Some(Self::Tired),
            "-w" | "--wired" => Some(Self::Wired),
            _ => None,
        }
    }
    const fn eyes(self) -> (&'static str, &'static str) {
        match self {
            Self::Normal => (">", "<"),
            Self::Crying => ("T", "T"),
            Self::Dead => ("x", "x"),
            Self::Greedy => ("$", "$"),
            Self::Meh => ("U", "U"),
            Self::Paranoid => ("@", "@"),
            Self::Stoned => ("*", "*"),
            Self::Tired => ("-", "-"),
            Self::Wired => ("O", "O"),
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
