use bevy::prelude::*;
// use rand::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_pipe).add_system(movement);
    }
}

#[derive(Component)]
pub struct Pipe;

fn spawn_pipe(mut commands: Commands, time: Res<Time>, mut t: Local<f32>) {
    if *t < 0.0 {
        *t = 3.0;
        let offset = (rand::random::<f32>() * 320.0).floor() - 160.0;
        commands.spawn((
            Pipe,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(50.0, 640.0, 50.0),
                    translation: Vec3::new(180.0, 420.0 + offset, 0.0),
                    ..default()
                },
                ..default()
            },
        ));
        commands.spawn((
            Pipe,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(50.0, 640.0, 50.0),
                    translation: Vec3::new(180.0, -420.0 + offset, 0.0),
                    ..default()
                },
                ..default()
            },
        ));
    }
    *t -= time.delta_seconds();
}

fn movement(mut pipe_query: Query<(&Pipe, &mut Transform)>, time: Res<Time>) {
    for (_pipe, mut transform) in pipe_query.iter_mut() {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }
}
