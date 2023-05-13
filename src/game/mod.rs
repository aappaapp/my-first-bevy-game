mod player;
mod systems;

use bevy::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(systems::Plug);
    }
}
