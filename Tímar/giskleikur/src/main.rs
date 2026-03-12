use std::{cmp::Ordering, io};
use rand::RngExt;

fn main() {
    println!("Giskaðu á töluna!");

    let leynitala = rand::rng().random_range(1..=100);
    loop {
        println!("Giskaðu núna: ");
        let mut inntak = String::new(); // býr tóman streng
        println!("stærð: {}, pláss: {}", inntak.len(), inntak.capacity()); // 0 og 0
        io::stdin()
            .read_line(&mut inntak)
            .expect("Gat ekki lesið frá notanda!!");
        
        let gisktala: u8 = match inntak.trim().parse() {
            Ok(tala) => tala,
            Err(_) => {
                println!("Gat ekki breytt {} í tölu!!!", inntak.trim());
                continue;
            }
        };
        match gisktala.cmp(&leynitala) {
            Ordering::Less => println!("{} er of lág!", gisktala),
            Ordering::Greater => println!("{} er of há!", gisktala),
            Ordering::Equal => {
                println!("{} er rétt!!!!!", gisktala);
                break;
            }
        }


        println!("Þú slóst inn {}", gisktala);
    }

}
