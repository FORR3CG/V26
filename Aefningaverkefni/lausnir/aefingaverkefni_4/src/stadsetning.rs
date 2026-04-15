use std::fmt::Display;

pub enum Stadsetning {
    Skolavorduholt,
    Hateigsvegur,
    Hafnarfjordur,
}

impl Display for Stadsetning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stadsetning::Skolavorduholt => write!(f, "Skólavörðuholt"),
            Stadsetning::Hateigsvegur => write!(f, "Háteigsvegur"),
            Stadsetning::Hafnarfjordur => write!(f, "Hafnarfjörður"),
        }
    }
}

impl TryFrom<&str> for Stadsetning {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "sh" | "holtið" => Ok(Stadsetning::Skolavorduholt),
            "hv" | "háteigsvegur" => Ok(Stadsetning::Hateigsvegur),
            "ha" | "hafnarfjörður" => Ok(Stadsetning::Hafnarfjordur),
            _ => Err(format!("Gat ekki breytt '{}' í staðsetningu!!", value)),
        }
    }
}