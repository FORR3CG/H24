mod hestur;
mod stada;

use hestur::Hestur;

fn main() {
    let mut sleipnir = Hestur::new(100, "Gráni", 15, "laus");
    println!("{}", sleipnir);
    sleipnir.breyta_stodu("leigður");
    println!("{}", sleipnir);
    sleipnir.breyta_stodu("ekki til leigu");
    println!("{}", sleipnir);
    sleipnir.breyta_stodu("veit ekki");
    println!("{}", sleipnir);
    
}
