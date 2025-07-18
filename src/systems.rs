use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

use crate::board::{GameBoard, BOARD_WIDTH, BOARD_HEIGHT};
use crate::gem::GemType;

// 初始化系统，生成随机棋盘
pub fn setup(mut board: ResMut<GameBoard>) {
    *board = GameBoard::new_random();
}

// 简单输入系统：这里只是示例，实际应监听鼠标点击或拖动
pub fn input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut board: ResMut<GameBoard>,
    mut selected: Local<Option<(usize, usize)>>,
) {
    // 示例：按下空格键，随机交换两个宝石
    if keyboard_input.just_pressed(KeyCode::Space) {
        let x1 = rand::random::<usize>() % BOARD_WIDTH;
        let y1 = rand::random::<usize>() % BOARD_HEIGHT;
        let x2 = rand::random::<usize>() % BOARD_WIDTH;
        let y2 = rand::random::<usize>() % BOARD_HEIGHT;
        board.grid.swap((x1, y1), (x2, y2));
        *selected = Some((x2, y2));
    }
}

trait Swap {
    fn swap(&mut self, a: (usize, usize), b: (usize, usize));
}
impl Swap for [[GemType; BOARD_WIDTH]; BOARD_HEIGHT] {
    fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let tmp = self[a.1][a.0];
        self[a.1][a.0] = self[b.1][b.0];
        self[b.1][b.0] = tmp;
    }
}

// 检测三消
pub fn match_system(mut board: ResMut<GameBoard>) {
    let mut to_clear = vec![];

    // 横向检测
    for y in 0..BOARD_HEIGHT {
        for x in 0..(BOARD_WIDTH - 2) {
            let g = board.grid[y][x];
            if g == board.grid[y][x + 1] && g == board.grid[y][x + 2] {
                to_clear.push((x, y));
                to_clear.push((x + 1, y));
                to_clear.push((x + 2, y));
            }
        }
    }
    // 纵向检测
    for x in 0..BOARD_WIDTH {
        for y in 0..(BOARD_HEIGHT - 2) {
            let g = board.grid[y][x];
            if g == board.grid[y + 1][x] && g == board.grid[y + 2][x] {
                to_clear.push((x, y));
                to_clear.push((x, y + 1));
                to_clear.push((x, y + 2));
            }
        }
    }

    // 实际项目中应避免重复清除
    for &(x, y) in &to_clear {
        board.grid[y][x] = GemType::random();
    }
}

// 下落补充系统（示意，实际需更多逻辑处理）
pub fn fall_system(mut board: ResMut<GameBoard>) {
    for x in 0..BOARD_WIDTH {
        for y in (1..BOARD_HEIGHT).rev() {
            if board.grid[y][x] == GemType::Red // 假设Red代表空
                && board.grid[y - 1][x] != GemType::Red {
                board.grid[y][x] = board.grid[y - 1][x];
                board.grid[y - 1][x] = GemType::Red;
            }
        }
    }
}

// 补充新宝石
pub fn refill_system(mut board: ResMut<GameBoard>) {
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if board.grid[y][x] == GemType::Red {
                board.grid[y][x] = GemType::random();
            }
        }
    }
}