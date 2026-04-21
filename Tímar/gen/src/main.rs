/* enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

Result<Bill, String>
Result<(), String> */

use std::{fmt::Display, str::FromStr};

struct Punktur<T> {
    x: T,
    y: T,
}

impl<T> Punktur<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y,}
    }
}

impl<T: Display> Display for Punktur<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn str_to_tala<T: FromStr>(texti: &str) -> Result<T, String> {
    match texti.parse::<T>() {
        Ok(tala) => Ok(tala),
        Err(_) => Err(format!("Gat ekki breytt {} í tölu", texti))
    }
}

impl<T: FromStr> TryFrom<&str> for Punktur<T> {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // "10 20", "3.14 4.15"
        let ordin = value.split_whitespace().collect::<Vec<&str>>();
        if ordin.len() != 2 {
            Err("Ekki hægt að gera Punkt!!".to_string())
        } else {
            let x = str_to_tala::<T>(ordin[0])?;
            let y = str_to_tala::<T>(ordin[1])?;
            Ok(Self {x, y})
        }
    }
}

//fn prenta_i_hastofum<T>(texti: T) where T: Display {
fn prenta_i_hastofum<T: Display>(texti: T) {
    println!("{}", texti.to_string().to_uppercase())
}

fn main() {
    let k = 23;
    prenta_i_hastofum(k);
    let p = Punktur::new(10_u8, 20);
    let p2 = Punktur::new(3.14, 4.15);
    let p3 = AnnarPunktur {x: 3.15, y: 20_u8};
    println!("{}", p);
    prenta_i_hastofum(p);
    println!("Hello, world!");
}


struct AnnarPunktur<T, U> {
    x: T,
    y: U,
}

impl<T, U> AnnarPunktur<T, U> {
    fn new(x: T, y: U) -> Self {
        Self {x, y}
    }
}

