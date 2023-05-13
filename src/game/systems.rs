use super::player;
use bevy::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_plugin(player::Plug);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
