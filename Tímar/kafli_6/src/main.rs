
// class Dagar
// struct Dagar

use std::fmt::Display;

enum Dagar {
    Manudagur, // 0
    Thridjudagur, // 1
    Midvikudagur,
    Fimmtudagur,
    Fostudagur,
    Laugardagur,
    Sunnudagur,
}

enum IpTala {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}

impl Display for IpTala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpTala::IPv4(a,b ,c ,d ) => write!(f, 
                                                                    "{}.{}.{}.{}", 
                                                                    a, b, c, d),
            IpTala::IPv6(s) => write!(f, "{}", s),
        }
    }
}

fn main() {
    let d = Dagar::Thridjudagur;

    match d {
        Dagar::Manudagur => println!("Vikan var að byrja."),
        Dagar::Thridjudagur | Dagar::Midvikudagur => println!("Það er þriðjudagur eða miðvikudagur"),
        Dagar::Fimmtudagur => println!("Það er fimmtudagur"),
        _ => println!("Það er löng helgi"),
    }

    let v = 23;
    match v {
        1 => println!("einn"),
        2..=5 => println!("tveir til og með 5"),
        7 => {
            println!("margar línur");
            println!("sjö")
        },
        _ => println!("einhver önnur tala"),
    }

    let localhost_ipv4 = IpTala::IPv4(127, 0, 0, 1);
    let localhost_ipv6 = IpTala::IPv6("::1".to_string());
    println!("ipv4: {}, ipv6: {}", localhost_ipv4, localhost_ipv6);
    println!("Hello, world!");

}
