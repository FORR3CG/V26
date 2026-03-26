use std::fmt::Display;

#[derive(Debug)]
pub enum Gerd {
    Folksbill,
    Jeppi,
    Vorubill,
    Annad,
}

impl From<&str> for Gerd {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() { // fólksbill Fólksbíll FÓLKSBÍLL
            "fb" | "fólksbíll" | "folksbill" => Gerd::Folksbill,
            "j" | "jeppi" => Gerd::Jeppi,
            "vb" | "vörubíll" => Gerd::Vorubill,
            _ => Gerd::Annad,
        }
    }
}

// alveg eins og önnur Display, bara misjafnt hvað skrifast út eftir því hvað variant er valinn.
impl Display for Gerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gerd::Folksbill => write!(f, "Fólksbíll"),
            Gerd::Jeppi => write!(f, "Jeppi"),
            Gerd::Vorubill => write!(f, "Vörubíll"),
            Gerd::Annad => write!(f, "Annað"),
        }
    }
}