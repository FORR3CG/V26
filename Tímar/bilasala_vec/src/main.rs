mod bill;
mod gerd;
mod litur;
mod bilasala;

use std::io::Write;

use bilasala::Bilasala;

fn main() {
    // hjálp
    // hætta
    // skrá 101 Volvo fb 345435 3500
    // selja 101
    // hækka 101 20
    // prenta allt
    // prenta 101
    // prenta verð
    let mut bs = Bilasala::new();
    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("Gat ekki flush-að");
        let mut inntak = String::new();
        std::io::stdin()
                .read_line(&mut inntak).expect("Gat ekki lesið frá notanda!");
        let skipanir = inntak.split_whitespace().collect::<Vec<&str>>();
        match skipanir.first() {
            Some(skipun) => {
                match skipun.to_lowercase().as_str() {
                    "hjálp" => println!("Prentum út svaka góða hjálp!"),
                    "hætta" => break,
                    "skrá" => {
                        if let Err(villa) = 
                            bs.skra(skipanir[1..].join(" ").as_str()) {
                                println!("{}", villa)
                            }
                    },
                    _ => println!("Skildi ekki skipunina {}", skipun)
                }
            },
            None => println!("Þú verður að slá eitthvað inn!!!")
        }
    }    
    
}
