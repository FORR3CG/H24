use std::fmt::Display;
use crate::litur::Litur;

#[derive(Debug)] // trait
pub struct Bill {
    pub id: u32,
    pub tegund: String,
    pub argerd: u16,
    pub litur: Litur,
}

impl Bill {
    pub fn new(id: u32, tegund: &str, argerd: u16, litur: u32) -> Self {
        Self {
            id,
            tegund: tegund.to_string(),
            argerd,
            litur: Litur::from(litur),
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
