use bevy::prelude::*;
mod components;
mod constants;
mod systems;

use components::{CurrentPlayer, BoardState, GameState};
use systems::board::{spawn_board, handle_hover};
use systems::game_logic::handle_mouse_clicks;
use systems::victory::{handle_victory_state, handle_reset_button};
use systems::camera::spawn_camera;
use systems::window::create_window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(create_window()))
        .insert_resource(CurrentPlayer(1))
        .insert_resource(BoardState {
            board: [[None; 4]; 4]
        })
        .insert_resource(GameState {
            game_over: false,
            winner: None,
        })
        .add_systems(Startup, (spawn_camera, spawn_board))
        .add_systems(Update, (
            handle_hover,
            handle_mouse_clicks,
            handle_victory_state,
            handle_reset_button,
        ))
        .run();
}
