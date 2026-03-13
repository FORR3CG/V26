const KLUKKUSTUNDIR_I_SOLARHRING: u8 = 24;
static ABC: i32 = 100;

fn main() {
    let y: i128 = 10;
    let mut x= 10_i128;
    x = x + 1;
    let klst = KLUKKUSTUNDIR_I_SOLARHRING;
    let a = ABC;
    let nafn = "Geir";
    let nafn: i32 = 10;

    // i8/u8, i16/u16, i32/u32, i64/u64, i128/u128, isize/usize
    // i == signed -(2^(n-1)) upp í 2^(n-1)-1
    // u == unsigned 0 upp í 2^n - 1
    let x: u8 = 255;
    //let y: i8 = 255; 
    let k = 123_i16;
    let j = 0o555;
    let idx: usize = 23;

    let a = 9;
    let b = 10;
    let c = a / b; // c verður 0
    let c = a as f32 / b as f32;
    let a = 10_i8;
    let b = 256_i16;
    let c: i16 = a as i16 + b;
    println!("{}", c);
    // %
    let k = 3.14;
    let j: f32 = 3.14;
    let k = 3.14_f32;
    let j = 10.;

    let satt = true; // false
    let stafur = "g";
    let stafur = 'g';
    let mut stafur = b'A';
    stafur = stafur | 32;
    println!("{}", stafur as char);

    let t = (10_u8, 3.14_f32, 'A');
    let (a, b, c) = t;
    let (_, b, _) = t;
    let k = t.2;

    // array - fylki 
    let mut f = [1_u8,2,3,4,5];
    let idx: usize = 2;
    let stak_nr_2 = f[idx];
    let f1 = [0; 5];
    println!("{:?}", f1);
    let f2 = [[0; 5]; 10];
    println!("{:#?}", f2);

    for stak in f1 {
        println!("{}", stak);
    }

    for lina in f2 {
        for stak in lina {
            println!("{}", stak);
        }
    }

    // for i in range(1,11,1):
    for i in 1..=10 {
        print!("{}", i);
    }

    // for i in range(5,1,-1):
    for i in (1..=5).rev() {
        print!("{}", i)
    }

    // for i in range(10, 0, -3):
    for i in (1..=10).rev().step_by(3) {
        print!("{}", i)
    }

    // while - alveg eins og í python
    let mut k = 5; 
    while k > 0 {
        println!("tskóli");
        k -= 1;
    }

    // loop lykkjan alveg eins og while True í python
    // python - rust
    // and    - &&
    // or     - ||
    // not    _ !
    if a > 10 || b == 30. {
        println!("Gerum eitthvað");
    } else if a < 5 {
        println!("Gerum eitthvað annað");
    } else {
        println!("eitthvað annað");
    }

    let k = 10;
    match k {
        1 => println!("K er einn"),
        2..4 => {
            println!("K getur verið 2");
            println!("K getur verið 3");
        }
        _ => println!("K er stærra en 3"),
    }

    let val = String::from("1\n");
    match val.trim() {
        "1" => println!("Þú valdir 1"),
        "2" => println!("Þú valdir 2"),
        _ => println!("Rangt val, veldu aftur!!"),
    }

}
