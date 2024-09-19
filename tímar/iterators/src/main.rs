fn leggja_aftur_saman(a: i32, b: i32) -> i32 {
    a + b
}

fn reikna(a: i32, b: i32, f: impl Fn(i32, i32) -> i32) -> i32 {
    f(a, b)
}

fn main() {
    let mut listi = vec![1, 2, 3, 4, 5, 51, 99];
    let stor_gildi = listi.iter().filter(|stak| **stak > 50);

    let listi_b = listi.clone().into_iter().rev().collect::<Vec<_>>();

    let nyr_listi = listi.iter().map(|x| x + 10).collect::<Vec<_>>();

    let ziplisti = listi.iter().zip(nyr_listi).collect::<Vec<_>>();
    let summa_zip = ziplisti.iter().fold(0, |s, x| s + x.0 + x.1);

    // summa = sum(listi) í python
    let summa = listi.iter().fold(0, |summa, stak| summa + stak);
    let summa2 = listi.iter().fold(100, |summa, stak| summa + stak);
    let summa3 = listi
        .clone()
        .into_iter()
        .reduce(|summa, stak| summa + stak)
        .unwrap();

    println!("acc=0: {}, acc=100: {}, reduce: {}", summa, summa2, summa3);

    let tomur_vector: Vec<i32> = Vec::new();
    let summa1 = tomur_vector.iter().fold(0, |a, x| a + x);
    let summa2 = tomur_vector.clone().into_iter().reduce(|a, x| a + x);
    //.unwrap();
    dbg!(summa1, summa2);

    for i in &mut listi {
        *i += 10;
    }
    listi.iter_mut().for_each(|stak| *stak += 10);

    listi.iter().for_each(|stak| print!("{}", stak));

    for i in &listi {
        print!("{}", i);
    }

    let leggja_saman = |a: i32, b: i32| a + b;
    let draga_fra = |x: i32, y: i32| y - x;
    println!("Leggja saman: {}", reikna(10, 20, leggja_saman));
    println!("Draga frá: {}", reikna(10, 20, draga_fra));

    let haekka_um_einn = |a: &mut i32| *a += 1;
    let margra_linu_fall = || {
        println!("Þetta fall");
        println!("hefur margar línur");
    };
    println!("{}", leggja_saman(10, 20));
}
