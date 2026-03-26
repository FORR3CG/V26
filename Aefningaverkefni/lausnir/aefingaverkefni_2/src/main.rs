mod hestur;
mod stada;

use hestur::Hestur;
use stada::Stada;

fn main() {
    let mut sleipnir = Hestur::new(100, "Gráni", 15, Stada::Laus);
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Laus

    sleipnir.breyta_stodu(Stada::Leigdur);
    sleipnir.haekka_aldur();
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 16, staða: Leigður    
}
