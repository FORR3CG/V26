use std::io;

fn main() {
    println!("Giskaðu á töluna!");

    loop {
        println!("Giskaðu núna: ");
        let mut inntak = String::new(); // býr tóman streng
        println!("stærð: {}, pláss: {}", inntak.len(), inntak.capacity()); // 0 og 0
        io::stdin().read_line(&mut inntak).expect("Gat ekki lesið frá notanda!!");
        
        let gisktala: u8 = match inntak.trim().parse() {
            Ok(tala) => tala,
            Err(_) => {
                println!("Gat ekki breytt {} í tölu!!!", inntak.trim());
                continue;
            }
        };

        println!("Þú slóst inn {}", gisktala);
    }

}
