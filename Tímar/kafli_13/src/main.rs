const tala: i32 = 25;

fn main() {
    // closures
    const c: i32 = 5;
    let mut d = 10;
    // python: leggja_saman = lambda a, b: a + b
    let leggja_saman_closure = |a: i32, b: i32| -> i32 { a + b };

    let leggja_saman_closure = |a, b| a + b;
    leggja_saman_closure(1.23, 2.34);
    let leggja_saman_closure = |a, b| a + b;

    fn leggja_saman_fall(a: i32, b: i32) -> i32 {
        a + b + tala + c
    }

    let einfalt_fall = || {
        println!("ein lína");
        println!("önnur lína");
    };

    println!("{}", leggja_saman_closure(10, 20));
    println!("{}", leggja_saman_fall(10, 20));

    // iterators
    let mut listi = vec![1, 2, 3, 4, 5];
    // for stak in listi:
    for stak in listi.iter() {
        println!("{}", stak);
    }
    /*     let mut listi_it = listi.iter();
    println!("iter: {:?}", listi_it.next());
    println!("iter: {:?}", listi_it.next());
    println!("iter: {:?}", listi_it.next()); */

    listi.iter().for_each(|stak| println!("{}", stak));
    for stak in &mut listi {
        // listi.iter_mut()
        //*stak = *stak + 5;
        *stak += 5
    }
    listi.iter_mut().for_each(|stak| *stak += 5);

    let mut nyr_listi: Vec<i32> = Vec::new();
    for stak in &listi {
        nyr_listi.push(stak * 5);
    }

    let nyr_listi: Vec<i32> = listi
                                .iter()
                                .map(|stak| stak * 5)
                                .collect();
    let nyr_listi = listi.iter().map(|stak| stak * 5).collect::<Vec<i32>>();
    let mut listi = vec![10, 6, 3, 20, 29, 8];
    let summa: i32 = listi.iter().sum();
    println!("{}", listi.iter().sum::<i32>());
    listi.sort(); // hækkandi röð
    listi.sort_by(|a, b| a.cmp(b)); // hækkandi röð
    listi.sort_by(|a, b| b.cmp(a)); // lækkandi röð
/*     let a = 10; let b= 20;
    match a.cmp(&b) { // a < b, a==b, a > b
        std::cmp::Ordering::Less => todo!(),
        std::cmp::Ordering::Equal => todo!(),
        std::cmp::Ordering::Greater => todo!(),
    } */
    listi.reverse();
    println!("{:?}", listi);
    let k = 0;
    let samtala = listi
                        .iter()
                        .fold(10000_f64, |summa: f64, stak| summa / *stak as f64);
    let nyr_listi = listi.iter().filter(|stak| **stak < 10).collect::<Vec<&i32>>();
    println!("{:?}", nyr_listi);

    let v = vec![1,2,3,4,5];
    let k = v
                    .iter()
                    .filter(|stak| **stak > 3)
                    .fold(0, |summa, stak| summa + stak); 
    let k: i32 = v.iter().filter(|stak: &&i32| **stak > 2).sum();
    let k_ref = &k;
    let k_ref_ref = &k_ref;
    println!("{}", k);   

}
//https://github.com/FORR3CG/V26/blob/main/Aefningaverkefni/lausnir/aefingaverkefni_iter/lausnir.md