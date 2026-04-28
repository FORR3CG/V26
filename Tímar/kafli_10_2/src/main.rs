// Traits, kafli 10.2 - Defining Shared Behavior
//                      Skilgreina sameiginlega hegðun

use std::fmt::Display;

#[derive(Debug)]
struct Hundur {
    nafn: String,
    einkunn: i8,
}

impl Hundur {
    fn new(nafn: &str, einkunn: i8) -> Self {
        Self {nafn: nafn.to_string(), einkunn}
    }
}

#[derive(Debug)]
struct Kottur {
    nafn: String,
    aldur: u8,
}

impl Kottur {
    fn new(nafn: &str, aldur: u8) -> Self {
        Self { nafn: nafn.to_string(), aldur }
    }

    fn skra_aldur(&mut self, nyr_aldur: u8) {
        self.aldur = nyr_aldur
    } 
}

trait Dyrahljod {
    fn segir(&self) -> String;

    fn hallo(&self) -> String {
        "Halló".to_string()
    }

    fn tegund(&self) -> String;
}

impl Dyrahljod for Hundur {
    fn segir(&self) -> String {
        "Voff".to_string()
    }

    fn tegund(&self) -> String {
        "Hundur".to_string()
    }
}

impl Dyrahljod for Kottur {
    fn segir(&self) -> String {
        "Mjá".to_string()
    }

    fn hallo(&self) -> String {
        "Mjalló".to_string()
    }

    fn tegund(&self) -> String {
        "Köttur".to_string()
    }
}

fn hvad_segir_dyrid_dyn(dyr: &dyn Dyrahljod) {
    println!("{} segir {}", dyr.tegund(), dyr.segir())
}

fn hvad_segir_dyrid_gen<T: Dyrahljod>(dyr: &T) {
    println!("{} segir {}", dyr.tegund(), dyr.segir())
}

struct Bill {
    id: u64,
    tegund: String,
    litur: String, 
}

impl Dyrahljod for Bill {
    fn segir(&self) -> String {
        "Brrum".to_string()
    }

    fn tegund(&self) -> String {
        "Bíll".to_string()
    }
}

impl Dyrahljod for i32 {
    fn segir(&self) -> String {
        "eitthvað talnalegt".to_string()
    }

    fn tegund(&self) -> String {
        "Tala".to_string()
    }
}

impl Display for Kottur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.nafn, self.aldur)
    }
}

impl PrentaIHastofum for Kottur {
    fn prenta(&self) -> String {
        self.to_string().to_uppercase()
    }
}

// Supertrait: kafli 20.2
trait PrentaIHastofum: std::fmt::Display {
    fn prenta(&self) -> String;
}

fn main() {
    let k = Box::new(23);
    let j = Box::new(27_u8);
    let h_p = Box::new(Hundur::new("Snotra", 9));
    let k_p = Box::new(Kottur::new("Fluffy", 5));
    let mut dyragardur: Vec<Box<dyn Dyrahljod>> = Vec::new();
    dyragardur.push(h_p);
    dyragardur.push(k_p);
    for dyr in &dyragardur {
        hvad_segir_dyrid_dyn(dyr.as_ref());
    }
    hvad_segir_dyrid_gen(&37);
    let bill = Bill { id: 123, 
                            tegund: "Toyota".to_string(), 
                            litur: "Blár".to_string()
                        };
    hvad_segir_dyrid_gen(&bill);
    let hundur = Hundur::new("Snati", 5);
    let kottur = Kottur::new("Grettir", 13);
    hvad_segir_dyrid_dyn(&hundur);
    hvad_segir_dyrid_dyn(&kottur);
    hvad_segir_dyrid_gen(&hundur);
    println!("{}", hundur.hallo());
    println!("{}", kottur.hallo());
    
}
