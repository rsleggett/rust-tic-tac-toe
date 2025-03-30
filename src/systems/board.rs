use bevy::prelude::*;
use crate::constants::*;
use crate::components::{Square, CurrentPlayer, BoardState, Piece, GameState, VictoryText};

pub fn spawn_board(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let board_size = window.resolution.height() - PADDING * 2.0;
    let square_size = board_size / 3.0;
    let positions = [board_size / 3.0, board_size / 3.0 * 2.0];
    let line_thickness = 3.0;

    // Spawn squares
    for y in 0..3 {
        for x in 0..3 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(square_size - line_thickness, square_size - line_thickness)),
                        color: Color::rgba(1.0, 1.0, 1.0, 0.1),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * square_size - board_size / 2.0 + square_size / 2.0,
                        y as f32 * square_size - board_size / 2.0 + square_size / 2.0,
                        0.0,
                    )),
                    ..default()
                },
                Square { x, y },
            ));
        }
    }

    // Horizontal lines
    for x_pos in positions {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(line_thickness, board_size)),
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x_pos - board_size / 2.0, 0.0, 0.0)),
            ..default()
        });
    }

    // Vertical lines
    for y_pos in positions {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(board_size, line_thickness)),
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, y_pos - board_size / 2.0, 0.0)),
            ..default()
        });
    }
}

pub fn handle_hover(
    mut square_query: Query<(&Square, &mut Sprite)>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let board_size = window.resolution.height() - PADDING * 2.0;
    let square_size = board_size / 3.0;
    
    if let Some(cursor_pos) = window.cursor_position() {
        let board_pos = Vec2::new(
            cursor_pos.x - window.resolution.width() / 2.0,
            -(cursor_pos.y - window.resolution.height() / 2.0),
        );
        
        for (square, mut sprite) in square_query.iter_mut() {
            let square_pos = Vec2::new(
                square.x as f32 * square_size - board_size / 2.0 + square_size / 2.0,
                square.y as f32 * square_size - board_size / 2.0 + square_size / 2.0,
            );
            
            let is_hovered = board_pos.distance(square_pos) < square_size / 2.0;
            sprite.color = if is_hovered {
                Color::rgba(1.0, 1.0, 1.0, 0.3)
            } else {
                Color::rgba(1.0, 1.0, 1.0, 0.1)
            };
        }
    }
}

fn check_win_condition(board: &[[Option<u8>; 3]; 3]) -> Option<u8> {
    // Check rows
    for row in board.iter() {
        if row[0] == row[1] && row[1] == row[2] {
            if let Some(player) = row[0] {
                return Some(player);
            }
        }
    }

    // Check columns
    for col in 0..3 {
        if board[0][col] == board[1][col] && board[1][col] == board[2][col] {
            if let Some(player) = board[0][col] {
                return Some(player);
            }
        }
    }

    // Check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        if let Some(player) = board[1][1] {
            return Some(player);
        }
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        if let Some(player) = board[1][1] {
            return Some(player);
        }
    }

    None
}

pub fn handle_victory_state(
    mut commands: Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    victory_query: Query<Entity, With<VictoryText>>,
) {
    // Only run this system when the game just ended
    if game_state.is_changed() && game_state.game_over {
        // Remove any existing victory text
        for entity in victory_query.iter() {
            commands.entity(entity).despawn();
        }

        // Create the victory text
        let message = if let Some(winner) = game_state.winner {
            format!("Player {} wins!", winner)
        } else {
            "It's a draw!".to_string()
        };

        // Spawn the text
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    message,
                    TextStyle {
                        font_size: 60.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 300.0, 1.0),
                ..default()
            },
            VictoryText,
        ));

        // Play victory sound if we have one
        if game_state.winner.is_some() {
            let victory_sound = asset_server.load("sounds/victory.ogg");
            commands.spawn(AudioBundle {
                source: victory_sound,
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

pub fn handle_mouse_clicks(
    buttons: Res<ButtonInput<MouseButton>>, 
    mut commands: Commands,
    windows: Query<&Window>,
    mut current_player: ResMut<CurrentPlayer>,
    mut board_state: ResMut<BoardState>,
    mut game_state: ResMut<GameState>,
    square_query: Query<(&Square, &Transform)>,
) {
    // Don't allow moves if game is over
    if game_state.game_over {
        return;
    }

    let window = windows.single();
    let board_size = window.resolution.height() - PADDING * 2.0;
    let square_size = board_size / 3.0;
    
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            let board_pos = Vec2::new(
                cursor_pos.x - window.resolution.width() / 2.0,
                -(cursor_pos.y - window.resolution.height() / 2.0),
            );
            
            // Find which square was clicked
            for (square, transform) in square_query.iter() {
                let square_pos = Vec2::new(transform.translation.x, transform.translation.y);
                if board_pos.distance(square_pos) < square_size / 2.0 {
                    // Check if square is empty
                    if board_state.board[square.y as usize][square.x as usize].is_none() {
                        // Place piece
                        let piece_size = square_size * 0.8;
                        let piece_color = if current_player.0 == 1 {
                            Color::rgb(1.0, 0.0, 0.0) // Red
                        } else {
                            Color::rgb(0.0, 0.0, 1.0) // Blue
                        };
                        
                        if current_player.0 == 1 {
                            // Draw X
                            let line_thickness = piece_size * 0.15;
                            let line_length = piece_size * 0.8;
                            
                            // First line of X (top-left to bottom-right)
                            commands.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(line_thickness, line_length)),
                                        color: piece_color,
                                        ..default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        square_pos.x,
                                        square_pos.y,
                                        0.0,
                                    )).with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4)),
                                    ..default()
                                },
                                Piece,
                            ));
                            // Second line of X (top-right to bottom-left)
                            commands.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(line_thickness, line_length)),
                                        color: piece_color,
                                        ..default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        square_pos.x,
                                        square_pos.y,
                                        0.0,
                                    )).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
                                    ..default()
                                },
                                Piece,
                            ));
                        } else {
                            // Draw O
                            let outer_size = piece_size * 0.9;
                            let inner_size = piece_size * 0.7;
                            
                            commands.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(outer_size, outer_size)),
                                        color: piece_color,
                                        ..default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        square_pos.x,
                                        square_pos.y,
                                        0.0,
                                    )),
                                    ..default()
                                },
                                Piece,
                            ));
                            // Inner circle (hole)
                            commands.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(inner_size, inner_size)),
                                        color: Color::BLACK,
                                        ..default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        square_pos.x,
                                        square_pos.y,
                                        0.1,
                                    )),
                                    ..default()
                                },
                                Piece,
                            ));
                        }
                        
                        // Update board state
                        board_state.board[square.y as usize][square.x as usize] = Some(current_player.0);
                        
                        // Check for win
                        if let Some(winner) = check_win_condition(&board_state.board) {
                            game_state.winner = Some(winner);
                            game_state.game_over = true;
                            println!("Player {} wins!", winner);
                        } else {
                            // Check for draw
                            let is_draw = board_state.board.iter().all(|row| {
                                row.iter().all(|cell| cell.is_some())
                            });
                            if is_draw {
                                game_state.game_over = true;
                                println!("Game ended in a draw!");
                            }
                        }

                        // Switch turns if game isn't over
                        if !game_state.game_over {
                            current_player.0 = if current_player.0 == 1 { 2 } else { 1 };
                        }
                        break;
                    }
                }
            }
        }
    }
}

