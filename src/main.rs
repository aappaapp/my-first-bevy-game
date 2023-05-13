#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;

use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(360., 640.),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(game::Plug)
        .run();
}
