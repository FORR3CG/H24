struct Skoli {
    nafn: String,
}

fn main() {

    let skoli = Skoli {nafn: "Tskóli" };

    let mut fylki2d = [[0; 10]; 10];

    for lina in fylki2d {
        for mut stak in lina {
            stak += 10;
        }
    }

    for i in (1..=10).rev().step_by(2) {
        println!("i: {}", i);
    }

    'ytri: loop {
        'innri: loop {
            break 'ytri;
        }
    }

    let (a, b, c) = (10, 20, 30);

    if a == 10 && b == 20 {
        // gera eitthvað
    } else if b == 20 || c == 30 {
        // gera eitthvað
    } else if !(a == 10 && b == 20 || c != 30) {
        // gera eitthvað
    } else {
        // gera eitthvað
    }

    let k = if a == 10 { 
                    20 
                } else if b < 30  { 
                    30 
                } else {
                    99
                };

    let texti = "abc";

    match texti {
        "aaa" | "aab" => {
            println!("abc");
            println!("def");
        }, // gera eitthvað,
        "abc" => println!("abc"), // gera eitthvað,
        _ => println!("abc"), // annars gera þetta,
    }

    let k = match texti {
                    "abc" => 56,
                    _ => 99,
                };



    //fylki[index] = 99;


    let mut tup = (12u8, 3.14, 'g');
    tup.0 = 23;
    println!("{}-{}-{}", tup.0, tup.1, tup.2);

    // u8: 255, u16: ca. 65000, u32: ca 4 miljarðar, u64: , u128: 340 undesilljon
    // int u8, u16, u32, u64, u128 og samsvarandi i (signed), usize, isize
    // f32, f64
    // bool, char, 
    let mut b = 255u8;
    //b += 1; // b => 
    let k = 999 as f32 / 1000f32;
    let fjoldi_leikmanna = 5;
    let fjarsjodurinn = 250f32;
    let hlutur_hvers_leikmanns = fjarsjodurinn / fjoldi_leikmanna as f32;
    let stafur: char = '\n';
    println!("{}", stafur); 
    //let k = 0f64;
    println!("k: {:.12}", k);
    let g = 10i128;
    let f = 3.14f32 as f64;
    let mut g = "20";
    //g = 10;
    let i: u8; // 
    let mut j: u32;

    //k = 20;
    println!("Hello, world!");

}
