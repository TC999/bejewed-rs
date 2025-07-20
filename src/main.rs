use bevy::prelude::*;

mod board;
mod gem;
mod drag;
mod systems;
mod scoreboard; // 新增

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(board::GameBoard::default())
        .insert_resource(drag::DragState::default())
        .insert_resource(scoreboard::Scoreboard::default()) // 新增
        .add_systems(Startup, (systems::setup, systems::render_board_system))
        .add_systems(Update, (
            systems::drag_system,
            systems::handle_drag_swap,
            systems::match_system,
            systems::fall_system,
            systems::refill_system,
            systems::gem_animation_system,
            systems::render_board_system,
            systems::scoreboard_ui_system, // 新增
        ))
        .run();
}