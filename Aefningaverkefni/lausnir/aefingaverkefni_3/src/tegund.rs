use std::fmt::Display;

pub enum Tegund {
    Kennslustofa,
    Skrifstofa,
    Annad,
}

impl From<&str> for Tegund {
    fn from(value: &str) -> Self {
        match value.to_lowercase().trim() {
            "kennslu" => Self::Kennslustofa,
            "skrif" => Self::Skrifstofa,
            _ => Self::Annad,
        }
    }
}

impl Display for Tegund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tegund::Kennslustofa => write!(f, "Kennslustofa"),
            Tegund::Skrifstofa => write!(f, "Strifstofa"),
            Tegund::Annad => write!(f, "Annað rými"),
        }
    }
}
