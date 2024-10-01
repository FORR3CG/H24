use std::fmt::Display;
use std::any::Any;

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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn tegund(&self) -> String {
        "Hundur".to_string()
    }
}

impl Dyr for Kottur {
    fn prenta_nafn(&self) {
        println!("{}", self.nafn.to_uppercase())
    }

    fn upplysingar(&self) -> String {
        self.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn tegund(&self) -> String {
        "Köttur".to_string()
    }
}

trait Dyr : Display {
    fn prenta_nafn(&self);
    fn upplysingar(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn tegund(&self) -> String;
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

    fn prenta_ketti(&self) {
        for d in self.dyrin.iter() {
            if let Some(kottur) = d.as_any().downcast_ref::<Kottur>() {
                println!("{}", kottur)
            }
        }
    }

    fn prenta_tegund(&self, tegund: &str) {
        for d in self.dyrin.iter() {
            if d.tegund() == tegund {
                println!("{}", d)
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
    dg.prenta_allt();
    dg.prenta_ketti();
}
