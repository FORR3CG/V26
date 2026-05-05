use std::fmt::Display;

use crate::dyragrunnur::Dyragrunnur;

pub struct Hundur {
    grunnur: Dyragrunnur,
    einkunn: u32,
}

impl Hundur {
    pub fn new(grunnur: Dyragrunnur, einkunn: u32) -> Self {
        Self { grunnur, einkunn }
    }

    pub fn id(&self) -> u32 {
        self.grunnur.id()
    }

    pub fn nafn(&self) -> &str {
        self.grunnur.nafn()
    }

    pub fn set_nafn(&mut self, nafn: &str) {
        self.grunnur.set_nafn(nafn);
    }

    pub fn einkunn(&self) -> u32 {
        self.einkunn
    }

    pub fn set_einkunn(&mut self, ny_einkunn: u32) {
        self.einkunn = ny_einkunn
    }
}

impl Display for Hundur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hundur, {}, einkunn: {}", self.grunnur, self.einkunn)
    }
}

impl TryFrom<(u32, &str)> for Hundur {
    type Error = String;

    fn try_from(value: (u32, &str)) -> Result<Self, Self::Error> {
        // (78, "Snati 270")
        let lidir = value.1.split_whitespace().collect::<Vec<&str>>();
        if lidir.len() != 2 {
            Err(format!(
                "Hundur villa, ekki réttur fjöldi orða í '{}' til að gera hund",
                value.1
            ))
        } else {
            let grunnur = Dyragrunnur::from((value.0, lidir[0]));
            let einkunn = match lidir[1].parse::<u32>() {
                Ok(tala) => tala,
                Err(_) => {
                    return Err(format!(
                        "Hundur villa, gat ekki breytt '{}' í einkunn",
                        lidir[1]
                    ));
                }
            };
            Ok(Self { grunnur, einkunn })
        }
    }
}
