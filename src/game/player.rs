use bevy::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system(log);
    }
}

#[derive(Component)]
struct PlayerBundle {
    pub sprite: SpriteBundle,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        sprite: SpriteBundle { ..default() },
    });
}

fn log(queries: Query<&Transform>) {
    for query in queries.iter() {
        println!("{}", query.local_x());
    }
}
