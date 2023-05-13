#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;
mod types;
mod ui;

use bevy::{prelude::*, window::WindowResolution};
use types::AppState;

fn main() {
    App::new()
        .add_state::<AppState>()
        // .add_plugin(bevy_egui::EguiPlugin)
        .add_plugin(game::Plug)
        .add_plugin(ui::Plug)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(360., 640.),
                ..default()
            }),
            ..default()
        }))
        .run();
}
