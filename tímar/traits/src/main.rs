use std::cmp::Reverse;


fn sidasta_i32(fylki: &[i32]) -> &i32 {
    fylki.last().unwrap()
}

fn sidasta_f64(fylki: &[f64]) -> &f64 {
    fylki.last().unwrap()
}

fn sidasta<T>(fylki: &[T]) -> &T { // T, U, V, W
    fylki.last().unwrap()
}



impl Bill {
    fn new(tegund: &str, litur: &str, verd: u32) -> Self {
        Self {
            tegund: tegund.to_string(),
            litur: litur.to_string(),
            verd,
        }
    }
}

// let b = bmw + audi;

impl std::ops::Add for Bill {
    type Output = Bill;

    fn add(self, rhs: Self) -> Self::Output {
        Bill::new(&self.tegund, &rhs.litur, self.verd + rhs.verd)
    }
}

impl TryFrom<&str> for Bill {
    type Error = String;
    // "bmw hvítur 1000"
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let hlutar = value.split_whitespace().collect::<Vec<&str>>(); // hlutar = value.split()
        if hlutar.len() == 3 {
            let tegund = hlutar.first().unwrap();
            let litur = hlutar.get(1).unwrap();
            if let Ok(verd) = hlutar.last().unwrap().trim().parse::<u32>() {
                return Ok(Bill::new(tegund, litur, verd))
            } else {
                return Err(format!("Gat ekki breytt {} í tölu!", hlutar.last().unwrap()))
            }
        }
        Err("Fékk ekki réttan fjölda orða!!".to_string())
    }
}

#[derive(Debug, Eq, Clone)]
struct Bill {
    tegund: String,
    litur: String,
    verd: u32,
}

impl Ord for Bill {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.tegund == other.tegund {
            self.verd.cmp(&other.verd)
        } else {
            self.tegund.cmp(&other.tegund)
        }
    }
}

impl PartialOrd for Bill {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Bill {
    fn eq(&self, other: &Self) -> bool {
        self.tegund == other.tegund
    }
}

struct Bilar {
    bilar: Vec<Bill>,
}

impl Bilar {
    fn new() -> Self {
        Self {
            bilar: Vec::new(),
        }
    }

    fn skra(&mut self, bill: &str) -> Result<(), String> {
        self.bilar.push(Bill::try_from(bill)?);
        Ok(())
    }
}

fn main() {
    let mut b = Bilar::new();
    loop {
        if let Err(villa) = b.skra("bmw gulur 2000") {
            println!("{}, reyndur aftur!", villa);
            continue;
        }
    }

    let mut bilar: Vec<Bill> = Vec::new();
    let audi = Bill::try_from("audi gulur 10000").unwrap();
    let toyota = Bill::try_from("audi hvítur 500").unwrap();
    if audi == toyota {
        // hvenær á þetta að vera true???
    }

    let ayota = audi.clone() + toyota.clone();
    let ayota = audi.clone() + toyota.clone() + ayota.clone();
    println!("{:?}", ayota);
    bilar.push(audi);
    bilar.push(toyota);
/*     if audi > toyota {
        println!("audi er stærri");
    } else {
        println!("toyota er stærri")
    } */

    let bmw = Bill::try_from("bmw hvítur 1000");
    match bmw {
        Ok(bill) => bilar.push(bill),
        Err(e) => println!("{}", e),
    }

    if let Ok(bill) = Bill::try_from("audi rauður 1200") {
        bilar.push(bill);
    } else {
        println!("Einhver villa! Reyndu aftur");
    }
    println!("fyrir röðun: {:#?}", bilar);
    bilar.sort();
    println!("eftir röðun: {:#?}", bilar);

    let i32fylki = [1,2,3,4,5];
    let f64fylki = [1.1, 1.2, 1.3];
    let i128fylki = [1i128, 2, 3, 4];

    println!("{}", sidasta(&i32fylki));
    println!("{}", sidasta(&f64fylki));
    println!("{}", sidasta(&i128fylki));

    println!("Hello, world!");
}
