
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