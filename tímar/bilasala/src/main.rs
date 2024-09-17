use std::fmt::Display;
use std::io::{stdin, stdout, Write};

enum Gerd {
    Folksbill,
    Jeppi,
    Annad,
}

impl Display for Gerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gerd::Folksbill => write!(f, "Fólksbíll"),
            Gerd::Jeppi => write!(f, "Jeppi"),
            Gerd::Annad => write!(f, "Annað"),
        }
    }
}

impl From<&str> for Gerd {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "fb" | "fólksbíll" => Gerd::Folksbill,
            "j" | "jeppi" => Gerd::Jeppi,
            _ => Gerd::Annad,
        }
    }
}

struct Bill {
    id: u32,
    framleidandi: String,
    gerd: Gerd,
    verd: u32,
}

impl Bill {
    fn new(id: u32, framleidandi: &str, gerd: Gerd, verd: u32) -> Self {
        Self {
            id,
            framleidandi: framleidandi.to_string(),
            gerd,
            verd,
        }
    }
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, {}, gerð: {}, verð: {} kr.", self.id, self.framleidandi, self.gerd, self.verd)
    }
}

struct Bilasala {
    bilar: Vec<Bill>,
    id: u32,
}

impl Bilasala {
    fn new() -> Self {
        Self {
            bilar: Vec::new(),
            id: 100,
        }
    }

    fn next_id(&mut self) -> u32 {
        self.id += 1;
        self.id
    }

    fn skra(&mut self, framleidandi: &str, gerd: &str, verd: u32) {
        let id = self.next_id(); // id er afritað (copy)
        // búinn að skila self (lesaðgangi)
        self.bilar.push(Bill::new(id, framleidandi, Gerd::from(gerd), verd));
        //self.bilar.push(Bill::new(self.next_id(), framleidandi, Gerd::from(gerd), verd));
    }

    fn skra2(&mut self, framleidandi: &str, gerd: &str, verd: u32) {
        let b = Bill::new(self.next_id(), framleidandi, Gerd::from(gerd), verd);
        self.bilar.push(b);
    }
}

impl Display for Bilasala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bilar_texti = String::new();
        for bill in &self.bilar {
            bilar_texti.push_str(format!("{}\n", bill).as_str());
        }
        write!(f, "{}", bilar_texti)
    }
}

fn main() {
    let mut bs = Bilasala::new();
    bs.skra("toyota", "jeppi", 100000);
    bs.skra2("Volvo", "fb", 200000);
    
    loop {
        
    }
}