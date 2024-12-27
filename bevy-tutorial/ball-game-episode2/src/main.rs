use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player))
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Sprite::from_image(
            asset_server.load("sprites/ball_blue_large.png"),
        ),
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
