use std::fmt::Display;

pub enum Stada {
    Leigdur,
    Laus,
    EkkiTilLeigu,
    Othekkt,
}

impl Display for Stada {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stada = match self {
            Stada::Leigdur => "Leigður",
            Stada::Laus => "Laus",
            Stada::EkkiTilLeigu => "Ekki til útleigu",
            Stada::Othekkt => "Óþekkt",
        };
        write!(f, "{}", stada)
    }
}

impl From<&str> for Stada {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() { 
            "laus" | "la" => Stada::Laus,
            "leigður" | "leigdur" => Stada::Leigdur,
            "ekki" | "óleigjanlegur" => Stada::EkkiTilLeigu,
            _ => Stada::Othekkt,
        }
    }
}