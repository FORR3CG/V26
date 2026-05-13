fn deila(a: i32, b: i32) -> Result<f32, String> {
    if b != 0 {
        Ok(a as f32 / b as f32)
    } else {
        Err("Ekki má deila með núll".to_string())
    }
}

fn main() {
    // heiltölur: i og u 8, 16, 32, 64 og 128, size
    // float (kommutölur): f32 og f64
                        //   0 1 2 3 4
    let fylki: [usize; 5] = [1,2,3,4,5];
    let idx = fylki[2]; // idx er 3
    println!("{}", fylki[idx]); // fylki[fylki[2]]
    let k: isize = 45;
    // print! vs println!

    // heiltöludeiling fyrir heiltölur
    let k = 9 / 10;

    // if og match
    if let Ok(tala) = deila(10, 0) {
        println!("{}", tala)
    } else {
        println!("villa í deilingu")
    }
    println!("{:?}", deila(10, 3));
    match deila(10, 20) {
        Ok(tala) => println!("{}", tala),
        Err(villa) => println!("{}", villa),
    }

    // nokkrar lykkjur
    // for
    for i in 0..10 {
        print!("{}", i);
    }

    for _ in 0..5 {
        println!("abc");
    }

    let listi = vec![1,2,3,4,5];
    for stak in &listi {
        print!("{}", stak);
    }
    println!("{:?}", listi);
    // while
    let mut k = 5;
    while k > 0 {
        println!("abc");
        k -= 1;
    }
    
    // loop => while true
    let mut k = 5;
    loop {
        println!("abc");
        k -= 1;
        if k < 1 {
            break;
        }
    }

    let mut k = 10;
    loop {
        k -= 1;
        if k % 2 == 0 {
            continue;
        }
        println!("{}", k);
        if k < 0 {
            break;
        }
    }

    // stack og heap
    let v_s = [1,2,3,4,5]; // stack
    let mut v_h = vec![1,2,3,4,5]; // heap
    v_h.push(23);
    let a = 5; // stack
    let s_s = "abc"; // stack
    let mut s = String::from("abc"); //heap
    s.push_str("g");
    let mut b = Box::new(5); // heap
    *b += 10;

    // trait - skilgeinir sameiginlega hegðun

    // útfæra Ord traitið fyrir eitt struct
  
}

struct Hundur {
    aldur: u8,
    nafn: String,
}

impl Ord for Hundur {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // fyrst að bera saman nafn og ef það er eins að þá bera saman aldur
        self.nafn.cmp(&other.nafn)
            .then(self.aldur.cmp(&other.aldur))
    }
}

// Það sem er hér fyrir neðan kemur EKKI á prófinu
impl PartialOrd for Hundur {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hundur {
    fn eq(&self, other: &Self) -> bool {
        self.nafn == other.nafn && self.aldur == other.aldur
    }
}

impl Eq for Hundur {}