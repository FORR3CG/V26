# Æfingaverkefni, föll

### Smáföll ([lausn](./lausnir/aefingaverkefni_1_foll/))

1. Skrifaðu fall sem tekur inn tvær `u8` tölur, margfaldar þær saman og skilar niðurstöðunni sem `u16`.
1. Skrifaðu fall sem tekur inn u8 töluna n, reiknar n! (n hrópmerkt) og skilar niðurstöðunni sem u128. Ef inntakstalan n er stærra en 34 á fallið að skila 0 (núll).
1. Skrifaðu fall sem tekur inn 3 (þrjár) i32 tölur, reiknar meðatal þeirra og skilar niðurstöðunni sem f32.
1. Skrifaðu fall sem tekur inn 5 staka fylki af i32 tölum, reiknar meðaltal þeirra og skilar niðurstöðunni sem i32.
1. Skrifaðu fall sem tekur inn 2 (tvær) i32 tölur og skilar tuple með tölunum í hækkandi röð.

### Æfingaverkefni 1 með óeðlilega mörgum föllum. ([lausn](./lausnir/aefingaverkefni_1_foll/)) 
Breyttu [æfingaverkefni 1](./Aefingaverkefni1.md) þannig að það noti föll í nánast allt sem þarf að gera.
Til að nota `enumerate` eins og í Python má gera eftirfarandi: 
```rust
let fylki = [1,2,3,4];
for (i, tala) in fylki.iter().enumerate() {
    println!("Í sæti nr. {} er {}", i, tala);
}
```
