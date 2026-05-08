mod dyr;
mod dyragardur;
mod dyragrunnur;
mod hundur;
mod kottur;

use dyragardur::Dyragardur;

use std::io::{Read, Write};

/*
    bættum við:
    cargo add serde -F derive
    cargo add serde_json
*/

fn main() {
    let mut dg = Dyragardur::new(None);
    dg.skra_hund("Snati 1000");
    dg.skra_hund("Snotra 8000");
    dg.skra_kott("Tluffy 77");
    dg.skra_hund("Tryggur 6000");
    dg.skra_hund("Lotta 5000");
    dg.skra_kott("Grettir 15");
    dg.skra_kott("Gréta 10");
    dg.skra_kott("Tluffy 7");
    match dg.haekka_aldur_kattar_med_id(9999, 11) {
        Ok(_) => println!("hækkaði aldurinn"),
        Err(villa) => println!("{}", villa),
    }
    println!("{}", dg);

    // býr til json úr dýragarðinu og skrifar í skrá
    let json_gogn = serde_json::to_string_pretty(&dg).unwrap();
    println!("{}", json_gogn);
    let mut skra = std::fs::File::create("gogn.json").unwrap();
    write!(skra, "{}", json_gogn);

    // lesa json skjal og búa til dýragarð
    let mut skra = std::fs::File::open("gogn.json").unwrap();
    let mut fra_skra = String::new();
    skra.read_to_string(&mut fra_skra).unwrap();
    let mut dg2 = serde_json::from_str::<Dyragardur>(&fra_skra).unwrap();    
    
    
    println!("--------------------------------------");
    println!("{}", dg2);
}
