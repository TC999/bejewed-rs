use rand::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GemType {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
}

impl GemType {
    pub fn random() -> Self {
        match rand::random::<u8>() % 5 {
            0 => GemType::Red,
            1 => GemType::Green,
            2 => GemType::Blue,
            3 => GemType::Yellow,
            _ => GemType::Purple,
        }
    }
}

impl Default for GemType {
    fn default() -> Self {
        GemType::Red
    }
}