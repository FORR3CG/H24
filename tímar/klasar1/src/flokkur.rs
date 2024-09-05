use std::fmt::Display;

#[derive(Debug)]
pub enum Flokkur {
    Folksbill, // 0
    Jeppi, // 1
    Vorubill, // 2
    Annad, // 3
}

impl Display for Flokkur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flokkur = match self {
            Flokkur::Folksbill => "Fólksbíll",
            Flokkur::Jeppi => "Jeppi",
            Flokkur::Vorubill => "Vörubíll",
            Flokkur::Annad => "Annað",
        };
        write!(f, "{}", flokkur)
    }
}

// value verður jeppi, JEPPI, Jeppi, fólksbíll, sjdlfkas fl
impl From<&str> for Flokkur {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "fólksbíll" => Flokkur::Folksbill,
            "jeppi" => Flokkur::Jeppi,
            "vörubíll" => Flokkur::Vorubill,
            _ => Flokkur::Annad,
        }
    }
}
