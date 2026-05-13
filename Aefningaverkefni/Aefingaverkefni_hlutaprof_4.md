# Æfingaverkefni fyrir hlutapróf 4

### 1. Skrifaðu forrit sem notar `loop` lykkjuna til að skrifa "Uppstigningardagur" 10 sinnum á skjáinn.

<details>
<summary>Lausn:</summary>

```rust
fn main() {
    let mut teljari = 10;
    loop {
        teljari -= 1;
        println!("Uppstigningardagur");
        if teljari == 0 {
            break;
        }
    }
}
```

</details>

### 2. Skrifaðu forrit sem geymir heiltölu á heap. Síðan á forritið að breyta tölunni og skrifa hana loks út.

<details>
<summary>Lausn</summary>

```rust
fn main() {
    let mut b = Box::new(10);
    *b += 10;
    println!("{}", b);
}
```

</details>

### 3. struct og trait o.fl.

```rust
struct Starfsmadur {
    id: u32,
    nafn: String,
    lifaldur: u8,
    starfsaldur: u8,
}
```

### A.
Gerðu fallið `set_lifaldur` fyrir Starfsmadur en tryggðu að starfsaldur geti ekki verið hærri en lífaldur. Ef lífaldur er lægri en starfsaldur skilar fallið viðeigandi villuskilaboðum. Fallið skilar `Result<(), String>`.

<details>
<summary>Lausn</summary>

```rust
impl Starfsmadur {
    fn set_lifaldur(&mut self, lifaldur: u8) -> Result<(), String> {
        if lifaldur > self.starfsaldur {
            self.lifaldur = lifaldur;
            Ok(())
        } else {
            Err("Starfsaldur má ekki vera hærri en lífaldur".to_string())
        }
    }
}
```

</details>

### B.
Útfærðu `Ord` trait-ið fyrir Starfsmadur. Samanburðurinn á fyrst að skoða nafn síðan starfsaldur og loks lífaldur.

<details>
<summary>Lausn</summary>

```rust
impl Ord for Starfsmadur {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.nafn.cmp(&other.nafn)
            .then(self.starfsaldur.cmp(&other.starfsaldur))
            .then(self.lifaldur.cmp(&other.lifaldur))
    }
}
```

</details>

## `iter` og `for`

Breyttu þessum kóða þannig að notuð sé `for` lykkja í stað `iter`.
```rust
let listi = vec![1,2,3,4,5];
let summa = listi.iter()
                 .filter(|stak| **stak < 3)
                 .fold(0, |summa, stak| summa + stak);
```

<details>
<summary>Lausn</summary>

```rust
fn main() {
    let listi = vec![1,2,3,4,5];
    let mut summa = 0;
    for tala in &listi {
        if *tala < 3 {
            summa += tala;
        }
    }
    println!("{}", summa);
}
```

</details>

Breyttu þessum kóða þannig að notuð sé `for` lykkja í stað `iter`.
```rust
let mut listi = vec![1,2,3,4,5];
listi.iter_mut()
     .for_each(|tala| *tala *= 2);
```

<details>
<summary>Lausn</summary>

```rust
fn main() {
    let mut listi = vec![1,2,3,4,5];
    for tala in &mut listi {
        *tala *= 2
    }
    println!("{:?}", listi);
}
```

</details>
