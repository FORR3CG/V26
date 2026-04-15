# FORR3CG - Æfingaverkefni 4 - `struct`, `enum`, `Result` og `TryFrom`

Bókasafn skólans vantar hugbúnað til að halda utan um safnkost sinn. Safnkostur er með auðkenni, titil, tegund og staðsetningu. Hafðu sér skrá fyrir hvert enum/struct.

Gerðu `enum` Tegund sem er annaðhvort *Bók* eða *Tímarit*. 
- Útfærðu `Display` og `TryFrom<&str>` (villuhlutinn String) fyrir Tegund.

Gerðu `enum` Staðsetning sem er *Skólavörðuholt*, *Háteigsvegur* eða *Hafnarfjörður*. 
- Útfærðu `Display` og `TryFrom<&str>` (villuhlutinn String) fyrir Stadsetning.

Gerðu svo `struct` Safnkostur sem inniheldur id (`u32`), titill (`String`), tegund (`Tegund`) og staðsetningu (`Stadsetning`).
- Útfærðu `new` fall fyrir Safnkost sem skilar `Result<Self, String>`
- Útfærðu `Display` fyrir Safnkost
- Útfærðu `TryFrom<&str>` (villuhlutinn String) fyrir Safnkost, gerðu ráð fyrir að tilillinn sé aðeins eitt orð.
    - Inntakið verður á forminu "id titill tegund staðsetning"
    - Dæmi um inntak "100 Python bók háteigsvegur"
    - Notaðu `split_whitespace()` til að brjóta inntakið í einstaka orð.
    - Notaðu `parse::<u32>()` til að breyta fyrsta orðinu í `u32`

Dæmi um notkun:
```rust
mod safnkostur;
mod stadsetning;
mod tegund;

use safnkostur::Safnkostur;

fn prenta_safnkost(safnkostur: Result<Safnkostur, String>) {
    match safnkostur {
        Ok(kostur) => println!("{}", kostur),
        Err(villa) => println!("{}", villa),
    }
}

fn main() {
    let a = Safnkostur::new(100, "Python", "bók", "hv");
    prenta_safnkost(a);
    // id: 100, titill: Python, tegund: Bók, staðsetning: Háteigsvegur

    let b = Safnkostur::new(101, "CSS", "tímarit", "hvergi");
    prenta_safnkost(b);
    // Gat ekki breytt 'hvergi' í staðsetningu!!

    let c = Safnkostur::new(102, "HTML", "myndband", "hv");
    prenta_safnkost(c);
    // Gat ekki breytt 'myndband' í tegund!!

    let r = Safnkostur::try_from("101 Rust tímarit sh");
    prenta_safnkost(r);
    // id: 101, nafn: Rust, tegund: Tímarit, staðsetning: Skólavörðuholt

    let s = Safnkostur::try_from("101 Linux handrit sh");
    prenta_safnkost(s);
    // Gat ekki breytt 'handrit' í tegund!!

    let t = Safnkostur::try_from("101 Cisco tímarit noregur");
    prenta_safnkost(t);
    // Gat ekki breytt 'noregur' í staðsetningu!!
}
```

Lausn á verkefninu má finna [hér](./lausnir/aefingaverkefni_4/src/)