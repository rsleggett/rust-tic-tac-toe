use bevy::prelude::*;

pub fn create_window() -> WindowPlugin {
    WindowPlugin    {
        primary_window: Some(Window {
            title: "TicTacToe".to_string(),
            ..default()
        }),
        ..default()
    }
}