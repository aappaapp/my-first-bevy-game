mod pipe;
mod player;

use crate::state::AppState;
use bevy::prelude::*;

use self::player::Player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_systems((update, self::player::update).in_set(OnUpdate(AppState::InGame)))
            .add_system(cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Game;

pub fn cleanup(mut commands: Commands, game_query: Query<Entity, With<Game>>) {
    for game in game_query.iter() {
        commands.entity(game).despawn_recursive();
    }
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn((Game, SpatialBundle { ..default() }))
        .with_children(|parent| {
            parent.spawn(Camera2dBundle { ..default() });
            parent.spawn((
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
            ));
        });
}

pub fn update() {}
