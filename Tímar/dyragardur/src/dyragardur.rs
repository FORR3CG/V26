use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::dyr::Dyr;
use crate::hundur::Hundur;
use crate::kottur::Kottur;

#[derive(Serialize, Deserialize)]
pub struct Dyragardur {
    dyrin: Vec<Dyr>,
    id: u32,
}

impl Dyragardur {
    pub fn new(id: Option<u32>) -> Self {
        /*  let id = match id {
            Some(tala) => tala,
            None => 1000,
        }; */
        Self {
            dyrin: Vec::new(),
            id: id.unwrap_or(1000),
        }
    }

    fn next_id(&mut self) -> u32 {
        self.id += 1;
        self.id
    }

    fn skra_dyr(&mut self, dyr: Dyr) {
        self.dyrin.push(dyr);
        self.dyrin.sort();
    }

    pub fn skra_hund(&mut self, hundur: &str) -> Result<(), String> {
        let hundur = Hundur::try_from((self.next_id(), hundur))?;
        self.skra_dyr(Dyr::Hundurinn(hundur));
        //self.dyrin.push(Dyr::Hundurinn(hundur));
        //self.dyrin.sort();
        Ok(())
    }

    pub fn skra_kott(&mut self, kottur: &str) -> Result<(), String> {
        let id = self.next_id();
/*         self
            .dyrin
            .push(Dyr::Kotturinn(Kottur::try_from((id, kottur))?));
        self.dyrin.sort(); */
        Ok(self.skra_dyr(Dyr::Kotturinn(Kottur::try_from((id, kottur))?)))
    }

    pub fn medaleinkunn_hunda(&mut self) -> Option<f32> {
        let mut fjoldi_hunda = 0;
        let heildareinkunn = self.dyrin.iter().fold(0_f32, |summa, dyr| {
            if let Some(hundur) = dyr.hundur() {
                fjoldi_hunda += 1;
                summa + hundur.einkunn() as f32
            } else {
                summa + 0_f32
            }
        });
        if fjoldi_hunda > 0 {
            Some(heildareinkunn / fjoldi_hunda as f32)
        } else {
            None
        }
    }

    pub fn haekka_aldur_kattar_med_id(&mut self, id: u32, nyr_aldur: u8) -> Result<(), String> {
        let mut dyr = self.dyrin.iter_mut()
                                                  .find(|dyr| dyr.id() == id);
        match dyr {
            Some(dyr) => {
                match dyr {
                    Dyr::Hundurinn(_) => Err(format!("Dýrið með id: '{}' er ekki köttur!!", id)),
                    Dyr::Kotturinn(kottur) => {
                        kottur.set_aldur(nyr_aldur);
                        Ok(())
                        // Ok(kottur.set_aldur(nyr_aldur))
                    }
                }
            },
            None => Err(format!("Fann ekkert dýr með id: '{}'!!", id))
        }
    }

    pub fn breyta_einkunn_hunds_med_id(&mut self, id: u32, ny_einkunn: u32) -> Result<(), String> {
        match self.dyrin.iter().position(|dyr| dyr.id() == id) {
            Some(idx) => self.dyrin[idx].set_einkunn(ny_einkunn),
            None => Err(format!("Fann ekkert dýr með id: '{}'", id))
        }
    }
}

impl Display for Dyragardur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.dyrin
                .iter()
                .map(|dyr| dyr.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
