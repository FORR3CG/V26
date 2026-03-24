use std::fmt::Display;

#[derive(Debug)]
pub struct Punktur {
    x: i32,
    pub y: i32,
}

impl Punktur {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x, // jafngildir x: x, þar sem fyrra x-ið er gagnabreytan og seinni inntaksbreytan
            y, // hefðum í python gert: self.x = x og self.y = y í __init__ fallinu
        }
    }

    pub fn x(&self) -> i32 {
        self.x // eða return self.x;
    }

    pub fn set_x(&mut self, x: i32) {
        if x == 0 {
            self.x = 1;
        } else {
            self.x = x
        }
    }
}

impl std::fmt::Display for Punktur { // svipað og __str__ eða __repr__ í python
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
