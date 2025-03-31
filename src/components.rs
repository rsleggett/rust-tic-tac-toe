use bevy::prelude::*;

#[derive(Component)]
pub struct Board;

#[derive(Component)]
pub struct Piece;

#[derive(Component)]
pub struct Player {
    pub id: u8,
}

#[derive(Component)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
pub struct HoverSquare;

#[derive(Component)]
pub struct VictoryText;

#[derive(Component)]
pub struct ResetButton;

#[derive(Resource)]
pub struct CurrentPlayer(pub u8);

impl Default for CurrentPlayer {
    fn default() -> Self {
        CurrentPlayer(1)
    }
}

#[derive(Resource, Default)]
pub struct GameState {
    pub winner: Option<u8>,
    pub game_over: bool,
}

#[derive(Resource)]
pub struct BoardState {
    pub board: [[Option<u8>; 3]; 3],
    pub current_player: u8,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            board: [[None; 3]; 3],
            current_player: 1,
        }
    }
}