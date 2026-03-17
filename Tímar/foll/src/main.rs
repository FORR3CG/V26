fn main() {
    println!("Hello, world!");
    let k = 10;
    let k = haekka_um_einn(k);
    // let k: u16 = haekka_um_einn(10);
    let mut j = 12;
    haekka_um_tvo(&mut j);
    println!("{}", j);
    let f = [1,2,3,4,5];
    prenta_fylki(&f);
}

fn prenta_fylki(fylki: &[i32; 5]) {
    for stak in fylki {
        println!("{}", stak)
    }
}

// pass by reference
fn haekka_um_tvo(a: &mut i32) {
    *a = *a + 2
}

// pass by value
fn haekka_um_einn(a: u8) -> u16 {
    //return a as u16 + 1;
    a as u16 + 1
}

fn prenta_geir() {
    println!("ABC");
    println!("Geir")
}

fn fall_sem_skilar_fimm() -> i32 {
    // return 5;
    5
}

