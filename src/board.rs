use crate::gem::GemType;

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_HEIGHT: usize = 8;

#[derive(Default)]
pub struct GameBoard {
    pub grid: [[GemType; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl GameBoard {
    pub fn new_random() -> Self {
        let mut grid = [[GemType::Red; BOARD_WIDTH]; BOARD_HEIGHT];
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                grid[y][x] = GemType::random();
            }
        }
        Self { grid }
    }
}