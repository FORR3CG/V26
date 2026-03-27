# FORR3CG - Æfingaverkefni 2 - `struct` og `enum`

Útfærðu `struct` sem geymir upplýsingar um einn **hest**. Upplýsingarnar sem þarf að skrá fyrir hestinn eru eftirfarandi: id (u32), nafn (String), aldur (u8) og **staða**. Staða er `enum` sem getur haft eftirfarandi gildi: *Leigdur*, *Laus*, *EkkiTilLeigu* og *Othekkt*.

Bættu falli við `Hestur` sem getur breytt stöðunni á honum og svo öðru falli sem hækkar aldur hestsins um einn.

Hafðu `Hestur` og `Stöðu` í sér skrám.

Útfærðu `Display` og fyrir hestinn og stöðuna ásamt `From<&str>` fyrir stöðuna.

Dæmi um forrit fyrir ofantalið:
```rust
mod hestur;
mod stada;

use hestur::Hestur;
use stada::Stada;

fn main() {
    let mut sleipnir = Hestur::new(100, "Sleipnir", 15, "laus");
    println!("{}", sleipnir);
    // id: 100, nafn: Sleipnir, aldur: 15, staða: Laus

    sleipnir.breyta_stodu("leigður");
    sleipnir.haekka_aldur();
    println!("{}", sleipnir);
    // id: 100, nafn: Sleipnir, aldur: 16, staða: Leigður    
}
```

[Lausn á verkefninu](./lausnir/aefingaverkefni_2/)
