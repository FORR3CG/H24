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

    fn prenta(&self, gerd: Gerd) {
        for bill in &self.bilar {
            
        }
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
        print!("Sláðu inn skipun: ");
        stdout().flush().unwrap();
        let mut inntak = String::new();
        stdin().read_line(&mut inntak).unwrap();
        let skipanir = inntak.split_whitespace().collect::<Vec<&str>>();
        //let skipanir = inntak.split(' ').collect::<Vec<&str>>();
        println!("{:?}", skipanir);

        match skipanir.first() { // .get(4) í stað [4]
            Some(s) => {
                match s.to_lowercase().as_str() {
                    "hætta" => break,
                    "hjálp" => println!("skrifum út hjálpina"),
                    "skrá" => {
                        if skipanir.len() != 4 {
                            println!("skrá línana inniheldur ekki réttan fjölda orða!");
                            continue;
                        }
                        let framleidandi = skipanir[1];
                        let gerd = skipanir[2];
                        let verd = match skipanir[3].parse::<u32>() {
                            Ok(tala) => tala,
                            Err(_) => {
                                println!("Gat ekki breytt {} í tölu!", skipanir[3]);
                                continue;
                                //panic!("Þú slóst inn bókstafi í stað tölustafa!!!!");
                            },
                        };
                        bs.skra(framleidandi, gerd, verd);
                    },
                    "prenta" => {
                        println!("{}", bs);
                    },
                    _ => {
                        println!("skildi ekki skipunina: {}", s);
                        continue;
                    }
                }
            },
            None => {
                println!("Þú verður að slá eitthvað inn!");
                continue;
            },
        }

    }
}