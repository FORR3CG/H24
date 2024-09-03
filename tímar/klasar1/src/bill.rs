use std::fmt::Display;


#[derive(Debug)] // trait
pub struct Bill {
    pub id: u32,
    pub tegund: String,
    pub argerd: u16,
    pub litur: String,
}

impl Bill {
    pub fn new(id: u32, tegund: &str, argerd: u16, litur: &str) -> Self {
        Self {
            id,
            tegund: tegund.to_string(),
            argerd,
            litur: litur.to_string(),
        }
    }

/*     fn prenta(&self) {
        println!("{}", self.to_string());
    }

    fn to_string(&self) -> String {
        format!("Nr.: {}, tegund: {}, litur: {}, árgerð: {}", self.id, self.tegund, self.litur, self.argerd)
    } */
}

impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nr.: {}, tegund: {}, litur: {}, árgerð: {}", self.id, self.tegund, self.litur, self.argerd)
    }
}
