use bevy::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system(movement);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(50.0, 50.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
    mut y_speed: Local<f32>,
) {
    for (_player, mut transform) in query.iter_mut() {
        *y_speed -= 10.0;
        if input.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
            *y_speed = 500.0;
        }
        transform.translation.y += *y_speed * time.delta_seconds();
    }
}
