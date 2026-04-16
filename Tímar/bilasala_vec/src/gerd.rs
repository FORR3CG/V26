use std::fmt::Display;

#[derive(Debug)]
pub enum Gerd {
    Folksbill,
    Jeppi,
    Vorubill,
}

impl TryFrom<&str> for Gerd {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() { // fólksbill Fólksbíll FÓLKSBÍLL
            "fb" | "fólksbíll" | "folksbill" => Ok(Gerd::Folksbill),
            "j" | "jeppi" => Ok(Gerd::Jeppi),
            "vb" | "vörubíll" => Ok(Gerd::Vorubill),
            _ => Err(format!("Gerd: gat ekki búið til gerð úr {}", value)),
        }
    }
}

/* impl From<&str> for Gerd {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() { // fólksbill Fólksbíll FÓLKSBÍLL
            "fb" | "fólksbíll" | "folksbill" => Gerd::Folksbill,
            "j" | "jeppi" => Gerd::Jeppi,
            _ => Gerd::Vorubill,
            //_ => todo!()
        }
    }
} */

// alveg eins og önnur Display, bara misjafnt hvað skrifast út eftir því hvað variant er valinn.
impl Display for Gerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gerd::Folksbill => write!(f, "Fólksbíll"),
            Gerd::Jeppi => write!(f, "Jeppi"),
            Gerd::Vorubill => write!(f, "Vörubíll"),
        }
    }
}