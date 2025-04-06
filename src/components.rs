use bevy::prelude::*;
use crate::constants::BOARD_SIZE;

#[derive(Component)]
pub struct Piece;

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

#[derive(Resource)]
pub struct GameState {
    pub winner: Option<u8>,
    pub game_over: bool,
    pub board_size: usize,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            winner: None,
            game_over: false,
            board_size: BOARD_SIZE as usize,
        }
    }
}

#[derive(Resource)]
pub struct BoardState {
    pub board: Vec<Vec<Option<u8>>>,
}

impl Default for BoardState {
    fn default() -> Self {
        let size = BOARD_SIZE as usize;
        Self {
            board: vec![vec![None; size]; size],
        }
    }
}