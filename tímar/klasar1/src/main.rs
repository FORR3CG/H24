mod bill;
use bill::Bill;

fn main() {
    let nissan = Bill {
        id: 100,
        tegund: "Nissan".to_string(),
        argerd: 2010,
        litur: String::from("Grár"),
    };
    println!("{}", nissan.to_string()   );
    let mut toyota = Bill::new(101, "Toyota", 2010, "Rauður");
    toyota.argerd = 2011;
    println!("Teg: {}, árg.: {}", nissan.tegund, nissan.argerd);
    println!("{}", nissan);
}
