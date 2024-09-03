
fn hallo() {
    println!("halló")
}

fn hallo_nafn(nafn: &str) {
    println!("Halló {}", nafn);
}



fn skilar_fimm() -> u8 {
    5
}

fn leggja_saman(a: i32, b: i32) -> i32 {
    // let c = a + b;
    // return c;
    a + b
}



fn sjo_ef_haerri_en_fimm(x: i32) -> i32 {
    if x > 5 {
        7
    } else {
        10
    }
}

fn skila_tveimur_gildum(x: i32, y: i32) -> (i32, i32) {
    (x + 10, y - 10)
}

fn fall_sem_a_breytur() {
    let a = 20;
    let b = 30;
    let c = skila_tveimur_gildum(20, 10);
}

fn plus_einn(x: i32) -> i32 {
    x + 1
}

fn haekka_um_einn(x: &mut i64) {
    *x += 1;
}

fn mut_fall(x: i32) {
    //x = 10;
}
fn main() { 
    let mut t = 12i64;
    dbg!(t);
    haekka_um_einn(&mut t);
    dbg!(t);
    let j = plus_einn(t as i32);

    let (x, y) = skila_tveimur_gildum(20, 30);
    fall_sem_a_breytur();
    let tup = skila_tveimur_gildum(30, 40);
    let (k, mut j) = skila_tveimur_gildum(20, 30);
    dbg!(k, j, tup);
    dbg!(j);
    dbg!(tup.0);
    dbg!(tup.1);
    let summa = leggja_saman(tup.0, tup.1);
    dbg!(summa);
    let mut i = 10;
    i += 1;

   

    let b = skilar_fimm();
}
