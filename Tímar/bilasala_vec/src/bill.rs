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
    verd: u32,
}

impl Bill {          
    pub fn new(id: u32, tegund: &str, gerd: &str, litur: u32, verd: u32) -> Result<Self, String> {
        Ok(Self {
            id,
            tegund: tegund.to_string(),
            gerd: Gerd::try_from(gerd)?,
            litur: Litur::from(litur), // u32 => u8, u8, u8, u8
            verd,
        })
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

impl TryFrom<&str> for Bill {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // "Volvo fb 65280 2000"
        let ordin = value.split_whitespace().collect::<Vec<&str>>();
        if ordin.len() != 5 {
            return Err("Ekki nógu mörg orð til að búa til bíl!!!".to_string());
        }
        let id = match ordin[0].parse::<u32>() {
            Ok(tala) => tala,
            Err(_) => return Err(format!("Gat ekki breytt '{}' í id!!", ordin[0])),
        };
        let tegund = ordin[1].to_string(); 
        //let tegund = ordin.get(0); 
        let gerd = Gerd::try_from(ordin[2])?;
        let litur = Litur::try_from(ordin[3])?;
        if let Ok(verd) = ordin[4].parse::<u32>() {
            Ok(Self {
                id,
                tegund,
                gerd,
                litur,
                verd,
            })
        } else {
            Err(format!("Gat ekki breytt '{}' í verð!!", ordin[3]))
        }

    }
}