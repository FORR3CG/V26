use std::fmt::Display;

use crate::bill::Bill;

pub struct Bilasala {
    bilarnir: Vec<Bill>,
}

impl Bilasala {
    pub fn new() -> Self {
        Self {
            // Ekkert að gera hér nema að búa til tilvik af vectornum
            bilarnir: Vec::new(),
        }
    }

    // Fallið tekur inn allar upplýsingar um einn bíl. Býr hann til og setur í vectorinn.
    // Verðum að fá &mut self, af því við erum að "breyta" vectornum (skrá eitthvað í hann)
    pub fn skra_bil(&mut self, id: u32, tegund: &str, gerd: &str, litur: u32, verd: u32) {
        let b = Bill::new(id, tegund, gerd, litur, verd);
        self.bilarnir.push(b);
        // má einnig gera svona:
        // self.bilarnir.push(Bill::new(id, tegund, gerd, litur, verd));
    }

    // prentar út einn bíl eftir id, prentar viðeigandi meldingu ef enginn bíll
    // með það id finnst. Almennt viljum við ekki vera með print skipun í 
    // svona struct en við betrumbætum þetta þegar við erum búin að læra um Result
    pub fn skoda_bil_med_id(&self, id: u32) {
        for bill in &self.bilarnir {
            if bill.id() == id {
                println!("{}", bill);
                // ef bið erum búin að finna bíl, hættum við í fallinu
                return
            }
        }
        // Ef við komumst hingað höfum við ekki fundið bíl með id
        println!("Fann engan bíl með id: {}", id)
    }

    pub fn haekka_um_prosent_verd_a_bil_med_id(&mut self, id: u32, prosent: u8) {
        for bill in &mut self.bilarnir {
            if bill.id() == id {
                bill.haekka_verd_um_prosent(prosent);
                println!("Hækkaði verð á bíl með id: {} um {}%", id, prosent);
                return
            }
        }
        println!("Fann engan bíl með id: {}", id)
    }

    pub fn eyda_bil_med_id(&mut self, id: u32) {
        // þurfum að finna index-inn á bílnum sem á að eyða
        for idx in 0..(self.bilarnir.len()) {
            if self.bilarnir[idx].id() == id {
                self.bilarnir.remove(idx);
                println!("Eyddi bíl með id: {}", id);
                return
            }
        }
        println!("Fann engan bíl með id: {}", id)
    }

    // Fall sem reiknar verðmæti allra bílanna
    pub fn verdmaeti(&self) -> u64 {
        let mut verdmaeti = 0_u64;
        for bill in &self.bilarnir { // fáum reference á hvern bíl
            verdmaeti += bill.verd() as u64
        }
        verdmaeti
    }
}

impl Display for Bilasala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // eins og venjulegt Display fyrir utan að við getum bara kallað einu sinni
        // á write! þar sem það return-ar úr fallinu.
        // Þurfum því að búa til einn streng með öllum bílunum
        let mut skilastrengur = String::from("");
        for bill in &self.bilarnir {
            // Display útfærir to_string() sjálkrafa en push_str fallið vill fá &str.
            // Bið breytum því strengnum úr String í &str með .as_str()
            skilastrengur.push_str(bill.to_string().as_str());
            // push fallið tekur bara við einum staf
            skilastrengur.push('\n');
        }
        // bætum við trim() til að henda út síðasta \n
        write!(f, "{}", skilastrengur.trim())
    }
}