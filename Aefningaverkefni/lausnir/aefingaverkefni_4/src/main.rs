mod safnkostur;
mod stadsetning;
mod tegund;

use safnkostur::Safnkostur;

fn prenta_safnkost(safnkostur: Result<Safnkostur, String>) {
    match safnkostur {
        Ok(kostur) => println!("{}", kostur),
        Err(villa) => println!("{}", villa),
    }
}

fn main() {
    let a = Safnkostur::new(100, "Python", "bók", "hv");
    prenta_safnkost(a);
    // id: 100, titill: Python, tegund: Bók, staðsetning: Háteigsvegur

    let b = Safnkostur::new(101, "CSS", "tímarit", "hvergi");
    prenta_safnkost(b);
    // Gat ekki breytt 'hvergi' í staðsetningu!!

    let c = Safnkostur::new(102, "HTML", "myndband", "hv");
    prenta_safnkost(c);
    // Gat ekki breytt 'myndband' í tegund!!

    let r = Safnkostur::try_from("101 Rust tímarit sh");
    prenta_safnkost(r);
    // id: 101, nafn: Rust, tegund: Tímarit, staðsetning: Skólavörðuholt

    let s = Safnkostur::try_from("101 Linux handrit sh");
    prenta_safnkost(s);
    // Gat ekki breytt 'handrit' í tegund!!

    let t = Safnkostur::try_from("101 Cisco tímarit noregur");
    prenta_safnkost(t);
    // Gat ekki breytt 'noregur' í staðsetningu!!
}
