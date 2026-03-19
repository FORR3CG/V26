fn main() {
    let a = 10;
    println!("Hello, world!");
    print!("1");
    print!("2");
    print!("3");
    print!("4");
    let x = 10u16;
    let x = 10_u16;
    let x: u16 = 10;
    // i8/u8, i16/u16, i32/u32, i64/u64, i128/u128
    // isize/usize
    let x = 1;
    let y = 2;
    let z = 1 as f32 / 2 as f32;
    // f32, f64
    let f = 1.05f32;
    let f = 1.05_f32;
    let f: f32 = 1.05;
    
    // bool, char
    let b = true; // eða false
    let c = 'g';
    let t = "g";

    let mut x = 0;
    x += 10;

    // túplur
    let mut t = (10, 3.14, true);
    t.0 = 30;
    println!("{} - {} - {}", t.0, t.1, t.2);
    let (a, _, c) = t;
    println!("{} - {} - {}", a, t.1, c);

    // fylki - array
    let fylki = [1_u8,2,3,4,5];
    let fylki = [1u8,2,3,4,5];
    let fylki: [u8; 5] = [1,2,3,4,5];
    let mut fylki = [0; 7];
    fylki[1] = 99;
    println!("fylkið: {:p}, annað stakið: {:p}", &fylki, &fylki[0]);
    println!("{}", 0x30 - 0x2c);

    // loop-ur
    loop { // alveg ein og while True: í python
        break;
    }

    let mut k = 5;
    while k > 0 {
        // gerum eitthvað
        k -= 1;
    }

    for i in (1..=5).rev().step_by(2) { // for i in range(5,1,-2)
        print!("{}", i);
    }

    // if 
    let a = 10;
    let b = 3;
    if a == 10 && b < 20 || !(a > 5) {
        println!("A er tíu");
    } else if a == 20 {
        println!("A er 20");
    } else {
        println!("A er eitthvað annað");
    }
    // js: let k = a > 10 ? 10 : 30;
    // python: k = 20 if a > 10 else 30
    let k = if a > 10 { 20 } else { 30 };

    let k: u8 = 10;
    let j: u8 = 20;
    let x = leggja_saman(k, j);
    println!("{} + {} = {}", k, j, x);
    let mut y = 11;
    haekka_um_einn(&mut y);
}

fn haekka_um_einn(a: &mut i32) {
    *a = *a + 1;
}

fn leggja_saman(a: u8, b: u8) -> u16 {
    // let summa: u16 = a as u16 + b as u16;
    // return summa;
    a as u16 + b as u16
}


