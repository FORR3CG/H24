use std::io::{stdin, stdout, Write};

fn fall() {
    let y = Box::new(32);
    dbg!(y);
}

fn fylla_breytu(s: &mut String) {
    s.insert_str(0,"tskóli");
}

fn fylla_virkar_ekki(s: mut String) {
    s.insert_str(0, "ekki tskóli");
}

fn main() {
   // print!("Sláðu inn tölu: ");
    let mut skoli = String::new();
    fylla_breytu(&mut skoli);
    println!("{}", skoli);
    print!("Sláðu inn tölu: ");
    stdout().flush().unwrap();
    let mut inntak = String::new(); // python: inntak = ""
    stdin()
        .read_line(&mut inntak)
        .expect("Gat ekki lesið frá terminal!");
    println!("inntak: {}", inntak);
 
    //println!("{:p}", k); 
    /* let j = 20;
    println!("{:p}", &j);
    for _ in 0..5 {
        fall();
    } */
}
