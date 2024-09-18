use std::fmt::Display;
use std::io::{stdin, stdout, Write};

enum AleggsTegund {
    Avoxtur,
    Fiskur,
    Graenmeti,
    Kjot,
    Ostur,
}

impl Display for AleggsTegund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let texti = match &self {
            AleggsTegund::Avoxtur => "Ávöxtur",
            AleggsTegund::Fiskur => "Fiskur",
            AleggsTegund::Graenmeti => "Grænmeti",
            AleggsTegund::Kjot => "Kjöt",
            AleggsTegund::Ostur => "Ostur",
        };
        write!(f, "{}", texti)
    }
}

impl From<&str> for AleggsTegund {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "a" | "á" | "ávöxtur" => AleggsTegund::Avoxtur,
            "f" | "fiskur" => AleggsTegund::Fiskur,
            "g" | "grænmeti" => AleggsTegund::Graenmeti,
            "k" | "kjöt" => AleggsTegund::Kjot,
            _ => AleggsTegund::Ostur,
        }
    }
}

struct Alegg {
    nafn: String,
    tegund: AleggsTegund,
    verd: u32,
}

impl Alegg {
    fn new(nafn: &str, tegund: &str, verd: u32) -> Self {
        Self {
            nafn: nafn.to_string(),
            tegund: AleggsTegund::from(tegund),
            verd,
        }
    }
}

impl Display for Alegg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Álegg: {}, tegund: {}, verð: {} kr.", self.nafn, self.tegund, self.verd)
    }
}

enum BotnTegund {
    Thunnur,
    Thykkur,
}

impl Display for BotnTegund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let texti = match self {
            BotnTegund::Thunnur => "Þunnur",
            BotnTegund::Thykkur => "Þykkur",
        };
        write!(f, "{}", texti)
    }
}

impl From<&str> for BotnTegund {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "þu" | "þunnur" => BotnTegund::Thunnur,
            _ => BotnTegund::Thykkur,
        }
    }
}

struct Botn {
    nafn: String,
    tegund: BotnTegund,
    verd: u32,
}

impl Botn {
    fn new(nafn: &str, tegund: &str, verd: u32) -> Self {
        Self {
            nafn: nafn.to_string(),
            tegund: BotnTegund::from(tegund),
            verd,
        }
    }
}

impl Display for Botn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Botn: {}, tegund: {}, verð: {} kr.", self.nafn, self.tegund, self.verd)
    }
}

struct Pizza {
    nafn: String,
    botn: Botn,
    alegg: Vec<Alegg>,
}

impl Pizza {
    fn new(nafn: &str, botn: Botn) -> Self {
        Self {
            nafn: nafn.to_string(),
            botn,
            alegg: Vec::new(),
        }
    }
    
    fn baeta_vid_aleggi(&mut self, nafn: &str, aleggstegund: &str, verd: u32) {
        self.alegg.push(Alegg::new(nafn, aleggstegund, verd));
    }
    
    fn verd(&self) -> u32 {
        let mut verd = 0;
        for a in &self.alegg {
            verd += a.verd;
        }
        verd + self.botn.verd
    }
}

impl Display for Pizza {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut texti = format!("Nafn: {}, verð: {} kr.", self.nafn, self.verd());
        texti += format!("\n\t{}", self.botn).as_str();
        for a in &self.alegg {
            texti += format!("\n\t{}", a).as_str();
        }

        write!(f, "{}", texti)
    }
}

fn main() {
    let mut pizza = Pizza::new("Tskóla pizzan", Botn::new("Þunnur", "þunnur", 1000));
    pizza.alegg.push(Alegg::new("Pepperoni", "kjöt", 200));
    pizza.baeta_vid_aleggi("Ostur", "ostur", 100);
    println!("{}", pizza);

    loop {
        print!("Sláðu inn skipun: ");
        stdout().flush().unwrap();
        let mut inntak = String::new();
        stdin().read_line(&mut inntak).unwrap();

        let skipanir = inntak.split_whitespace().collect::<Vec<&str>>();

        match skipanir.first() {
            Some(skipun) => {
                match skipun.to_lowercase().as_str() {
                    "hætta" => break,
                    "hjálp" => println!("Prentar hjálpina"),
                    "álegg" => { 
                        // t.d.: álegg Ananas ávöxtur 100
                        if skipanir.len() != 4 {
                            println!("Ekki réttur orðafjöldi til að búa til álegg!");
                            continue;
                        }
                        let nafn = skipanir[1];
                        let tegund = skipanir[2];
                        if let Ok(verd) = skipanir[3].trim().parse::<u32>() {
                            pizza.baeta_vid_aleggi(nafn, tegund, verd);
                        } else {
                            println!("Gat ekki breytt {} í tölu.", skipanir[3])
                        }
                        
                    }, 
                    "prenta" => println!("{}", pizza),
                    _ => println!("Skil ekki skipunina {}, reyndu aftur!", skipun),
                }
            },
            None => println!("Fann enga skipun! Reyndu aftur!!"),
        }
    }
}
