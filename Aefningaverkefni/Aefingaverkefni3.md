# FORR3CG - Æfingaverkefni 3 - `struct`, `enum` og `vec`

## Verkefnið

Gerðu forrit til að halda utan um herbergi í byggingu, herbergi geta verið *Kennslustofa*, *Skrifstofa* eða *Annað*. Hvert herbergi hefur id og er af ákveðinni stærð (lengd og breidd). Herbergin eru svo skráð í byggingu.

Útfærðu eftirfarandi:
- enum fyrir *tegund* sem getur verið *Kennslustofa*, *Skrifstofa* eða *Annað*.
    - útfæra Display og From<&str>
- struct fyrir *stærð* sem inniheldur *lengd* og *breidd* (`f32`).
    - útfæra `new` fall.
    - útfæra fall sem skilar flatarmálinu.
    - útfæra Display
- struct fyrir *herbergi* sem inniheldur id (u32), *stærð* og *tegund*. Það þarf að eiga föll til að gera eftirfarandi:
    - breyta tegund
    - breyta stærð (lengd og breidd)
    - fá fermetrafjöldann (kallar á flatarmálsfallið í *stærð*)
    - útfæra Display
- struct fyrir byggingu sem heldur utan um `Vec` af herbergjum. Það þarf að eiga föll til að gera eftirfarandi:
    - skrá herbergi
    - breyta stærð ákveðins herbergis
    - breyta tegund ákveðins herbergis
    - prenta út heildar stærð (fm.) allra herbergja í byggingunni
    - útfæra Display

Sjá útfærsludæmi í [bílasölunni](../Timar/bilasala_vec/).

Dæmi um notkun:
```rust
    let mut h209 = Herbergi::new(209, 5.0, 4.0, "kennslu");
    println!("{}", h209);
    // Nr: 209, Stærð: 20.00 fm., tegund: Kennslustofa.
    h209.breyta_staerd(6.0, 5.0);
    h209.breyta_tegund("skrif");
    println!("{}", h209);
    // Nr: 209, Stærð: 30.00 fm., tegund: Strifstofa.
    let mut hateigsvegur = Bygging::new();
    hateigsvegur.skra_herbergi(101, 3.4, 4.5, "skrif");
    hateigsvegur.skra_herbergi(102, 3.3, 4.5, "kennslu");
    hateigsvegur.skra_herbergi(103, 5.4, 4.5, "kennslu");
    hateigsvegur.skra_herbergi(104, 6.4, 6.5, "geymsla");
    println!("-------------------\n{}", hateigsvegur);
    hateigsvegur.breyta_tegund_herbergis_med_id(999, "skrifstofa");
    // Fann ekkert herbergi með id: 999
    hateigsvegur.breyta_tegund_herbergis_med_id(103, "skrif");
    // Breytti tegund herbergis nr: 103
    hateigsvegur.breyta_staerd_herbergis_med_id(101, 23.3, 10.7);
    // Breytti stærð herbergis nr: 101
    hateigsvegur.prenta_heildar_fermetrafjolda_byggingar();
    // Heildar stærð: 330.06 fm.
    println!("-------------------\n{}", hateigsvegur);
    // Nr: 101, Stærð: 249.31 fm., tegund: Strifstofa.
    // Nr: 102, Stærð: 14.85 fm., tegund: Kennslustofa.
    // Nr: 103, Stærð: 24.30 fm., tegund: Strifstofa.
    // Nr: 104, Stærð: 41.60 fm., tegund: Annað rými.
```

[Dæmi um lausn á verkefninu](./lausnir/aefingaverkefni_3/)
