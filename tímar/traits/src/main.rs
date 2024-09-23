
fn sidasta_i32(fylki: &[i32]) -> &i32 {
    fylki.last().unwrap()
}

fn sidasta_f64(fylki: &[f64]) -> &f64 {
    fylki.last().unwrap()
}

fn sidasta<T>(fylki: &[T]) -> &T { // T, U, V, W
    fylki.last().unwrap()
}

#[derive(Debug)]
struct Bill {
    tegund: String,
    litur: String,
    verd: u32,
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

impl TryFrom<&str> for Bill {
    type Error = String;
    // "bmw hvítur 1000"
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let hlutar = value.split_whitespace().collect::<Vec<&str>>(); // hlutar = value.split()
        if hlutar.len() == 3 {
            let tegund = hlutar.first().unwrap();
            let litur = hlutar.get(1).unwrap();
            match hlutar.last().unwrap().trim().parse::<u32>() {
                Ok(verd) => return Ok(Bill::new(tegund, litur, verd)),
                Err(_) => return Err(format!("Gat ekki breytt {} í tölu!", hlutar.last().unwrap())),
            }
        }
        Err("Fékk ekki réttan fjölda orða!!".to_string())
    }
}

fn main() {
    let bmw = Bill::try_from("bmw hvítur geir");
    match bmw {
        Ok(bill) => println!("{:?}", bill),
        Err(e) => println!("{}", e),
    }
   

    let i32fylki = [1,2,3,4,5];
    let f64fylki = [1.1, 1.2, 1.3];
    let i128fylki = [1i128, 2, 3, 4];

    println!("{}", sidasta(&i32fylki));
    println!("{}", sidasta(&f64fylki));
    println!("{}", sidasta(&i128fylki));

    println!("Hello, world!");
}
