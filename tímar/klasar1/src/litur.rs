use std::fmt::Display;

#[derive(Debug)]
pub struct Litur {
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
}

impl Litur {
    pub fn new(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Self {
            r,
            g,
            b,
            alpha,
        }
    }

/*     pub fn nafn(&self) -> String {
        format!("{}", self.alpha)
    } */
}

impl Display for Litur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // viljum sýna litinn sem #RRGGBBAA
        //write!(f, "#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.alpha)
        let tala: u32 = (self.r as u32) << 24 |  (self.g as u32) << 16 | 
                        (self.b as u32) << 8 | self.alpha as u32;
       // for i in (0..=24).rev().step_by(8)
        write!(f, "#{:08X}", tala)
    }
}

impl From<u32> for Litur {
    fn from(value: u32) -> Self {
        Self {
            r: (value >> 24) as u8,
            g: (value >> 16) as u8,
            b: (value >> 8) as u8,
            alpha: value as u8,
        }
    }
}

