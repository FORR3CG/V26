

// 1. Skrifaðu fall sem tekur inn tvær u8 tölur, margfaldar þær saman og skilar niðurstöðunni sem u16.
fn margfalda(a: u8, b: u8) -> u16 {
    a as u16 * b as u16
}

// 2. Skrifaðu fall sem tekur inn u8 töluna n, reiknar n! (n hrópmerkt) og skilar niðurstöðunni sem u128. Ef inntakstalan n er stærra en 34 á fallið að skila 0 (núll).
fn hropmerkt(n: u8) -> u128 {
    if n > 34 {
        0
    } else {
        let mut skila: u128 = 1;
        for i in 1..=n {
            skila *= i as u128;
        }
        skila
    }
}
// eða endurkvæmt
fn hropmerkt_endurkvaemt(n: u8) -> u128 {
    if n == 0 {
        1
    } else {
        n as u128 * hropmerkt(n - 1)
    }
}

// 3. Skrifaðu fall sem tekur inn 3 (þrjár) i32 tölur, reiknar meðatal þeirra og skilar niðurstöðunni sem f32.
fn medaltal(a: i32, b: i32, c: i32) -> f32 {
    (a as f32 + b as f32 + c as f32) / 3_f32
}

// 4. Skrifaðu fall sem tekur inn 5 staka fylki af i32 tölum, reiknar meðaltal þeirra og skilar niðurstöðunni sem f64.
fn medaltal_fylki(fylki: [i32; 5]) -> f64 {
    let mut summa: i32 = 0;
    for i in fylki {
        summa += i
    }
    summa as f64 / fylki.len() as f64
}

// 5. Skrifaðu fall sem tekur inn 2 (tvær) i32 tölur og skilar tuple með tölunum í hækkandi röð.
fn haekkandi(a: i32, b: i32) -> (i32, i32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

fn main() {
    println!("10 * 20 = {}", margfalda(10, 20));
    println!("0! er {} {}", hropmerkt(0), hropmerkt_endurkvaemt(0));
    println!("5! er {} {}", hropmerkt(5), hropmerkt_endurkvaemt(5));
    println!("Meðaltal 10, 11 og 13 er: {:.2}", medaltal(10, 11, 13));
    println!("Meðaltal [3,4,5,6,8] er: {}", medaltal_fylki([3,4,5,6,8]));
    println!("7 og -3 í hækkandi röð er: {:?}", haekkandi(7, -3));
}
