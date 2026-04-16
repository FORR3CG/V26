use std::fmt::Display;

#[derive(Debug)]
pub struct Litur(u8, u8, u8, u8);

impl From<u32> for Litur {
    fn from(value: u32) -> Self {
        let r = (value >> 24) as u8;
        let g = ((value >> 16) & 0xFF) as u8;
        let b = ((value >> 8) & 0xFF) as u8;
        let alpha = (value & 0xFF) as u8;
        Self(r, g, b, alpha)
    }
}

impl TryFrom<&str> for Litur {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // value = "65234"
        match value.parse::<u32>() {
            Ok(tala) => Ok(Litur::from(tala)),
            Err(_) => Err(format!("Litur: gat ekki breytt `{}` í tölu!!", value)),
        }
    }
}

impl Display for Litur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r: {}, g: {}, b: {}, alpha: {}", self.0, self.1, self.2, self.3)
    }
}