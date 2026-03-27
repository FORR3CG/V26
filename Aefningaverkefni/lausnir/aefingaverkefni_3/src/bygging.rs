use std::fmt::Display;

use crate::herbergi::Herbergi;

pub struct Bygging {
    herbergin: Vec<Herbergi>,
}

impl Bygging {
    pub fn new() -> Self {
        Self {
            herbergin: Vec::new(),
        }
    }

    pub fn skra_herbergi(&mut self, id: u32, lengd: f32, breidd: f32, tegund: &str) {
        self.herbergin.push(Herbergi::new(id, lengd, breidd, tegund));
    }

    pub fn breyta_staerd_herbergis_med_id(&mut self, id: u32, ny_lengd: f32, ny_breidd: f32) {
        for herbergi in &mut self.herbergin {
            if herbergi.id() == id {
                herbergi.breyta_staerd(ny_lengd, ny_breidd);
                println!("Breytti stærð herbergis nr: {}", id);
                return;
            }
        }
        println!("Fann ekkert herbergi með id: {}", id)
    }

    pub fn breyta_tegund_herbergis_med_id(&mut self, id: u32, ny_tegund: &str) {
        for herbergi in &mut self.herbergin {
            if herbergi.id() == id {
                herbergi.breyta_tegund(ny_tegund);
                println!("Breytti tegund herbergis nr: {}", id);
                return;
            }
        }
        println!("Fann ekkert herbergi með id: {}", id)
    }

    pub fn prenta_heildar_fermetrafjolda_byggingar(&self) {
        let mut heildar_fermetrafjoldi = 0.0;
        for herbergi in &self.herbergin {
            heildar_fermetrafjoldi += herbergi.fermetrar();
        }
        println!("Heildar stærð: {:.2} fm.", heildar_fermetrafjoldi)
    }

}

impl Display for Bygging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut oll_herbergi = String::new();
        for herbergi in &self.herbergin {
            oll_herbergi.push_str(format!("{}\n", herbergi).as_str());
        }
        write!(f, "{}", oll_herbergi.trim())
    }
}