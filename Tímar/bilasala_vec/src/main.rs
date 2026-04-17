mod bill;
mod gerd;
mod litur;
mod bilasala;

use std::io::Write;

use bilasala::Bilasala;

fn main() {
    // hjálp
    // hætta
    // skrá 101 Volvo fb 345435 3500
    // selja 101
    // hækka 101 20
    // prenta allt
    // prenta 101
    // prenta verð
    let mut bs = Bilasala::new();
    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("Gat ekki flush-að");
        let mut inntak = String::new();
        std::io::stdin()
                .read_line(&mut inntak).expect("Gat ekki lesið frá notanda!");
        let skipanir = inntak.split_whitespace().collect::<Vec<&str>>();
        match skipanir.first() {
            Some(skipun) => {
                match skipun.to_lowercase().as_str() {
                    "hjálp" => println!("Prentum út svaka góða hjálp!"),
                    "hætta" => break,
                    "skrá" => {
                        if let Err(villa) = 
                            bs.skra(skipanir[1..].join(" ").as_str()) {
                                println!("{}", villa)
                            }
                    },
                    "selja" => {
                        // selja 101
                        // Athugum fyrst hvort það er eitthvað orð á eftir "selja"
                        if let Some(id_str) = skipanir.get(1) {
                            // Reynum þá að breyta því í u32
                            if let Ok(id) = id_str.parse::<u32>() {
                                // Reynum þá að eyða bílnum úr vectornum
                                match bs.eyda_bil_med_id(id) {
                                    Ok(()) => println!("Bíll með id: {} seldur!", id),
                                    Err(villa) => println!("{}", villa),
                                }
                            } else {
                                println!("Gat ekki breytt {} í u32", id_str)
                            }
                        } else {
                            println!("Þú verður að setja in id")
                        }
                    },
                    "hækka" => {
                        // hækka 101 20
                        // Athugum fyrst hvort það eru 3 orð í skipanir
                        if skipanir.len() != 3 {
                            println!("Vantar id eða hækkun")
                        } else {
                            // reynum að breyta fyrra orðinu í u32
                            if let Ok(id) = skipanir[1].parse::<u32>() {
                                // Reynum þá að breyta seinna orðinu í u8
                                if let Ok(prosenta) = skipanir[2].parse::<u8>() {
                                    // reynum þá að hækka verðið
                                    match bs.haekka_um_prosent_verd_a_bil_med_id(id, prosenta) {
                                        Ok(()) => println!("Hækkaði verð á bíl: {} um {}%!", id, prosenta),
                                        Err(villa) => println!("{}", villa),
                                    }
                                } else {
                                    println!("Gat ekki breytt {} í u8", skipanir[2])
                                }
                            } else {
                                println!("Gat ekki breytt {} í u32", skipanir[1])
                            }
                        }
                    },
                    "prenta" => {
                        // prenta allt
                        // prenta verð
                        // prenta 101
                        // Athugum fyrst hvort það er eitthvað orð á eftir prenta
                        if let Some(prenta_hvad) = skipanir.get(1) {
                            match *prenta_hvad {
                                "allt" => println!("{}", bs),
                                "verð" | "heildarverð" => println!("Verðmæti: {} kr.", bs.verdmaeti()),
                                _ => {
                                    // prófum fyrst að breyta í u32
                                    if let Ok(id) = prenta_hvad.parse::<u32>() {
                                        if let Some(billstrengur) = bs.skoda_bil_med_id(id) {
                                            println!("{}", billstrengur)
                                        } else {
                                            println!("Fann engan bíl með id: {}!", id)
                                        }
                                    } else {
                                        println!("Kann ekki að prenta {}", prenta_hvad)
                                    }
                                }
                            }
                        } else {
                            println!("Þú verður að segja mér hvað ég á að prenta!!")
                        }
                    }
                    _ => println!("Skildi ekki skipunina {}", skipun)
                }
            },
            None => println!("Þú verður að slá eitthvað inn!!!")
        }
    }    
    
}
