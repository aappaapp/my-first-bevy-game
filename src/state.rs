use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
