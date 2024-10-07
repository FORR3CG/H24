use std::fmt::Display;

struct Bill<'a> {
    tegund: &'a str,
    verd: u32,
}

impl<'a> Bill<'a> {
    fn new(tegund: &'a str, verd:u32) -> Self {
        Self {
            tegund,
            verd,
        }
    }
}

impl<'a> Display for Bill<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "teg: {}, verð: {}", self.tegund, self.verd)
    }
}

fn lengri<'a>(a: &'a str, b : &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn hallo() -> &'static str {
    "hallo"
}

fn main() {
    let a = "abc";
    let b = "abcde";

    let c = lengri(a, b);
}
