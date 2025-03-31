use bevy::prelude::*;
use crate::constants::*;
use crate::components::{Square, CurrentPlayer, BoardState, Piece, GameState, VictoryText, ResetButton, HoverSquare};

pub fn spawn_board(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();
    let board_size = window.resolution.height() - PADDING * 2.0;
    let square_size = board_size / 3.0;

    // Create the board squares
    for y in 0..3 {
        for x in 0..3 {
            let pos_x = (x as f32 - 1.0) * square_size;
            let pos_y = (y as f32 - 1.0) * square_size;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(square_size, square_size)),
                        color: Color::rgb(0.1, 0.1, 0.1),
                        ..default()
                    },
                    transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                    ..default()
                },
                Square { x: x as u8, y: y as u8 },
            ));
        }
    }

    // Add grid lines
    let line_thickness = 4.0;
    let line_color = Color::rgb(0.2, 0.2, 0.2);

    // Vertical lines
    for x in 1..3 {
        let pos_x = (x as f32 - 1.5) * square_size;
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(line_thickness, board_size)),
                color: line_color,
                ..default()
            },
            transform: Transform::from_xyz(pos_x, 0.0, 0.1),
            ..default()
        });
    }

    // Horizontal lines
    for y in 1..3 {
        let pos_y = (y as f32 - 1.5) * square_size;
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(board_size, line_thickness)),
                color: line_color,
                ..default()
            },
            transform: Transform::from_xyz(0.0, pos_y, 0.1),
            ..default()
        });
    }

    // Create hover highlight square
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(square_size, square_size)),
                color: Color::rgba(0.3, 0.3, 0.3, 0.5),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.5),
            visibility: Visibility::Hidden,
            ..default()
        },
        HoverSquare,
    ));
}

pub fn handle_hover(
    windows: Query<&Window>,
    mut hover_query: Query<(&mut Transform, &mut Visibility), With<HoverSquare>>,
    square_query: Query<(&Square, &Transform), Without<HoverSquare>>,
) {
    let window = windows.single();
    let board_size = window.resolution.height() - PADDING * 2.0;
    let square_size = board_size / 3.0;

    if let Some(cursor_pos) = window.cursor_position() {
        let board_pos = Vec2::new(
            cursor_pos.x - window.resolution.width() / 2.0,
            -(cursor_pos.y - window.resolution.height() / 2.0),
        );

        let (mut hover_transform, mut hover_visibility) = hover_query.single_mut();
        let mut found_square = false;

        for (_, transform) in square_query.iter() {
            let square_pos = Vec2::new(transform.translation.x, transform.translation.y);
            if board_pos.distance(square_pos) < square_size / 2.0 {
                hover_transform.translation.x = square_pos.x;
                hover_transform.translation.y = square_pos.y;
                *hover_visibility = Visibility::Visible;
                found_square = true;
                break;
            }
        }

        if !found_square {
            *hover_visibility = Visibility::Hidden;
        }
    }
}

