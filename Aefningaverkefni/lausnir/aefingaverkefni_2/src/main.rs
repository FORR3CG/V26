mod hestur;
mod stada;

use hestur::Hestur;

fn main() {
    let mut sleipnir = Hestur::new(100, "Sleipnir", 15, "laus");
    println!("{}", sleipnir);
    // id: 100, nafn: Sleipnir, aldur: 15, staða: Laus

    sleipnir.breyta_stodu("leigður");
    sleipnir.haekka_aldur();
    println!("{}", sleipnir);
    // id: 100, nafn: Sleipnir, aldur: 16, staða: Leigður    
}
