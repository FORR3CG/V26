mod bill;
mod gerd;
mod litur;

use bill::Bill;

// "cargo add rand" í terminal eða bæta línunni, rand = "0.10.0" undir [dependencies] í Cargo.toml
use rand::random_range;

fn main() {
    //let b = Bill { tegund: "Volvo".to_string(), gerd: gerd::Gerd::Folksbill, litur: Litur(255, 0, 0, 255), verd: 5000};
    //let b = Bill::new("Volvo", Gerd::Folksbill, 0xff0000ff, 5000);
    let b = Bill::new("Volvo", "jeppi", 0xff0000ff, 5000);
    // fylki (e. array) til að búa til random bíla
    let tegundir = ["BMW", "Ford", "Kia", "Volvo"];
    let gerdir = ["fólksbíll", "jeppi", "vörubíll", "annað"];
    let verd = [2000, 3000, 4000, 5000];
    let litir = [0xff0000ff, 0xff1f, 0xff00f0, 0xff001ff0];
    // Vector (ekki array) til að geyma bílana í, sjálfstækkandi (svipað og listi í python)
    let mut bilar = Vec::new();
    // búum til 10 random bíla
    for _ in 0..10 {
        let t: usize = random_range(0..4);
        let g: usize = random_range(0..4);
        let l: usize = random_range(0..4);
        let v: usize = random_range(0..4);
        // push er eins og append á lista í python
        bilar.push(Bill::new(tegundir[t], gerdir[g], litir[l], verd[v]));
    }

    // verðum að hafa reference á bílar til að for-lykkjan taki ekki ownership
    for bill in &bilar {
        println!("{}", bill)
    }

    // reikna heildarverð allra bílanna
    let mut heildarverd = 0_u32;
    for bill in &bilar {
        heildarverd += bill.verd()
    }
    println!("Heildarverð: {}", heildarverd);

    // hækka verðið á öllum bílunum um 20%, verðum að fá mut reference til að mega breyta bílunum
    for bill in &mut bilar {
        bill.haekka_verd_um_prosent(20);
    }

    heildarverd = 0;
    for bill in &bilar {
        heildarverd += bill.verd()
    }
    println!("Heildarverð: {}", heildarverd);

}
