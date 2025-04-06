use bevy::prelude::*;
use crate::constants::*;
use crate::components::{Square, CurrentPlayer, BoardState, Piece, GameState, HoverSquare};

pub fn check_win_condition(board: &[[Option<u8>; 4]; 4]) -> Option<u8> {
    // Check rows
    for row in board.iter() {
        if row[0] == row[1] && row[1] == row[2] && row[2] == row[3] {
            if let Some(player) = row[0] {
                return Some(player);
            }
        }
    }

    // Check columns
    for col in 0..4 {
        if board[0][col] == board[1][col] && board[1][col] == board[2][col] && board[2][col] == board[3][col]{
            if let Some(player) = board[0][col] {
                return Some(player);
            }
        }
    }

    // Check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[2][2] == board[3][3] {
        if let Some(player) = board[1][1] {
            return Some(player);
        }
    }
    if board[0][3] == board[1][2] && board[1][2] == board[2][1] && board[2][1] == board[3][0] {
        if let Some(player) = board[1][2] {
            return Some(player);
        }
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
    let square_size = board_size / 4.0;
    
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

    // Helper function to create an empty board
    fn empty_board() -> [[Option<u8>; 4]; 4] {
        [[None; 4]; 4]
    }

    // Helper function to create a board with a specific pattern
    fn create_board(pattern: &[&[Option<u8>]]) -> [[Option<u8>; 4]; 4] {
        let mut board = empty_board();
        for (i, row) in pattern.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if i < 4 && j < 4 {
                    board[i][j] = cell;
                }
            }
        }
        board
    }

    #[test]
    fn test_no_win() {
        let board = empty_board();
        assert_eq!(check_win_condition(&board), None);
    }

    #[test]
    fn test_row_win() {
        // Test each row for a win
        for row in 0..4 {
            let mut board = empty_board();
            board[row] = [Some(1), Some(1), Some(1), Some(1)];
            assert_eq!(check_win_condition(&board), Some(1));
        }
    }

    #[test]
    fn test_column_win() {
        // Test each column for a win
        for col in 0..4 {
            let mut board = empty_board();
            for row in 0..4 {
                board[row][col] = Some(1);
            }
            assert_eq!(check_win_condition(&board), Some(1));
        }
    }

    #[test]
    fn test_diagonal_win() {
        // Test main diagonal (top-left to bottom-right)
        let mut board = empty_board();
        for i in 0..4 {
            board[i][i] = Some(1);
        }
        assert_eq!(check_win_condition(&board), Some(1));

        // Test counter diagonal (top-right to bottom-left)
        let mut board = empty_board();
        for i in 0..4 {
            board[i][3-i] = Some(1);
        }
        assert_eq!(check_win_condition(&board), Some(1));
    }

    #[test]
    fn test_player_2_win() {
        // Test that player 2 can win
        let mut board = empty_board();
        for i in 0..4 {
            board[i][i] = Some(2);
        }
        assert_eq!(check_win_condition(&board), Some(2));
    }

    #[test]
    fn test_partial_board() {
        // Test a partially filled board with no win
        // The previous pattern had a diagonal win for player 1
        // Let's create a new pattern that doesn't have any win conditions
        let board = create_board(&[
            &[Some(1), Some(2), None, None],
            &[None, Some(2), Some(1), None],
            &[None, None, Some(2), Some(1)],
            &[Some(2), None, None, Some(2)],
        ]);
        assert_eq!(check_win_condition(&board), None);
    }

    #[test]
    fn test_almost_win() {
        // Test a board that's almost a win but not quite
        let mut board = empty_board();
        for i in 0..3 {
            board[i][i] = Some(1);
        }
        assert_eq!(check_win_condition(&board), None);
    }

    #[test]
    fn test_draw_game() {
        // Test a full board with no winner
        // Adjust the pattern to avoid any winning combinations
        let board = create_board(&[
            &[Some(1), Some(2), Some(1), Some(2)],
            &[Some(2), Some(1), Some(2), Some(1)],
            &[Some(2), Some(1), Some(2), Some(1)],
            &[Some(1), Some(2), Some(1), Some(2)],
        ]);
        assert_eq!(check_win_condition(&board), None);
    }

    #[test]
    fn test_alternating_pattern() {
        // Test a pattern that alternates players but doesn't result in a win
        let board = create_board(&[
            &[Some(1), Some(2), Some(1), None],
            &[Some(2), Some(1), Some(2), None],
            &[Some(2), Some(1), Some(2), None],
            &[Some(1), Some(2), Some(1), None],
        ]);
        assert_eq!(check_win_condition(&board), None);
    }

    #[test]
    fn test_near_wins() {
        // Test near-win in a row
        let mut board = empty_board();
        board[0] = [Some(1), Some(1), Some(1), Some(2)];
        assert_eq!(check_win_condition(&board), None);

        // Test near-win in a column
        let mut board = empty_board();
        for i in 0..3 {
            board[i][0] = Some(1);
        }
        board[3][0] = Some(2);
        assert_eq!(check_win_condition(&board), None);

        // Test near-win in main diagonal
        let mut board = empty_board();
        for i in 0..3 {
            board[i][i] = Some(1);
        }
        board[3][3] = Some(2);
        assert_eq!(check_win_condition(&board), None);

        // Test near-win in counter diagonal
        let mut board = empty_board();
        for i in 0..3 {
            board[i][3-i] = Some(1);
        }
        board[3][0] = Some(2);
        assert_eq!(check_win_condition(&board), None);
    }

    #[test]
    fn test_immediate_win_detection() {
        // Test that a win is detected as soon as it occurs
        let mut board = empty_board();
        
        // Set up almost-win condition
        for i in 0..3 {
            board[0][i] = Some(1);
        }
        assert_eq!(check_win_condition(&board), None);
        
        // Complete the win
        board[0][3] = Some(1);
        assert_eq!(check_win_condition(&board), Some(1));
    }
} 