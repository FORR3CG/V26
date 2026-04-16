# FORR3CG - Æfingaverkefni iter – breyta `for` lykkjum í `iter` - Lausnir

## Verkefni 1 
**Lausn A – `sum()`**
```rust
fn main() {
    let tolur = vec![1, 2, 3, 4, 5];
    let summa: i32 = tolur.iter().sum();
    println!("Summa: {}", summa);
}
```

**Lausn B – `fold()`**
```rust
fn main() {
    let tolur = vec![1, 2, 3, 4, 5];
    let summa = tolur.iter().fold(0, |acc, t| acc + *t);
    println!("Summa: {}", summa);
}
```

---

## Verkefni 2 
```rust
fn main() {
    let tolur = vec![1, 2, 3, 4, 5, 6];
    tolur
        .iter()
        .filter(|t| *t % 2 == 0)
        .for_each(|t| println!("{}", t));
}
```

---

## Verkefni 3 
```rust
fn main() {
    let tolur = vec![1, 2, 3, 4];
    let nyjar: Vec<i32> = tolur.iter().map(|t| t + 10).collect();
    println!("{:?}", nyjar);
}
```

---

## Verkefni 4 
```rust
fn main() {
    let ord = vec!["Rust", "Go", "Ruby", "Python"];
    let fundid = ord.iter().find(|o| o.starts_with("R"));
    println!("{:?}", fundid);
}
```

---

## Verkefni 5 
```rust
fn main() {
    let tolur = vec![5, 10, 15, 20];
    let summa: i32 = tolur.iter().sum();
    let fjoldi = tolur.len();
    let medaltal = summa as f64 / fjoldi as f64;
    println!("Medaltal: {}", medaltal);
}
```

**Lausn B – `fold()` (samtals og fjöldi i einu)**
```rust
fn main() {
    let tolur = vec![5, 10, 15, 20];
    let (summa, fjoldi) = tolur.iter().fold((0, 0usize), |(s, c), t| (s + *t, c + 1));
    let medaltal = summa as f64 / fjoldi as f64;
    println!("Meðaltal: {}", medaltal);
}
```

---

## Verkefni 6
```rust
fn main() {
    let ord = vec!["hallo", "rust", "heimur"];
    let samtals_stafir: usize = ord.iter().map(|o| o.len()).sum();
    println!("Samtals stafir: {}", samtals_stafir);
}
```

---

## Verkefni 7 
**Lausn A – `product()`**
```rust
fn main() {
    let tolur = vec![2, 3, 4];
    let margfeldi: i32 = tolur.iter().product();
    println!("Margfeldi: {}", margfeldi);
}
```

**Lausn B – `fold()`**
```rust
fn main() {
    let tolur = vec![2, 3, 4];
    let margfeldi = tolur.iter().fold(1, |acc, t| acc * *t);
    println!("Margfeldi: {}", margfeldi);
}
```

---

## Verkefni 8 
```rust
fn main() {
    let ord = vec!["rust", "rocks"];
    let stor_stafur: Vec<String> = ord.iter().map(|o| o.to_uppercase()).collect();
    println!("{:?}", stor_stafur);
}
```

---

## Verkefni 9 
```rust
fn main() {
    let tolur = vec![3, -1, 5, -7, 9];
    let jakvaedar: Vec<i32> = tolur.iter().cloned().filter(|t| *t >= 0).collect();
    println!("{:?}", jakvaedar);
}
```

---

## Verkefni 10 
```rust
fn main() {
    let ord = vec!["stutt", "lengra", "lengstaverkefni"];
    let lengsta = ord.iter().max_by_key(|o| o.len());
    println!("Lengsta orðið: {:?}", lengsta);
}
```
