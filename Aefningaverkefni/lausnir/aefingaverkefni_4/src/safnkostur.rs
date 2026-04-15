use std::fmt::Display;
use crate::stadsetning::{Stadsetning};
use crate::tegund::Tegund;

pub struct Safnkostur {
    id: u32,
    titill: String,
    tegund: Tegund,
    stadsetning: Stadsetning,
}

impl Safnkostur {
    pub fn new(id: u32, titill: &str, tegund: &str, stadsetning: &str) -> Result<Self, String> {
        Ok(Self {
            id, 
            titill: String::from(titill),
            tegund: Tegund::try_from(tegund)?,
            stadsetning: Stadsetning::try_from(stadsetning)?,
        })
    }
}

impl Display for Safnkostur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, titill: {}, tegund: {}, staðsetning: {}", self.id, self.titill, self.tegund, self.stadsetning)
    }
}

impl TryFrom<&str> for Safnkostur {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // inntakið er á forminu "id titill tegund staðsetning"
        //                 dæmi: "100 Python bók háteigsvegur"
        // þurfum því að byrja á að brjóta strenginn í fjögur orð
        let ordin: Vec<&str> = value.split_whitespace().collect();
        // ef orðin eru ekki fjögur er okkur óhætt að hætta strax í fallinu
        if ordin.len() != 4 {
            return Err("Ekki nægur fjöldi orða til að búa bók".to_string());
        }
        // breytum fyrsta orðinu í u32
        let id = match ordin[0].parse::<u32>() {
            Ok(tala) => tala,
                    // verðum að hafa return hér því við erum að hætta snemma í fallinu
            Err(_) => return Err(format!("Gat ekki búið til id úr '{}'", ordin[0])),
        };
        // breytum nafninu í String
        let nafn = ordin[1].to_string();
        // reynum að búa til tegundina, notum ? til að flytja villuna áfram (ef einhver villa er)
        let tegund = Tegund::try_from(ordin[2])?;
        // gerum svo sama fyrir staðsetninguna
        let stadsetning = Stadsetning::try_from(ordin[3])?;
        // Ef allt hefur hepnast getum við skilað nýrri bók
        Ok(Self {
            id,
            titill: nafn,
            tegund,
            stadsetning,
        })
        
    }
}