mod bill;
mod gerd;
mod litur;
mod bilasala;

use bilasala::Bilasala;
// "cargo add rand" í terminal eða bæta línunni, rand = "0.10.0" undir [dependencies] í Cargo.toml
use rand::random_range;

fn main() {
    // fylki (e. array) til að búa til random bíla
    let tegundir = ["BMW", "Ford", "Kia", "Volvo"];
    let gerdir = ["fólksbíll", "jeppi", "vörubíll", "annað"];
    let verd = [2000, 3000, 4000, 5000];
    let litir = [0xff0000ff, 0xff1f, 0xff00f0, 0xff001ff0];
    // Tilvik af bílasölu struct-inu
    let mut bilar = Bilasala::new();
    // búum til 10 random bíla
    for id in 1..=10 {
        let t: usize = random_range(0..4);
        let g: usize = random_range(0..4);
        let l: usize = random_range(0..4);
        let v: usize = random_range(0..4);
        // notum skra_bil fallið úr bílasölu struct-inu
        bilar.skra_bil(id, tegundir[t], gerdir[g], litir[l], verd[v]);
    }

    println!("Allir bílarnir:\n{}", bilar); // ætti að prenta út 10 bíla
    println!("---------------------------------");
    bilar.skoda_bil_med_id(5); // ætti að prenta einn bíl    
    bilar.haekka_um_prosent_verd_a_bil_med_id(5, 50); // ætti að segjast hafa hækkað verð á bíl með id 5
    bilar.skoda_bil_med_id(5); // ætti að sýna bíl með id 5 með nýja verðinu
    println!("---------------------------------");
    bilar.eyda_bil_med_id(5); // ætti að segjast hafa eytt bíl með id 5
    bilar.skoda_bil_med_id(5); // ætti að koma með meldingu um að bíll með id 5 finnist ekki
    println!("---------------------------------");
    println!("Heildarverðmæti: {}", bilar.verdmaeti()); // ætti að skrifa út heildarverðmæti allra bílanna

}
