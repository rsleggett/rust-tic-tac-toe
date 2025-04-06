use bevy::prelude::*;
use crate::constants::*;
use crate::components::{Square, CurrentPlayer, BoardState, Piece, GameState, HoverSquare};

pub fn check_win_condition(board: &Vec<Vec<Option<u8>>>, board_size: usize) -> Option<u8> {
    // Check rows
    for row in 0..board_size {
        let first = board[row][0];
        if first.is_some() && (0..board_size).all(|col| board[row][col] == first) {
            return first;
        }
    }

    // Check columns
    for col in 0..board_size {
        let first = board[0][col];
        if first.is_some() && (0..board_size).all(|row| board[row][col] == first) {
            return first;
        }
    }

    // Check main diagonal
    let first = board[0][0];
    if first.is_some() && (0..board_size).all(|i| board[i][i] == first) {
        return first;
    }

    // Check counter diagonal
    let first = board[0][board_size - 1];
    if first.is_some() && (0..board_size).all(|i| board[i][board_size - 1 - i] == first) {
        return first;
    }

    None
}

pub fn handle_mouse_clicks(
    buttons: Res<ButtonInput<MouseButton>>, 
    mut commands: Commands,
    windows: Query<&Window>,
    mut current_player: ResMut<CurrentPlayer>,
    mut board_state: ResMut<BoardState>,
    mut game_state: ResMut<GameState>,
    square_query: Query<(&Square, &Transform), Without<HoverSquare>>,
    ui_interaction: Query<&Interaction, With<Button>>,
) {
    // Don't allow moves if game is over
    if game_state.game_over {
        return;
    }

    // Don't process board clicks if we're interacting with UI
    if ui_interaction.iter().any(|interaction| *interaction == Interaction::Pressed) {
        return;
    }

    let window = windows.single();
    let board_size = window.resolution.height() - PADDING * 2.0;
    let square_size = board_size / game_state.board_size as f32;
    
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
                        spawn_piece(&mut commands, current_player.0, square_pos, square_size);
                        
                        // Update board state
                        board_state.board[square.y as usize][square.x as usize] = Some(current_player.0);
                        
                        // Check for win
                        if let Some(winner) = check_win_condition(&board_state.board, game_state.board_size) {
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

fn spawn_piece(commands: &mut Commands, player: u8, pos: Vec2, size: f32) {
    let piece_size = size * 0.8;
    let piece_color = if player == 1 {
        Color::srgb(1.0, 0.0, 0.0) // Red
    } else {
        Color::srgb(0.0, 0.0, 1.0) // Blue
    };
    
    if player == 1 {
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
                    pos.x,
                    pos.y,
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
                    pos.x,
                    pos.y,
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
                    pos.x,
                    pos.y,
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
                    pos.x,
                    pos.y,
                    0.1,
                )),
                ..default()
            },
            Piece,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_conditions() {
        // Test with 4x4 board
        let board_size = 4;
        let mut board = vec![vec![None; board_size]; board_size];

        // Test row win
        board[0] = vec![Some(1), Some(1), Some(1), Some(1)];
        assert_eq!(check_win_condition(&board, board_size), Some(1));

        // Test column win
        board = vec![vec![None; board_size]; board_size];
        for row in 0..board_size {
            board[row][0] = Some(2);
        }
        assert_eq!(check_win_condition(&board, board_size), Some(2));

        // Test main diagonal win
        board = vec![vec![None; board_size]; board_size];
        for i in 0..board_size {
            board[i][i] = Some(1);
        }
        assert_eq!(check_win_condition(&board, board_size), Some(1));

        // Test counter diagonal win
        board = vec![vec![None; board_size]; board_size];
        for i in 0..board_size {
            board[i][board_size - 1 - i] = Some(2);
        }
        assert_eq!(check_win_condition(&board, board_size), Some(2));

        // Test no win
        board = vec![vec![None; board_size]; board_size];
        assert_eq!(check_win_condition(&board, board_size), None);
    }

    #[test]
    fn test_near_wins() {
        let board_size = 4;
        let mut board = vec![vec![None; board_size]; board_size];

        // Test near win in row
        board[0] = vec![Some(1), Some(1), Some(1), Some(2)];
        assert_eq!(check_win_condition(&board, board_size), None);

        // Test near win in column
        board = vec![vec![None; board_size]; board_size];
        for row in 0..board_size - 1 {
            board[row][0] = Some(2);
        }
        board[board_size - 1][0] = Some(1);
        assert_eq!(check_win_condition(&board, board_size), None);

        // Test near win in main diagonal
        board = vec![vec![None; board_size]; board_size];
        for i in 0..board_size - 1 {
            board[i][i] = Some(1);
        }
        board[board_size - 1][board_size - 1] = Some(2);
        assert_eq!(check_win_condition(&board, board_size), None);

        // Test near win in counter diagonal
        board = vec![vec![None; board_size]; board_size];
        for i in 0..board_size - 1 {
            board[i][board_size - 1 - i] = Some(2);
        }
        board[board_size - 1][0] = Some(1);
        assert_eq!(check_win_condition(&board, board_size), None);
    }

    #[test]
    fn test_different_board_sizes() {
        // Test 3x3 board
        let board_size = 3;
        let mut board = vec![vec![None; board_size]; board_size];
        
        // Test row win on 3x3
        board[0] = vec![Some(1), Some(1), Some(1)];
        assert_eq!(check_win_condition(&board, board_size), Some(1));
        
        // Test 5x5 board
        let board_size = 5;
        let mut board = vec![vec![None; board_size]; board_size];
        
        // Test row win on 5x5
        board[0] = vec![Some(2), Some(2), Some(2), Some(2), Some(2)];
        assert_eq!(check_win_condition(&board, board_size), Some(2));
        
        // Test column win on 5x5
        board = vec![vec![None; board_size]; board_size];
        for row in 0..board_size {
            board[row][0] = Some(1);
        }
        assert_eq!(check_win_condition(&board, board_size), Some(1));
        
        // Test main diagonal win on 5x5
        board = vec![vec![None; board_size]; board_size];
        for i in 0..board_size {
            board[i][i] = Some(2);
        }
        assert_eq!(check_win_condition(&board, board_size), Some(2));
        
        // Test counter diagonal win on 5x5
        board = vec![vec![None; board_size]; board_size];
        for i in 0..board_size {
            board[i][board_size - 1 - i] = Some(1);
        }
        assert_eq!(check_win_condition(&board, board_size), Some(1));
    }

    #[test]
    fn test_draw_conditions() {
        // Test 4x4 draw
        let board_size = 4;
        let mut board = vec![vec![None; board_size]; board_size];
        
        // Fill board with a pattern that doesn't create a win
        // 1 2 2 1
        // 2 1 1 2
        // 1 2 2 1
        // 2 1 1 2
        let pattern = vec![
            vec![1, 2, 2, 1],
            vec![2, 1, 1, 2],
            vec![1, 2, 2, 1],
            vec![2, 1, 1, 2],
        ];
        for y in 0..board_size {
            for x in 0..board_size {
                board[y][x] = Some(pattern[y][x]);
            }
        }
        assert_eq!(check_win_condition(&board, board_size), None);
        
        // Test 5x5 draw
        let board_size = 5;
        let mut board = vec![vec![None; board_size]; board_size];
        
        // Fill board with a pattern that doesn't create a win
        // 1 2 2 1 2
        // 2 1 1 2 1
        // 1 2 2 1 2
        // 2 1 1 2 1
        // 1 2 2 1 2
        let pattern = vec![
            vec![1, 2, 2, 1, 2],
            vec![2, 1, 1, 2, 1],
            vec![1, 2, 2, 1, 2],
            vec![2, 1, 1, 2, 1],
            vec![1, 2, 2, 1, 2],
        ];
        for y in 0..board_size {
            for x in 0..board_size {
                board[y][x] = Some(pattern[y][x]);
            }
        }
        assert_eq!(check_win_condition(&board, board_size), None);
    }

    #[test]
    fn test_edge_cases() {
        // Test empty board
        let board_size = 4;
        let board = vec![vec![None; board_size]; board_size];
        assert_eq!(check_win_condition(&board, board_size), None);
        
        // Test board with only one move
        let mut board = vec![vec![None; board_size]; board_size];
        board[0][0] = Some(1);
        assert_eq!(check_win_condition(&board, board_size), None);
        
        // Test board with scattered moves (no win)
        board = vec![vec![None; board_size]; board_size];
        // Create a pattern that doesn't form any winning lines
        // 1 2 _ _
        // 2 1 _ _
        // _ _ 2 1
        // _ _ 1 2
        board[0][0] = Some(1);
        board[0][1] = Some(2);
        board[1][0] = Some(2);
        board[1][1] = Some(1);
        board[2][2] = Some(2);
        board[2][3] = Some(1);
        board[3][2] = Some(1);
        board[3][3] = Some(2);
        assert_eq!(check_win_condition(&board, board_size), None);
        
        // Test board with almost full board but no win
        board = vec![vec![None; board_size]; board_size];
        // Fill with a pattern that doesn't create a win
        // 1 2 2 1
        // 2 1 1 2
        // 1 2 2 1
        // 2 _ 1 2
        let pattern = vec![
            vec![1, 2, 2, 1],
            vec![2, 1, 1, 2],
            vec![1, 2, 2, 1],
            vec![2, 0, 1, 2],  // 0 represents empty cell
        ];
        for y in 0..board_size {
            for x in 0..board_size {
                if pattern[y][x] != 0 {
                    board[y][x] = Some(pattern[y][x]);
                }
            }
        }
        assert_eq!(check_win_condition(&board, board_size), None);
    }
} 