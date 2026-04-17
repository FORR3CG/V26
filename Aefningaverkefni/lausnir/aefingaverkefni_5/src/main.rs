use std::io::Write;

use crate::adflugsstjorn::Adflugsstjorn;

mod adflugsstjorn;
mod flugvel;

fn main() {
    let mut fs = Adflugsstjorn::new();
    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("Gat ekki flush-að!");
        let mut inntak = String::new();
        std::io::stdin()
            .read_line(&mut inntak)
            .expect("Gat ekki lesið frá notanda!");
        let skipanir = inntak.split_whitespace().collect::<Vec<&str>>();
        match skipanir.first() {
            Some(skipun) => {
                match skipun.to_lowercase().as_str() {
                    "hætta" => break,
                    "skrá" => {
                        // skrá tf-abc 100 200
                        if let Err(villa) = fs.skra_flugvel(skipanir[1..].join(" ").as_str()) {
                            println!("{}", villa)
                        }
                    }
                    "prenta" => {
                        // prenta allt, prenta 250
                        if let Some(undirskipun) = skipanir.get(1) {
                            match *undirskipun {
                                "allt" => println!("{}", fs),
                                _ => match undirskipun.parse::<i32>() {
                                    Ok(fjarlaegd) => println!("{}",fs.flugvelar_inna_fjarlaegdar(fjarlaegd)),
                                    Err(_) => println!("Gat ekki breytt '{}' í fjarlægð!", undirskipun),
                                },
                            }
                        } else {
                            println!("Þú verður að setja inn undirskipun með 'prenta'!")
                        }
                    }
                    "uppfæra" => {
                        // uppfæra 40
                        if let Some(timi) = skipanir.get(1) {
                            if let Ok(minutur) = timi.parse::<u32>() {
                                fs.lata_minutur_lida(minutur);
                            } else {
                                println!("Gat ekki breytt '{}' í mínútur", timi)
                            }
                        } else {
                            println!("Þú verður að setja inn tölu með 'uppfæra'!")
                        }
                    }
                    _ => println!("Skil ekki skipunina: '{}'", skipun),
                }
            }
            None => println!("Þú verður að slá eitthvað inn!"),
        }
    }
}
