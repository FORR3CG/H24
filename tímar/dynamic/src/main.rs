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
        write!(f, "nafn: {}, hlýðnieinkunn {}", self.nafn, self.hlydnieinkunn)
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

impl Dyr for Hundur {
    fn prenta_nafn(&self) {
        println!("{}", self.nafn)
    }

    fn upplysingar(&self) -> String {
        self.to_string()
    }
}

impl Dyr for Kottur {
    fn prenta_nafn(&self) {
        println!("{}", self.nafn.to_uppercase())
    }

    fn upplysingar(&self) -> String {
        self.to_string()
    }
}

trait Dyr {
    fn prenta_nafn(&self);
    fn upplysingar(&self) -> String;
}

struct Dyragardur {
    dyrin: Vec<Box::<dyn Dyr>>,
}

impl Dyragardur {
    fn new() -> Self {
        Self {
            dyrin: Vec::new(),
        }
    }

    fn skra_hund(&mut self, hundur: Hundur) {
        self.dyrin.push(Box::new(hundur));
    }

    fn skra_kott(&mut self, kottur: Kottur) {
        self.dyrin.push(Box::new(kottur));
    }

    fn prenta_allt(&self) {
        for d in self.dyrin.iter() {
            println!("{}", d.upplysingar())
        }
    }
}

fn main() {
    println!("Hello, world!");
}
