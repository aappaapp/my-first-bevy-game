use crate::types::AppState;
use bevy::prelude::*;
use bevy_egui::egui;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(update_menu.in_set(OnUpdate(AppState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::Menu)));
    }
}

fn setup_menu() {
    egui::Window::new("Test");
    // commands.spawn();
}

fn update_menu() {}

fn cleanup_menu() {}
