// 

pub struct Ferdagrunnur {
    id: u32,
    fjoldi_bokadra: u16,
    hamarksfjoldi: u16,
}

impl TryFrom<(u32, &str)> for Ferdagrunnur {
    type Error = String;

    // (234, "100 200")
    fn try_from(value: (u32, &str)) -> Result<Self, Self::Error> {
        let id = value.0;
        todo!()
    }
}