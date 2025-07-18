//fn main() {
//    println!("Hello, world!");
//}

use bevy::prelude::*;
mod board;
mod gem;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(board::GameBoard::default())
        .add_startup_system(systems::setup)
        .add_systems((
            systems::input_system,
            systems::swap_system,
            systems::match_system,
            systems::fall_system,
            systems::refill_system,
        ))
        .run();
}