use crate::gerd::Gerd;
use crate::litur::Litur;

#[derive(Debug)]
pub struct Bill {
    tegund: String,
    gerd: Gerd,
    litur: Litur,
    verd: u32,
}

impl Bill {          
    pub fn new(tegund: &str, gerd: &str, litur: u32, verd: u32) -> Self {
        Self {
            tegund: tegund.to_string(),
            gerd: Gerd::from(gerd),
            litur: Litur::from(litur), // u32 => u8, u8, u8, u8
            verd,
        }
    }
}