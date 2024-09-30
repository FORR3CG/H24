use core::f32;

struct Ferhyrningur {
    lengd: f32,
    breidd: f32,
}

impl Form for Ferhyrningur {
    fn flatarmal(&self) -> f32 {
        self.lengd * self.breidd
    }

    fn ummal(&self) -> f32 {
        2. * self.lengd + 2. * self.breidd
    }
}

struct Hringur {
    radius: f32,
}

impl Form for Hringur {
    fn flatarmal(&self) -> f32 {
        self.radius.powf(2f32) * f32::consts::PI
    }

    fn ummal(&self) -> f32 {
        self.radius * 2f32 * f32::consts::PI
    }
}

trait Form {
    fn flatarmal(&self) -> f32;
    fn ummal(&self) -> f32;
    fn nafn(&self) {
        println!("Ónefnt!")
    }
}

struct Hundur(String);

struct Kottur(String);

trait Hljod {
    fn segir(&self);
}

impl Hljod for Hundur {
    fn segir(&self) {
        println!("Voff")
    }
}

impl Hljod for Kottur {
    fn segir(&self) {
        println!("Mjá")
    }
}

fn main() {
    let snati = Hundur("Snati".to_string());
    let grettir = Kottur("Grettir".to_string());
    println!("{}", snati.0);
    snati.segir();
    grettir.segir();
    let f = Ferhyrningur {lengd: 10f32, breidd: 15f32};
    let h = Hringur { radius: 5f32 };
    println!("fh: fm: {}, um: {}", f.flatarmal(), f.ummal());
    f.nafn();
    println!("hr: fm: {}, um: {}", h.flatarmal(), h.ummal());
}
