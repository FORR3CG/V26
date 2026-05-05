use std::fmt::Display;

pub struct Dyragrunnur {
    id: u32,
    nafn: String,
}

impl Dyragrunnur {
    pub fn new(id: u32, nafn: &str) -> Self {
        Self {
            id,
            nafn: nafn.to_string(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn nafn(&self) -> &str {
        self.nafn.as_str()
    }

    pub fn set_nafn(&mut self, nafn: &str) {
        self.nafn = nafn.to_string()
    }
}

impl Display for Dyragrunnur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, nafn: {}", self.id, self.nafn)
    }
}

impl From<(u32, &str)> for Dyragrunnur {
    fn from(value: (u32, &str)) -> Self {
        Self {
            id: value.0,
            nafn: value.1.to_string(),
        }
    }
}