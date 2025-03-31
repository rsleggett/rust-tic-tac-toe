use bevy::prelude::*;
mod components;
mod constants;
mod systems;

use components::*;
use systems::board::{spawn_board, handle_hover};
use systems::game_logic::handle_mouse_clicks;
use systems::victory::{handle_victory_state, handle_reset_button};
use systems::camera::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tic Tac Toe".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(CurrentPlayer(1))
        .insert_resource(BoardState {
            board: [[None; 3]; 3],
            current_player: 1,
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
