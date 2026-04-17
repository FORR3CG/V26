use std::fmt::Display;

pub struct Flugvel {
    kallmerki: String,
    hradi: u32,
    fjarlaegd: i32,
}

impl Flugvel {
    pub fn new(kallmerki: &str, hradi: u32, fjarlaegd: i32) -> Self {
        Self {
            kallmerki: kallmerki.to_uppercase(), // to_uppercase skilar String
            hradi,
            fjarlaegd,
        }
    }

    pub fn hradi(&self) -> u32 {
        self.hradi
    }

    pub fn fjarlaegd(&self) -> i32 {
        self.fjarlaegd
    }

    pub fn set_fjarlaegd(&mut self, ny_fjarlaegd: i32) {
        self.fjarlaegd = ny_fjarlaegd
    }

    pub fn minutur_til_flugvallar(&self) -> i32 {
        (self.fjarlaegd as f32 / self.hradi as f32 * 60.) as i32
    }
}

impl TryFrom<&str> for Flugvel {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // "tf-abc 100 200"
        let lidir = value.split_whitespace().collect::<Vec<&str>>();
        if lidir.len() < 3 {
            Err(format!(
                "Ekki nógu mörg orð til að búa til flugvél: '{}'",
                value
            ))
        } else {
            if let Ok(hradi) = lidir[1].parse::<u32>() {
                if let Ok(fjarlaegd) = lidir[2].parse::<i32>() {
                    Ok(Flugvel::new(lidir[0], hradi, fjarlaegd))
                } else {
                    Err(format!("Gat ekki breytt '{}' í fjarlægð!", lidir[2]))
                }
            } else {
                Err(format!("Gat ekki breytt '{}' í hraða!", lidir[1]))
            }
        }
    }
}

impl Display for Flugvel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, hraði: {} hn., fjarl.: {} sm., tími: {} mín.",
            self.kallmerki,
            self.hradi,
            self.fjarlaegd,
            self.minutur_til_flugvallar()
        )
    }
}
