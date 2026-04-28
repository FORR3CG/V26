use std::f64::consts::PI;

trait Form {
    fn teikna(&self) -> String;

    fn flatarmal(&self) -> u64;
}

struct Punktur {
    x: u32,
    y: u32,
}

impl Punktur {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

struct Hringur {
    midja: Punktur,
    radius: u32,
}

impl Hringur {
    fn new(x: u32, y: u32, radius: u32) -> Self {
        Self {
            midja: Punktur::new(x, y),
            radius,
        }
    }
}

impl Form for Hringur {
    fn teikna(&self) -> String{
        "Teiknar hring!".to_string()
    }

    fn flatarmal(&self) -> u64 {
        ((self.radius * self.radius) as f64 * PI) as u64
    }
}

struct Lina {
    upphaf: Punktur,
    endir: Punktur,
}

impl Lina {
    fn new(upphaf_x: u32, upphaf_y: u32, endir_x: u32, endir_y: u32) -> Self {
        Self {
            upphaf: Punktur::new(upphaf_x, upphaf_y),
            endir: Punktur::new(endir_x, endir_y),
        }
    }
}

impl Form for Lina {
    fn teikna(&self) -> String {
        "Teiknar línu!".to_string()
    }

    fn flatarmal(&self) -> u64 {
        0
    }
}

struct Retthyrningur {
    nedra_vinstra_horn: Punktur,
    breidd: u32,
    haed: u32,
}

impl Retthyrningur {
    fn new(nvh_x: u32, nvh_y: u32, breidd: u32, haed: u32) -> Self {
        Self {
            nedra_vinstra_horn: Punktur::new(nvh_x, nvh_y),
            breidd,
            haed,
        }
    }
}

impl Form for Retthyrningur {
    fn teikna(&self) -> String {
        "Teiknar rétthyrning".to_string()
    }

    fn flatarmal(&self) -> u64 {
        self.breidd as u64 * self.haed as u64
    }
}

fn teikna_og_syna_flatarmal(form: &dyn Form) {
    println!("{}, flatarmálið er: {}", form.teikna(), form.flatarmal())
}

fn main() {
    let mut teikniform: Vec<Box<dyn Form>> = Vec::new();
    teikniform.push(Box::new(Lina::new(10,10, 20, 20)));
    teikniform.push(Box::new(Hringur::new(10, 10, 30)));
    teikniform.push(Box::new(Retthyrningur::new(10,10, 10,10)));
    teikniform.iter().for_each(|form| teikna_og_syna_flatarmal(form.as_ref()));
}
