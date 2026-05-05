use std::fmt::Display;

use crate::dyragrunnur::Dyragrunnur;

pub struct Kottur {
    grunnur: Dyragrunnur,
    aldur: u8,
}

impl Kottur {
    pub fn new(id: u32, nafn: &str, aldur: u8) -> Self {
        Self {
            grunnur: Dyragrunnur::from((id, nafn)),
            aldur,
        }
    }

    pub fn id(&self) -> u32 {
        self.grunnur.id()
    }

    pub fn nafn(&self) -> &str {
        self.grunnur.nafn()
    }

    pub fn set_nafn(&mut self, nafn: &str) {
        self.grunnur.set_nafn(nafn)
    }

    pub fn aldur(&self) -> u8 {
        self.aldur
    }

    pub fn set_aldur(&mut self, aldur: u8) {
        self.aldur = aldur
    }
}

impl Display for Kottur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hundur, {}, aldur {}", self.grunnur, self.aldur)
    }
}

impl TryFrom<(u32, &str)> for Kottur {
    type Error = String;

    fn try_from(value: (u32, &str)) -> Result<Self, Self::Error> {
        let lidir = value.1.split_whitespace().collect::<Vec<&str>>();
        if lidir.len() != 2 {
            Err(format!("Köttur villa: '{}' inniheldur ekki réttan fjölda orða til að búa til kött!",value.1))
        } else {
            let grunnur = Dyragrunnur::from((value.0, lidir[0]));
            let aldur = match lidir[1].parse::<u8>() {
                Ok(tala) => tala,
                Err(_) => return Err(format!("Gat ekki gert einkunn úr '{}'", lidir[1]))
            };
            Ok(Self {
                grunnur, aldur
            })
        }
    }
}