mod game;
mod pipe;
mod player;

use bevy::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(game::Plug);
    }
}
