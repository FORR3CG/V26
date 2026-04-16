# FORR3CG - Æfingaverkefni iter – breyta `for` lykkjum í `iter`

## Bjargir
- [`cloned`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.cloned)
- [`collect`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect)
- [`filter`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter)
- [`find`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.find)
- [`fold`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold)
- [`for_each`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.for_each)
- [`map`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.map)
- [`max_by_key`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.max_by_key)
- [`product`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.product)
- [`sum`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.sum)

## Verkefni 1 – Summa lista
```rust
fn main() {
    let tolur = vec![1, 2, 3, 4, 5];
    let mut summa = 0;

    for t in &tolur {
        summa += t;
    }

    println!("Summa: {}", summa);
}
```
Breyttu í `.iter().sum()` eða `.iter().fold()`.

---

## Verkefni 2 – Prenta bara sléttar tolur
```rust
fn main() {
    let tolur = vec![1, 2, 3, 4, 5, 6];

    for t in &tolur {
        if t % 2 == 0 {
            println!("{}", t);
        }
    }
}
```
Breyttu í `.iter().filter(...).for_each(...)`.

---

## Verkefni 3 – Hækka öll gildi um 10
```rust
fn main() {
    let mut tolur = vec![1, 2, 3, 4];

    for i in 0..tolur.len() {
        tolur[i] += 10;
    }

    println!("{:?}", tolur);
}
```
Breyttu í `.iter().map(...).collect()`.

---

## Verkefni 4 – Finna fyrsta orð sem byrjar a "R"
```rust
fn main() {
    let ord = vec!["Rust", "Go", "Ruby", "Python"];
    let mut fundid = None;

    for o in &ord {
        if o.starts_with("R") {
            fundid = Some(o);
            break;
        }
    }

    println!("{:?}", fundid);
}
```
Breyttu í `.iter().find(...)`.

---

## Verkefni 5 – Reikna meðaltal
```rust
fn main() {
    let tolur = vec![5, 10, 15, 20];
    let mut summa = 0;
    let mut fjoldi = 0;

    for t in &tolur {
        summa += t;
        fjoldi += 1;
    }

    let meðaltal = summa as f64 / fjoldi as f64;
    println!("Meðaltal: {}", meðaltal);
}
```
Breyttu í `.iter().fold(...)` eða sameina með `.iter().map(...).sum()`.

---

## Verkefni 6 – Telja fjölda stafa
```rust
fn main() {
    let ord = vec!["hallo", "rust", "heimur"];
    let mut samtals_stafir = 0;

    for o in &ord {
        samtals_stafir += o.len();
    }

    println!("Samtals stafir: {}", samtals_stafir);
}
```
Breyttu í `.iter().map(|o| o.len()).sum()`.

---

## Verkefni 7 – Margfalda öll gildi saman
```rust
fn main() {
    let tolur = vec![2, 3, 4];
    let mut margfeldi = 1;

    for t in &tolur {
        margfeldi *= t;
    }

    println!("Margfeldi: {}", margfeldi);
}
```
Breyttu í `.iter().product()` eða `.iter().fold(1, |acc, t| acc * t)`.

---

## Verkefni 8 – Búa til lista af stórum stöfum
```rust
fn main() {
    let ord = vec!["rust", "rocks"];
    let mut stor_stafur = Vec::new();

    for o in &ord {
        stor_stafur.push(o.to_uppercase());
    }

    println!("{:?}", stor_stafur);
}
```
Breyttu í `.iter().map(|o| o.to_uppercase()).collect::<Vec<_>>()`.

---

## Verkefni 9 – Fjarlægja neikvæðar tölur
```rust
fn main() {
    let tolur = vec![3, -1, 5, -7, 9];
    let mut jakvaeðar = Vec::new();

    for t in &tolur {
        if *t >= 0 {
            jakvaeðar.push(*t);
        }
    }

    println!("{:?}", jakvaeðar);
}
```
Breyttu í `.iter().cloned().filter(|t| *t >= 0).collect()`.

---

## Verkefni 10 – Finna lengsta orðið
```rust
fn main() {
    let ord = vec!["stutt", "lengra", "lengstaverkefni"];
    let mut lengsta = "";

    for o in &ord {
        if o.len() > lengsta.len() {
            lengsta = o;
        }
    }

    println!("Lengsta orðið: {}", lengsta);
}
```
Breyttu í `.iter().max_by_key(|o| o.len())`.

## Lausnir
Dæmi um lausnir má finna [hér](./lausnir/Aefingaverkefni_iter/lausnir.md).

