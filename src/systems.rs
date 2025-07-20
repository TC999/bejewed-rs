use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::window::PrimaryWindow;

use crate::board::{GameBoard, BOARD_WIDTH, BOARD_HEIGHT};
use crate::gem::GemType;
use crate::drag::DragState;
use crate::scoreboard::Scoreboard;
use bevy::text::TextStyle;
use bevy::ui::{PositionType, Style, Val};
use crate::gem::Gem;

// 初始化系统，初始化棋盘和相机
pub fn setup(mut commands: Commands, mut board: ResMut<GameBoard>) {
    println!("setup 系统执行，初始化棋盘");
    *board = GameBoard::new_random();
    commands.spawn(Camera2dBundle::default());
}

// 颜色映射
fn gem_color(gem: GemType) -> Color {
    match gem {
        GemType::Red => Color::RED,
        GemType::Green => Color::GREEN,
        GemType::Blue => Color::BLUE,
        GemType::Yellow => Color::YELLOW,
        GemType::Purple => Color::PURPLE,
    }
}

// 渲染系统
pub fn render_board_system(
    mut commands: Commands,
    board: Res<GameBoard>,
    query: Query<Entity, With<Sprite>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    let offset_x = -(BOARD_WIDTH as f32) * 20.0 + 20.0;
    let offset_y = -(BOARD_HEIGHT as f32) * 20.0 + 20.0;

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let color = gem_color(board.grid[y][x]);
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(36.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    offset_x + x as f32 * 40.0,
                    offset_y + y as f32 * 40.0,
                    0.0,
                ),
                ..Default::default()
            });
        }
    }
}

// 鼠标拖拽系统
pub fn drag_system(
    mut drag_state: ResMut<DragState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();
    if let Some(pos) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) {
            let board_x = ((world_pos.x + (BOARD_WIDTH as f32) * 20.0) / 40.0).floor() as usize;
            let board_y = ((world_pos.y + (BOARD_HEIGHT as f32) * 20.0) / 40.0).floor() as usize;
            if board_x < BOARD_WIDTH && board_y < BOARD_HEIGHT {
                // 按下鼠标左键，记录起点
                if mouse_button_input.just_pressed(MouseButton::Left) {
                    drag_state.start = Some((board_x, board_y));
                    drag_state.is_dragging = true;
                }
                // 拖拽中记录终点
                if drag_state.is_dragging {
                    drag_state.end = Some((board_x, board_y));
                }
                // 松开左键，结束拖拽
                if mouse_button_input.just_released(MouseButton::Left) && drag_state.is_dragging {
                    drag_state.is_dragging = false;
                }
            }
        }
    }
}

// 拖拽交换逻辑
pub fn handle_drag_swap(
    mut board: ResMut<GameBoard>,
    mut drag_state: ResMut<DragState>,
) {
    if !drag_state.is_dragging {
        if let (Some(start), Some(end)) = (drag_state.start, drag_state.end) {
            let dx = start.0 as i32 - end.0 as i32;
            let dy = start.1 as i32 - end.1 as i32;
            // 只允许交换相邻格
            if (dx.abs() == 1 && dy == 0) || (dy.abs() == 1 && dx == 0) {
                println!("交换宝石: {:?} <-> {:?}", start, end);
                board.grid.swap(start, end);
            }
            // 重置拖拽状态
            drag_state.start = None;
            drag_state.end = None;
        }
    }
}

// 交换 trait
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
pub fn match_system(
    mut commands: Commands,
    mut board: ResMut<GameBoard>,
    mut scoreboard: ResMut<Scoreboard>,
    gem_query: Query<(Entity, &Gem)>,
) {
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

    if !to_clear.is_empty() {
        for &(x, y) in &to_clear {
            // 移除对应Gem Entity
            for (entity, gem) in gem_query.iter() {
                if gem.x == x && gem.y == y {
                    commands.entity(entity).despawn();
                }
            }
            board.grid[y][x] = GemType::random();
        }
        scoreboard.total_removed += to_clear.len();
        scoreboard.score += to_clear.len() * 10;
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

pub fn gem_animation_system(
    mut query: Query<(&Gem, &mut Transform)>,
    time: Res<Time>,
) {
    let offset_x = -(BOARD_WIDTH as f32) * 20.0 + 20.0;
    let offset_y = -(BOARD_HEIGHT as f32) * 20.0 + 20.0;
    let speed = 20.0; // 每秒移动像素

    for (gem, mut transform) in query.iter_mut() {
        let target = Vec3::new(
            offset_x + gem.x as f32 * 40.0,
            offset_y + gem.y as f32 * 40.0,
            0.0,
        );
        let direction = target - transform.translation;
        let distance = direction.length();
        if distance > 1.0 {
            let step = direction.normalize() * speed * time.delta_seconds();
            if step.length() > distance {
                transform.translation = target;
            } else {
                transform.translation += step;
            }
        } else {
            transform.translation = target;
        }
    }
}

pub fn scoreboard_ui_system(
    scoreboard: Res<Scoreboard>,
    mut commands: Commands,
    query: Query<Entity, With<ScoreboardTag>>,
) {
    // 先清空旧的分数文本
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    // 居中显示分数
    commands.spawn((
        TextBundle::from_section(
            format!(
                "CLEAR: {}\nSCORE: {}",
                scoreboard.total_removed, scoreboard.score
            ),
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..Default::default()
        }),
        ScoreboardTag,
    ));
}

/// 仅用于标记计分板UI
#[derive(Component)]
pub struct ScoreboardTag;