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

impl Kottur {
    fn nafn(&self) {
        println!("{}", self.0)
    }
}

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

fn hvad_segir(dyr: &impl Hljod) {
    dyr.segir()
}

fn bua_til_dyr(teg: bool, nafn: &str) -> Box::<dyn Hljod> {
    if teg {
        // skila hundi
        Box::<Hundur>::new(Hundur(nafn.to_string()))
    } else {
        // skila ketti
        Box::new(Kottur(nafn.to_string()))
    }
}


fn main() {
    let depill = bua_til_dyr(true, "Depill");
    depill.segir();
    //hvad_segir(depill);
    let snati = Hundur("Snati".to_string());
    let grettir = Kottur("Grettir".to_string());
    println!("{}", snati.0);
    hvad_segir(&snati);
    hvad_segir(&grettir);
    snati.segir();
    grettir.segir();
    let f = Ferhyrningur {lengd: 10f32, breidd: 15f32};
    let h = Hringur { radius: 5f32 };
    println!("fh: fm: {}, um: {}", f.flatarmal(), f.ummal());
    f.nafn();
    println!("hr: fm: {}, um: {}", h.flatarmal(), h.ummal());
}
