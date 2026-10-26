use super::palette::{Palette, paint};
use super::speech::Speech;

static FLAGS: &[(Expression, &str, &str)] = &[
    (Expression::Borg, "-b", "--borg"),
    (Expression::Crying, "-c", "--crying"),
    (Expression::Dead, "-d", "--dead"),
    (Expression::Greedy, "-g", "--greedy"),
    (Expression::Meh, "-m", "--meh"),
    (Expression::Paranoid, "-p", "--paranoid"),
    (Expression::Stoned, "-s", "--stoned"),
    (Expression::Tired, "-t", "--tired"),
    (Expression::Wired, "-w", "--wired"),
    (Expression::Youthful, "-y", "--youthful"),
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    Normal,
    Borg,
    Crying,
    Dead,
    Greedy,
    Meh,
    Paranoid,
    Stoned,
    Tired,
    Wired,
    Youthful,
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
            Self::Borg => ("=", "="),
            Self::Crying => ("T", "T"),
            Self::Dead => ("x", "x"),
            Self::Greedy => ("$", "$"),
            Self::Meh => ("U", "U"),
            Self::Paranoid => ("@", "@"),
            Self::Stoned => ("*", "*"),
            Self::Tired => ("-", "-"),
            Self::Wired => ("O", "O"),
            Self::Youthful => (".", "."),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Face {
    pub left_eye: String,
    pub right_eye: String,
    pub beak: String,
}

impl Face {
    pub fn new(
        expression: Expression,
        custom_eye: Option<&str>,
        custom_beak: Option<&str>,
    ) -> Self {
        use unicode_segmentation::UnicodeSegmentation;

        let (default_left, default_right) = expression.eyes();
        let mut left_eye = default_left.to_string();
        let mut right_eye = default_right.to_string();
        let mut beak = " ^".to_string();

        if let Some(eye_str) = custom_eye {
            let chars: Vec<&str> = eye_str.graphemes(true).collect();
            if chars.len() == 1 {
                left_eye = chars[0].to_string();
                right_eye = chars[0].to_string();
            } else if chars.len() >= 2 {
                left_eye = chars[0].to_string();
                right_eye = chars[1].to_string();
            }
        }

        if let Some(beak_str) = custom_beak {
            beak = beak_str.to_string();
        }

        Self {
            left_eye,
            right_eye,
            beak,
        }
    }
}

impl Default for Face {
    fn default() -> Self {
        Self::new(Expression::Normal, None, None)
    }
}

pub(super) fn print(p: &Palette, face: &Face, speech: Speech) {
    let tail = speech.tail();

    if speech != Speech::Quiet {
        println!("       {}", paint(p.border, tail));
    }

    println!(
        "        {}    {}",
        paint(p.border, tail),
        paint(p.comb, "##")
    );
    println!(
        "          {} {}{}{} {}",
        paint(p.body, "<"),
        paint(p.eye, &face.left_eye),
        paint(p.beak, &face.beak),
        paint(p.eye, &face.right_eye),
        paint(p.body, ">"),
    );
}
