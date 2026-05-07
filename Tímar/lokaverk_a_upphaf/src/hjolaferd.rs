
use crate::ferdagrunnur::Ferdagrunnur;

pub struct Hjolaferd {
    grunnur: Ferdagrunnur,
    klst: u8,
}

impl TryFrom<(u32, &str)> for Hjolaferd {
    type Error = String; 

    // (234, "100 200 8")
    fn try_from(value: (u32, &str)) -> Result<Self, Self::Error> {
        let lidir = value.1.split_whitespace().collect::<Vec<&str>>();
        // tékka á hvort nægur fjölda er
        let grunnur = Ferdagrunnur::try_from((value.0, format!("{} {}", lidir[0], lidir[1]).as_str()))?;
        let klst = match lidir[2].parse::<u8>() {
            Ok(tala) => tala,
            Err(_) => todo!()
        };

        Ok(Self { grunnur, klst })
    }
}