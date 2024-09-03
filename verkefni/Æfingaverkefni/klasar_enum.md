# Æfingaverkefni - `struct` og `enum`

Útfærið `struct` sem geymir upplýsingar um einn **hest**. Upplýsingarnar sem þarf að skrá fyrir hestinn eru eftirfarandi: id (u32), nafn (String), aldur (u8) og **staða**. Staða er `enum` sem getur haft eftirfarandi gildi: *Leigdur*, *Laus*, *EkkiTilLeigu* og *Othekkt*.

Bættu falli við `Hestur` sem getur breytt stöðunni á honum.

Hafðu `Hestur` og `Stöðu` í sér skrám.

Útfærið `Display` fyrir hestinn og stöðuna, útfærið svo líka `From<&str>` fyrir stöðuna.

Dæmi um forrit fyrir ofantalið:
```rust
mod hestur;
mod stada;

use hestur::Hestur;

fn main() {
    let mut sleipnir = Hestur::new(100, "Gráni", 15, "laus");
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Laus

    sleipnir.breyta_stodu("leigður");
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Leigður

    sleipnir.breyta_stodu("ekki til útleigu");
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Ekki til útleigu

    sleipnir.breyta_stodu("eitthvað");
    println!("{}", sleipnir);
    // id: 100, nafn: Gráni, aldur: 15, staða: Óþekkt
}
```