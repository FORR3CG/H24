fn main() {
    let mut nafn = String::from("Geir");
    //let n = nafn;
    //prenta_kvedju(nafn);
    //prenta_kvedju_ref(&nafn);
    nafn = prenta_kvedju_og_skila(nafn);
    println!("{}", nafn);
}

fn prenta_kvedju(n: String) {
    println!("Hæ {}", n);
}

fn prenta_kvedju_ref(n: &String) {
    println!("Hæ {}", n);
}

fn prenta_kvedju_og_skila(n: String) -> String {
    println!("Hæ {}", n);
    n
}
