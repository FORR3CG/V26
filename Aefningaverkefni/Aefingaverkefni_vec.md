# Æfingaverkefni, `Vec`

## `Vec`

 `Vec` er sjálfstækkandi fylki, sambærilegt og `list` í Python. Er þó ólíkt list í Python á þann hátt að það getur bara geymt eina gerð gagna. `Vec` er geymdur á **heap** minninu.

 ### Hægt er að skilgreina `Vec` á eftirfarandi hátt:
 ```rust
// Verðum alltaf að taka fram af hvaða tagi gögnin sem vector-inn á að geyma eru
let mut listi: Vec<i32> = Vec::new(); 
// eða látum rust finna út úr því sjált hvaða gagnatag verður geymt í vector-num
let mut listi = Vec::new();
// hér er i32 sett í vektor-inn og því verður hann Vec<i32>
listi.push(10);
// einnig má frumstilla vektor með vec! macro-num
let mut listi = vec![1_u8, 2, 3, 4, 5]; // verður Vec<u8>
 ```

 ### Aðgangur að stökum
 ```rust
 // alveg eins og í Python
 listi[1] = 99;
 println!("{}", listi[1])
 ```


### Setja í og taka úr vektor:
```rust
// push notað til að setja í vektorinn, setur aftast í hann, alveg eins og append í Python
let mut listi: Vec<i32> = Vec::new();
listi.push(10);
listi.push(20);
listi.push(30);
println!("{:?}", &listi); // myndi skrifa út [10, 20, 30]
// pop tekur svo aftasta stakið úr vektor-num
listi.pop(); // tekur 30 út
listi.pop(); // tekur 20 út
```

### Taka ákveðið stak úr vektornum:
```rust
// remove er notað til að taka stak nr. X úr vektor-num
let mut listi = vec![10, 20, 30, 40, 50, 60];
listi.remove(2); // fjarlægir 30
// þetta virkar þannig að stökin 40 og 50 eru færð 
// um eitt sæti til vinstri sem getur verið hægvirk

// ef röðin á stökunum skiptir ekki máli má nota swap_remove sem 
//tekur aftasta stakið og setur í stað þess sem var fjarlægt
listi.swap_remove(1); // fjarlægir 20 og setur 60 í staðinn
println!("{:?}", &listi); // [10, 60, 40, 50]
```

### `for` og `Vec`
```rust
// af því að Vec er á heap þarf að passa að nota reference
let mut listi = vec![1, 2, 3, 4, 5];
for stak in &listi {
    print!("{}", stak)
}

// af á að breyta vec, nota þá &mut
for stak in &mut listi {
    // og derefence-a
    *stak = *stak + 10;
}

// síðan má nota index
for idx in 0..listi.len() {
    print!("{}", listi[idx])
}

// ef þarf að enumerate-a, ATH. ekki ref (iter sér um það)
// skoðum iter seinna á spönninni
for (idx, stak) in listi.iter().enumerate() {
    println!("idx: {}, stak: {}", idx, stak)
}
```

### Senda `Vec` í fall
Virkar svipað og þegar `String` er sent í föll
```rust 
// tekur eignarhald
fn fall(listi: Vec<i32>) {}

// fær lánað
fn fall(listi: &Vec<i32>) {}

// fær lánað mut
fn fall(listi: &mut Vec<i32>) {}

// tekur eignarhald og skilar því svo til baka
fn fall(listi: &Vec<i32>) -> Vec<i32> {}
```

Ýmis föll eru svo tengd Vec svipað og með lista í Python en við skoðum þau síðar.

## Æfingaverkefni 

**Í verkefnunum hér að neðan passaðu að þú getir notað vektorinn eftir að þú kallar á fall.**

1. Skrifaðu fall sem tekur inn Vec af i32, leggur öll stökin saman og skilar niðurstöðunni sem i64.
1. Skrifaðu fall sem tekur inn Vec af i32, reiknar meðaltal stakanna og skilar því sem f32.
1. Skrifaðu fall sem tekur inn Vec af i32, finnur minnsta stakið og skilar því.
1. Skrifaðu fall sem tekur inn Vec af i32, finnur indexinn á stærsta stakinu og skilar honum.
1. Gerðu tvær útgáfur af falli sem tekur inn Vec af i32 og hækkar öll stök um 20.
    1. Önnur útgáfan á að fá vektorinn lánaðan
    2. Hin á að taka eignarhald og skila því svo aftur

[Dæmi um lausn](./lausnir/Aefingaverkefni_vec)



