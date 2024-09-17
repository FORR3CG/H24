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

fn main() {
   
}
