use bevy::prelude::*;
pub mod components;
mod systems;
mod constants;
use systems::*;
use components::{CurrentPlayer, BoardState, GameState};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(create_window()))
    .init_resource::<CurrentPlayer>()
    .init_resource::<BoardState>()
    .init_resource::<GameState>()
    .add_systems(Startup, (
        spawn_board,
        spawn_camera,
    ))
    .add_systems(Update, (
        handle_mouse_clicks,
        handle_hover,
    ))
    .add_systems(Update, handle_victory_state)
    .run();
}
