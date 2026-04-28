# Æfingaverkefni - `trait`

Búðu til eftirfarandi `struct`:
- Punktur, inniheldur x (u32) og y (u32).
    - Útfærðu new fall sem tekur inn x og y.
- Hringur, inniheldur upphafspunkt (Punktur) og radíus (u32).
    - Útfærðu new fall sem tekur inn x og y (fyrir upphafspunktinn) og svo radíusinn.
- Lína, inniheldur upphaf (Punktur) og endir (Punktur).
    - Útfærðu new fall sem tekur inn x og y fyrir bæði upphafs og endapunktinn.
- Rétthyrningur, inniheldur upphafspunkt (Punktur), breidd (u32) og hæð (u32)

Búðu svo til `trait`ið Form sem inniheldur tvö föll:
- teikna, skilar streng
- flatarmál, sem skilar u64

Láttu Hring, Línu og Rétthyrning útfæra Form.
- teikna fallið á að skila strengnum "Teiknar <Nafnið á hlutnum>", til dæmis "Teikna hring"
- flatarmál fallið á að reikna og skila flatarmáli hlutarins sem u64. Þú finnur π í `std::f64::consts::PI;`

Gerðu svo að lokum fallið `teikna_og_syna_flatarmal` sem tekur inn Form og skrifar út útkomuna úr teikna og flatarmál föllunum.

Dæmi um notkun:
```rust
fn main() {
    let mut teikniform: Vec<Box<dyn Form>> = Vec::new();
    teikniform.push(Box::new(Lina::new(10,10, 20, 20)));
    teikniform.push(Box::new(Hringur::new(10, 10, 30)));
    teikniform.push(Box::new(Retthyrningur::new(10,10, 10,10)));
    teikniform.iter().for_each(|form| teikna_og_syna_flatarmal(form.as_ref()));
    // Ætti að skrifa út:
    //      Teiknar línu!, flatarmálið er: 0
    //      Teiknar hring!, flatarmálið er: 2827
    //      Teiknar rétthyrning, flatarmálið er: 100
}
```
Dæmi um lausn á verkefninu má finna [hér](./lausnir/aefingaverkefni_trait/src/main.rs).