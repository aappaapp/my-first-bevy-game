use super::{
    pipe::{self, Pipe},
    player::{self, Player},
};
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(player::Plug)
            .add_plugin(pipe::Plug)
            .add_system(check_death);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

fn check_death(
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
