/*
setja inn serde + derive með eftirfarandi: cargo add serde -F derive
setja líka inn serde_json með eftirfarandi: cargo add serde_json
*/

use std::fmt::Display;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Deserialize, Serialize)]
struct Hundur {
    nafn: String,
    hlydnieinkunn: u32,
}

impl Hundur {
    fn new(nafn: &str, hlydnieinkunn: u32) -> Self {
        Self {
            nafn: nafn.to_string(),
            hlydnieinkunn,
        }
    }
}

impl Display for Hundur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "nafn: {}, hlýðnieinkunn {}",
            self.nafn, self.hlydnieinkunn
        )
    }
}

#[derive(Deserialize, Serialize)]
struct Kottur {
    nafn: String,
    aldur: u8,
}

impl Kottur {
    fn new(nafn: &str, aldur: u8) -> Self {
        Self {
            nafn: nafn.to_string(),
            aldur,
        }
    }
}

impl Display for Kottur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nafn: {}, aldur {}", self.nafn, self.aldur)
    }
}

#[derive(Deserialize, Serialize)]
enum Dyr {
    Hundurinn(Hundur),
    Kotturinn(Kottur),
}

impl Display for Dyr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dyr::Hundurinn(hundur) => write!(f, "{}", hundur),
            Dyr::Kotturinn(kottur) => write!(f, "{}", kottur),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Dyragardur {
    dyrin: Vec<Dyr>,
}

impl Dyragardur {
    fn new() -> Self {
        Self { dyrin: Vec::new() }
    }

    fn skra_hund(&mut self, hundur: Hundur) {
        self.dyrin.push(Dyr::Hundurinn(hundur));
    }

    fn skra_kott(&mut self, kottur: Kottur) {
        self.dyrin.push(Dyr::Kotturinn(kottur));
    }

    fn prenta_allt(&self) {
        for d in self.dyrin.iter() {
            println!("{}", d)
        }
    }

    fn prenta_hunda(&self) {
        self.dyrin.iter().for_each(|d| match d {
            Dyr::Hundurinn(hundur) => println!("{}", hundur),
            Dyr::Kotturinn(kottur) => {}
        });
        for d in self.dyrin.iter() {
            match d {
                Dyr::Hundurinn(hundur) => println!("{}", hundur),
                Dyr::Kotturinn(kottur) => {}
            }
        }
    }

    // gera svo svipað fyrir prenta_ketti

    fn prenta_tegund(&self, tegund: &str) {
        if tegund == "hundar" {
            self.prenta_hunda();
        } else {
            // útfæra prenta_ketti fallið
            todo!()
        }
    }

    fn haekka_aldur(&mut self) {
        for d in self.dyrin.iter_mut() {
            match d {
                Dyr::Hundurinn(_) => {}
                Dyr::Kotturinn(kottur) => kottur.aldur += 1,
            }
        }
    }

}

fn main() {
    let mut dg = Dyragardur::new();
    let h = Hundur::new("abc", 99);
    println!("{}", h);
    let json_hundur = serde_json::to_string(&h).unwrap();
    println!("{}", json_hundur);
    dg.skra_hund(Hundur::new("Snati", 58));
    dg.skra_kott(Kottur::new("Grettir", 8));
    dg.prenta_tegund("hundar");
    dg.haekka_aldur();
    dg.prenta_allt();
    
    // búa til json streng út frá lista structinu
    let json_listinn = serde_json::to_string_pretty(&dg).unwrap();
    // skrifa í skrá
    let mut skra = std::fs::File::create("gogn.json").unwrap();
    write!(skra, "{}", json_listinn);

    println!("{}", json_listinn);
    // lesa json frá skrá
    let mut skra = std::fs::File::open("gogn.json").unwrap();
    let mut inntak = String::new();
    skra.read_to_string(&mut inntak).unwrap();
    println!("Dýragarður 2 -------------------------");
    let mut dg2 = serde_json::from_str::<Dyragardur>(&inntak).unwrap();
    dg2.prenta_allt();



}
