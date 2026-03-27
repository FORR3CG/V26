use std::fmt::Display;

use crate::gerd::Gerd;
use crate::litur::Litur;

#[derive(Debug)]
pub struct Bill {
    // id bættist við fyrri útgáfu ásamt id() falli í impl
    id: u32,
    tegund: String,
    gerd: Gerd,
    litur: Litur,
    // bætt við set_verd falli í impl
    verd: u32,
}

impl Bill {          
    pub fn new(id: u32, tegund: &str, gerd: &str, litur: u32, verd: u32) -> Self {
        Self {
            id,
            tegund: tegund.to_string(),
            gerd: Gerd::from(gerd),
            litur: Litur::from(litur), // u32 => u8, u8, u8, u8
            verd,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn verd(&self) -> u32 {
        self.verd
    }

    // tekur inn prósentutölu og hækkar verðið um þá prósentu
    pub fn haekka_verd_um_prosent(&mut self, prosentuhaekkun: u8) {
        self.verd = (self.verd as f32 * (1. + prosentuhaekkun as f32 / 100.)) as u32
    }
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, tegund: {}, gerd: {}, litur: {}, verð: {} kr.", self.id, self.tegund, self.gerd, self.litur, self.verd)
    }
}