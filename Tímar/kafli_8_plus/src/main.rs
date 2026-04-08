//use std::collections::VecDeque;

fn main() {
    let mut fylki = [10, 20, 30 ,40]; // geymt á stack
    //fylki.push(50);
    let mut listi = vec![10, 20, 30, 40]; // geymt á heap
    println!("stærð: {}, pláss: {}, {:?}", listi.len(), listi.capacity(), listi);
    listi.push(50); // bætir aftast í listann
    listi.push(50); // bætir aftast í listann
    listi.push(50); // bætir aftast í listann
    listi.push(50); // bætir aftast í listann
    println!("stærð: {}, pláss: {}, {:?}", listi.len(), listi.capacity(), listi);
    listi.pop(); // tekur síðasta stakið
    listi.remove(1);
    listi.swap_remove(1);
    listi.insert(0, 99);
    
    let mut listi = vec![1,2,3,4,5];
    listi[0] = 99;
    
    for stak in &listi {
        print!("{}", stak)
    }
    
    for stak in &mut listi {
        *stak = *stak + 1
    }
    
    for idx in 0..listi.len() {
        listi[idx] += 5;
        print!("{}", listi[idx])
    }
    
    for (idx, stak) in listi.iter().enumerate() {
        println!("idx: {}, stak: {}", idx, stak)
    }
    
    println!("stærð: {}, pláss: {}, {:?}", listi.len(), listi.capacity(), listi);
    
    //println!("stak nr. 99: {}", listi[99]);
    println!("stak nr. 99: {:?}", listi.get(99));
    println!("stak nr. 1: {:?}", listi.get(1));

/*     // VecDeque
    let mut vd = VecDeque::from([10, 20, 30, 40]);
    println!("stærð: {}, pláss: {}, {:?}", vd.len(), vd.capacity(), vd);
    vd.push_front(5);
    vd.push_back(45);
    vd.pop_back();
    vd.pop_front();

    listi.sort(); */
}
