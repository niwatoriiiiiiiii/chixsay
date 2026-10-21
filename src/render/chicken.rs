use super::palette::{Palette, paint};

static FLAGS: &[(Expression, &str, &str)] = &[
    (Expression::Crying, "-c", "--crying"),
    (Expression::Dead, "-d", "--dead"),
    (Expression::Greedy, "-g", "--greedy"),
    (Expression::Meh, "-m", "--meh"),
    (Expression::Paranoid, "-p", "--paranoid"),
    (Expression::Stoned, "-s", "--stoned"),
    (Expression::Tired, "-t", "--tired"),
    (Expression::Wired, "-w", "--wired"),
];

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
    pub fn from_flag(flag: &str) -> Option<Self> {
        FLAGS
            .iter()
            .find(|(_, short, long)| flag == *short || flag == *long)
            .map(|&(e, _, _)| e)
    }

    pub fn options() -> impl Iterator<Item = (&'static str, &'static str, String)> {
        FLAGS.iter().map(|&(e, short, long)| {
            let (left, right) = e.eyes();
            (short, long, format!("{left} ^{right}"))
        })
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
