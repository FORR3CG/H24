#[derive(Debug)]
enum IP {
    IPv4(u8, u8, u8, u8), // 192.168.1.1 // stack
    IPv6(String), // 2001:db8::ff00:42:8329 // heap
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Debug)]
enum Teikniform {
    Punktur(Point),
    Lina {p1: Point, p2: Point},
    Kassi {upphaf: Point, breidd: u32, haed: u32},
    Hringur { upphaf: Point, radius: f64},
}

fn teikna_hring(x: i32, y: i32, radius: f64) {
    println!("Teikna hring í {},{} með radius: {}", x, y, radius)
}

fn main() {
    //use Teikniform::{Punktur, Lina, Kassi, Hringur};
    let p = Teikniform::Punktur(Point {x: 10, y: 20});
    let l = Teikniform::Lina { p1: Point::new(20, 30), p2: Point::new(40, 30) };
    let h = Teikniform::Hringur { upphaf: Point::new(50, 50), radius: 50.0 };
    let k = Teikniform::Kassi { upphaf: Point::new(100, 100), breidd: 75, haed: 20 }; 

    let t = [p, l, h, k];

    for teikna in t {
        match teikna {
            Teikniform::Punktur(p) => println!("Teikna punkt í {},{}", p.x, p.y),
            Teikniform::Lina { p1, p2 } => println!("Teikna línu frá {:?} til {:?}", p1, p2),
            Teikniform::Kassi { upphaf, breidd, haed } => println!("Teikna kassa"),
            Teikniform::Hringur { upphaf, radius } => teikna_hring(upphaf.x, upphaf.y, radius),
        }
    }


    let localhost = IP::IPv4(127, 0, 0, 1);
    let maski = IP::IPv4(255, 255, 255, 0);
    let ip = IP::IPv6("2001:db8::ff00:42:8329".to_string());
    let iptolur = [&localhost, &maski, &ip];
    for iptala in iptolur {
        match iptala {
            IP::IPv4(a, b, c, d) => println!("fyrsta oktetið er: {}", a),
            IP::IPv6(i6) => println!("ip í hástöfum {}", i6.to_uppercase()),
        }
    } //
    let k = 255u8 as u32 + 255u8 as u32 + 255u8 as u32;
    println!("lh: {:?}, sm: {:?}, ip: {:?}", localhost, maski, ip);
    println!("Hello, world!");
}
