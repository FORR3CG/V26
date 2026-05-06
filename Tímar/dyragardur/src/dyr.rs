use std::fmt::Display;

use crate::hundur::{self, Hundur};
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

    pub fn id(&self) -> u32 {
        match self {
            Self::Hundurinn(h) => h.id(),
            Self::Kotturinn(k) => k.id(),
        }
    }

    pub fn nafn(&self) -> &str {
        match self {
            Self::Hundurinn(h) => h.nafn(),
            Self::Kotturinn(k) => k.nafn(),
        }
    }

    pub fn set_einkunn(&mut self, ny_einkunn: u32) -> Result<(), String> {
        match self {
            Self::Hundurinn(h) => Ok(h.set_einkunn(ny_einkunn)),
            _ => Err("Dýrið er ekki hundur!!".to_string())
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

impl Ord for Dyr { 
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // raða fyrst eftir nafni og síðan id
        self.nafn().cmp(&other.nafn())
            .then(self.id().cmp(&other.id()))
    }
}

impl PartialOrd for Dyr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Dyr { // útfærir ==
    fn eq(&self, other: &Self) -> bool {
        self.nafn() == other.nafn() && self.id() == other.id()
    }
}

impl Eq for Dyr {}
