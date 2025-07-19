use bevy::prelude::*;

mod board;
mod gem;
mod drag;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(board::GameBoard::default())
        .insert_resource(drag::DragState::default())
        .add_systems(Startup, (systems::setup, systems::render_board_system))
        .add_systems(Update, (
            systems::drag_system,
            systems::handle_drag_swap,
            systems::match_system,
            systems::fall_system,
            systems::refill_system,
            systems::render_board_system, // 放最后，保证显示最新
        ))
        .run();
}