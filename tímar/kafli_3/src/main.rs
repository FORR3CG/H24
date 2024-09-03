use core::f64;
use std::path::MAIN_SEPARATOR;

fn fall_sem_tekur_fylki(fylkid: &[[u8; 10]; 10]) {

}

fn main() {

    let mut fylki = [[0u8; 10]; 10];

    let n = "texti".to_string();
    let n1 = "texti".to_string();
    let n2 = "ógeðslega langur texti".to_string();
    let n3 = "texti".to_string();
    let n4 = "texti".to_string();
    let n5 = "texti".to_string();

    let t = Box::new(1);

    let stor_tala = 1u128;

    // fyllum fylkið af tölum

    fall_sem_tekur_fylki(&fylki);

    let (x, y, mut z) = (10, 20u8, 1.0);
    z = f64::consts::PI;
    let mut t: (i32, u8, f64) = (20, 30, 9.9);
    println!("{}", t.2);

    let a = [10, 20, 30 ,40, 50];

    let tomt_array: [u8; 10];

    let null_array = [0u8; 100];

    println!("{:#?}", null_array);

    let f2d = [[[1, 2], [3,4]], [[5,6], [7,8]]];

    for i in (1..=10).rev().step_by(2) {
        print!("{} ", i);
    }

    let mut tiu_taflan = [[0; 10]; 10];

    for i in 1..=10 {
        for j in 1..=10 {
            tiu_taflan[i - 1][j - 1] = i * j;
        }
    }
    
    println!();
    for linu in tiu_taflan {
        for tala in linu {
            print!("{:4} ", tala);
        }
        println!();
    }

    for lina in &mut tiu_taflan {
        for tala in lina {
            *tala = *tala * 2;
        }
    }
    
}
