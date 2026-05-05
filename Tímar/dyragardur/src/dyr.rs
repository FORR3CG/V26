use std::fmt::Display;

use crate::hundur::Hundur;
use crate::kottur::Kottur;

pub enum Dyr {
    // variant(struct)
    Hundurinn(Hundur),
    Kotturinn(Kottur),
}

impl Dyr {
    pub fn hundur(&self) -> Option<&Hundur> {
        match self {
            Self::Hundurinn(hundur) => Some(hundur),
            _ => None,
        }
    }

    pub fn kottur(&self) -> Option<&Kottur> {
        match self {
            Self::Kotturinn(kottur) => Some(kottur),
            _ => None,
        }
    }
}

impl Display for Dyr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dyr::Hundurinn(hundur) => write!(f, "{}", hundur),
            Dyr::Kotturinn(kottur) => write!(f, "{}", kottur),
        }
    }
}
