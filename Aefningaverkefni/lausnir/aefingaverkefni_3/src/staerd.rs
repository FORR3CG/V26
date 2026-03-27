use std::fmt::Display;

pub struct Staerd {
    pub lengd: f32,
    pub breidd: f32,
}

impl Staerd {
    pub fn new(lengd: f32, breidd: f32) -> Self {
        Self { lengd, breidd }
    }

    pub fn flatarmal(&self) -> f32 {
        self.lengd * self.breidd
    }
}

impl Display for Staerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} fm.", self.flatarmal())
    }
}
