# FORR3CG - Skilaverkefni 2 (5%)

- **Verkefnið er einstaklingsverkefni**. Ef tveir eða fleiri nemendur skila sömu lausnunum er gefið 0 (núll) fyrir þær lausnir.
- **Ef kóði er tekinn af netinu** (eða öðrum álíka stöðum) skal taka það fram, benda á hvaðan hann kemur og skrifa skýringar (e. comment) við hverja línu kóðans. Sé það ekki gert verður gefið 0 (núll) fyrir verkefnið í heild.
- **Frjálst val er** um að skrifa skýringar við kóðann, að öðru leiti en þar sem liðurinn hér að ofan kallar á, en þú þarft að geta útskýrt hverja línu kóðans nákvæmlega ef beðið er um. Geti nemandi ekki útskýrt eitthvað í kóðanum sem skilað er er gefið 0 (núll) fyrir verkefnið.
- **Notið** eingöngu þær aðferðir sem kynntar hafa verið í fyrstu sex köflum bókarinnar við lausn verkefnisins.

## Verkefnið

Gerðu forrit til að skrá **Herbergi** í skólanum, herbergi geta verið *Kennslustofa*, *Skrifstofa* eða *Annað*. Hvert herbergi er síðan af ákveðinni stærð (lengd og breidd). Herbergi getur svo innihaldið **Tölvu** eða ekki, fyrir tölvu þarf að skrá stærð disks og vinnsluminnis ásamt því að skrá hvort tölvan er með *CISC* eða *RISC* örgjörva.

Útfærðu eftirfarandi:
- struct fyrir *stærð* sem inniheldur *lengd* og *breidd* (`f32`).
- enum fyrir *tegund* sem getur verið *Kennslustofa*, *Skrifstofa* eða *Annað*.
- enum fyrir örgjörvagerð sem getur verið *Cisc* eða *Risc*
- struct fyrir *tölvu* sem inniheldur stærð disks og vinnsluminnis (`u32`) ásamt örgjörvagerð.
- struct fyrir *herbergi* sem inniheldur *stærð*, *tegund* og *tölvu* eða ekki (`Option`).
    - *herbergi* þarf að eiga fallið *breyta_staerd* sem setur inn nýjar tölur fyrir *lengd* og *breidd*.
    - *herbergi* þarf líka að eiga fallið *skipta_um_tolvu* sem notað er til að breyta um tölvu í herberginu.

Útfærðu svo `Display` og `From` eftir þörfum.

Dæmi um notkun:
```rust
    let mut h202 = Herbergi::new(4.3, 3.7, Tegund::Kennslustofa, None);
    println!("{}", h202);
    // Stærð: 15.91 fm., tegund: Tegund: Kennslustofa, Tölva: Engin tölva
    h202.breyta_staerd(6.1, 5.3);
    h202.skipta_um_tolvu(Some(Tolva::new(1000, 16, "arm")));
    println!("{}", h202);
    // Stærð: 32.33 fm., tegund: Tegund: Kennslustofa, Tölva: HDD: 1000 GB, RAM 16 GB, CPU: RISC
```

Skilist á Innu.
