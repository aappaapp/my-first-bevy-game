use super::{setup_game, GameData};
use crate::types::AppState;
use bevy::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system(
            setup_player
                .after(setup_game)
                .in_schedule(OnEnter(AppState::Game)),
        )
        .add_system(update_player);
    }
}

#[derive(Component)]
pub struct Player;

pub fn setup_player(mut commands: Commands, game_data: Res<GameData>) {
    let player = commands
        .spawn((
            Player,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(50.0, 50.0, 0.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    commands.entity(game_data.game_entity).add_child(player);
}

pub fn update_player(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
    mut y_speed: Local<f32>,
) {
    for (_player, mut transform) in query.iter_mut() {
        *y_speed -= 10.0;
        if input.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
            *y_speed = 500.0;
        }
        transform.translation.y += *y_speed * time.delta_seconds();
    }
}
