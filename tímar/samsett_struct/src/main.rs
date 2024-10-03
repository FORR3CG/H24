struct Stadsetning {
    haed: u8,
    herbergi: u8,
}

struct Audkenni {
    id: u32,
    nafn: String,
}

impl TryFrom<&str> for Audkenni {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // parsa value og græja
        Ok(Self {
            id: 20,
            nafn: "Nafn".to_string(),
        })
    }
}

struct Hundur {
    audkenni: Audkenni,
    hlydnieinkunn: u32,
}

impl TryFrom<&str> for Hundur {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // id nafn einkunn
        // 24 Snati 87
        // nafn einkunn id
        let lidir = value.split_whitespace().collect::<Vec<&str>>();
        let hlydnieinkunn = 87;
        Ok(Self {
            audkenni: Audkenni::try_from(format!("{} {}", lidir[0], lidir[2]).as_str())?,
            hlydnieinkunn,
        })
    }
}

struct Kottur {
    audkenni: Audkenni,
    aldur: u8,
}

impl TryFrom<&str> for Kottur {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // 24 Snati 87
        let aldur = 87;
        Ok(Self {
            audkenni: Audkenni::try_from("24 Snati")?,
            aldur,
        })
    }
}

fn main() {
    println!("Hello, world!");
}
