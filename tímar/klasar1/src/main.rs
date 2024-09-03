mod bill;
mod litur;
use bill::Bill;
use litur::Litur;

fn main() {
    // #7f7f7fff, #00ff00ff, #0000ffff
    let k = std::mem::size_of_val(&0u128);
    println!("{}", k);
    let graenn = Litur::new(0, 255, 0, 255);
    println!("{}", graenn);
    let nissan = Bill {
        id: 100,
        tegund: "Nissan".to_string(),
        argerd: 2010,
        //litur: String::from("Grár"),
        //litur: Litur::new(127, 127,127, 255),
        litur: Litur::from(0xff0000ff),
    };

    println!("{}", nissan.to_string()   );
    let mut toyota = Bill::new(101, "Toyota", 2010, 0xff0000ff);
    toyota.argerd = 2011;
    println!("Teg: {}, árg.: {}", nissan.tegund, nissan.argerd);
    println!("{}", nissan);
}
