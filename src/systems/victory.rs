use bevy::prelude::*;
use crate::components::{GameState, VictoryText, ResetButton, Piece, BoardState, CurrentPlayer};

pub fn handle_victory_state(
    mut commands: Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    victory_query: Query<Entity, With<VictoryText>>,
    reset_button_query: Query<Entity, With<ResetButton>>,
) {
    // Only run this system when the game just ended
    if game_state.is_changed() && game_state.game_over {
        // Remove any existing victory text and reset button
        for entity in victory_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in reset_button_query.iter() {
            commands.entity(entity).despawn_recursive();
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

        // Spawn reset button
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(400.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },
                ResetButton,
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Play Again",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
    }
}

pub fn handle_reset_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetButton>)
    >,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut board_state: ResMut<BoardState>,
    mut current_player: ResMut<CurrentPlayer>,
    pieces_query: Query<Entity, With<Piece>>,
    victory_text_query: Query<Entity, With<VictoryText>>,
    reset_button_query: Query<Entity, With<ResetButton>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Reset game state
                game_state.game_over = false;
                game_state.winner = None;
                board_state.board = [[None; 3]; 3];
                current_player.0 = 1;

                // Remove all pieces
                for entity in pieces_query.iter() {
                    commands.entity(entity).despawn();
                }

                // Remove victory text and reset button
                for entity in victory_text_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                for entity in reset_button_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                // Change button color when pressed
                *color = Color::rgb(0.35, 0.35, 0.35).into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
} 