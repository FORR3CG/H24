use std::fmt::Display;

use crate::stada::Stada;

pub struct Hestur {
    id: u32,
    nafn: String,
    aldur: u8,
    stada: Stada,
}

impl Hestur {
    pub fn new(id: u32, nafn: &str, aldur: u8, stada: &str) -> Self {
        Self {
            id,
            nafn: nafn.to_string(),
            aldur,
            stada: Stada::from(stada),
        }
    }

    pub fn breyta_stodu(&mut self, stada: &str) {
        self.stada = Stada::from(stada)
    }
}

impl Display for Hestur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, nafn: {}, aldur: {}, staða: {}", self.id, self.nafn, self.aldur, self.stada)
    }
}