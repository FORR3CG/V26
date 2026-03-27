use std::fmt::Display;

use crate::staerd::Staerd;
use crate::tegund::Tegund;

pub struct Herbergi {
    id: u32,
    staerd: Staerd,
    tegund: Tegund,
}

impl Herbergi {
    pub fn new(id: u32, lengd: f32, breidd: f32, tegund: &str) -> Self {
        Self {
            id,
            staerd: Staerd::new(lengd, breidd),
            tegund: Tegund::from(tegund),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn breyta_staerd(&mut self, lengd: f32, breidd: f32) {
        self.staerd.lengd = lengd;
        self.staerd.breidd = breidd;
    }

    pub fn breyta_tegund(&mut self, ny_tegund: &str) {
        self.tegund = Tegund::from(ny_tegund)
    }

    pub fn fermetrar(&self) -> f32 {
        self.staerd.flatarmal()
    }
}

impl Display for Herbergi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Nr: {}, Stærð: {}, tegund: {}.",
            self.id, self.staerd, self.tegund
        )
    }
}
