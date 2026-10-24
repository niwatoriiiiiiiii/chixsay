#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Speech {
    Say,
    Think,
    Quiet,
}

impl Speech {
    pub fn from_flag(flag: &str) -> Option<Self> {
        match flag {
            "--think" => Some(Self::Think),
            "-q" | "--quiet" => Some(Self::Quiet),
            _ => None,
        }
    }

    pub(super) const fn tail(self) -> &'static str {
        match self {
            Self::Say => "\\",
            Self::Think => "o",
            Self::Quiet => " ",
        }
    }
}
