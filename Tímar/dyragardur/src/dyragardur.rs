
use std::fmt::Display;

use crate::dyr::Dyr;
use crate::hundur::Hundur;
use crate::kottur::Kottur;

struct Dyragardur {
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

    pub fn skra_hund(&mut self, hundur: &str) -> Result<(), String> {
        let hundur = Hundur::try_from((self.next_id(), hundur))?;
        self.dyrin.push(Dyr::Hundurinn(hundur));
        Ok(())
    }

    pub fn skra_kott(&mut self, kottur: &str) -> Result<(), String> {
        let id = self.next_id();
        Ok(
            self.dyrin.push(
                Dyr::Kotturinn(
                    Kottur::try_from((id, kottur))?
                )
            )
        )
    }
}

impl Display for Dyragardur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}",
            self.dyrin.iter()
                .map(|dyr| dyr.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}