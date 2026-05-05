
use std::fmt::Display;

use crate::hundur::Hundur;
use crate::kottur::Kottur;

pub enum Dyr {
    // variant(struct)
    Hundurinn(Hundur),
    Kotturinn(Kottur),
}

impl Display for Dyr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dyr::Hundurinn(hundur) => write!(f, "{}", hundur),
            Dyr::Kotturinn(kottur) => write!(f, "{}", kottur),
        }
    }

}