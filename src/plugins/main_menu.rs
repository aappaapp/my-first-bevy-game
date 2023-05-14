use crate::state::AppState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(update.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(cleanup.in_schedule(OnExit(AppState::MainMenu)));
    }
}

fn cleanup(mut commands: Commands, ui_query: Query<Entity, With<Node>>) {
    for ui in ui_query.iter() {
        commands.entity(ui).despawn();
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::width(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::width(Val::Percent(50.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Start Game!",
                            TextStyle {
                                color: Color::BLACK,
                                font: asset_server.load("NotoSansTC-Regular.otf"),
                                font_size: 50.0,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn update(button_query: Query<&Interaction, With<Button>>, mut state: ResMut<NextState<AppState>>) {
    for interaction in button_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::InGame);
            }
            _ => (),
        }
    }
}
