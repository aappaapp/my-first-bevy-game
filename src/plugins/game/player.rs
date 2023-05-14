use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn update(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
    mut y_speed: Local<f32>,
) {
    for (_player, mut transform) in query.iter_mut() {
        *y_speed -= 10.0;
        if input.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
            *y_speed = 250.0;
        }
        transform.translation.y += *y_speed * time.delta_seconds();
    }
}
