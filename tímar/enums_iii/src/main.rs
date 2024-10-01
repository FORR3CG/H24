use std::fmt::Display;

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
    dg.skra_hund(Hundur::new("Snati", 58));
    dg.skra_kott(Kottur::new("Grettir", 8));
    dg.prenta_tegund("hundar");
    dg.haekka_aldur();
    dg.prenta_allt();
}
