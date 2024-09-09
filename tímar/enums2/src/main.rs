

/*
enum Option<T> {
    None,
    Some(T),
}
*/



use std::fmt::Display;

#[derive(Debug)]
struct Afangi {
    nafn: String,
    kennari: Option<Kennari>,
}

impl Display for Afangi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kennari { // Aðferð 1
            Some(k) => write!(f, "Áfangi: {}, Kennari: {}", self.nafn, k.nafn),
            None => write!(f, "Áfangi: {}, Enginn kennari skráður!", self.nafn),
        } 

        /* let kannski_kennari = match &self.kennari { // Aðferð 2
            Some(k) => k.nafn.as_ref(),
            None => "Enginn kennari!",
        };
        write!(f, "Áfangi: {}, Kennari: {}", self.nafn, kannski_kennari) */  

        /* if let Some(k) = &self.kennari { // Aðferð 3
            write!(f, "Áfangi: {}, Kennari: {}", self.nafn, k.nafn)
        }  else {
            write!(f, "Áfangi: {}, Enginn kennari skráður!", self.nafn)
        } */

        /* if self.kennari.is_some() { // Aðferð 4
            write!(f, "Áfangi: {}, Kennari: {}", self.nafn, self.kennari.as_ref().unwrap().nafn)
        } else {
            write!(f, "Áfangi: {}, Enginn kennari skráður!", self.nafn)
        } */
    } 

}

impl Afangi {
    fn new(nafn: &str, kennari: Option<Kennari>) -> Self {
        Self {
            nafn: nafn.to_string(),
            kennari,
        }
    }
}

#[derive(Debug)]
struct Kennari {
    nafn: String,
}

impl Kennari {
    fn new(nafn: &str) -> Self {
        Self {
            nafn: nafn.to_string(),
        }
    }
}

enum KannskiStarfsmadur {
    None,
    Starfsmadur(Kennari),
}

fn main() {
    let kk = KannskiStarfsmadur::Starfsmadur(Kennari::new("geir"));
    let ke = KannskiStarfsmadur::None;

    let kennarar = [kk, ke];

    for k in kennarar {
        match k {
            KannskiStarfsmadur::None => println!("enginn kennari"),
            KannskiStarfsmadur::Starfsmadur(kennari) => println!("Kennari: {:?}", kennari),
        }
    }

    let forr3cg = Afangi::new("forr3cg", Some(Kennari::new("Geir")));
    let forr4ab = Afangi::new("forr4ab", None);
    println!("{}", forr3cg);
    println!("{}", forr4ab);
    let t = 3.456789;
    let tala = format!("{:.2}", t);
    println!("{}", tala);
}

/* struct Herbergi {
    staerd: Staerd,
    tegund: Tegund,
    tolva: Option<Tolva>,
} */

struct Litur {
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
}

impl From<(u8, u8, u8, u8)> for Litur {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
            alpha: value.3,
        }
    }
}
