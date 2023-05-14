#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plugins;
mod state;

use self::{
    plugins::{game::GamePlugin, main_menu::MainMenuPlugin},
    state::AppState,
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_system(setup.on_startup())
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
