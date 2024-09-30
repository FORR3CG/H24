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

fn main() {
    println!("Hello, world!");
}
