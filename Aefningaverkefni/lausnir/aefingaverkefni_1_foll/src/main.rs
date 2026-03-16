use std::io::Write;

fn fylla_fylki_med_random(tolurnar: &mut [[u8; 10]; 10]) -> [u8; 10] {
    let mut talning: [u8; 10] = [0; 10];
    for linur in tolurnar {
        for dalkur in linur {
            let r_tala: u8 = rand::random_range(0..10);
            *dalkur = r_tala;
            talning[r_tala as usize] += 1;
        }
    }
    talning
}

fn finna_hamark(fylki: &[u8; 10]) -> (u8, u8) {
    let (mut maxfjoldi, mut maxtala) = (0_u8, 0_u8);
    for (i, fjoldi) in fylki.iter().enumerate() {
        if *fjoldi > maxfjoldi {
            maxfjoldi = *fjoldi;
            maxtala = i as u8;
        }
    }
    (maxfjoldi, maxtala)
}

fn finna_lagmark(fylki: &[u8; 10]) -> (u8, u8) {
    let (mut minfjoldi, mut mintala) = (255_u8, 255_u8);
    for (i, fjoldi) in fylki.iter().enumerate() {
        if *fjoldi < minfjoldi {
            minfjoldi = *fjoldi;
            mintala = i as u8;
        }
    }
    (minfjoldi, mintala)
}

fn prenta_valmynd() {
    println!("1. Sýna allar tölurnar.");
    println!("2. Sýna fjölda af hverri tölu.");
    println!("3. Sýna töluna sem kom oftast upp.");
    println!("4. Sýna töluna sem kom sjaldnast upp.");
    println!("0. Hætta.");
    print!("Veldu núna: ");
}

fn lesa_fra_notanda() -> String {
    std::io::stdout().flush().expect("Gat ekki flush-að!");
    let mut inntak = String::new();
    std::io::stdin().read_line(&mut inntak).expect("Gat ekki lesið frá notanda!");
    inntak
}

fn prenta_allar_tolur(fylkid: &[[u8; 10]; 10]) {
    for linur in fylkid {
        for dalkur in linur {
            print!("{:2}", dalkur);
        }
        println!();
    }
}

fn prenta_fjolda_hverrar_tolu(fylkid: &[u8; 10]) {
    for (i, tala) in fylkid.iter().enumerate() {
        println!("{} kom upp {} sinnum.", i, tala);
    }
}

fn main() {
    let mut tolurnar = [[0u8; 10]; 10];
    let talning = fylla_fylki_med_random(&mut tolurnar);

    let (maxfjoldi, maxtala) = finna_hamark(&talning);
    let (minfjoldi, mintala) = finna_lagmark(&talning);
    
    loop {
        prenta_valmynd();
        match lesa_fra_notanda().trim() {
            "0" => break,
            "1" => prenta_allar_tolur(&tolurnar),
            "2" => prenta_fjolda_hverrar_tolu(&talning),
            "3" => println!("{} kom oftast upp eða {} sinnum.", maxtala, maxfjoldi),
            "4" => println!("{} kom sjaldnast upp eða {} sinnum.", mintala, minfjoldi),
            _ => println!("Ekki rétt valið reyndu aftur!"),
        }
    }
}
