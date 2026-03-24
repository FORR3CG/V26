use std::fmt::Display;
mod punktur;
use punktur::Punktur;

// unit struct
struct Reikniadgerdir;

impl Reikniadgerdir {
    fn leggja_saman(a: i32, b: i32) -> i32 {
        a + b
    }

    fn margfalda(a: u8, b: u8) -> u16 {
        a as u16 * b as u16
    }
}

// tuple struct
#[derive(Debug)]
struct Litur(u8, u8, u8);

impl Litur {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    fn red(&self) -> u8 {
        self.0
    }

    fn set_blue(&mut self, blue: u8) {
        self.2 = blue
    }
}

impl Display for Litur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r: {}, g: {}, b: {}", self.0, self.1, self.2)
    }
}


/* fn bua_til_punkt(x: i32, y: i32) -> Punktur {
    Punktur { x, y }
} */

fn main() {
    //println!("Hello, world!");
    //let mut p = Punktur {x: 23, y: 47};
    let mut p2 = Punktur::new(10, 20);
    //let p3 = bua_til_punkt(10, 20);
    let s = p2.to_string();
    println!("{}", p2);
    println!("{}", Reikniadgerdir::margfalda(10, 20));
    //println!("{}",p2.x());
    //println!("{}", p.x);
    p2.y = 99;
    prenta_punkt(&p2);
    //println!("{}", p.x); 
}

fn prenta_punkt(punktur: &Punktur) {
    println!("x: {}, y: {}", punktur.x(), punktur.y)
}
