use std::fmt::Display;

pub enum Tegund {
    Bok,
    Timarit,
}

impl Display for Tegund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tegund::Bok => write!(f, "Bók"),
            Tegund::Timarit => write!(f, "Tímarit"),
        }
    }
}

impl TryFrom<&str> for Tegund {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "b" | "bók" => Ok(Tegund::Bok),
            "t" | "tímarit" => Ok(Tegund::Timarit),
            _ => Err(format!("Gat ekki breytt '{}' í tegund!!", value))
        }
    }
}