use std::i32;

fn main() {
    let mut listi = vec![10, 20, 30, 40, 50, 60];
    println!("Summa listans er: {}", summa(&listi));
    println!("Meðaltal stakanna er: {}", medaltal(&listi));
    println!("Gildið á minnsta stakinu er: {}", minnsta(&listi));
    println!("Index-inn á stærsta stakinu er: {}", staersta_idx(&listi));
    haekka_lanad(&mut listi);
    println!("Listinn eftir hækkun á stökum: {:?}", &listi)
}

// 1. Skrifaðu fall sem tekur inn Vec af i32, leggur öll stökin saman og skilar niðurstöðunni.
fn summa(listinn: &Vec<i32>) -> i32 {
    let mut summa: i32 = 0;
    // ATH þurfum ekki & á listann því hann er tekinn inn með ref
    for stak in listinn {
        summa += stak
    }
    summa
}
// 2. Skrifaðu fall sem tekur inn Vec af i32, reiknar meðaltal stakanna og skilar því sem f32.
fn medaltal(listinn: &Vec<i32>) -> f32 {
    let lengd = listinn.len();
    // ef listinn er tómur skilum við 0
    if lengd == 0 {
        0_f32
    } else {
        let mut summa: i32 = 0;
        for stak in listinn {
            summa += stak
        }
        summa as f32 / lengd as f32
    }
}

// 3. Skrifaðu fall sem tekur inn Vec af i32, finnur minnsta stakið og skilar því.
fn minnsta(listinn: &Vec<i32>) -> i32 {
    let mut minnst = i32::MAX;
    for stak in listinn {
        // stak er &i32 en við viljum fá i32 til að bera saman og skrifa
        if *stak < minnst {
            minnst = *stak
        }
    }
    minnst
}

// 4. Skrifaðu fall sem tekur inn Vec af i32, finnur indexinn á minnsta stakinu og skilar honum.
fn staersta_idx(listinn: &Vec<i32>) -> usize {
    let mut staerst = i32::MIN;
    let mut idx_a_staersta = 0_usize;
    for idx in 0..listinn.len() {
        if listinn[idx] > staerst {
            staerst = listinn[idx];
            idx_a_staersta = idx
        }
    }
    idx_a_staersta
}

// 5. Gerðu falli sem tekur inn Vec af i32 sem &mut og hækkar öll stök um 20.
fn haekka_lanad(listinn: &mut Vec<i32>) {
    // er þegar lánaður með &mut
    for stak in listinn {
        *stak += 10
    }
}

 