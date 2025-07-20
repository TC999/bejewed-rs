use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum GemType {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
}
impl GemType {
    pub fn random() -> Self {
        use GemType::*;
        match fastrand::usize(0..5) {
            0 => Red,
            1 => Green,
            2 => Blue,
            3 => Yellow,
            _ => Purple,
        }
    }
}

#[derive(Component)]
pub struct Gem {
    pub x: usize,
    pub y: usize,
}