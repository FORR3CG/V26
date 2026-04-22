use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
struct Punktur {
    nafn: String,
    x: i32,
    y: i32,
}

fn str_to_tala<T: FromStr>(texti: &str) -> Result<T, String> {
    match texti.parse::<T>() {
        Ok(tala) => Ok(tala),
        Err(_) => Err("Gat ekki breyt í tölu!!".to_string())
    }
}

impl TryFrom<&str> for Punktur {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // "upphafs 30 40" 
        let ordin = value.split_whitespace().collect::<Vec<&str>>();
        // eða
        let ordin: Vec<&str> = value.split_whitespace().collect();
        if ordin.len() != 3 {
            Err("Ekki nógu mörg orð!!".to_string())
        } else {
            let nafn = ordin[0].to_string();
            /* let x = match ordin[1].parse::<i32>() {
                Ok(tala) => tala,
                Err(_) => return Err("gat ekki breytt í tölu!!".to_string()),
            }; */
            let x = str_to_tala(ordin[1])?;
            /* if let Ok(y) = ordin[2].parse::<i32>() {           
                Ok( Self { nafn, x, y })
            } else {
                Err("Gat ekki breytt í tölu!!".to_string())
            }  */
            let y = str_to_tala(ordin[2])?;
            Ok(Self {nafn, x, y})
        }
    }
}

impl Display for Punktur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nafn: {}, x: {}, y: {}", self.nafn, self.x, self.y)
    }
}

fn leggja_saman_fall(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let p = Punktur { nafn: "endapunktur".to_string(), x: 10, y: 20};
    let p2 = Punktur::try_from("upphaf 30 40");
    println!("{:?}", p);
    let leggja_saman_closure = | a, b | { 
        println!("{} + {}", a, b);
        a + b
    };
    println!("{}", leggja_saman_fall(10, 20));
    println!("{}", leggja_saman_closure(10, 20));
    let mut v = vec![10, 20, 30, 40, 50];
    v.iter_mut().for_each(|stak| *stak += 1 /* *stak = *stak + 1  */);
    let v = v
                        .iter()
                        .map(|stak| stak + 1)
                        .collect::<Vec<i32>>();
    let mut v = vec![4, 1, 6, 8, 2];
    v.iter().for_each(|stak| {stak + 1;});
    v.sort_by(|a, b| a.cmp(&b)); // hækkandi röð
    v.sort_by(|a, b| b.cmp(&a)); // lækkandi röð
    let v = vec![1,2,3,4,5];
    let k: i32 = v.iter().filter(|stak| **stak > 3).sum();
    let samtala = v
                        .iter()
                        .fold(15, |summa, stak| summa + stak);
    
}
