mod pipe;
mod player;

use crate::types::AppState;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use pipe::Pipe;
use player::Player;

use self::player::setup_player;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(pipe::Plug)
            .add_plugin(player::Plug)
            .add_system(cleanup_game.in_schedule(OnExit(AppState::Game)))
            .add_system(
                setup_game
                    .before(setup_player)
                    .in_schedule(OnEnter(AppState::Game)),
            )
            .add_system(update_game.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Resource)]
pub struct GameData {
    game_entity: Entity,
}

#[derive(Component)]
struct Game;

pub fn cleanup_game(mut commands: Commands, game_data: Res<GameData>) {
    commands.entity(game_data.game_entity).despawn_recursive();
}

pub fn setup_game(mut commands: Commands) {
    let camera = commands.spawn(Camera2dBundle { ..default() }).id();
    let game = commands.spawn(Game).add_child(camera).id();
    commands.insert_resource(GameData { game_entity: game })
}

pub fn update_game(
    pipe_query: Query<&Transform, With<Pipe>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for player in player_query.iter() {
        for pipe in pipe_query.iter() {
            if collide(
                player.translation,
                Vec2::new(player.scale.x, player.scale.y),
                pipe.translation,
                Vec2::new(pipe.scale.x, pipe.scale.y),
            )
            .is_some()
            {
                println!("ded");
            }
        }
    }
}
