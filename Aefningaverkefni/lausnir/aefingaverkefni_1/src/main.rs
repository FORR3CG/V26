use rand;
use std::io::Write;

fn main() {
    let mut tolurnar = [[0u8; 10]; 10];
    let mut talning = [0u8; 10];

    // fylla tvíviða fylkið og telja hverja tölu og setja í einvíða fylkið
    for i in 0..10 {
        for j in 0..10 {
            let r_tala = rand::random_range(0..10);
            tolurnar[i][j] = r_tala;
            talning[r_tala as usize] += 1;
        }
    }

    // finna há- og lágmörk
    let (mut maxfjoldi, mut minfjoldi) = (0u8, 255u8);
    let (mut maxtala, mut mintala) = (0u8, 0u8);

    for i in 0..10 {
        if talning[i] > maxfjoldi {
            maxfjoldi = talning[i];
            maxtala = i as u8;
        }
        if talning[i] < minfjoldi {
            minfjoldi = talning[i];
            mintala = i as u8;
        }
    }

    loop {
        println!("1. Sýna allar tölurnar.");
        println!("2. Sýna fjölda af hverri tölu.");
        println!("3. Sýna töluna sem kom oftast upp.");
        println!("4. Sýna töluna sem kom sjaldnast upp.");
        println!("0. Hætta.");
        print!("Veldu núna: ");

        std::io::stdout().flush().expect("Gat ekki flush-að!");
        let mut inntak = String::new();
        std::io::stdin().read_line(&mut inntak).expect("Gat ekki lesið frá notanda!");

        match inntak.trim() {
            "0" => break,
            "1" => {
                for i in 0..10 {
                    for j in 0..10 {
                        print!("{:2}", tolurnar[i][j]);
                    }
                    println!();
                }
            }
            "2" => {
                for i in 0..10 {
                    println!("{} kom upp {} sinnum.", i, talning[i]);
                }
            }
            "3" => println!("{} kom oftast upp eða {} sinnum.", maxtala, maxfjoldi),
            "4" => println!("{} kom sjaldnast upp eða {} sinnum.", mintala, minfjoldi),
            _ => {
                println!("Ekki rétt valið reyndu aftur!");
            }
        }
    }
}
