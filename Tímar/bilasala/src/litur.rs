
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