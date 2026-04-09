
// kafli 5
// struct

use std::{collections::VecDeque, fmt::Display};

#[derive(Debug)]
struct Punktur {
    p1: i32,
    p2: i32,
    nafn: String,
    litur: Litur,
}

/*
class Punktur:
    def __init__

    def hitt

    def þetta
*/

impl Punktur {
    // __init__
    fn new(p1: i32, p2: i32, nafn: &str, litur: Litur) -> Self {
        Self {
            p1,
            p2,
            nafn: nafn.to_string(),
            litur,
        }
    }

    fn p1(&self) -> i32 {
        self.p1
    }

    fn set_p1(&mut self, nytt_p1: i32) {
        self.p1 = nytt_p1
    }

    fn haeka_p2(&mut self) {
        self.p2 += 10
    }
}

impl Display for Punktur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, texti: {}, litur: {}", 
                    self.p1, self.p2, self.nafn, self.litur)
    }
}

// kafli 6.1 
// enum

#[derive(Debug)]
enum Litur {
    Graenn,
    Blar,
    Annad,
}

impl Display for Litur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Litur::Blar => write!(f, "Blár litur"),
            Litur::Graenn => write!(f, "Grænn litur"),
            Litur::Annad => write!(f, "veit ekki"),
        }
    }
}

impl From<&str> for Litur {
    fn from(value: &str) -> Self {
        match value {
            "blár" => Litur::Blar,
            "græenn" | "graenn" | "g" => Litur::Graenn,
            _ => Litur::Annad,
        }
    }
}

fn main() {
    let punkur = Punktur 
        {p1: 23, p2: 44, nafn: "endi".to_string(), litur: Litur::Graenn};
    let punktur = Punktur::new(12, 14, "upphaf", Litur::Graenn);
    let litur = Litur::Blar;
    let blar = Litur::from("blue");
    println!("{}", litur);
    let punktstrengur = punktur.to_string();
    println!("{}", punktstrengur);
    println!("Hello, world!");

    let k = 1;
    match k {
        1 => println!("einn"),
        2 => println!("tveir"),
        _ => println!("annað"),
    }

    let mut listi = vec![10, 20, 30, 40];
    listi.pop();
    listi.push(99);
    listi.insert(0, 88);
    // [88, 10, 20, 30, 99]
    listi.remove(1);
    // [88, 20, 30, 99]
    listi.swap_remove(1);
    // [88, 99, 30]
    let mut vd = VecDeque::from([10, 20, 30]);
    vd.push_front(88);

}
