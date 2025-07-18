use bevy::prelude::*;

mod board;
mod gem;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(board::GameBoard::default())
        .add_systems(Startup, (systems::setup, systems::render_board_system))
        .add_systems(Update, (
            systems::input_system,
            systems::match_system,
            systems::fall_system,
            systems::refill_system,
            systems::render_board_system, // 最后渲染
        ))
        .run();
}