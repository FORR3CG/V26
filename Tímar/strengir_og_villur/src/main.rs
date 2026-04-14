/*
enum Result {
    Ok(i32),
    Err(String),
}
*/


fn deila(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        return Ok(a / b);
    } else {
        return Err("DEILA: Ekki má deila með núll".to_string());
    }
}

fn einfalt_fall() -> Result<i32, String> {
    Ok(42)
} 

fn annad_einfalt_fall() -> Option<i32> {
    Some(42)
}

fn einfalt() -> i32 {
    42
}

fn deiling(a: i32, b: i32) -> Result<i32, String> {
    // gera fullt af hlutum
    if a > 100 {
        return Err("DEILING: a of hátt".to_string());
    }

    // gera fullt í viðbót
    let k = deila(a, b)?;
    Ok(k)
}

fn main() {
    // Option og Result enum - Kafli 6.1 (seinni hlutinn)
    let k = deiling(101, 0);
    match k {
        Ok(tala) => println!("Niðurstaða deilingarinnar var: {}", tala),
        Err(e) => println!("VILLA: {}", e),
    }
    if let Ok(k) = deila(10, 5) {
        println!("talan er {}", k)
    } else {
        println!("Villa við deilingu!!!")
    }
    //println!("{}", deila(10, 0));

    let k = Some(9);
    match k {
        Some(tala) => println!("talan er {}", tala),
        None => println!("fann enga tölu"),
    }
    if let Some(tala) = k {
        println!("{}", tala)
    } else {
        println!("Engin tala")
    }
    let j = match k {
        Some(tala) => tala,
        None => 0,
    };
    

    println!("{:?}", k);


    // Strengir - Kafli 8.2
    let s1 = String::from("halló😊, heimur!");
    for stafur in s1.chars() {
        println!("{}", stafur)
    }
    let se = String::from("A");
    for b in se.bytes() {
        println!("\n{}\n", b)
    }
    let s2 = &s1[2..4];
    println!("{}", &s1[0..2]);
    let texti = "Það er leikur að læra".to_string();
    let ordin: Vec<&str> = texti.split_whitespace().collect();
    let ordin = texti.split_whitespace().collect::<Vec<&str>>();
    let tolur = "1 2 99 abc".to_string();
    let talnagildi = tolur.split_whitespace().collect::<Vec<&str>>();
    let tala = talnagildi[2].parse::<i32>();
    println!("{:?}", tala);
    println!("{:?}", talnagildi)
}
