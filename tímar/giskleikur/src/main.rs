use std::io::{stdin, stdout, Write};

use rand::Rng;

fn geir() {
    let g = "geir".to_string();
}

fn main() {
    let mut inntak2: i64;
    {
        let geir = "Geir".to_string();
    }
   // dbg!(geir);
    geir();
    println!("Ég hugsa mér tölu á bilinu 1 til og með 100, giskaðu á hver hún er.");
    //rand::SeedableRng::seed_from_u64(10);
    let mut r = rand::Rng::seed_from_u64(10);
    let tala = rand::thread_rng().gen_range(1..=100);
    let tala2 = rand::thread_rng().gen_range(1..=100);
    let mut x = rand::thread_rng();
    let l1 = x.gen_range(0..10);
    let l2 = x.gen_range(0..10);
    dbg!(l1, l2);
    println!("Leynitalan er {}", tala);
    loop {
        print!("Sláðu inn tölu: ");
        stdout().flush().unwrap();
        let mut inntak = String::new(); // python: inntak = ""
        stdin()
            .read_line(&mut inntak)
            .expect("Gat ekki lesið frá terminal!");

        let gisk = match inntak.trim().parse::<i32>() {
            Ok(t) => t,
            Err(_) => {
                println!("Gat ekki breytt {} í tölu, reyndu aftur!", inntak.trim());
                continue;
            }
        };

        if gisk > tala {
            println!("Of há tala!");
        } else if gisk < tala {
            println!("Of lág tala!");
        } else {
            println!("Þú giskaðir rétt!!!!");
            break;
        }

        // println!("Þú slóst inn {}", gisk);
    }
}
