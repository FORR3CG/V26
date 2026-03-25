mod bill;
mod gerd;
mod litur;

use bill::Bill;
use litur::Litur;
use gerd::Gerd;

fn main() {
    //let b = Bill { tegund: "Volvo".to_string(), gerd: gerd::Gerd::Folksbill, litur: Litur(255, 0, 0, 255), verd: 5000};
    //let b = Bill::new("Volvo", Gerd::Folksbill, 0xff0000ff, 5000);
    let b = Bill::new("Volvo", "jeppi", 0xff0000ff, 5000);
    println!("{:?}", b);
    println!("Hello, world!");
}
