use std::fmt::Display;

use crate::flugvel::Flugvel;

pub struct Adflugsstjorn {
    flugvelar: Vec<Flugvel>,
}

impl Adflugsstjorn {
    pub fn new() -> Self {
        Self {
            flugvelar: Vec::new(),
        }
    }

    pub fn skra_flugvel(&mut self, flugvel: &str) -> Result<(), String> {
        self.flugvelar.push(Flugvel::try_from(flugvel)?);
        self.flugvelar
            .sort_by_key(|flugvel| flugvel.minutur_til_flugvallar());
        Ok(())
    }

    pub fn lata_minutur_lida(&mut self, fjoldi_minutna: u32) {
        self.flugvelar.iter_mut().for_each(|f| {
            // má líka hafa fall í Flugvel sem reiknar þetta
            let ny_fjarlaegd =
                f.fjarlaegd() - (fjoldi_minutna as f32 / 60. * f.hradi() as f32) as i32;
            f.set_fjarlaegd(ny_fjarlaegd);
        });
        self.flugvelar.retain(|f| f.fjarlaegd() > 10);
    }

    pub fn flugvelar_inna_fjarlaegdar(&self, fjarlaegd: i32) -> String {
        self.flugvelar
            .iter()
            .filter(|f| f.fjarlaegd() < fjarlaegd)
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for Adflugsstjorn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.flugvelar
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
