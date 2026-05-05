mod dyr;
mod dyragardur;
mod dyragrunnur;
mod hundur;
mod kottur;

use dyragardur::Dyragardur;

fn main() {
    let mut dg = Dyragardur::new(None);
    dg.skra_hund("Snati 1000");
    dg.skra_hund("Snotra 8000");
    dg.skra_hund("Tryggur 6000");
    dg.skra_hund("Lotta 5000");
    dg.skra_kott("Grettir 15");
    dg.skra_kott("Gréta 10");
    dg.skra_kott("Fluffy 7");
    println!("{}", dg);
}
