use bevy::prelude::*;
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
pub const NUMBER_OF_ENEMIES: usize = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
      Sprite::from_image(
            asset_server.load("sprites/ball_blue_large.png"),
        ),
        Player,
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(
      Camera2d
    );
}

pub fn spawn_enemies(
    mut commands: Commands,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.single();

    let half_enemy_size = PLAYER_SIZE / 2.0; 
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width() - window.width() / 2.0 + half_enemy_size;
        let random_y = random::<f32>() * window.height() - window.height() / 2.0 + half_enemy_size;

        commands.spawn((
            Sprite::from_image(
              asset_server.load("sprites/ball_red_large.png"),
            ),
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy,
        ));
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta().as_secs_f32();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    windows: Query<&Window>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = windows.single();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = - window.width() / 2.0 + half_player_size;
        let x_max = window.width() / 2.0 - half_player_size;
        let y_min = - window.height() / 2.0 + half_player_size;
        let y_max = window.height() / 2.0 - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the player's y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

